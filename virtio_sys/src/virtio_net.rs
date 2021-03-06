// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}
pub const __BITS_PER_LONG: ::std::os::raw::c_uint = 64;
pub const __FD_SETSIZE: ::std::os::raw::c_uint = 1024;
pub const VIRTIO_ID_NET: ::std::os::raw::c_uint = 1;
pub const VIRTIO_ID_BLOCK: ::std::os::raw::c_uint = 2;
pub const VIRTIO_ID_CONSOLE: ::std::os::raw::c_uint = 3;
pub const VIRTIO_ID_RNG: ::std::os::raw::c_uint = 4;
pub const VIRTIO_ID_BALLOON: ::std::os::raw::c_uint = 5;
pub const VIRTIO_ID_RPMSG: ::std::os::raw::c_uint = 7;
pub const VIRTIO_ID_SCSI: ::std::os::raw::c_uint = 8;
pub const VIRTIO_ID_9P: ::std::os::raw::c_uint = 9;
pub const VIRTIO_ID_RPROC_SERIAL: ::std::os::raw::c_uint = 11;
pub const VIRTIO_ID_CAIF: ::std::os::raw::c_uint = 12;
pub const VIRTIO_ID_GPU: ::std::os::raw::c_uint = 16;
pub const VIRTIO_ID_INPUT: ::std::os::raw::c_uint = 18;
pub const VIRTIO_CONFIG_S_ACKNOWLEDGE: ::std::os::raw::c_uint = 1;
pub const VIRTIO_CONFIG_S_DRIVER: ::std::os::raw::c_uint = 2;
pub const VIRTIO_CONFIG_S_DRIVER_OK: ::std::os::raw::c_uint = 4;
pub const VIRTIO_CONFIG_S_FEATURES_OK: ::std::os::raw::c_uint = 8;
pub const VIRTIO_CONFIG_S_FAILED: ::std::os::raw::c_uint = 128;
pub const VIRTIO_TRANSPORT_F_START: ::std::os::raw::c_uint = 28;
pub const VIRTIO_TRANSPORT_F_END: ::std::os::raw::c_uint = 33;
pub const VIRTIO_F_NOTIFY_ON_EMPTY: ::std::os::raw::c_uint = 24;
pub const VIRTIO_F_ANY_LAYOUT: ::std::os::raw::c_uint = 27;
pub const VIRTIO_F_VERSION_1: ::std::os::raw::c_uint = 32;
pub const ETH_ALEN: ::std::os::raw::c_uint = 6;
pub const ETH_HLEN: ::std::os::raw::c_uint = 14;
pub const ETH_ZLEN: ::std::os::raw::c_uint = 60;
pub const ETH_DATA_LEN: ::std::os::raw::c_uint = 1500;
pub const ETH_FRAME_LEN: ::std::os::raw::c_uint = 1514;
pub const ETH_FCS_LEN: ::std::os::raw::c_uint = 4;
pub const ETH_P_LOOP: ::std::os::raw::c_uint = 96;
pub const ETH_P_PUP: ::std::os::raw::c_uint = 512;
pub const ETH_P_PUPAT: ::std::os::raw::c_uint = 513;
pub const ETH_P_TSN: ::std::os::raw::c_uint = 8944;
pub const ETH_P_IP: ::std::os::raw::c_uint = 2048;
pub const ETH_P_X25: ::std::os::raw::c_uint = 2053;
pub const ETH_P_ARP: ::std::os::raw::c_uint = 2054;
pub const ETH_P_BPQ: ::std::os::raw::c_uint = 2303;
pub const ETH_P_IEEEPUP: ::std::os::raw::c_uint = 2560;
pub const ETH_P_IEEEPUPAT: ::std::os::raw::c_uint = 2561;
pub const ETH_P_BATMAN: ::std::os::raw::c_uint = 17157;
pub const ETH_P_DEC: ::std::os::raw::c_uint = 24576;
pub const ETH_P_DNA_DL: ::std::os::raw::c_uint = 24577;
pub const ETH_P_DNA_RC: ::std::os::raw::c_uint = 24578;
pub const ETH_P_DNA_RT: ::std::os::raw::c_uint = 24579;
pub const ETH_P_LAT: ::std::os::raw::c_uint = 24580;
pub const ETH_P_DIAG: ::std::os::raw::c_uint = 24581;
pub const ETH_P_CUST: ::std::os::raw::c_uint = 24582;
pub const ETH_P_SCA: ::std::os::raw::c_uint = 24583;
pub const ETH_P_TEB: ::std::os::raw::c_uint = 25944;
pub const ETH_P_RARP: ::std::os::raw::c_uint = 32821;
pub const ETH_P_ATALK: ::std::os::raw::c_uint = 32923;
pub const ETH_P_AARP: ::std::os::raw::c_uint = 33011;
pub const ETH_P_8021Q: ::std::os::raw::c_uint = 33024;
pub const ETH_P_IPX: ::std::os::raw::c_uint = 33079;
pub const ETH_P_IPV6: ::std::os::raw::c_uint = 34525;
pub const ETH_P_PAUSE: ::std::os::raw::c_uint = 34824;
pub const ETH_P_SLOW: ::std::os::raw::c_uint = 34825;
pub const ETH_P_WCCP: ::std::os::raw::c_uint = 34878;
pub const ETH_P_MPLS_UC: ::std::os::raw::c_uint = 34887;
pub const ETH_P_MPLS_MC: ::std::os::raw::c_uint = 34888;
pub const ETH_P_ATMMPOA: ::std::os::raw::c_uint = 34892;
pub const ETH_P_PPP_DISC: ::std::os::raw::c_uint = 34915;
pub const ETH_P_PPP_SES: ::std::os::raw::c_uint = 34916;
pub const ETH_P_LINK_CTL: ::std::os::raw::c_uint = 34924;
pub const ETH_P_ATMFATE: ::std::os::raw::c_uint = 34948;
pub const ETH_P_PAE: ::std::os::raw::c_uint = 34958;
pub const ETH_P_AOE: ::std::os::raw::c_uint = 34978;
pub const ETH_P_8021AD: ::std::os::raw::c_uint = 34984;
pub const ETH_P_802_EX1: ::std::os::raw::c_uint = 34997;
pub const ETH_P_TIPC: ::std::os::raw::c_uint = 35018;
pub const ETH_P_8021AH: ::std::os::raw::c_uint = 35047;
pub const ETH_P_MVRP: ::std::os::raw::c_uint = 35061;
pub const ETH_P_1588: ::std::os::raw::c_uint = 35063;
pub const ETH_P_PRP: ::std::os::raw::c_uint = 35067;
pub const ETH_P_FCOE: ::std::os::raw::c_uint = 35078;
pub const ETH_P_TDLS: ::std::os::raw::c_uint = 35085;
pub const ETH_P_FIP: ::std::os::raw::c_uint = 35092;
pub const ETH_P_80221: ::std::os::raw::c_uint = 35095;
pub const ETH_P_LOOPBACK: ::std::os::raw::c_uint = 36864;
pub const ETH_P_QINQ1: ::std::os::raw::c_uint = 37120;
pub const ETH_P_QINQ2: ::std::os::raw::c_uint = 37376;
pub const ETH_P_QINQ3: ::std::os::raw::c_uint = 37632;
pub const ETH_P_EDSA: ::std::os::raw::c_uint = 56026;
pub const ETH_P_AF_IUCV: ::std::os::raw::c_uint = 64507;
pub const ETH_P_802_3_MIN: ::std::os::raw::c_uint = 1536;
pub const ETH_P_802_3: ::std::os::raw::c_uint = 1;
pub const ETH_P_AX25: ::std::os::raw::c_uint = 2;
pub const ETH_P_ALL: ::std::os::raw::c_uint = 3;
pub const ETH_P_802_2: ::std::os::raw::c_uint = 4;
pub const ETH_P_SNAP: ::std::os::raw::c_uint = 5;
pub const ETH_P_DDCMP: ::std::os::raw::c_uint = 6;
pub const ETH_P_WAN_PPP: ::std::os::raw::c_uint = 7;
pub const ETH_P_PPP_MP: ::std::os::raw::c_uint = 8;
pub const ETH_P_LOCALTALK: ::std::os::raw::c_uint = 9;
pub const ETH_P_CAN: ::std::os::raw::c_uint = 12;
pub const ETH_P_CANFD: ::std::os::raw::c_uint = 13;
pub const ETH_P_PPPTALK: ::std::os::raw::c_uint = 16;
pub const ETH_P_TR_802_2: ::std::os::raw::c_uint = 17;
pub const ETH_P_MOBITEX: ::std::os::raw::c_uint = 21;
pub const ETH_P_CONTROL: ::std::os::raw::c_uint = 22;
pub const ETH_P_IRDA: ::std::os::raw::c_uint = 23;
pub const ETH_P_ECONET: ::std::os::raw::c_uint = 24;
pub const ETH_P_HDLC: ::std::os::raw::c_uint = 25;
pub const ETH_P_ARCNET: ::std::os::raw::c_uint = 26;
pub const ETH_P_DSA: ::std::os::raw::c_uint = 27;
pub const ETH_P_TRAILER: ::std::os::raw::c_uint = 28;
pub const ETH_P_PHONET: ::std::os::raw::c_uint = 245;
pub const ETH_P_IEEE802154: ::std::os::raw::c_uint = 246;
pub const ETH_P_CAIF: ::std::os::raw::c_uint = 247;
pub const ETH_P_XDSA: ::std::os::raw::c_uint = 248;
pub const VIRTIO_NET_F_CSUM: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_F_GUEST_CSUM: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_F_CTRL_GUEST_OFFLOADS: ::std::os::raw::c_uint = 2;
pub const VIRTIO_NET_F_MTU: ::std::os::raw::c_uint = 3;
pub const VIRTIO_NET_F_MAC: ::std::os::raw::c_uint = 5;
pub const VIRTIO_NET_F_GUEST_TSO4: ::std::os::raw::c_uint = 7;
pub const VIRTIO_NET_F_GUEST_TSO6: ::std::os::raw::c_uint = 8;
pub const VIRTIO_NET_F_GUEST_ECN: ::std::os::raw::c_uint = 9;
pub const VIRTIO_NET_F_GUEST_UFO: ::std::os::raw::c_uint = 10;
pub const VIRTIO_NET_F_HOST_TSO4: ::std::os::raw::c_uint = 11;
pub const VIRTIO_NET_F_HOST_TSO6: ::std::os::raw::c_uint = 12;
pub const VIRTIO_NET_F_HOST_ECN: ::std::os::raw::c_uint = 13;
pub const VIRTIO_NET_F_HOST_UFO: ::std::os::raw::c_uint = 14;
pub const VIRTIO_NET_F_MRG_RXBUF: ::std::os::raw::c_uint = 15;
pub const VIRTIO_NET_F_STATUS: ::std::os::raw::c_uint = 16;
pub const VIRTIO_NET_F_CTRL_VQ: ::std::os::raw::c_uint = 17;
pub const VIRTIO_NET_F_CTRL_RX: ::std::os::raw::c_uint = 18;
pub const VIRTIO_NET_F_CTRL_VLAN: ::std::os::raw::c_uint = 19;
pub const VIRTIO_NET_F_CTRL_RX_EXTRA: ::std::os::raw::c_uint = 20;
pub const VIRTIO_NET_F_GUEST_ANNOUNCE: ::std::os::raw::c_uint = 21;
pub const VIRTIO_NET_F_MQ: ::std::os::raw::c_uint = 22;
pub const VIRTIO_NET_F_CTRL_MAC_ADDR: ::std::os::raw::c_uint = 23;
pub const VIRTIO_NET_F_GSO: ::std::os::raw::c_uint = 6;
pub const VIRTIO_NET_S_LINK_UP: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_S_ANNOUNCE: ::std::os::raw::c_uint = 2;
pub const VIRTIO_NET_HDR_F_NEEDS_CSUM: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_HDR_F_DATA_VALID: ::std::os::raw::c_uint = 2;
pub const VIRTIO_NET_HDR_GSO_NONE: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_HDR_GSO_TCPV4: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_HDR_GSO_UDP: ::std::os::raw::c_uint = 3;
pub const VIRTIO_NET_HDR_GSO_TCPV6: ::std::os::raw::c_uint = 4;
pub const VIRTIO_NET_HDR_GSO_ECN: ::std::os::raw::c_uint = 128;
pub const VIRTIO_NET_OK: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_ERR: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_CTRL_RX: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_CTRL_RX_PROMISC: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_CTRL_RX_ALLMULTI: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_CTRL_RX_ALLUNI: ::std::os::raw::c_uint = 2;
pub const VIRTIO_NET_CTRL_RX_NOMULTI: ::std::os::raw::c_uint = 3;
pub const VIRTIO_NET_CTRL_RX_NOUNI: ::std::os::raw::c_uint = 4;
pub const VIRTIO_NET_CTRL_RX_NOBCAST: ::std::os::raw::c_uint = 5;
pub const VIRTIO_NET_CTRL_MAC: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_CTRL_MAC_TABLE_SET: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_CTRL_MAC_ADDR_SET: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_CTRL_VLAN: ::std::os::raw::c_uint = 2;
pub const VIRTIO_NET_CTRL_VLAN_ADD: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_CTRL_VLAN_DEL: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_CTRL_ANNOUNCE: ::std::os::raw::c_uint = 3;
pub const VIRTIO_NET_CTRL_ANNOUNCE_ACK: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_CTRL_MQ: ::std::os::raw::c_uint = 4;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_SET: ::std::os::raw::c_uint = 0;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_MIN: ::std::os::raw::c_uint = 1;
pub const VIRTIO_NET_CTRL_MQ_VQ_PAIRS_MAX: ::std::os::raw::c_uint = 32768;
pub const VIRTIO_NET_CTRL_GUEST_OFFLOADS: ::std::os::raw::c_uint = 5;
pub const VIRTIO_NET_CTRL_GUEST_OFFLOADS_SET: ::std::os::raw::c_uint = 0;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
#[ignore]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { &(*(0 as *const __kernel_fd_set)).fds_bits as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
impl Clone for __kernel_fd_set {
    fn clone(&self) -> Self {
        *self
    }
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { &(*(0 as *const __kernel_fsid_t)).val as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
}
impl Clone for __kernel_fsid_t {
    fn clone(&self) -> Self {
        *self
    }
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __virtio16 = __u16;
pub type __virtio32 = __u32;
pub type __virtio64 = __u64;
#[repr(C, packed)]
#[derive(Debug, Copy)]
pub struct ethhdr {
    pub h_dest: [::std::os::raw::c_uchar; 6usize],
    pub h_source: [::std::os::raw::c_uchar; 6usize],
    pub h_proto: __be16,
}
#[test]
fn bindgen_test_layout_ethhdr() {
    assert_eq!(
        ::std::mem::size_of::<ethhdr>(),
        14usize,
        concat!("Size of: ", stringify!(ethhdr))
    );
    assert_eq!(
        ::std::mem::align_of::<ethhdr>(),
        1usize,
        concat!("Alignment of ", stringify!(ethhdr))
    );
    assert_eq!(
        unsafe { &(*(0 as *const ethhdr)).h_dest as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(ethhdr),
            "::",
            stringify!(h_dest)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const ethhdr)).h_source as *const _ as usize },
        6usize,
        concat!(
            "Alignment of field: ",
            stringify!(ethhdr),
            "::",
            stringify!(h_source)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const ethhdr)).h_proto as *const _ as usize },
        12usize,
        concat!(
            "Alignment of field: ",
            stringify!(ethhdr),
            "::",
            stringify!(h_proto)
        )
    );
}
impl Clone for ethhdr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed)]
#[derive(Debug, Copy)]
pub struct virtio_net_config {
    pub mac: [__u8; 6usize],
    pub status: __u16,
    pub max_virtqueue_pairs: __u16,
    pub mtu: __u16,
}
#[test]
fn bindgen_test_layout_virtio_net_config() {
    assert_eq!(
        ::std::mem::size_of::<virtio_net_config>(),
        12usize,
        concat!("Size of: ", stringify!(virtio_net_config))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_net_config>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_net_config))
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_config)).mac as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_config),
            "::",
            stringify!(mac)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_config)).status as *const _ as usize },
        6usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_config),
            "::",
            stringify!(status)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_config)).max_virtqueue_pairs as *const _ as usize },
        8usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_config),
            "::",
            stringify!(max_virtqueue_pairs)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_config)).mtu as *const _ as usize },
        10usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_config),
            "::",
            stringify!(mtu)
        )
    );
}
impl Clone for virtio_net_config {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct virtio_net_hdr_v1 {
    pub flags: __u8,
    pub gso_type: __u8,
    pub hdr_len: __virtio16,
    pub gso_size: __virtio16,
    pub csum_start: __virtio16,
    pub csum_offset: __virtio16,
    pub num_buffers: __virtio16,
}
#[test]
fn bindgen_test_layout_virtio_net_hdr_v1() {
    assert_eq!(
        ::std::mem::size_of::<virtio_net_hdr_v1>(),
        12usize,
        concat!("Size of: ", stringify!(virtio_net_hdr_v1))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_net_hdr_v1>(),
        2usize,
        concat!("Alignment of ", stringify!(virtio_net_hdr_v1))
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_v1)).flags as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_v1),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_v1)).gso_type as *const _ as usize },
        1usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_v1),
            "::",
            stringify!(gso_type)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_v1)).hdr_len as *const _ as usize },
        2usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_v1),
            "::",
            stringify!(hdr_len)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_v1)).gso_size as *const _ as usize },
        4usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_v1),
            "::",
            stringify!(gso_size)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_v1)).csum_start as *const _ as usize },
        6usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_v1),
            "::",
            stringify!(csum_start)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_v1)).csum_offset as *const _ as usize },
        8usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_v1),
            "::",
            stringify!(csum_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_v1)).num_buffers as *const _ as usize },
        10usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_v1),
            "::",
            stringify!(num_buffers)
        )
    );
}
impl Clone for virtio_net_hdr_v1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct virtio_net_hdr {
    pub flags: __u8,
    pub gso_type: __u8,
    pub hdr_len: __virtio16,
    pub gso_size: __virtio16,
    pub csum_start: __virtio16,
    pub csum_offset: __virtio16,
}
#[test]
fn bindgen_test_layout_virtio_net_hdr() {
    assert_eq!(
        ::std::mem::size_of::<virtio_net_hdr>(),
        10usize,
        concat!("Size of: ", stringify!(virtio_net_hdr))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_net_hdr>(),
        2usize,
        concat!("Alignment of ", stringify!(virtio_net_hdr))
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr)).flags as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr)).gso_type as *const _ as usize },
        1usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr),
            "::",
            stringify!(gso_type)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr)).hdr_len as *const _ as usize },
        2usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr),
            "::",
            stringify!(hdr_len)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr)).gso_size as *const _ as usize },
        4usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr),
            "::",
            stringify!(gso_size)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr)).csum_start as *const _ as usize },
        6usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr),
            "::",
            stringify!(csum_start)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr)).csum_offset as *const _ as usize },
        8usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr),
            "::",
            stringify!(csum_offset)
        )
    );
}
impl Clone for virtio_net_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct virtio_net_hdr_mrg_rxbuf {
    pub hdr: virtio_net_hdr,
    pub num_buffers: __virtio16,
}
#[test]
fn bindgen_test_layout_virtio_net_hdr_mrg_rxbuf() {
    assert_eq!(
        ::std::mem::size_of::<virtio_net_hdr_mrg_rxbuf>(),
        12usize,
        concat!("Size of: ", stringify!(virtio_net_hdr_mrg_rxbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_net_hdr_mrg_rxbuf>(),
        2usize,
        concat!("Alignment of ", stringify!(virtio_net_hdr_mrg_rxbuf))
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_mrg_rxbuf)).hdr as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_mrg_rxbuf),
            "::",
            stringify!(hdr)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_hdr_mrg_rxbuf)).num_buffers as *const _ as usize },
        10usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_hdr_mrg_rxbuf),
            "::",
            stringify!(num_buffers)
        )
    );
}
impl Clone for virtio_net_hdr_mrg_rxbuf {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed)]
#[derive(Debug, Copy)]
pub struct virtio_net_ctrl_hdr {
    pub class: __u8,
    pub cmd: __u8,
}
#[test]
fn bindgen_test_layout_virtio_net_ctrl_hdr() {
    assert_eq!(
        ::std::mem::size_of::<virtio_net_ctrl_hdr>(),
        2usize,
        concat!("Size of: ", stringify!(virtio_net_ctrl_hdr))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_net_ctrl_hdr>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_net_ctrl_hdr))
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_ctrl_hdr)).class as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_ctrl_hdr),
            "::",
            stringify!(class)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_ctrl_hdr)).cmd as *const _ as usize },
        1usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_ctrl_hdr),
            "::",
            stringify!(cmd)
        )
    );
}
impl Clone for virtio_net_ctrl_hdr {
    fn clone(&self) -> Self {
        *self
    }
}
pub type virtio_net_ctrl_ack = __u8;
#[repr(C, packed)]
#[derive(Debug, Copy)]
pub struct virtio_net_ctrl_mac {
    pub entries: __virtio32,
    pub macs: __IncompleteArrayField<[__u8; 6usize]>,
}
#[test]
fn bindgen_test_layout_virtio_net_ctrl_mac() {
    assert_eq!(
        ::std::mem::size_of::<virtio_net_ctrl_mac>(),
        4usize,
        concat!("Size of: ", stringify!(virtio_net_ctrl_mac))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_net_ctrl_mac>(),
        1usize,
        concat!("Alignment of ", stringify!(virtio_net_ctrl_mac))
    );
}
impl Clone for virtio_net_ctrl_mac {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct virtio_net_ctrl_mq {
    pub virtqueue_pairs: __virtio16,
}
#[test]
fn bindgen_test_layout_virtio_net_ctrl_mq() {
    assert_eq!(
        ::std::mem::size_of::<virtio_net_ctrl_mq>(),
        2usize,
        concat!("Size of: ", stringify!(virtio_net_ctrl_mq))
    );
    assert_eq!(
        ::std::mem::align_of::<virtio_net_ctrl_mq>(),
        2usize,
        concat!("Alignment of ", stringify!(virtio_net_ctrl_mq))
    );
    assert_eq!(
        unsafe { &(*(0 as *const virtio_net_ctrl_mq)).virtqueue_pairs as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(virtio_net_ctrl_mq),
            "::",
            stringify!(virtqueue_pairs)
        )
    );
}
impl Clone for virtio_net_ctrl_mq {
    fn clone(&self) -> Self {
        *self
    }
}
