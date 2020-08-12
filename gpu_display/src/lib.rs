// Copyright 2018 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Crate for displaying simple surfaces and GPU buffers over wayland.

use std::fmt::{self, Display};
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;

use base::Error as SysError;
use data_model::VolatileSlice;

mod event_device;
mod gpu_display_stub;
mod gpu_display_wl;
#[cfg(feature = "x")]
mod gpu_display_x;
mod keycode_converter;

pub use event_device::{EventDevice, EventDeviceKind};

/// An error generated by `GpuDisplay`.
#[derive(Debug)]
pub enum GpuDisplayError {
    /// An internal allocation failed.
    Allocate,
    /// Connecting to the compositor failed.
    Connect,
    /// Creating event file descriptor failed.
    CreateEventFd,
    /// Creating shared memory failed.
    CreateShm(SysError),
    /// Setting the size of shared memory failed.
    SetSize(SysError),
    /// Failed to create a surface on the compositor.
    CreateSurface,
    /// Failed to import a buffer to the compositor.
    FailedImport,
    /// The surface ID is invalid.
    InvalidSurfaceId,
    /// A required feature was missing.
    RequiredFeature(&'static str),
    /// The path is invalid.
    InvalidPath,
    /// The method is unsupported by the implementation.
    Unsupported,
}

impl Display for GpuDisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::GpuDisplayError::*;

        match self {
            Allocate => write!(f, "internal allocation failed"),
            Connect => write!(f, "failed to connect to compositor"),
            CreateEventFd => write!(f, "failed to create event file descriptor"),
            CreateShm(e) => write!(f, "failed to create shared memory: {}", e),
            CreateSurface => write!(f, "failed to crate surface on the compositor"),
            FailedImport => write!(f, "failed to import a buffer to the compositor"),
            InvalidPath => write!(f, "invalid path"),
            InvalidSurfaceId => write!(f, "invalid surface ID"),
            RequiredFeature(feature) => write!(f, "required feature was missing: {}", feature),
            SetSize(e) => write!(f, "failed to set size of shared memory: {}", e),
            Unsupported => write!(f, "unsupported by the implementation"),
        }
    }
}

#[derive(Clone)]
pub struct GpuDisplayFramebuffer<'a> {
    framebuffer: VolatileSlice<'a>,
    slice: VolatileSlice<'a>,
    stride: u32,
    bytes_per_pixel: u32,
}

impl<'a> GpuDisplayFramebuffer<'a> {
    fn new(
        framebuffer: VolatileSlice<'a>,
        stride: u32,
        bytes_per_pixel: u32,
    ) -> GpuDisplayFramebuffer {
        GpuDisplayFramebuffer {
            framebuffer,
            slice: framebuffer,
            stride,
            bytes_per_pixel,
        }
    }

    fn sub_region(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Option<GpuDisplayFramebuffer<'a>> {
        let x_byte_offset = x.checked_mul(self.bytes_per_pixel)?;
        let y_byte_offset = y.checked_mul(self.stride)?;
        let byte_offset = x_byte_offset.checked_add(y_byte_offset)?;

        let width_bytes = width.checked_mul(self.bytes_per_pixel)?;
        let count = height
            .checked_mul(self.stride)?
            .checked_sub(self.stride)?
            .checked_add(width_bytes)?;
        let slice = self
            .framebuffer
            .sub_slice(byte_offset as usize, count as usize)
            .unwrap();

        Some(GpuDisplayFramebuffer { slice, ..*self })
    }

    pub fn as_volatile_slice(&self) -> VolatileSlice<'a> {
        self.slice
    }

    pub fn stride(&self) -> u32 {
        self.stride
    }
}

trait DisplayT: AsRawFd {
    fn import_dmabuf(
        &mut self,
        fd: RawFd,
        offset: u32,
        stride: u32,
        modifiers: u64,
        width: u32,
        height: u32,
        fourcc: u32,
    ) -> Result<u32, GpuDisplayError>;
    fn release_import(&mut self, import_id: u32);
    fn dispatch_events(&mut self);
    fn create_surface(
        &mut self,
        parent_surface_id: Option<u32>,
        width: u32,
        height: u32,
    ) -> Result<u32, GpuDisplayError>;
    fn release_surface(&mut self, surface_id: u32);
    fn framebuffer(&mut self, surface_id: u32) -> Option<GpuDisplayFramebuffer>;
    fn framebuffer_region(
        &mut self,
        surface_id: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Option<GpuDisplayFramebuffer> {
        let framebuffer = self.framebuffer(surface_id)?;
        framebuffer.sub_region(x, y, width, height)
    }
    fn commit(&mut self, surface_id: u32);
    fn next_buffer_in_use(&self, surface_id: u32) -> bool;
    fn flip(&mut self, surface_id: u32);
    fn flip_to(&mut self, surface_id: u32, import_id: u32);
    fn close_requested(&self, surface_id: u32) -> bool;
    fn set_position(&mut self, surface_id: u32, x: u32, y: u32);
    fn import_event_device(&mut self, event_device: EventDevice) -> Result<u32, GpuDisplayError>;
    fn release_event_device(&mut self, event_device_id: u32);
    fn attach_event_device(&mut self, surface_id: u32, event_device_id: u32);
}

/// A connection to the compositor and associated collection of state.
///
/// The user of `GpuDisplay` can use `AsRawFd` to poll on the compositor connection's file
/// descriptor. When the connection is readable, `dispatch_events` can be called to process it.
pub struct GpuDisplay {
    inner: Box<dyn DisplayT>,
    is_x: bool,
}

impl GpuDisplay {
    pub fn open_x<S: AsRef<str>>(display_name: Option<S>) -> Result<GpuDisplay, GpuDisplayError> {
        let _ = display_name;
        #[cfg(feature = "x")]
        {
            let display = match display_name {
                Some(s) => gpu_display_x::DisplayX::open_display(Some(s.as_ref()))?,
                None => gpu_display_x::DisplayX::open_display(None)?,
            };
            let inner = Box::new(display);
            Ok(GpuDisplay { inner, is_x: true })
        }
        #[cfg(not(feature = "x"))]
        Err(GpuDisplayError::Unsupported)
    }

    /// Opens a fresh connection to the compositor.
    pub fn open_wayland<P: AsRef<Path>>(
        wayland_path: Option<P>,
    ) -> Result<GpuDisplay, GpuDisplayError> {
        let display = match wayland_path {
            Some(s) => gpu_display_wl::DisplayWl::new(Some(s.as_ref()))?,
            None => gpu_display_wl::DisplayWl::new(None)?,
        };
        let inner = Box::new(display);
        Ok(GpuDisplay { inner, is_x: false })
    }

    pub fn open_stub() -> Result<GpuDisplay, GpuDisplayError> {
        let display = gpu_display_stub::DisplayStub::new()?;
        let inner = Box::new(display);
        Ok(GpuDisplay { inner, is_x: false })
    }

    /// Return whether this display is an X display
    pub fn is_x(&self) -> bool {
        self.is_x
    }

    /// Imports a dmabuf to the compositor for use as a surface buffer and returns a handle to it.
    pub fn import_dmabuf(
        &mut self,
        fd: RawFd,
        offset: u32,
        stride: u32,
        modifiers: u64,
        width: u32,
        height: u32,
        fourcc: u32,
    ) -> Result<u32, GpuDisplayError> {
        self.inner
            .import_dmabuf(fd, offset, stride, modifiers, width, height, fourcc)
    }

    /// Releases a previously imported dmabuf identified by the given handle.
    pub fn release_import(&mut self, import_id: u32) {
        self.inner.release_import(import_id);
    }

    /// Dispatches internal events that were received from the compositor since the last call to
    /// `dispatch_events`.
    pub fn dispatch_events(&mut self) {
        self.inner.dispatch_events()
    }

    /// Creates a surface on the the compositor as either a top level window, or child of another
    /// surface, returning a handle to the new surface.
    pub fn create_surface(
        &mut self,
        parent_surface_id: Option<u32>,
        width: u32,
        height: u32,
    ) -> Result<u32, GpuDisplayError> {
        self.inner.create_surface(parent_surface_id, width, height)
    }

    /// Releases a previously created surface identified by the given handle.
    pub fn release_surface(&mut self, surface_id: u32) {
        self.inner.release_surface(surface_id)
    }

    /// Gets a reference to an unused framebuffer for the identified surface.
    pub fn framebuffer(&mut self, surface_id: u32) -> Option<GpuDisplayFramebuffer> {
        self.inner.framebuffer(surface_id)
    }

    /// Gets a reference to an unused framebuffer for the identified surface.
    pub fn framebuffer_region(
        &mut self,
        surface_id: u32,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Option<GpuDisplayFramebuffer> {
        self.inner
            .framebuffer_region(surface_id, x, y, width, height)
    }

    /// Commits any pending state for the identified surface.
    pub fn commit(&mut self, surface_id: u32) {
        self.inner.commit(surface_id)
    }

    /// Returns true if the next buffer in the buffer queue for the given surface is currently in
    /// use.
    ///
    /// If the next buffer is in use, the memory returned from `framebuffer_memory` should not be
    /// written to.
    pub fn next_buffer_in_use(&self, surface_id: u32) -> bool {
        self.inner.next_buffer_in_use(surface_id)
    }

    /// Changes the visible contents of the identified surface to the contents of the framebuffer
    /// last returned by `framebuffer_memory` for this surface.
    pub fn flip(&mut self, surface_id: u32) {
        self.inner.flip(surface_id)
    }

    /// Changes the visible contents of the identified surface to that of the identified imported
    /// buffer.
    pub fn flip_to(&mut self, surface_id: u32, import_id: u32) {
        self.inner.flip_to(surface_id, import_id)
    }

    /// Returns true if the identified top level surface has been told to close by the compositor,
    /// and by extension the user.
    pub fn close_requested(&self, surface_id: u32) -> bool {
        self.inner.close_requested(surface_id)
    }

    /// Sets the position of the identified subsurface relative to its parent.
    ///
    /// The change in position will not be visible until `commit` is called for the parent surface.
    pub fn set_position(&mut self, surface_id: u32, x: u32, y: u32) {
        self.inner.set_position(surface_id, x, y)
    }

    pub fn import_event_device(
        &mut self,
        event_device: EventDevice,
    ) -> Result<u32, GpuDisplayError> {
        self.inner.import_event_device(event_device)
    }

    pub fn release_event_device(&mut self, event_device_id: u32) {
        self.inner.release_event_device(event_device_id)
    }

    pub fn attach_event_device(&mut self, surface_id: u32, event_device_id: u32) {
        self.inner.attach_event_device(surface_id, event_device_id);
    }
}

impl AsRawFd for GpuDisplay {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}
