//! Phoenix-RTOS, a real-time operating system with a POSIX-compatible libc
//! (libphoenix).
//!
//! The authority for everything here is the target sysroot, e.g.
//! `_build/riscv64-generic-qemu/sysroot/usr/include`.
//!
//! * libphoenix: <https://github.com/phoenix-rtos/libphoenix>
//! * kernel-shared headers: `phoenix/*.h` in the sysroot

use crate::prelude::*;

pub type sa_family_t = u16;
// `termios.h`: `typedef int speed_t;`
pub type speed_t = c_int;
// `termios.h`: `typedef int tcflag_t;`
pub type tcflag_t = c_int;
pub type clockid_t = c_int;

pub type useconds_t = u32;

extern_ty! {
    pub type timezone;
}

// phoenix/posix-types.h: `typedef int dev_t;`
pub type dev_t = c_int;
pub type socklen_t = u32;
pub type mode_t = u32;
pub type ino64_t = u64;
pub type off64_t = i64;
pub type blkcnt64_t = i64;
pub type rlim64_t = u64;

// `phoenix/posix-poll.h`: `typedef unsigned int nfds_t;`
pub type nfds_t = c_uint;

// In C this is `struct __pthread_key_t *` (identical ABI).
pub type pthread_key_t = usize;
pub type pthread_once_t = u32;

// `phoenix/types.h`: `typedef int handle_t;`
pub type handle_t = c_int;

pub type __u8 = c_uchar;
pub type __u16 = c_ushort;
pub type __s16 = c_short;
pub type __u32 = c_uint;
pub type __s32 = c_int;

pub type pthread_t = crate::uintptr_t;
// `sys/types.h`: `typedef int clock_t;`
pub type clock_t = c_int;
// `phoenix/types.h`: `typedef long long time_t;`
pub type time_t = i64;
// `sys/types.h`: `typedef int suseconds_t;`
pub type suseconds_t = c_int;
pub type ino_t = u64;
pub type off_t = i64;
pub type blkcnt_t = i64;

pub type fsblkcnt_t = c_ulonglong;
pub type fsblkcnt64_t = c_ulonglong;
pub type fsfilcnt_t = c_ulonglong;
pub type fsfilcnt64_t = c_ulonglong;
// `sys/resource.h`: `typedef int rlim_t;`
pub type rlim_t = c_int;

// libphoenix `stdio.h`: `typedef off_t fpos_t;`
pub type fpos_t = crate::off_t;
pub type fpos64_t = crate::off64_t;

// `phoenix/ioctl.h` casts every request through `(unsigned long)`.
#[doc(hidden)]
pub type Ioctl = c_ulong;

// RLIMIT Constants

// `phoenix/signal.h`: `typedef unsigned int sigset_t;`
pub type sigset_t = c_uint;

// `regex.h`: `typedef off_t regoff_t;`
pub type regoff_t = off_t;

// Phoenix has one set of resource-limit interfaces, not an LFS64 pair.
pub type rlimit64 = crate::rlimit;
pub type statvfs64 = crate::statvfs;

// Phoenix has no separate LFS64 directory entry.
pub type dirent64 = crate::dirent;

pub type stat64 = stat;

// `phoenix/posix-types.h`
pub type nlink_t = c_int;
pub type blksize_t = c_int;

s! {
    // FIXME(1.0): This should not implement `PartialEq`
    #[allow(unpredictable_function_pointer_comparisons)]
    pub struct __c_anonymous_sigev_thread {
        pub _function: Option<extern "C" fn(crate::sigval) -> *mut c_void>,
        pub _attribute: *mut crate::pthread_attr_t,
    }

    // libphoenix's `include/dirent.h`: no `d_off`, `d_type` is 32-bit, and
    // `d_name` is a flexible array member.
    pub struct dirent {
        pub d_ino: crate::ino_t,
        pub d_type: u32,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_name: [c_char; 0],
    }

    pub struct in_addr {
        pub s_addr: crate::in_addr_t,
    }

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    // `sys/socket.h`
    pub struct linger {
        pub l_onoff: c_int,
        pub l_linger: c_int,
    }

    // lwIP's `struct ip_mreq`; the libphoenix headers do not re-export it.
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    // The order of the `ai_addr` field in this struct is crucial
    // for converting between the Rust and C types.
    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: socklen_t,

        pub ai_addr: *mut crate::sockaddr,

        pub ai_canonname: *mut c_char,

        pub ai_next: *mut addrinfo,
    }

    pub struct sockaddr_ll {
        pub sll_family: c_ushort,
        pub sll_protocol: c_ushort,
        pub sll_ifindex: c_int,
        pub sll_hatype: c_ushort,
        pub sll_pkttype: c_uchar,
        pub sll_halen: c_uchar,
        pub sll_addr: [c_uchar; 8],
    }

    // `sys/select.h`: a `uint32_t` bit array.
    pub struct fd_set {
        fds_bits: [u32; (FD_SETSIZE as usize + 31) / 32],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
    }

    pub struct sched_param {
        pub sched_priority: c_int,
    }

    // libphoenix `locale.h`.
    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub n_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub int_frac_digits: c_char,
        pub int_p_cs_precedes: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
        pub int_n_sign_posn: c_char,
    }

    // libphoenix `ifaddrs.h`.
    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut c_char,
        pub ifa_flags: c_uint,
        pub ifa_addr: *mut crate::sockaddr,
        pub ifa_netmask: *mut crate::sockaddr,
        pub ifa_dstaddr: *mut crate::sockaddr,
        pub ifa_data: *mut c_void,
    }

    pub struct in6_rtmsg {
        rtmsg_dst: crate::in6_addr,
        rtmsg_src: crate::in6_addr,
        rtmsg_gateway: crate::in6_addr,
        rtmsg_type: u32,
        rtmsg_dst_len: u16,
        rtmsg_src_len: u16,
        rtmsg_metric: u32,
        rtmsg_info: c_ulong,
        rtmsg_flags: u32,
        rtmsg_ifindex: c_int,
    }

    pub struct arphdr {
        pub ar_hrd: u16,
        pub ar_pro: u16,
        pub ar_hln: u8,
        pub ar_pln: u8,
        pub ar_op: u16,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108],
    }

    // `sys/socket.h`:
    //
    //     struct sockaddr_storage {
    //         sa_family_t ss_family;
    //         char ss_data[128 - sizeof(sa_family_t)];
    //     } __attribute__((aligned(8)));
    //
    // The alignment is explicit in the header, so it has to be explicit here
    // too: a trailing `size_t` only happens to give 8 on a 64-bit target, and
    // on ia32 it would leave the struct 4-aligned.
    #[repr(align(8))]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_data: Padding<[u8; 128 - 2]>,
    }

    // `sys/resource.h`: CPU time only.
    pub struct rusage {
        pub ru_utime: crate::timeval,
        pub ru_stime: crate::timeval,
    }

    // `phoenix/utsname.h`.
    pub struct utsname {
        pub sysname: [c_char; 16],
        pub nodename: [c_char; HOST_NAME_MAX as usize + 1],
        pub release: [c_char; 32],
        pub version: [c_char; 64],
        pub machine: [c_char; 16],
    }

    pub struct if_nameindex {
        pub if_index: c_uint,
        pub if_name: *mut c_char,
    }
}

s_no_extra_traits! {
    pub union __c_anonymous_sigev_un {
        _pad: Padding<[c_int; SIGEV_PAD_SIZE]>,
        pub _tid: c_int,
        pub _sigev_thread: __c_anonymous_sigev_thread,
    }

    pub union __c_anonymous_ifaddrs_ifa_ifu {
        ifu_broadaddr: *mut sockaddr,
        ifu_dstaddr: *mut sockaddr,
    }
}

s! {
    // libphoenix `glob.h`; the alternate-access hooks are function pointers.
    #[allow(unpredictable_function_pointer_comparisons)]
    pub struct glob_t {
        pub gl_pathc: c_int,
        pub gl_matchc: c_int,
        pub gl_offs: c_int,
        pub gl_flags: c_int,
        pub gl_pathv: *mut *mut c_char,
        pub gl_errfunc: Option<extern "C" fn(*const c_char, c_int) -> c_int>,
        pub gl_closedir: Option<extern "C" fn(*mut c_void)>,
        pub gl_readdir: Option<extern "C" fn(*mut c_void) -> *mut crate::dirent>,
        pub gl_opendir: Option<extern "C" fn(*const c_char) -> *mut c_void>,
        pub gl_lstat: Option<extern "C" fn(*const c_char, *mut crate::stat) -> c_int>,
        pub gl_stat: Option<extern "C" fn(*const c_char, *mut crate::stat) -> c_int>,
    }

    // libphoenix `link.h` only exposes the load address.
    pub struct dl_phdr_info {
        pub dlpi_addr: c_uint,
    }

    pub struct regmatch_t {
        pub rm_so: crate::regoff_t,
        pub rm_eo: crate::regoff_t,
    }

    pub struct option {
        pub name: *const c_char,
        pub has_arg: c_int,
        pub flag: *mut c_int,
        pub val: c_int,
    }

    pub struct __c_anonymous_ifru_map {
        pub mem_start: c_ulong,
        pub mem_end: c_ulong,
        pub base_addr: c_ushort,
        pub irq: c_uchar,
        pub dma: c_uchar,
        pub port: c_uchar,
    }
}

s_no_extra_traits! {
    pub union __c_anonymous_ifr_ifru {
        pub ifru_addr: crate::sockaddr,
        pub ifru_dstaddr: crate::sockaddr,
        pub ifru_broadaddr: crate::sockaddr,
        pub ifru_netmask: crate::sockaddr,
        pub ifru_hwaddr: crate::sockaddr,
        pub ifru_flags: c_short,
        pub ifru_ifindex: c_int,
        pub ifru_metric: c_int,
        pub ifru_mtu: c_int,
        pub ifru_map: __c_anonymous_ifru_map,
        pub ifru_slave: [c_char; crate::IFNAMSIZ],
        pub ifru_newname: [c_char; crate::IFNAMSIZ],
        pub ifru_data: *mut c_char,
    }

    pub struct ifreq {
        /// interface name, e.g. "en0"
        pub ifr_name: [c_char; crate::IFNAMSIZ],
        pub ifr_ifru: __c_anonymous_ifr_ifru,
    }

    pub union __c_anonymous_ifc_ifcu {
        pub ifcu_buf: *mut c_char,
        pub ifcu_req: *mut crate::ifreq,
    }

    /// Structure used in SIOCGIFCONF request.  Used to retrieve interface configuration for
    /// machine (useful for programs which must know all networks accessible).
    pub struct ifconf {
        /// Size of buffer
        pub ifc_len: c_int,
        pub ifc_ifcu: __c_anonymous_ifc_ifcu,
    }
}

s! {
    pub struct __c_anonymous__kernel_fsid_t {
        pub val: [c_int; 2],
    }

    // `phoenix/threads.h`: `typedef struct lockAttr pthread_mutexattr_t;`
    pub struct pthread_mutexattr_t {
        type_: c_uchar,
        protocol: c_uchar,
        robust: c_uchar,
        prioceiling: crate::c_schar,
    }

    // `sys/types.h`: `typedef struct { int locked; } pthread_spinlock_t;`
    pub struct pthread_spinlock_t {
        locked: c_int,
    }

    pub struct pthread_rwlockattr_t {
        pshared: c_int,
    }

    pub struct pthread_condattr_t {
        pshared: c_int,
        clock_id: crate::clockid_t,
    }

    pub struct pthread_cond_t {
        condh: crate::handle_t,
        initialized: c_int,
    }

    // `sys/types.h`: a kernel handle plus an initialised flag.
    pub struct pthread_mutex_t {
        mutexh: crate::handle_t,
        initialized: c_int,
    }

    pub struct pthread_rwlock_t {
        lock: crate::handle_t,
        readCond: crate::handle_t,
        writeCond: crate::handle_t,
        readActive: size_t,
        writeActive: size_t,
        writeWaiting: size_t,
        initialized: c_int,
    }
}

s_no_extra_traits! {
    pub union __c_anonymous_iwreq {
        pub ifrn_name: [c_char; crate::IFNAMSIZ],
    }
}

s_no_extra_traits! {
    // Internal, for casts to access union fields
    struct sifields_sigchld {
        si_pid: crate::pid_t,
        si_uid: crate::uid_t,
        si_status: c_int,
        si_utime: c_long,
        si_stime: c_long,
    }

    // Internal, for casts to access union fields
    union sifields {
        _align_pointer: *mut c_void,
        sigchld: sifields_sigchld,
    }

    // Internal, for casts to access union fields. Note that some variants
    // of sifields start with a pointer, which makes the alignment of
    // sifields vary on 32-bit and 64-bit architectures.
    struct siginfo_f {
        _siginfo_base: [c_int; 3],
        sifields: sifields,
    }
}

s! {
    // Field order follows libphoenix's `include/pwd.h`.
    pub struct passwd {
        pub pw_name: *mut c_char,
        pub pw_uid: crate::uid_t,
        pub pw_gid: crate::gid_t,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char,
        pub pw_passwd: *mut c_char,
        pub pw_gecos: *mut c_char,
    }

    pub struct mbstate_t {
        __count: c_int,
        __wchb: [c_char; 4],
    }

    // FIXME(1.0): This should not implement `PartialEq`
    #[allow(unpredictable_function_pointer_comparisons)]
    // `phoenix/signal.h`: the two handlers share an anonymous union, spelled
    // here as one `sighandler_t`.
    pub struct sigaction {
        pub sa_sigaction: crate::sighandler_t,
        pub sa_mask: crate::sigset_t,
        pub sa_flags: c_int,
    }

    // libphoenix `signal.h`.
    //
    // FIXME(union): C implementation uses unions
    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_code: c_int,
        pub si_pid: crate::pid_t,
        pub si_uid: crate::uid_t,
        pub si_addr: *mut c_void,
        pub si_status: c_int,
        pub si_value: crate::sigval,
    }

    // `phoenix/posix-statvfs.h`.
    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_favail: crate::fsfilcnt_t,
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
    }

    // libphoenix `termios.h`.
    pub struct termios {
        pub c_iflag: crate::tcflag_t,
        pub c_oflag: crate::tcflag_t,
        pub c_cflag: crate::tcflag_t,
        pub c_lflag: crate::tcflag_t,
        pub c_cc: [crate::cc_t; crate::NCCS],
        pub c_ispeed: crate::speed_t,
        pub c_ospeed: crate::speed_t,
    }

    pub struct flock {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_start: off_t,
        pub l_len: off_t,
        pub l_pid: crate::pid_t,
    }

    // libphoenix ships the BSD regex implementation.
    pub struct regex_t {
        re_magic: c_int,
        re_nsub: size_t,
        re_endp: *const c_char,
        re_g: *mut c_void,
    }

    // libphoenix `net/route.h`.
    pub struct rtentry {
        pub rt_dst: crate::sockaddr,
        pub rt_gateway: crate::sockaddr,
        pub rt_genmask: crate::sockaddr,
        pub rt_flags: c_short,
        pub rt_metric: c_short,
        pub rt_dev: *mut c_char,
        pub rt_mss: c_ulong,
        pub rt_window: c_ulong,
        pub rt_irtt: c_ushort,
    }

    // netinet/tcp.h
}

s! {
    // `sys/types.h`.
    pub struct pthread_attr_t {
        stackaddr: *mut c_void,
        schedpolicy: c_int,
        priority: c_int,
        detachstate: c_int,
        inheritsched: c_int,
        stacksize: size_t,
        guardsize: size_t,
    }

    // `phoenix/posix-stat.h`.
    pub struct stat {
        pub st_dev: crate::dev_t,
        pub st_ino: crate::ino_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: crate::dev_t,
        pub st_size: off_t,
        pub st_atime: crate::time_t,
        pub st_atime_nsec: c_long,
        pub st_mtime: crate::time_t,
        pub st_mtime_nsec: c_long,
        pub st_ctime: crate::time_t,
        pub st_ctime_nsec: c_long,
        #[cfg(all(target_pointer_width = "32", not(target_arch = "x86")))]
        __st_ctim_pad: Padding<[u8; 4]>,
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt_t,
    }
}

pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;
pub const RAND_MAX: c_int = 2147483647;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 8192;
pub const _IONBF: c_int = 32768;
pub const _IOLBF: c_int = 16384;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 2;
pub const F_SETFD: c_int = 3;
pub const F_GETFL: c_int = 4;
pub const F_SETFL: c_int = 5;

pub const F_DUPFD_CLOEXEC: c_int = 1;

// FIXME(#235): Include file sealing fcntls once we have a way to verify them.

pub const SIGTRAP: c_int = 5;

pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;

pub const CLOCK_REALTIME: crate::clockid_t = 2;
pub const CLOCK_MONOTONIC: crate::clockid_t = 0;
pub const CLOCK_MONOTONIC_RAW: crate::clockid_t = 1;
pub const TIMER_ABSTIME: c_int = 1;

pub const RUSAGE_SELF: c_int = 0;

pub const O_RDONLY: c_int = 0x00001;
pub const O_WRONLY: c_int = 0x00002;
pub const O_RDWR: c_int = 0x00004;
pub const O_ACCMODE: c_int = O_RDONLY | O_WRONLY | O_RDWR;

pub const SOCK_CLOEXEC: c_int = O_CLOEXEC;

pub const S_IFIFO: mode_t = 0o1_0000;
pub const S_IFCHR: mode_t = 0o2_0000;
pub const S_IFBLK: mode_t = 0o6_0000;
pub const S_IFDIR: mode_t = 0o4_0000;
pub const S_IFREG: mode_t = 0o10_0000;
pub const S_IFLNK: mode_t = 0o12_0000;
pub const S_IFSOCK: mode_t = 0o14_0000;
pub const S_IFMT: mode_t = 0o17_0000;
pub const S_IRWXU: mode_t = 0o0700;
pub const S_IXUSR: mode_t = 0o0100;
pub const S_IWUSR: mode_t = 0o0200;
pub const S_IRUSR: mode_t = 0o0400;
pub const S_IRWXG: mode_t = 0o0070;
pub const S_IXGRP: mode_t = 0o0010;
pub const S_IWGRP: mode_t = 0o0020;
pub const S_IRGRP: mode_t = 0o0040;
pub const S_IRWXO: mode_t = 0o0007;
pub const S_IXOTH: mode_t = 0o0001;
pub const S_IWOTH: mode_t = 0o0002;
pub const S_IROTH: mode_t = 0o0004;
pub const F_OK: c_int = 0;
pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGABRT: c_int = 6;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGSEGV: c_int = 11;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;

const SIGEV_MAX_SIZE: usize = 64;
cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        const __ARCH_SIGEV_PREAMBLE_SIZE: usize = 4 * 2 + 8;
    } else {
        const __ARCH_SIGEV_PREAMBLE_SIZE: usize = 4 * 2 + 4;
    }
}
const SIGEV_PAD_SIZE: usize = (SIGEV_MAX_SIZE - __ARCH_SIGEV_PREAMBLE_SIZE) / 4;

pub const PROT_NONE: c_int = 0;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const PROT_EXEC: c_int = 4;

pub const LC_CTYPE: c_int = 2;
pub const LC_NUMERIC: c_int = 4;
pub const LC_TIME: c_int = 5;
pub const LC_COLLATE: c_int = 1;
pub const LC_MONETARY: c_int = 3;
pub const LC_ALL: c_int = 0;

pub const MAP_SHARED: c_int = 0x0;
pub const MAP_PRIVATE: c_int = 0x0;

pub const MS_MGC_VAL: c_ulong = 0xc0ed0000;

pub const SCM_RIGHTS: c_int = 0x01;
pub const SCM_CREDENTIALS: c_int = 0x02;

pub const IFF_UP: c_int = 0x1;
pub const IFF_BROADCAST: c_int = 0x2;
pub const IFF_DEBUG: c_int = 0x4;
pub const IFF_LOOPBACK: c_int = 0x8;
pub const IFF_POINTOPOINT: c_int = 0x10;
pub const IFF_NOTRAILERS: c_int = 0x20;
pub const IFF_RUNNING: c_int = 0x40;
pub const IFF_NOARP: c_int = 0x80;
pub const IFF_PROMISC: c_int = 0x100;
pub const IFF_ALLMULTI: c_int = 0x200;
pub const IFF_MASTER: c_int = 0x1000;
pub const IFF_SLAVE: c_int = 0x2000;
pub const IFF_MULTICAST: c_int = 0x4000;
pub const IFF_DYNAMIC: c_int = 0x8000;

pub const SOL_IPV6: c_int = 41;
pub const SOL_RAW: c_int = 255;

pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_LOCAL: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const AF_KEY: c_int = 15;
pub const AF_PACKET: c_int = 17;

pub const PF_UNSPEC: c_int = AF_UNSPEC;
pub const PF_UNIX: c_int = AF_UNIX;
pub const PF_INET: c_int = AF_INET;
pub const PF_INET6: c_int = AF_INET6;
pub const PF_KEY: c_int = AF_KEY;
pub const PF_PACKET: c_int = AF_PACKET;

pub const MSG_OOB: c_int = 0x4;
pub const MSG_PEEK: c_int = 0x1;
pub const MSG_CTRUNC: c_int = 8;
pub const MSG_TRUNC: c_int = 0x20;
pub const MSG_DONTWAIT: c_int = 0x8;
pub const MSG_WAITALL: c_int = 0x2;
pub const MSG_NOSIGNAL: c_int = 0x20;
pub const MSG_MORE: c_int = 0x10;
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;

// `netinet/in.h` defines these in an anonymous enum.
pub const IP_TOS: c_int = 1;
pub const IP_TTL: c_int = 2;
pub const IP_MULTICAST_TTL: c_int = 5;
pub const IP_MULTICAST_IF: c_int = 6;
pub const IP_IPSEC_POLICY: c_int = 16;

// Past this point the libphoenix headers stop, but the lwIP socket layer
// underneath implements these (lib-lwip/src/include/lwip/sockets.h).

pub const IP_MULTICAST_LOOP: c_int = 7;
pub const IP_ADD_MEMBERSHIP: c_int = 3;
pub const IP_DROP_MEMBERSHIP: c_int = 4;

// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: c_int = 2;
/// for compatibility
pub const IPPROTO_IPIP: c_int = 4;
/// DCCP
pub const IPPROTO_DCCP: c_int = 33;
/// General Routing Encap.
pub const IPPROTO_GRE: c_int = 47;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: c_int = 51;
/// SCTP
pub const IPPROTO_SCTP: c_int = 132;
pub const IPPROTO_UDPLITE: c_int = 136;
/// raw IP packet
pub const IPPROTO_RAW: c_int = 255;

pub const IPV6_CHECKSUM: c_int = 7;
pub const IPV6_UNICAST_HOPS: c_int = 0x20;
pub const IPV6_MULTICAST_IF: c_int = 0x8;
pub const IPV6_MULTICAST_HOPS: c_int = 0x4;
pub const IPV6_MULTICAST_LOOP: c_int = 0x10;
pub const IPV6_JOIN_GROUP: c_int = 0x1;
pub const IPV6_ADD_MEMBERSHIP: c_int = IPV6_JOIN_GROUP;
pub const IPV6_LEAVE_GROUP: c_int = 0x2;
pub const IPV6_DROP_MEMBERSHIP: c_int = IPV6_LEAVE_GROUP;
pub const IPV6_V6ONLY: c_int = 0x40;
pub const IPV6_HOPLIMIT: c_int = 52;

pub const TCP_NODELAY: c_int = 1;
pub const TCP_KEEPIDLE: c_int = 0x3;
pub const TCP_KEEPINTVL: c_int = 0x4;
pub const TCP_KEEPCNT: c_int = 0x5;
pub const TCP_DEFER_ACCEPT: c_int = 9;
pub const TCP_QUICKACK: c_int = 12;

pub const SO_DEBUG: c_int = 1;

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const LOCK_SH: c_int = 1;
pub const LOCK_EX: c_int = 2;
pub const LOCK_NB: c_int = 4;
pub const LOCK_UN: c_int = 8;

pub const NAME_MAX: c_int = 256;
pub const PATH_MAX: c_int = 1024;

pub const UIO_MAXIOV: c_int = 1024;

pub const FD_SETSIZE: c_int = 1024;

pub const TCIOFF: c_int = 0x0;
pub const TCION: c_int = 0x1;
pub const TCOOFF: c_int = 0x2;
pub const TCOON: c_int = 0x3;
pub const TCIFLUSH: c_int = 0;
pub const TCOFLUSH: c_int = 1;
pub const TCIOFLUSH: c_int = 2;
pub const NL0: crate::tcflag_t = 256;
pub const NL1: crate::tcflag_t = 512;
pub const TAB0: crate::tcflag_t = 65536;
pub const CR0: crate::tcflag_t = 2048;
pub const FF0: crate::tcflag_t = 134217728;
pub const BS0: crate::tcflag_t = 2097152;
pub const VT0: crate::tcflag_t = 16777216;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VLNEXT: usize = 14;
pub const IGNBRK: crate::tcflag_t = 4;
pub const BRKINT: crate::tcflag_t = 1;
pub const IGNPAR: crate::tcflag_t = 16;
pub const PARMRK: crate::tcflag_t = 8192;
pub const INPCK: crate::tcflag_t = 64;
pub const ISTRIP: crate::tcflag_t = 128;
pub const INLCR: crate::tcflag_t = 32;
pub const IGNCR: crate::tcflag_t = 8;
pub const ICRNL: crate::tcflag_t = 2;
pub const IXANY: crate::tcflag_t = 512;
pub const IMAXBEL: crate::tcflag_t = 4096;
pub const OPOST: crate::tcflag_t = 0x1;
pub const CS5: crate::tcflag_t = 0x00000000;
pub const ECHO: crate::tcflag_t = 1;
pub const OCRNL: crate::tcflag_t = 0o000010;
pub const ONOCR: crate::tcflag_t = 0o000020;
pub const ONLRET: crate::tcflag_t = 0o000040;
pub const OFILL: crate::tcflag_t = 0o000100;

pub const WNOHANG: c_int = 0x00000001;
pub const WUNTRACED: c_int = 0x00000002;
pub const WEXITED: c_int = 0x00000004;
pub const WCONTINUED: c_int = 0x00000008;

pub const RTLD_LOCAL: c_int = 0;
pub const RTLD_LAZY: c_int = 1;

pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_EMPTY_PATH: c_int = 0x1000;

pub const LOG_PERROR: c_int = 0x20;

pub const PIPE_BUF: usize = 4096;

// si_code values for SIGCHLD signal
pub const CLD_EXITED: c_int = 1;
pub const CLD_KILLED: c_int = 2;
pub const CLD_DUMPED: c_int = 3;
pub const CLD_TRAPPED: c_int = 4;
pub const CLD_STOPPED: c_int = 5;
pub const CLD_CONTINUED: c_int = 6;

pub const UTIME_OMIT: c_long = 1073741822;

pub const POLLIN: c_short = 0x1;
pub const POLLPRI: c_short = 0x8;
pub const POLLOUT: c_short = 0x10;
pub const POLLERR: c_short = 0x80;
pub const POLLHUP: c_short = 0x100;
pub const POLLNVAL: c_short = 0x200;
pub const POLLRDNORM: c_short = 0x2;
pub const POLLRDBAND: c_short = 0x4;

pub const ARPOP_RREQUEST: u16 = 3;
pub const ARPOP_RREPLY: u16 = 4;
pub const ARPOP_InREQUEST: u16 = 8;
pub const ARPOP_InREPLY: u16 = 9;
pub const ARPOP_NAK: u16 = 10;

pub const ARPHRD_NETROM: u16 = 0;
pub const ARPHRD_ETHER: u16 = 1;
pub const ARPHRD_EETHER: u16 = 2;
pub const ARPHRD_AX25: u16 = 3;
pub const ARPHRD_PRONET: u16 = 4;
pub const ARPHRD_CHAOS: u16 = 5;
pub const ARPHRD_IEEE802: u16 = 6;
pub const ARPHRD_ARCNET: u16 = 7;
pub const ARPHRD_APPLETLK: u16 = 8;
pub const ARPHRD_DLCI: u16 = 15;
pub const ARPHRD_ATM: u16 = 19;
pub const ARPHRD_METRICOM: u16 = 23;
pub const ARPHRD_IEEE1394: u16 = 24;
pub const ARPHRD_EUI64: u16 = 27;
pub const ARPHRD_INFINIBAND: u16 = 32;

pub const ARPHRD_CSLIP: u16 = 0x80;
pub const ARPHRD_CSLIP6: u16 = 0x81;
pub const ARPHRD_PPP: u16 = 0x82;

pub const ARPHRD_LOOPBACK: u16 = 0x83;
pub const ARPHRD_SIT: u16 = 0x84;

pub const IOCPARM_MASK: c_ulong = 0x1fff;

pub const IOC_VOID: c_ulong = 0x00000000;
pub const IOC_NESTED: c_ulong = 0x20000000;
pub const IOC_OUT: c_ulong = 0x40000000;
pub const IOC_IN: c_ulong = 0x80000000;
pub const IOC_INOUT: c_ulong = IOC_IN | IOC_OUT;
pub const IOC_DIRMASK: c_ulong = 0xc0000000;

/// `phoenix/ioctl.h`: `TTY_IOC_TYPE`.
pub const TTY_IOC_TYPE: c_ulong = b't' as c_ulong;
/// `phoenix/ioctl.h`: `SOCK_IOC_TYPE`.
pub const SOCK_IOC_TYPE: c_ulong = b'S' as c_ulong;

pub const RUSAGE_CHILDREN: c_int = -1;

pub const L_tmpnam: c_uint = 20;
pub const _PC_LINK_MAX: c_int = 1;
pub const _PC_MAX_CANON: c_int = 2;
pub const _PC_MAX_INPUT: c_int = 3;
pub const _PC_NAME_MAX: c_int = 22;
pub const _PC_PATH_MAX: c_int = 4;
pub const _PC_PIPE_BUF: c_int = 5;
pub const _PC_CHOWN_RESTRICTED: c_int = 14;
pub const _PC_NO_TRUNC: c_int = 15;
pub const _PC_VDISABLE: c_int = 16;
pub const _PC_SYNC_IO: c_int = 20;
pub const _PC_ASYNC_IO: c_int = 17;
pub const _PC_PRIO_IO: c_int = 19;
pub const _PC_FILESIZEBITS: c_int = 0;
pub const _PC_REC_INCR_XFER_SIZE: c_int = 8;
pub const _PC_REC_MAX_XFER_SIZE: c_int = 9;
pub const _PC_REC_MIN_XFER_SIZE: c_int = 10;
pub const _PC_REC_XFER_ALIGN: c_int = 11;
pub const _PC_ALLOC_SIZE_MIN: c_int = 7;
pub const _PC_SYMLINK_MAX: c_int = 12;
pub const _PC_2_SYMLINKS: c_int = 6;

pub const _SC_CLK_TCK: c_int = 3;
pub const _SC_OPEN_MAX: c_int = 0;
pub const _SC_PAGESIZE: c_int = 4;
pub const _SC_PAGE_SIZE: c_int = _SC_PAGESIZE;
pub const _SC_IOV_MAX: c_int = 1;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 70;
pub const _SC_THREAD_STACK_MIN: c_int = 75;
pub const _SC_NPROCESSORS_ONLN: c_int = 84;
pub const _SC_ATEXIT_MAX: c_int = 2;
pub const _SC_SPIN_LOCKS: c_int = 5;

// `phoenix/limits.h`

pub const _SC_HOST_NAME_MAX: c_int = 180;

pub const _CS_PATH: c_int = 0;

pub const AT_HWCAP: c_ulong = 16;

pub const AT_HWCAP2: c_ulong = 26;

pub const AT_MINSIGSTKSZ: c_ulong = 51;

pub const GLOB_ERR: c_int = 0x04;
pub const GLOB_MARK: c_int = 0x08;
pub const GLOB_NOSORT: c_int = 0x20;
pub const GLOB_DOOFFS: c_int = 0x02;
pub const GLOB_NOCHECK: c_int = 1 << 4;
pub const GLOB_APPEND: c_int = 0x01;

pub const S_IEXEC: crate::mode_t = 0o0100;
pub const S_IWRITE: crate::mode_t = 0o0200;
pub const S_IREAD: crate::mode_t = 0o0400;

pub const RTLD_DEFAULT: *mut c_void = ptr::null_mut();

pub const PTHREAD_ONCE_INIT: crate::pthread_once_t = 1;
pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_MUTEX_STALLED: c_int = 0;

// `pthread.h` spells these `{ 0, 0 }`, `{ 0, 0 }` and `{ 0 }`.
pub const PTHREAD_MUTEX_INITIALIZER: crate::pthread_mutex_t = crate::pthread_mutex_t {
    mutexh: 0,
    initialized: 0,
};
pub const PTHREAD_COND_INITIALIZER: crate::pthread_cond_t = crate::pthread_cond_t {
    condh: 0,
    initialized: 0,
};
pub const PTHREAD_RWLOCK_INITIALIZER: crate::pthread_rwlock_t = crate::pthread_rwlock_t {
    lock: 0,
    readCond: 0,
    writeCond: 0,
    readActive: 0,
    writeActive: 0,
    writeWaiting: 0,
    initialized: 0,
};
pub const PTHREAD_MUTEX_ROBUST: c_int = 1;
pub const PTHREAD_PRIO_NONE: c_int = 0x1;
pub const PTHREAD_PRIO_INHERIT: c_int = 0x0;
pub const PTHREAD_PRIO_PROTECT: c_int = 2;
pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_PROCESS_SHARED: c_int = 1;
pub const PTHREAD_INHERIT_SCHED: c_int = 1;
pub const PTHREAD_EXPLICIT_SCHED: c_int = 0;

// netinet/in.h
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

#[deprecated(
    since = "0.2.80",
    note = "This value was increased in the newer kernel \
            and we'll change this following upstream in the future release. \
            See #1896 for more info."
)]
pub const IPPROTO_MAX: c_int = 255;

pub const LOG_NFACILITIES: c_int = 24;

pub const EAI_BADFLAGS: c_int = -1;
pub const EAI_NONAME: c_int = -2;
pub const EAI_AGAIN: c_int = -3;
pub const EAI_FAIL: c_int = -4;
pub const EAI_NODATA: c_int = -5;
pub const EAI_FAMILY: c_int = -6;
pub const EAI_SOCKTYPE: c_int = -7;
pub const EAI_SERVICE: c_int = -8;
pub const EAI_MEMORY: c_int = -10;
pub const EAI_SYSTEM: c_int = -11;
pub const EAI_OVERFLOW: c_int = -12;

pub const _POSIX_VDISABLE: crate::cc_t = 0;

pub const PACKET_HOST: c_uchar = 0;
pub const PACKET_BROADCAST: c_uchar = 1;
pub const PACKET_MULTICAST: c_uchar = 2;
pub const PACKET_OTHERHOST: c_uchar = 3;
pub const PACKET_OUTGOING: c_uchar = 4;
pub const PACKET_LOOPBACK: c_uchar = 5;

// `sys/sockios.h`
pub const SIOCGIFNAME: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x10);
pub const SIOCGIFCONF: Ioctl = _IOC_NESTED::<ifconf>(IOC_INOUT, SOCK_IOC_TYPE, 0x12);
pub const SIOCGIFFLAGS: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x13);
pub const SIOCSIFFLAGS: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x14);
pub const SIOCGIFADDR: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x15);
pub const SIOCSIFADDR: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x16);
pub const SIOCGIFDSTADDR: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x17);
pub const SIOCSIFDSTADDR: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x18);
pub const SIOCGIFBRDADDR: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x19);
pub const SIOCSIFBRDADDR: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x1a);
pub const SIOCGIFNETMASK: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x1b);
pub const SIOCSIFNETMASK: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x1c);
pub const SIOCGIFMETRIC: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x1d);
pub const SIOCSIFMETRIC: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x1e);
pub const SIOCGIFMTU: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x21);
pub const SIOCSIFMTU: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x22);
pub const SIOCSIFHWADDR: Ioctl = _IOW::<ifreq>(SOCK_IOC_TYPE, 0x24);
pub const SIOCGIFHWADDR: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x27);
pub const SIOCADDMULTI: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x31);
pub const SIOCDELMULTI: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x32);
pub const SIOCGIFINDEX: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x33);
pub const SIOCGIFTXQLEN: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x42);
pub const SIOCSIFTXQLEN: Ioctl = _IOWR::<ifreq>(SOCK_IOC_TYPE, 0x43);
pub const SIOCADDRT: Ioctl = _IOC_NESTED::<rtentry>(IOC_IN, SOCK_IOC_TYPE, 0x44);
pub const SIOCDELRT: Ioctl = _IOC_NESTED::<rtentry>(IOC_IN, SOCK_IOC_TYPE, 0x45);

// `sys/ioctl.h`
pub const FIONREAD: Ioctl = _IOR::<c_int>(b'f' as c_ulong, 127);
pub const FIONBIO: Ioctl = _IOW::<c_ulong>(b'f' as c_ulong, 126);
pub const TIOCGPTN: Ioctl = _IOR::<c_uint>(b'T' as c_ulong, 0x30);
pub const TIOCSPTLCK: Ioctl = _IOW::<c_int>(b'T' as c_ulong, 0x31);

pub const RTF_UP: c_ushort = 0x0001;
pub const RTF_GATEWAY: c_ushort = 0x0002;

pub const RTF_HOST: c_ushort = 0x0004;
pub const RTF_REINSTATE: c_ushort = 0x0008;
pub const RTF_DYNAMIC: c_ushort = 0x0010;
pub const RTF_MODIFIED: c_ushort = 0x0020;
pub const RTF_MTU: c_ushort = 0x0040;
pub const RTF_MSS: c_ushort = RTF_MTU;
pub const RTF_WINDOW: c_ushort = 0x0080;
pub const RTF_IRTT: c_ushort = 0x0100;
pub const RTF_REJECT: c_ushort = 0x0200;

pub const RTF_DEFAULT: u32 = 0x400;
pub const RTF_ADDRCONF: u32 = 0x800;
pub const RTF_NONEXTHOP: u32 = 0x00200000;
pub const RTF_CACHE: u32 = 0x1000;

pub const REG_EXTENDED: c_int = 1;
pub const REG_ICASE: c_int = 2;
pub const REG_NEWLINE: c_int = 8;
pub const REG_NOSUB: c_int = 4;

pub const REG_NOTBOL: c_int = 1;
pub const REG_NOTEOL: c_int = 2;

pub const REG_NOMATCH: c_int = 1;
pub const REG_BADPAT: c_int = 2;
pub const REG_ECOLLATE: c_int = 3;
pub const REG_ECTYPE: c_int = 4;
pub const REG_EESCAPE: c_int = 5;
pub const REG_ESUBREG: c_int = 6;
pub const REG_EBRACK: c_int = 7;
pub const REG_EPAREN: c_int = 8;
pub const REG_EBRACE: c_int = 9;
pub const REG_BADBR: c_int = 10;
pub const REG_ERANGE: c_int = 11;
pub const REG_ESPACE: c_int = 12;
pub const REG_BADRPT: c_int = 13;

// errno.h
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const ENOTBLK: c_int = 15;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOTTY: c_int = 25;
pub const ETXTBSY: c_int = 26;
pub const EFBIG: c_int = 27;
pub const ENOSPC: c_int = 28;
pub const ESPIPE: c_int = 29;
pub const EROFS: c_int = 30;
pub const EMLINK: c_int = 31;
pub const EPIPE: c_int = 32;
pub const EDOM: c_int = 33;
pub const ERANGE: c_int = 34;
pub const EWOULDBLOCK: c_int = EAGAIN;

pub const SCHED_OTHER: c_int = 2;
pub const SCHED_FIFO: c_int = 0;
pub const SCHED_RR: c_int = 1;

// `phoenix/limits.h`
pub const HOST_NAME_MAX: c_uint = 255;

pub const POSIX_SPAWN_SETSID: c_short = 128;

pub const RB_AUTOBOOT: c_int = 1;
pub const RB_HALT_SYSTEM: c_int = 0;

pub const PIDFD_INFO_PID: c_uint = 1 << 0;
pub const PIDFD_INFO_EXIT: c_uint = 1 << 3;

pub const GRND_NONBLOCK: c_uint = 0x0001;
pub const GRND_INSECURE: c_uint = 0x0004;

pub const ETH_ALEN: c_int = 6;
pub const ETH_HLEN: c_int = 14;
pub const ETH_ZLEN: c_int = 60;
pub const ETH_DATA_LEN: c_int = 1500;
pub const ETH_FRAME_LEN: c_int = 1514;
pub const ETH_FCS_LEN: c_int = 4;

pub const ETH_P_ALL: c_int = 0x0003;

pub const POSIX_SPAWN_SETPGROUP: c_short = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: c_short = 0x04;

pub const CTL_KERN: c_int = 1;

// include/sys/mman.h

pub const NCCS: usize = 17;

pub const O_TRUNC: c_int = 0x00200;
pub const O_CLOEXEC: c_int = 0x04000;

pub const ENODATA: c_int = 61;
pub const ETIME: c_int = 62;
pub const ENONET: c_int = 64;
pub const EPROTO: c_int = 71;

pub const F_RDLCK: c_int = 0;
pub const F_WRLCK: c_int = 2;
pub const F_UNLCK: c_int = 1;

pub const SA_NODEFER: c_int = 0x4;
pub const SA_RESETHAND: c_int = 0x10;
pub const SA_RESTART: c_int = 0x20;
pub const SA_NOCLDSTOP: c_int = 0x00000001;

pub const BUFSIZ: c_uint = 4096;
pub const TMP_MAX: c_uint = 32767;
pub const FOPEN_MAX: c_uint = 16;
pub const FILENAME_MAX: c_uint = 128;
pub const O_NDELAY: c_int = O_NONBLOCK;
pub const NI_MAXHOST: crate::socklen_t = 1025;
pub const PTHREAD_STACK_MIN: size_t = 256;

pub const SOCK_NONBLOCK: c_int = 0x8000;

pub const SOMAXCONN: c_int = 128;

pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;

pub const B0: crate::speed_t = 0o000000;
pub const B50: crate::speed_t = 50;
pub const B75: crate::speed_t = 75;
pub const B110: crate::speed_t = 110;
pub const B134: crate::speed_t = 134;
pub const B150: crate::speed_t = 150;
pub const B200: crate::speed_t = 200;
pub const B300: crate::speed_t = 300;
pub const B600: crate::speed_t = 600;
pub const B1200: crate::speed_t = 1200;
pub const B1800: crate::speed_t = 1800;
pub const B2400: crate::speed_t = 2400;
pub const B4800: crate::speed_t = 4800;
pub const B9600: crate::speed_t = 9600;
pub const B19200: crate::speed_t = 19200;
pub const B38400: crate::speed_t = 38400;

pub const PRIO_PROCESS: c_int = 0;
pub const PRIO_PGRP: c_int = 1;
pub const PRIO_USER: c_int = 2;

pub const ADJ_OFFSET: c_uint = 0x0001;
pub const ADJ_STATUS: c_uint = 0x2;
pub const ADJ_TIMECONST: c_uint = 0x4;
pub const STA_PLL: c_int = 0x0001;
pub const STA_INS: c_int = 0x2;
pub const STA_DEL: c_int = 0x4;

// `phoenix/posix-socket.h`; lwIP's values.
pub const SOL_SOCKET: c_int = 0xfff;

pub const SO_REUSEADDR: c_int = 0x4;
pub const SO_TYPE: c_int = 0x1008;
pub const SO_ERROR: c_int = 0x1007;
pub const SO_DONTROUTE: c_int = 0x10;
pub const SO_BROADCAST: c_int = 0x20;
pub const SO_SNDBUF: c_int = 0x1001;
pub const SO_RCVBUF: c_int = 0x1002;
pub const SO_KEEPALIVE: c_int = 0x8;
pub const SO_OOBINLINE: c_int = 0x100;
pub const SO_LINGER: c_int = 0x80;
pub const SO_PASSCRED: c_int = 16;
pub const SO_RCVLOWAT: c_int = 0x1004;
pub const SO_SNDLOWAT: c_int = 0x1003;
pub const SO_BINDTODEVICE: c_int = 0x100b;

// `phoenix/posix-socket.h`
pub const SO_RCVTIMEO: c_int = 0x1006;
pub const SO_SNDTIMEO: c_int = 0x1005;

pub const SO_ACCEPTCONN: c_int = 0x2;
pub const SO_MARK: c_int = 36;

// Ioctl Constants

// `termios.h`
pub const TCGETS: Ioctl = _IOR::<termios>(TTY_IOC_TYPE, 0x1);
pub const TCSETS: Ioctl = _IOW::<termios>(TTY_IOC_TYPE, 0x2);
pub const TCSETSW: Ioctl = _IOW::<termios>(TTY_IOC_TYPE, 0x3);
pub const TCSETSF: Ioctl = _IOW::<termios>(TTY_IOC_TYPE, 0x4);
pub const TCFLSH: Ioctl = _IOV::<c_int>(TTY_IOC_TYPE, 0x5);
pub const TCDRAIN: Ioctl = _IO(TTY_IOC_TYPE, 0x6);
pub const TCXONC: Ioctl = _IOV::<c_int>(TTY_IOC_TYPE, 0x7);
pub const TCSBRK: Ioctl = _IOV::<c_int>(TTY_IOC_TYPE, 0x9);
pub const TIOCSCTTY: Ioctl = _IOV::<c_int>(TTY_IOC_TYPE, 0xE);
pub const TIOCGPGRP: Ioctl = _IOR::<crate::pid_t>(TTY_IOC_TYPE, 0xF);
pub const TIOCSPGRP: Ioctl = _IOW::<crate::pid_t>(TTY_IOC_TYPE, 0x10);
pub const TIOCGWINSZ: Ioctl = _IOR::<crate::winsize>(TTY_IOC_TYPE, 0x13);
pub const TIOCSWINSZ: Ioctl = _IOW::<crate::winsize>(TTY_IOC_TYPE, 0x14);
pub const TIOCNOTTY: Ioctl = _IO(TTY_IOC_TYPE, 0x22);
pub const TIOCGSID: Ioctl = _IOR::<crate::pid_t>(TTY_IOC_TYPE, 0x29);
pub const TIOCGHALFD: Ioctl = _IOR::<c_int>(TTY_IOC_TYPE, 0x80);
pub const TIOCSHALFD: Ioctl = _IOV::<c_int>(TTY_IOC_TYPE, 0x81);

pub const IUCLC: crate::tcflag_t = 256;
pub const XCASE: crate::tcflag_t = 512;

// `sys/resource.h`: `enum { RLIM_INFINITY = -1, ... }`
pub const RLIM_INFINITY: crate::rlim_t = -1;

// `phoenix/posix-fcntl.h`
pub const O_APPEND: c_int = 0x00008;
pub const O_CREAT: c_int = 0x00100;
pub const O_EXCL: c_int = 0x00400;
pub const O_NOCTTY: c_int = 0x02000;
pub const O_NONBLOCK: c_int = 0x01000;
pub const O_SYNC: c_int = 0x00800;
pub const O_RSYNC: c_int = 0x08000;
pub const O_DSYNC: c_int = 0x10000;
pub const O_DIRECTORY: c_int = 0;
pub const O_NOFOLLOW: c_int = 0;

pub const SIGSTKSZ: size_t = 8192;

pub const ENAMETOOLONG: c_int = 36;
pub const ENOLCK: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTEMPTY: c_int = 39;
pub const ELOOP: c_int = 40;
pub const ENOMSG: c_int = 42;
pub const EUNATCH: c_int = 49;
pub const EOVERFLOW: c_int = 75;
pub const EBADFD: c_int = 77;
pub const EBADMSG: c_int = 74;
pub const EILSEQ: c_int = 84;
pub const ENOTSOCK: c_int = 88;
pub const EDESTADDRREQ: c_int = 89;
pub const EMSGSIZE: c_int = 90;
pub const EPROTOTYPE: c_int = 91;
pub const ENOPROTOOPT: c_int = 92;
pub const EPROTONOSUPPORT: c_int = 93;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUP: c_int = EOPNOTSUPP;
pub const EPFNOSUPPORT: c_int = 96;
pub const EAFNOSUPPORT: c_int = 97;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ENETRESET: c_int = 102;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOBUFS: c_int = 105;
pub const EISCONN: c_int = 106;
pub const ENOTCONN: c_int = 107;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;
pub const EHOSTDOWN: c_int = 112;
pub const EHOSTUNREACH: c_int = 113;
pub const EALREADY: c_int = 114;
pub const EINPROGRESS: c_int = 115;
pub const ESTALE: c_int = 116;
pub const EUCLEAN: c_int = 117;
pub const EDQUOT: c_int = 122;
pub const EOWNERDEAD: c_int = 130;
pub const ENOTRECOVERABLE: c_int = 131;

pub const SA_ONSTACK: c_int = 0x8;
pub const SA_SIGINFO: c_int = 0x40;
pub const SA_NOCLDWAIT: c_int = 0x00000002;

pub const SIGCHLD: c_int = 20;
pub const SIGBUS: c_int = 10;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGWINCH: c_int = 28;
pub const SIGUSR1: c_int = 30;
pub const SIGUSR2: c_int = 31;
pub const SIGCONT: c_int = 19;
pub const SIGSTOP: c_int = 17;
pub const SIGTSTP: c_int = 18;
pub const SIGURG: c_int = 16;
pub const SIGIO: c_int = 23;
pub const SIGSYS: c_int = 12;
pub const SIGSTKFLT: c_int = 16;
pub const SIGPOLL: c_int = 29;
pub const SIGPWR: c_int = 30;
pub const SIG_SETMASK: c_int = 1;
pub const SIG_BLOCK: c_int = 0x000000;
pub const SIG_UNBLOCK: c_int = 2;

pub const F_GETLK: c_int = 8;
pub const F_GETOWN: c_int = 6;
pub const F_SETLK: c_int = 9;
pub const F_SETLKW: c_int = 10;
pub const F_SETOWN: c_int = 7;

pub const VEOF: usize = 4;

pub const POLLWRNORM: c_short = 0x20;
pub const POLLWRBAND: c_short = 0x40;

pub const MAP_ANON: c_int = 0x40;
pub const MAP_STACK: c_int = 0x020000;

pub const TAB1: c_int = 131072;
pub const TAB2: c_int = 262144;
pub const TAB3: c_int = 524288;
pub const CR1: c_int = 4096;
pub const CR2: c_int = 8192;
pub const CR3: c_int = 16384;
pub const FF1: c_int = 268435456;
pub const BS1: c_int = 4194304;
pub const VT1: c_int = 33554432;
pub const VWERASE: usize = 13;
pub const VREPRINT: usize = 11;
pub const VSUSP: usize = 9;
pub const VSTART: usize = 7;
pub const VSTOP: usize = 8;
pub const VDISCARD: usize = 12;
pub const VTIME: usize = 5;
pub const IXON: crate::tcflag_t = 2048;
pub const IXOFF: crate::tcflag_t = 1024;
pub const ONLCR: crate::tcflag_t = 0x4;
pub const CSIZE: crate::tcflag_t = 3;
pub const CS6: crate::tcflag_t = 1;
pub const CS7: crate::tcflag_t = 2;
pub const CS8: crate::tcflag_t = 3;
pub const CSTOPB: crate::tcflag_t = 8;
pub const CREAD: crate::tcflag_t = 16;
pub const PARENB: crate::tcflag_t = 32;
pub const PARODD: crate::tcflag_t = 64;
pub const HUPCL: crate::tcflag_t = 128;
pub const CLOCAL: crate::tcflag_t = 256;
pub const ECHOKE: crate::tcflag_t = 0x00000800;
pub const ECHOE: crate::tcflag_t = 2;
pub const ECHOK: crate::tcflag_t = 4;
pub const ECHONL: crate::tcflag_t = 8;
pub const ECHOCTL: crate::tcflag_t = 1024;
pub const ISIG: crate::tcflag_t = 64;
pub const ICANON: crate::tcflag_t = 16;
pub const NOFLSH: crate::tcflag_t = 0x00000080;
pub const OLCUC: crate::tcflag_t = 0o000002;
pub const NLDLY: crate::tcflag_t = 128;
pub const CRDLY: crate::tcflag_t = 1024;
pub const TABDLY: crate::tcflag_t = 32768;
pub const BSDLY: crate::tcflag_t = 1048576;
pub const FFDLY: crate::tcflag_t = 67108864;
pub const VTDLY: crate::tcflag_t = 8388608;
pub const XTABS: crate::tcflag_t = 524288;
pub const B57600: crate::speed_t = 57600;
pub const B115200: crate::speed_t = 115200;
pub const B230400: crate::speed_t = 230400;
pub const B460800: crate::speed_t = 460800;

pub const EDEADLK: c_int = 35;
pub const VEOL: usize = 10;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: crate::tcflag_t = 32;
pub const TOSTOP: crate::tcflag_t = 0x00000100;

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for __c_anonymous_ifaddrs_ifa_ifu {
            fn eq(&self, _other: &__c_anonymous_ifaddrs_ifa_ifu) -> bool {
                unimplemented!("traits")
            }
        }
        impl Eq for __c_anonymous_ifaddrs_ifa_ifu {}
        impl hash::Hash for __c_anonymous_ifaddrs_ifa_ifu {
            fn hash<H: hash::Hasher>(&self, _state: &mut H) {
                unimplemented!("traits")
            }
        }
    }
}

/// Recover the argument length an ioctl number carries.
pub const fn IOCPARM_LEN(x: c_ulong) -> c_ulong {
    (x >> 16) & IOCPARM_MASK
}

/// Strip the argument length from an ioctl number.
pub const fn IOCBASECMD(x: c_ulong) -> c_ulong {
    x & !(IOCPARM_MASK << 16)
}

/// The group letter an ioctl number carries.
pub const fn IOCGROUP(x: c_ulong) -> c_ulong {
    (x >> 8) & 0xff
}

/// Build an ioctl number, analogous to the C macro of the same name.
pub const fn _IOC(inout: c_ulong, group: c_ulong, num: c_ulong, len: usize) -> Ioctl {
    (inout | (((len as c_ulong) & IOCPARM_MASK) << 16) | (group << 8) | num) as Ioctl
}

/// Build an ioctl number for an argumentless ioctl.
pub const fn _IO(group: c_ulong, num: c_ulong) -> Ioctl {
    _IOC(IOC_VOID, group, num, 0)
}

/// Build an ioctl number whose argument is passed by value.
pub const fn _IOV<T>(group: c_ulong, num: c_ulong) -> Ioctl {
    _IOC(IOC_VOID, group, num, size_of::<T>())
}

/// Build an ioctl number for a read-only ioctl.
pub const fn _IOR<T>(group: c_ulong, num: c_ulong) -> Ioctl {
    _IOC(IOC_OUT, group, num, size_of::<T>())
}

/// Build an ioctl number for a write-only ioctl.
pub const fn _IOW<T>(group: c_ulong, num: c_ulong) -> Ioctl {
    _IOC(IOC_IN, group, num, size_of::<T>())
}

/// Build an ioctl number for a read-write ioctl.
pub const fn _IOWR<T>(group: c_ulong, num: c_ulong) -> Ioctl {
    _IOC(IOC_INOUT, group, num, size_of::<T>())
}

/// Build an ioctl number whose argument is itself passed by reference.
pub const fn _IOC_NESTED<T>(inout: c_ulong, group: c_ulong, num: c_ulong) -> Ioctl {
    _IOC(inout | IOC_NESTED, group, num, size_of::<T>())
}

const fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut c_void {
        #[repr(C)]
        struct siginfo_sigfault {
            _si_signo: c_int,
            _si_errno: c_int,
            _si_code: c_int,
            si_addr: *mut c_void,
        }
        (*(self as *const siginfo_t as *const siginfo_sigfault)).si_addr
    }

    pub unsafe fn si_value(&self) -> crate::sigval {
        #[repr(C)]
        struct siginfo_si_value {
            _si_signo: c_int,
            _si_errno: c_int,
            _si_code: c_int,
            _si_timerid: c_int,
            _si_overrun: c_int,
            si_value: crate::sigval,
        }
        (*(self as *const siginfo_t as *const siginfo_si_value)).si_value
    }
}

impl siginfo_t {
    unsafe fn sifields(&self) -> &sifields {
        &(*(self as *const siginfo_t as *const siginfo_f)).sifields
    }

    pub unsafe fn si_pid(&self) -> crate::pid_t {
        self.sifields().sigchld.si_pid
    }

    pub unsafe fn si_uid(&self) -> crate::uid_t {
        self.sifields().sigchld.si_uid
    }

    pub unsafe fn si_status(&self) -> c_int {
        self.sifields().sigchld.si_status
    }

    pub unsafe fn si_utime(&self) -> c_long {
        self.sifields().sigchld.si_utime
    }

    pub unsafe fn si_stime(&self) -> c_long {
        self.sifields().sigchld.si_stime
    }
}

f! {
    pub unsafe fn FD_CLR(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] &= !(1 << (fd % size));
        return;
    }

    pub unsafe fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool {
        let fd = fd as usize;
        let size = size_of_val(&(*set).fds_bits[0]) * 8;
        return ((*set).fds_bits[fd / size] & (1 << (fd % size))) != 0;
    }

    pub unsafe fn FD_SET(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] |= 1 << (fd % size);
        return;
    }

    pub unsafe fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in &mut (*set).fds_bits {
            *slot = 0;
        }
    }

    pub const safe fn WIFSTOPPED(status: c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub const safe fn WSTOPSIG(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const safe fn WIFCONTINUED(status: c_int) -> bool {
        status == 0xffff
    }

    pub const safe fn WIFSIGNALED(status: c_int) -> bool {
        ((status & 0x7f) + 1) as i8 >= 2
    }

    pub const safe fn WTERMSIG(status: c_int) -> c_int {
        status & 0x7f
    }

    pub const safe fn WIFEXITED(status: c_int) -> bool {
        (status & 0x7f) == 0
    }

    pub const safe fn WEXITSTATUS(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub const safe fn WCOREDUMP(status: c_int) -> bool {
        (status & 0x80) != 0
    }

    pub const safe fn W_EXITCODE(ret: c_int, sig: c_int) -> c_int {
        (ret << 8) | sig
    }

    pub const safe fn W_STOPCODE(sig: c_int) -> c_int {
        (sig << 8) | 0x7f
    }

    pub const safe fn QCMD(cmd: c_int, type_: c_int) -> c_int {
        (cmd << 8) | (type_ & 0x00ff)
    }

    #[allow(ellipsis_inclusive_range_patterns)]
    pub const safe fn KERNEL_VERSION(a: u32, b: u32, c: u32) -> u32 {
        ((a << 16) + (b << 8)) + if c > 255 { 255 } else { c }
    }

    pub unsafe fn CMSG_NXTHDR(
        mhdr: *const crate::msghdr,
        cmsg: *const crate::cmsghdr,
    ) -> *mut crate::cmsghdr {
        if ((*cmsg).cmsg_len as usize) < size_of::<crate::cmsghdr>() {
            return core::ptr::null_mut::<crate::cmsghdr>();
        }

        // FIXME(msrv): `.wrapping_byte_add()` stabilized in 1.75
        let next_cmsg = cmsg
            .cast::<u8>()
            .wrapping_add(CMSG_ALIGN((*cmsg).cmsg_len as usize))
            .cast::<crate::cmsghdr>();

        // In case the addition wrapped. `next_addr > max_addr`
        // would otherwise not work as intended.
        if (next_cmsg as usize) < (cmsg as usize) {
            return core::ptr::null_mut();
        }

        let max_addr = (*mhdr).msg_control as usize + (*mhdr).msg_controllen as usize;

        if next_cmsg as usize + size_of::<crate::cmsghdr>() > max_addr {
            core::ptr::null_mut::<crate::cmsghdr>()
        } else {
            next_cmsg as *mut crate::cmsghdr
        }
    }

    pub unsafe fn RT_ADDRCLASS(flags: u32) -> u32 {
        flags >> 23
    }
}

extern "C" {
    pub fn ioctl(fd: c_int, request: Ioctl, ...) -> c_int;
}

extern "C" {
    pub fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int;
    pub fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int;

    pub fn setgroups(ngroups: size_t, ptr: *const crate::gid_t) -> c_int;
    pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn clearenv() -> c_int;
    pub fn execvpe(
        file: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    pub fn getifaddrs(ifap: *mut *mut crate::ifaddrs) -> c_int;
    pub fn freeifaddrs(ifa: *mut crate::ifaddrs);
    pub fn bind(
        socket: c_int,
        address: *const crate::sockaddr,
        address_len: crate::socklen_t,
    ) -> c_int;

    pub fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    pub fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;

    pub fn uname(buf: *mut crate::utsname) -> c_int;

    pub fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;

    pub fn strftime(
        s: *mut c_char,
        max: size_t,
        format: *const c_char,
        tm: *const crate::tm,
    ) -> size_t;
    pub fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char;

    pub fn mkostemp(template: *mut c_char, flags: c_int) -> c_int;
    pub fn mkostemps(template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int;

    pub fn getpwnam_r(
        name: *const c_char,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn getpwuid_r(
        uid: crate::uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
}

// `sys/sysmacros.h`: functions in libphoenix.
extern "C" {
    pub fn makedev(major: c_uint, minor: c_uint) -> crate::dev_t;
    pub fn major(dev: crate::dev_t) -> c_uint;
    pub fn minor(dev: crate::dev_t) -> c_uint;
}

extern "C" {
    pub fn getnameinfo(
        sa: *const crate::sockaddr,
        salen: crate::socklen_t,
        host: *mut c_char,
        hostlen: crate::socklen_t,
        serv: *mut c_char,
        servlen: crate::socklen_t,
        flags: c_int,
    ) -> c_int;
    pub fn futimes(fd: c_int, times: *const crate::timeval) -> c_int;
}

extern "C" {
    // POSIX `strerror_r`.
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;

    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);

    pub fn setpwent();
    pub fn endpwent();
    pub fn getpwent() -> *mut crate::passwd;
    pub fn setgrent();
    pub fn endgrent();
    pub fn getgrent() -> *mut crate::group;

    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    pub fn __errno_location() -> *mut c_int;

    pub fn glob(
        pattern: *const c_char,
        flags: c_int,
        errfunc: Option<extern "C" fn(epath: *const c_char, errno: c_int) -> c_int>,
        pglob: *mut crate::glob_t,
    ) -> c_int;
    pub fn globfree(pglob: *mut crate::glob_t);

    pub fn seekdir(dirp: *mut crate::DIR, loc: c_long);

    pub fn telldir(dirp: *mut crate::DIR) -> c_long;

    pub fn recvfrom(
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        addr: *mut crate::sockaddr,
        addrlen: *mut crate::socklen_t,
    ) -> ssize_t;

    pub fn sched_get_priority_max(policy: c_int) -> c_int;
    pub fn settimeofday(tv: *const crate::timeval, tz: *const crate::timezone) -> c_int;
    pub fn mount(
        src: *const c_char,
        target: *const c_char,
        fstype: *const c_char,
        flags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    pub fn sethostname(name: *const c_char, len: size_t) -> c_int;
    pub fn sched_get_priority_min(policy: c_int) -> c_int;
    pub fn sigsuspend(mask: *const crate::sigset_t) -> c_int;
    pub fn getgrgid_r(
        gid: crate::gid_t,
        grp: *mut crate::group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut crate::group,
    ) -> c_int;
    pub fn getgrnam_r(
        name: *const c_char,
        grp: *mut crate::group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut crate::group,
    ) -> c_int;
    pub fn initgroups(user: *const c_char, group: crate::gid_t) -> c_int;
    pub fn getgrnam(name: *const c_char) -> *mut crate::group;
    pub fn getgrgid(gid: crate::gid_t) -> *mut crate::group;
    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut crate::FILE;

    pub fn regcomp(preg: *mut crate::regex_t, pattern: *const c_char, cflags: c_int) -> c_int;

    pub fn regexec(
        preg: *const crate::regex_t,
        input: *const c_char,
        nmatch: size_t,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;

    pub fn regerror(
        errcode: c_int,
        preg: *const crate::regex_t,
        errbuf: *mut c_char,
        errbuf_size: size_t,
    ) -> size_t;

    pub fn regfree(preg: *mut crate::regex_t);

    pub fn gettid() -> crate::pid_t;

    pub fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
}

extern "C" {
    pub fn lutimes(file: *const c_char, times: *const crate::timeval) -> c_int;

    pub fn accept4(fd: c_int, addr: *mut crate::sockaddr, len: *mut socklen_t, flg: c_int)
        -> c_int;
    pub fn reboot(how_to: c_int) -> c_int;

    pub fn mkstemps(template: *mut c_char, suffixlen: c_int) -> c_int;

    pub fn sync();
    pub fn umount(target: *const c_char) -> c_int;

    pub fn sched_rr_get_interval(pid: crate::pid_t, tp: *mut crate::timespec) -> c_int;
    pub fn sched_setparam(pid: crate::pid_t, param: *const crate::sched_param) -> c_int;
    pub fn sched_getparam(pid: crate::pid_t, param: *mut crate::sched_param) -> c_int;
    pub fn sched_getscheduler(pid: crate::pid_t) -> c_int;
    pub fn clock_nanosleep(
        clk_id: crate::clockid_t,
        flags: c_int,
        rqtp: *const crate::timespec,
        rmtp: *mut crate::timespec,
    ) -> c_int;
    pub fn sched_setscheduler(
        pid: crate::pid_t,
        policy: c_int,
        param: *const crate::sched_param,
    ) -> c_int;

    pub fn fread_unlocked(
        buf: *mut c_void,
        size: size_t,
        nobj: size_t,
        stream: *mut crate::FILE,
    ) -> size_t;

    pub fn gethostid() -> c_long;
}

extern "C" {
    pub fn getrlimit(resource: c_int, rlim: *mut crate::rlimit) -> c_int;
    pub fn setrlimit(resource: c_int, rlim: *const crate::rlimit) -> c_int;
    pub fn gettimeofday(tp: *mut crate::timeval, tz: *mut c_void) -> c_int;
    pub fn getpriority(which: c_int, who: crate::id_t) -> c_int;
    pub fn setpriority(which: c_int, who: crate::id_t, prio: c_int) -> c_int;

    pub fn malloc_usable_size(ptr: *mut c_void) -> size_t;

    pub fn asctime_r(tm: *const crate::tm, buf: *mut c_char) -> *mut c_char;

    pub fn dirname(path: *mut c_char) -> *mut c_char;
    pub fn basename(path: *mut c_char) -> *mut c_char;
}

mod lfs64;
pub use self::lfs64::*;

cfg_if! {
    if #[cfg(target_arch = "riscv64")] {
        mod riscv64;
        pub use self::riscv64::*;
    } else if #[cfg(target_arch = "x86")] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(target_arch = "arm")] {
        mod arm;
        pub use self::arm::*;
    } else if #[cfg(target_arch = "sparc")] {
        mod sparc;
        pub use self::sparc::*;
    } else {
        compile_error!("unsupported target_arch for target_os = \"phoenix\"");
    }
}
