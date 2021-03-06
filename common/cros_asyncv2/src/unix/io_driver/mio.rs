// Copyright 2021 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    alloc::Layout,
    cell::RefCell,
    cmp::min,
    convert::TryFrom,
    future::Future,
    io,
    mem::{align_of, replace, size_of, MaybeUninit},
    os::{raw::c_int, unix::io::RawFd},
    pin::Pin,
    ptr::{null_mut, NonNull},
    rc::Rc,
    sync::Arc,
    task,
    time::Duration,
};

use anyhow::{anyhow, bail, ensure, Context};
use data_model::IoBufMut;
use mio::{unix::SourceFd, Events, Interest, Token};
use once_cell::unsync::{Lazy, OnceCell};
use slab::Slab;
use sys_util::{
    add_fd_flags, error, AsRawDescriptor, FromRawDescriptor, LayoutAllocation, SafeDescriptor,
};
use thiserror::Error as ThisError;

use crate::AsIoBufs;

// Tokens assigned to pending operations are based on their index in a Slab and we should run out of
// memory well before we end up with `usize::MAX` pending operations.
const WAKER_TOKEN: Token = Token(usize::MAX);

struct DriverState {
    poll: mio::Poll,
    events: Events,
}

impl DriverState {
    fn new() -> anyhow::Result<RefCell<DriverState>> {
        let poll = mio::Poll::new().context("failed to create `mio::Poll`")?;

        Ok(RefCell::new(DriverState {
            poll,
            events: Events::with_capacity(128),
        }))
    }
}
thread_local! (static DRIVER: OnceCell<RefCell<DriverState>> = OnceCell::new());

fn with_driver<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce(&mut DriverState) -> anyhow::Result<R>,
{
    DRIVER.with(|driver_cell| {
        let driver = driver_cell.get_or_try_init(DriverState::new)?;
        f(&mut driver.borrow_mut())
    })
}

pub fn new_waker() -> anyhow::Result<mio::Waker> {
    with_driver(|driver| {
        mio::Waker::new(driver.poll.registry(), WAKER_TOKEN)
            .context("failed to create `mio::Waker`")
    })
}

// Wait for more events.
pub fn wait(timeout: Option<Duration>) -> anyhow::Result<()> {
    with_driver(|driver| {
        driver
            .poll
            .poll(&mut driver.events, timeout)
            .context("failed to poll for events")
    })
}

// Wake up any tasks that are ready.
pub fn dispatch() -> anyhow::Result<()> {
    with_driver(|driver| {
        OPS.with(|ops| {
            let mut ops = ops.borrow_mut();
            for event in driver
                .events
                .into_iter()
                .filter(|e| e.token() != WAKER_TOKEN)
            {
                let token = event.token();
                let op = ops.get_mut(token.0).ok_or(InvalidToken(token.0))?;
                match replace(op, OpStatus::Ready) {
                    OpStatus::New => {}
                    OpStatus::Waiting(waker) => waker.wake(),
                    // When under heavy load the Executor will try to fetch more events even when it
                    // hasn't finished processing its run queue in order to prevent starvation of
                    // tasks waiting on IO. So it's not unreasonable to see readiness more than
                    // once. That just means the Executor hasn't been able to poll the task waiting
                    // on the IO yet.
                    OpStatus::Ready => {}
                }
            }
            Ok(())
        })
    })
}

enum OpStatus {
    New,
    Waiting(task::Waker),
    Ready,
}
thread_local! (static OPS: Lazy<Rc<RefCell<Slab<OpStatus>>>> = Lazy::new(Default::default));

#[derive(Debug, ThisError)]
#[error("Invalid token in events: {0}")]
struct InvalidToken(usize);

struct Op<'a> {
    ops: Rc<RefCell<Slab<OpStatus>>>,
    desc: &'a Arc<SafeDescriptor>,
    idx: usize,
}

impl<'a> Future for Op<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Self::Output> {
        let mut ops = self.ops.borrow_mut();
        let status = ops.get_mut(self.idx).expect("`OpStatus` missing");

        match status {
            OpStatus::New => *status = OpStatus::Waiting(cx.waker().clone()),
            OpStatus::Waiting(w) if !w.will_wake(cx.waker()) => {
                *status = OpStatus::Waiting(cx.waker().clone())
            }
            // If `cx.waker()` and the currently stored waker are the same then no need to do
            // anything.
            OpStatus::Waiting(_) => {}
            OpStatus::Ready => {
                return task::Poll::Ready(());
            }
        }

        task::Poll::Pending
    }
}

impl<'a> Drop for Op<'a> {
    fn drop(&mut self) {
        let mut ops = self.ops.borrow_mut();
        ops.remove(self.idx);

        let res = with_driver(|driver| {
            driver
                .poll
                .registry()
                .deregister(&mut SourceFd(&self.desc.as_raw_descriptor()))
                .context("Failed to deregister descriptor")
        });

        if let Err(e) = res {
            error!("{}", e);
        }
    }
}

async fn wait_for(desc: &Arc<SafeDescriptor>, interest: Interest) -> anyhow::Result<()> {
    let ops = OPS.with(|ops| Rc::clone(ops));
    let idx = {
        let mut ops_ref = ops.borrow_mut();
        let entry = ops_ref.vacant_entry();
        let idx = entry.key();
        with_driver(|driver| {
            driver
                .poll
                .registry()
                .register(
                    &mut SourceFd(&desc.as_raw_descriptor()),
                    Token(idx),
                    interest,
                )
                .context("failed to register interest for descriptor")
        })?;

        entry.insert(OpStatus::New);
        idx
    };
    Op { ops, desc, idx }.await;
    Ok(())
}

pub async fn read(
    desc: &Arc<SafeDescriptor>,
    buf: &mut [u8],
    offset: Option<u64>,
) -> anyhow::Result<usize> {
    loop {
        // Safe because this will only modify `buf` and we check the return value.
        let res = if let Some(off) = offset {
            unsafe {
                libc::pread64(
                    desc.as_raw_descriptor(),
                    buf.as_mut_ptr() as *mut libc::c_void,
                    buf.len(),
                    off as libc::off64_t,
                )
            }
        } else {
            unsafe {
                libc::read(
                    desc.as_raw_descriptor(),
                    buf.as_mut_ptr() as *mut libc::c_void,
                    buf.len(),
                )
            }
        };

        if res >= 0 {
            return Ok(res as usize);
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK => wait_for(desc, Interest::READABLE).await?,
            e => return Err(io::Error::from_raw_os_error(e.errno()).into()),
        }
    }
}

pub async fn read_iobuf<B: AsIoBufs + 'static>(
    desc: &Arc<SafeDescriptor>,
    mut buf: B,
    offset: Option<u64>,
) -> (anyhow::Result<usize>, B) {
    loop {
        // Safe because this will only modify `buf` and we check the return value.
        let res = {
            let iovecs = IoBufMut::as_iobufs(buf.as_iobufs());
            if let Some(off) = offset {
                unsafe {
                    libc::preadv64(
                        desc.as_raw_descriptor(),
                        iovecs.as_ptr(),
                        iovecs.len() as c_int,
                        off as libc::off64_t,
                    )
                }
            } else {
                unsafe {
                    libc::readv(
                        desc.as_raw_descriptor(),
                        iovecs.as_ptr(),
                        iovecs.len() as c_int,
                    )
                }
            }
        };

        if res >= 0 {
            return (Ok(res as usize), buf);
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK => {
                if let Err(e) = wait_for(desc, Interest::READABLE).await {
                    return (Err(e), buf);
                }
            }
            e => return (Err(io::Error::from_raw_os_error(e.errno()).into()), buf),
        }
    }
}

pub async fn write(
    desc: &Arc<SafeDescriptor>,
    buf: &[u8],
    offset: Option<u64>,
) -> anyhow::Result<usize> {
    loop {
        // Safe because this will only modify `buf` and we check the return value.
        let res = if let Some(off) = offset {
            unsafe {
                libc::pwrite64(
                    desc.as_raw_descriptor(),
                    buf.as_ptr() as *const libc::c_void,
                    buf.len(),
                    off as libc::off64_t,
                )
            }
        } else {
            unsafe {
                libc::write(
                    desc.as_raw_descriptor(),
                    buf.as_ptr() as *const libc::c_void,
                    buf.len(),
                )
            }
        };

        if res >= 0 {
            return Ok(res as usize);
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK => wait_for(desc, Interest::WRITABLE).await?,
            e => return Err(io::Error::from_raw_os_error(e.errno()).into()),
        }
    }
}

pub async fn write_iobuf<B: AsIoBufs + 'static>(
    desc: &Arc<SafeDescriptor>,
    mut buf: B,
    offset: Option<u64>,
) -> (anyhow::Result<usize>, B) {
    loop {
        // Safe because this will only modify `buf` and we check the return value.
        let res = {
            let iovecs = IoBufMut::as_iobufs(buf.as_iobufs());
            if let Some(off) = offset {
                unsafe {
                    libc::pwritev64(
                        desc.as_raw_descriptor(),
                        iovecs.as_ptr(),
                        iovecs.len() as c_int,
                        off as libc::off64_t,
                    )
                }
            } else {
                unsafe {
                    libc::writev(
                        desc.as_raw_descriptor(),
                        iovecs.as_ptr(),
                        iovecs.len() as c_int,
                    )
                }
            }
        };

        if res >= 0 {
            return (Ok(res as usize), buf);
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK => {
                if let Err(e) = wait_for(desc, Interest::WRITABLE).await {
                    return (Err(e), buf);
                }
            }
            e => return (Err(io::Error::from_raw_os_error(e.errno()).into()), buf),
        }
    }
}

pub async fn fallocate(
    desc: &Arc<SafeDescriptor>,
    file_offset: u64,
    len: u64,
    mode: u32,
) -> anyhow::Result<()> {
    let ret = unsafe {
        libc::fallocate64(
            desc.as_raw_descriptor(),
            mode as libc::c_int,
            file_offset as libc::off64_t,
            len as libc::off64_t,
        )
    };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error().into())
    }
}

pub async fn ftruncate(desc: &Arc<SafeDescriptor>, len: u64) -> anyhow::Result<()> {
    let ret = unsafe { libc::ftruncate64(desc.as_raw_descriptor(), len as libc::off64_t) };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error().into())
    }
}

pub async fn stat(desc: &Arc<SafeDescriptor>) -> anyhow::Result<libc::stat64> {
    let mut st = MaybeUninit::zeroed();

    // Safe because this will only modify `st` and we check the return value.
    let ret = unsafe { libc::fstat64(desc.as_raw_descriptor(), st.as_mut_ptr()) };

    if ret == 0 {
        // Safe because the kernel guarantees that `st` is now initialized.
        Ok(unsafe { st.assume_init() })
    } else {
        Err(io::Error::last_os_error().into())
    }
}

pub async fn fsync(desc: &Arc<SafeDescriptor>, datasync: bool) -> anyhow::Result<()> {
    // TODO: If there is a lot of buffered data then this may take a long time and block the thread.
    // Consider offloading to a BlockingPool if this becomes an issue.
    let ret = unsafe {
        if datasync {
            libc::fdatasync(desc.as_raw_descriptor())
        } else {
            libc::fsync(desc.as_raw_descriptor())
        }
    };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error().into())
    }
}

pub async fn connect(
    desc: &Arc<SafeDescriptor>,
    addr: libc::sockaddr_un,
    len: libc::socklen_t,
) -> anyhow::Result<()> {
    ensure!(
        len <= size_of::<libc::sockaddr_un>() as libc::socklen_t,
        io::Error::from_raw_os_error(libc::EINVAL)
    );

    // Safe because this will only read `len` bytes from `addr`, does not modify any memory, and we
    // check the return value.
    let ret =
        unsafe { libc::connect(desc.as_raw_descriptor(), &addr as *const _ as *const _, len) };
    if ret >= 0 {
        return Ok(());
    }

    match sys_util::Error::last() {
        e if e.errno() == libc::EWOULDBLOCK || e.errno() == libc::EINPROGRESS => {
            wait_for(desc, Interest::WRITABLE).await?;

            let mut result: c_int = 0;
            let mut result_len = size_of::<c_int>() as libc::socklen_t;
            // Safe because this will only modify `result` and we check the return value.
            let ret = unsafe {
                libc::getsockopt(
                    desc.as_raw_descriptor(),
                    libc::SOL_SOCKET,
                    libc::SO_ERROR,
                    &mut result as *mut _ as *mut libc::c_void,
                    &mut result_len,
                )
            };
            if ret < 0 {
                bail!(io::Error::last_os_error());
            }

            if result == 0 {
                Ok(())
            } else {
                Err(anyhow!(io::Error::from_raw_os_error(result)))
            }
        }
        e => Err(anyhow!(io::Error::from(e))),
    }
}

pub async fn next_packet_size(desc: &Arc<SafeDescriptor>) -> anyhow::Result<usize> {
    #[cfg(not(debug_assertions))]
    let buf = null_mut();
    // Work around for qemu's syscall translation which will reject null pointers in recvfrom.
    // This only matters for running the unit tests for a non-native architecture. See the
    // upstream thread for the qemu fix:
    // https://lists.nongnu.org/archive/html/qemu-devel/2021-03/msg09027.html
    #[cfg(debug_assertions)]
    let buf = NonNull::dangling().as_ptr();

    loop {
        // Safe because this will not modify any memory and we check the return value.
        let ret = unsafe {
            libc::recvfrom(
                desc.as_raw_descriptor(),
                buf,
                0,
                libc::MSG_TRUNC | libc::MSG_PEEK | libc::MSG_DONTWAIT,
                null_mut(),
                null_mut(),
            )
        };

        if ret >= 0 {
            return Ok(ret as usize);
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK || e.errno() == libc::EAGAIN => {
                wait_for(desc, Interest::READABLE).await?;
            }
            e => bail!(io::Error::from(e)),
        }
    }
}

// Allocates a buffer to hold a `libc::cmsghdr` with `cap` bytes of data.
//
// Returns the `LayoutAllocation` for the buffer as well as the size of the allocation, which is
// guaranteed to be at least `size_of::<libc::cmsghdr>() + cap` bytes.
fn allocate_cmsg_buffer(cap: u32) -> anyhow::Result<(LayoutAllocation, usize)> {
    // Not sure why this is unsafe.
    let cmsg_cap = usize::try_from(unsafe { libc::CMSG_SPACE(cap) })
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let alloc = Layout::from_size_align(cmsg_cap, align_of::<libc::cmsghdr>())
        .map(LayoutAllocation::zeroed)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok((alloc, cmsg_cap))
}

// Adds a control message with the file descriptors from `fds` to `msg`.
//
// Returns the `LayoutAllocation` backing the control message.
fn add_fds_to_message(msg: &mut libc::msghdr, fds: &[RawFd]) -> anyhow::Result<LayoutAllocation> {
    let fd_len = fds
        .len()
        .checked_mul(size_of::<RawFd>())
        .and_then(|l| u32::try_from(l).ok())
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;

    let (cmsg_buffer, cmsg_cap) = allocate_cmsg_buffer(fd_len)?;
    msg.msg_control = cmsg_buffer.as_ptr();
    msg.msg_controllen = cmsg_cap;

    unsafe {
        // Safety:
        // * CMSG_FIRSTHDR will either return a null pointer or a pointer to `msg.msg_control`.
        // * `msg.msg_control` is properly aligned because `cmsg_buffer` is properly aligned.
        // * The buffer is zeroed, which is a valid bit-pattern for `libc::cmsghdr`.
        // * The reference does not escape this function.
        let cmsg = libc::CMSG_FIRSTHDR(msg).as_mut().unwrap();
        cmsg.cmsg_len = libc::CMSG_LEN(fd_len) as libc::size_t;
        cmsg.cmsg_level = libc::SOL_SOCKET;
        cmsg.cmsg_type = libc::SCM_RIGHTS;

        // Safety: `libc::CMSG_DATA(cmsg)` and `fds` are valid for `fd_len` bytes of memory.
        libc::memcpy(
            libc::CMSG_DATA(cmsg).cast(),
            fds.as_ptr().cast(),
            fd_len as usize,
        );
    }

    Ok(cmsg_buffer)
}

// Copies file descriptors from the control message in `msg` into `fds`.
//
// Returns the number of file descriptors that were copied from `msg`.
fn take_fds_from_message(msg: &libc::msghdr, fds: &mut [RawFd]) -> anyhow::Result<usize> {
    let cap = fds
        .len()
        .checked_mul(size_of::<RawFd>())
        .ok_or_else(|| anyhow!(io::Error::from(io::ErrorKind::InvalidInput)))?;

    let mut rem = cap;
    unsafe {
        let mut cmsg = libc::CMSG_FIRSTHDR(msg);

        // Safety:
        // * CMSG_FIRSTHDR will either return a null pointer or a pointer to `msg.msg_control`.
        // * `msg.msg_control` is properly aligned because it was allocated by `allocate_cmsg_buffer`.
        // * The buffer was zero-initialized, which is a valid bit-pattern for `libc::cmsghdr`.
        // * The reference does not escape this function.
        while let Some(current) = cmsg.as_ref() {
            if current.cmsg_level != libc::SOL_SOCKET || current.cmsg_type != libc::SCM_RIGHTS {
                cmsg = libc::CMSG_NXTHDR(msg, cmsg);
                continue;
            }

            let data_len = min(current.cmsg_len - libc::CMSG_LEN(0) as usize, rem);

            // Safety: `fds` and `libc::CMSG_DATA(cmsg)` are valid for `data_len` bytes of memory.
            libc::memcpy(
                fds.as_mut_ptr().cast(),
                libc::CMSG_DATA(cmsg).cast(),
                data_len,
            );
            rem -= data_len;
            if rem == 0 {
                break;
            }

            cmsg = libc::CMSG_NXTHDR(msg, cmsg);
        }
    }

    Ok((cap - rem) / size_of::<RawFd>())
}

pub async fn sendmsg(
    desc: &Arc<SafeDescriptor>,
    buf: &[u8],
    fds: &[RawFd],
) -> anyhow::Result<usize> {
    let mut iov = libc::iovec {
        iov_base: buf.as_ptr() as *const libc::c_void as *mut libc::c_void,
        iov_len: buf.len() as libc::size_t,
    };

    let mut msg = libc::msghdr {
        msg_name: null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_flags: 0,
        msg_control: null_mut(),
        msg_controllen: 0,
    };

    let _cmsg_buffer = if !fds.is_empty() {
        Some(add_fds_to_message(&mut msg, fds)?)
    } else {
        None
    };

    loop {
        // Safe because this doesn't modify any memory and we check the return value.
        let ret = unsafe {
            libc::sendmsg(
                desc.as_raw_descriptor(),
                &msg,
                libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT,
            )
        };

        if ret >= 0 {
            return Ok(ret as usize);
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK || e.errno() == libc::EAGAIN => {
                wait_for(desc, Interest::WRITABLE).await?;
            }
            e => return Err(anyhow!(io::Error::from(e))),
        }
    }
}

pub async fn recvmsg(
    desc: &Arc<SafeDescriptor>,
    buf: &mut [u8],
    fds: &mut [RawFd],
) -> anyhow::Result<(usize, usize)> {
    let mut iov = libc::iovec {
        iov_base: buf.as_mut_ptr().cast(),
        iov_len: buf.len() as libc::size_t,
    };

    let fd_cap = fds
        .len()
        .checked_mul(size_of::<RawFd>())
        .and_then(|l| u32::try_from(l).ok())
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    let (cmsg_buffer, cmsg_cap) = allocate_cmsg_buffer(fd_cap)?;

    let mut msg = libc::msghdr {
        msg_name: null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_flags: 0,
        msg_control: cmsg_buffer.as_ptr(),
        msg_controllen: cmsg_cap,
    };

    let buflen = loop {
        // Safe because this will only modify `buf` and `cmsg_buffer` and we check the return value.
        let ret = unsafe {
            libc::recvmsg(
                desc.as_raw_descriptor(),
                &mut msg,
                libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT,
            )
        };

        if ret >= 0 {
            break ret as usize;
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK || e.errno() == libc::EAGAIN => {
                wait_for(desc, Interest::READABLE).await?;
            }
            e => return Err(anyhow!(io::Error::from(e))),
        }
    };

    let fd_count = take_fds_from_message(&msg, fds)?;

    Ok((buflen, fd_count))
}

pub async fn send_iobuf_with_fds<B: AsIoBufs + 'static>(
    desc: &Arc<SafeDescriptor>,
    mut buf: B,
    fds: &[RawFd],
) -> (anyhow::Result<usize>, B) {
    let inner = async {
        let iovs = IoBufMut::as_iobufs(buf.as_iobufs());

        let mut msg = libc::msghdr {
            msg_name: null_mut(),
            msg_namelen: 0,
            msg_iov: iovs.as_ptr() as *mut libc::iovec,
            msg_iovlen: iovs.len(),
            msg_flags: 0,
            msg_control: null_mut(),
            msg_controllen: 0,
        };

        let _cmsg_buffer = if !fds.is_empty() {
            Some(add_fds_to_message(&mut msg, fds)?)
        } else {
            None
        };

        loop {
            // Safe because this doesn't modify any memory and we check the return value.
            let ret = unsafe {
                libc::sendmsg(
                    desc.as_raw_descriptor(),
                    &msg,
                    libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT,
                )
            };

            if ret >= 0 {
                return Ok(ret as usize);
            }

            match sys_util::Error::last() {
                e if e.errno() == libc::EWOULDBLOCK || e.errno() == libc::EAGAIN => {
                    wait_for(desc, Interest::WRITABLE).await?;
                }
                e => return Err(anyhow!(io::Error::from(e))),
            }
        }
    };

    (inner.await, buf)
}

pub async fn recv_iobuf_with_fds<B: AsIoBufs + 'static>(
    desc: &Arc<SafeDescriptor>,
    mut buf: B,
    fds: &mut [RawFd],
) -> (anyhow::Result<(usize, usize)>, B) {
    let inner = async {
        let iovs = IoBufMut::as_iobufs(buf.as_iobufs());

        let fd_cap = fds
            .len()
            .checked_mul(size_of::<RawFd>())
            .and_then(|l| u32::try_from(l).ok())
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
        let (cmsg_buffer, cmsg_cap) = allocate_cmsg_buffer(fd_cap)?;

        let mut msg = libc::msghdr {
            msg_name: null_mut(),
            msg_namelen: 0,
            msg_iov: iovs.as_ptr() as *mut libc::iovec,
            msg_iovlen: iovs.len(),
            msg_flags: 0,
            msg_control: cmsg_buffer.as_ptr(),
            msg_controllen: cmsg_cap,
        };

        let buflen = loop {
            // Safe because this will only modify `buf` and `cmsg_buffer` and we check the return value.
            let ret = unsafe {
                libc::recvmsg(
                    desc.as_raw_descriptor(),
                    &mut msg,
                    libc::MSG_NOSIGNAL | libc::MSG_DONTWAIT,
                )
            };

            if ret >= 0 {
                break ret as usize;
            }

            match sys_util::Error::last() {
                e if e.errno() == libc::EWOULDBLOCK || e.errno() == libc::EAGAIN => {
                    wait_for(desc, Interest::READABLE).await?;
                }
                e => return Err(anyhow!(io::Error::from(e))),
            }
        };

        let fd_count = take_fds_from_message(&msg, fds)?;
        Ok((buflen, fd_count))
    };

    (inner.await, buf)
}

pub async fn accept(desc: &Arc<SafeDescriptor>) -> anyhow::Result<SafeDescriptor> {
    loop {
        // Safe because this doesn't modify any memory and we check the return value.
        let ret = unsafe {
            libc::accept4(
                desc.as_raw_descriptor(),
                null_mut(),
                null_mut(),
                libc::SOCK_CLOEXEC,
            )
        };

        if ret >= 0 {
            // Safe because we own this fd.
            return Ok(unsafe { SafeDescriptor::from_raw_descriptor(ret) });
        }

        match sys_util::Error::last() {
            e if e.errno() == libc::EWOULDBLOCK || e.errno() == libc::EAGAIN => {
                wait_for(desc, Interest::READABLE).await?;
            }
            e => return Err(anyhow!(io::Error::from(e))),
        }
    }
}

pub async fn wait_readable(desc: &Arc<SafeDescriptor>) -> anyhow::Result<()> {
    wait_for(desc, Interest::READABLE).await
}

pub async fn wait_writable(desc: &Arc<SafeDescriptor>) -> anyhow::Result<()> {
    wait_for(desc, Interest::WRITABLE).await
}

pub fn prepare(fd: &dyn AsRawDescriptor) -> anyhow::Result<()> {
    add_fd_flags(fd.as_raw_descriptor(), libc::O_NONBLOCK)
        .map_err(io::Error::from)
        .context("failed to make descriptor non-blocking")
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fs::File;
    use std::io::Write;
    use std::os::unix::io::AsRawFd;

    use crate::Executor;

    #[test]
    fn wait_readable() {
        async fn go() {
            let (rx, mut tx) = sys_util::pipe(true).unwrap();

            prepare(&rx).unwrap();
            let rx = Arc::new(SafeDescriptor::from(rx));

            let buf = [0x99u8; 32];
            tx.write_all(&buf[..]).unwrap();

            wait_for(&rx, Interest::READABLE).await.unwrap();

            let mut v = vec![0x55u8; 32];
            let count = read(&rx, &mut v, None).await.unwrap();
            assert_eq!(count, v.len());
            assert!(v.iter().all(|&b| b == 0x99));
        }

        let ex = Executor::new();
        ex.run_until(go()).unwrap();
    }

    #[test]
    fn readpipe() {
        async fn read_from_pipe(ex: &Executor) {
            let (rx, tx) = sys_util::pipe(true).unwrap();
            add_fd_flags(rx.as_raw_fd(), libc::O_NONBLOCK).unwrap();
            let rx = Arc::new(SafeDescriptor::from(rx));

            ex.spawn_local(write_to_pipe(tx)).detach();

            let mut buf = [0x2cu8; 18];
            let count = read(&rx, &mut buf[..], None).await.unwrap();
            assert_eq!(count, buf.len());
            assert!(buf.iter().all(|&b| b == 0x99));
        }

        async fn write_to_pipe(mut tx: File) {
            let buf = [0x99u8; 18];
            tx.write_all(&buf[..]).unwrap();
        }

        let ex = Executor::new();
        ex.run_until(read_from_pipe(&ex)).unwrap();
    }
}
