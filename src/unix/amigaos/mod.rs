//! Definitions for AmigaOS 4 with the clib4 C library.
//!
//! Sizes, offsets and constant values were measured with ppc-amigaos-gcc
//! 15.3.0 against the clib4 headers, by emitting them as assembly data.
use crate::prelude::*;
use crate::{gid_t, iovec, timespec, uid_t, DIR};

pub type nlink_t = u32;
pub type ino_t = u32;
pub type dev_t = u32;
pub type mode_t = u32;
pub type off_t = i64;
pub type time_t = i32;
pub type clock_t = i32;
pub type suseconds_t = i32;
pub type blksize_t = i32;
pub type blkcnt_t = i32;
pub type rlim_t = u32;
pub type socklen_t = u32;
pub type clockid_t = c_int;
pub type pthread_t = c_uint;
pub type pthread_key_t = c_uint;
pub type sa_family_t = u8;
pub type sigset_t = c_uint;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type nfds_t = c_uint;
pub type wchar_t = c_int;
pub type cc_t = c_uchar;

pub const NCCS: usize = 16;

s! {
    pub struct in_addr {
        pub s_addr: crate::in_addr_t,
    }

    pub struct sockaddr {
        pub sa_len: c_uchar,
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_len: c_uchar,
        pub sin_family: sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [c_char; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_len: c_uchar,
        pub sin6_family: sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108],
    }

    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut c_char,
        pub ai_next: *mut addrinfo,
    }

    pub struct passwd {
        pub pw_name: *mut c_char,
        pub pw_passwd: *mut c_char,
        pub pw_uid: crate::uid_t,
        pub pw_gid: crate::gid_t,
        pub pw_gecos: *mut c_char,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char,
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
        pub __tm_gmtoff: c_long,
        pub __tm_zone: *const c_char,
    }

    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_cc: [cc_t; NCCS],
        pub c_ispeed: speed_t,
        pub c_ospeed: speed_t,
        pub __type: c_int,
        pub __flags: c_uint,
    }

    pub struct fd_set {
        pub fds_bits: [c_ulong; 8],
    }

    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: c_ulong,
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        pub f_mntonname: [c_char; 256],
        pub f_fstypename: [c_char; 32],
    }

    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
    }

    pub struct Dl_info {
        pub dli_fname: *const c_char,
        pub dli_fbase: *mut c_void,
        pub dli_sname: *const c_char,
        pub dli_saddr: *mut c_void,
    }

    pub struct sigaction {
        pub sa_sigaction: crate::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: c_int,
    }

    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_mode: mode_t,
        pub st_nlink: nlink_t,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: dev_t,
        pub st_size: off_t,
        pub st_atime: time_t,
        pub st_spare1: c_long,
        pub st_mtime: time_t,
        pub st_spare2: c_long,
        pub st_ctime: time_t,
        pub st_spare3: c_long,
        pub st_blksize: c_long,
        pub st_blocks: c_long,
        pub st_spare4: [c_long; 2],
    }

    pub struct dirent {
        pub d_ino: ino_t,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_namlen: c_uchar,
        pub d_name: [c_char; 256],
    }

    pub struct pthread_attr_t {
        __size: [u32; 8],
    }

    pub struct pthread_mutex_t {
        __size: [u32; 4],
    }

    pub struct pthread_mutexattr_t {
        __size: [u32; 2],
    }

    pub struct pthread_cond_t {
        __size: [u32; 5],
    }

    pub struct pthread_condattr_t {
        __size: [u32; 2],
    }

    pub struct pthread_rwlock_t {
        __size: [u32; 1],
    }

    pub struct pthread_rwlockattr_t {
        __size: [u32; 1],
    }

    pub struct pthread_barrier_t {
        __size: [u32; 11],
    }

    pub struct pthread_barrierattr_t {
        __size: [u32; 1],
    }

    pub struct sem_t {
        __size: [u32; 15],
    }
}

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t { __size: [0; 4] };
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t { __size: [0, 0, 0, 0, 2] };
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t { __size: [0; 1] };

pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 2;
pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;
pub const PTHREAD_STACK_MIN: size_t = 65536;

pub const FD_SETSIZE: usize = 256;

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_APPEND: c_int = 4;
pub const O_CREAT: c_int = 8;
pub const O_EXCL: c_int = 16;
pub const O_TRUNC: c_int = 32;
pub const O_NONBLOCK: c_int = 64;
pub const O_CLOEXEC: c_int = 16384;

pub const S_IFMT: mode_t = 61440;
pub const S_IFDIR: mode_t = 16384;
pub const S_IFREG: mode_t = 32768;
pub const S_IFLNK: mode_t = 40960;
pub const S_IFIFO: mode_t = 4096;
pub const S_IFCHR: mode_t = 8192;
pub const S_IFBLK: mode_t = 24576;
pub const S_IFSOCK: mode_t = 49152;

pub const ST_RDONLY: c_ulong = 1;
pub const ST_NOSUID: c_ulong = 2;

pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

pub const F_OK: c_int = 0;
pub const R_OK: c_int = 4;
pub const W_OK: c_int = 2;
pub const X_OK: c_int = 1;


pub const PATH_MAX: c_int = 1024;
pub const NAME_MAX: c_int = 255;

pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const EINTR: c_int = 4;
pub const EBADF: c_int = 9;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const EAGAIN: c_int = 35;
pub const EWOULDBLOCK: c_int = 35;
pub const ENOSYS: c_int = 78;
pub const ENOTEMPTY: c_int = 66;

pub const EAI_NONAME: c_int = -2;
pub const EAI_AGAIN: c_int = -3;
pub const EAI_FAIL: c_int = -4;
pub const EAI_FAMILY: c_int = -6;
pub const EAI_SOCKTYPE: c_int = -7;
pub const EAI_SERVICE: c_int = -8;
pub const EAI_MEMORY: c_int = -10;
pub const EAI_SYSTEM: c_int = -11;
pub const EAI_OVERFLOW: c_int = -12;

pub const AI_PASSIVE: c_int = 1;
pub const AI_CANONNAME: c_int = 2;
pub const AI_NUMERICHOST: c_int = 4;
pub const AI_V4MAPPED: c_int = 8;
pub const AI_ALL: c_int = 16;
pub const AI_ADDRCONFIG: c_int = 32;

pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGTRAP: c_int = 5;
pub const SIGABRT: c_int = 6;
pub const SIGBUS: c_int = 10;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGUSR1: c_int = 30;
pub const SIGSEGV: c_int = 11;
pub const SIGUSR2: c_int = 31;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;
pub const SIGCHLD: c_int = 20;
pub const SIGCONT: c_int = 19;
pub const SIGSTOP: c_int = 17;
pub const SIGTSTP: c_int = 18;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 16;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
pub const SIGVTALRM: c_int = 26;
pub const SIGPROF: c_int = 27;
pub const SIGWINCH: c_int = 28;
pub const SIGSYS: c_int = 12;
pub const SIGIO: c_int = 23;
pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const SIG_SETMASK: c_int = 2;
pub const SA_NODEFER: c_int = 2;
pub const SA_RESTART: c_int = 4;
pub const SA_ONSTACK: c_int = 16;
pub const SA_SIGINFO: c_int = 8;

pub const WNOHANG: c_int = 1;

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;

pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
pub const ISIG: tcflag_t = 1;
pub const ICANON: tcflag_t = 2;
pub const ECHO: tcflag_t = 4;
pub const VTIME: usize = 7;
pub const VMIN: usize = 5;
pub const B38400: speed_t = 38400;

pub const LC_ALL: c_int = 0;
pub const LC_MONETARY: c_int = 3;
pub const LC_CTYPE: c_int = 2;
pub const LC_NUMERIC: c_int = 4;

pub const RTLD_DEFAULT: *mut c_void = 0 as *mut c_void;
pub const RTLD_LAZY: c_int = 1;
pub const RTLD_NOW: c_int = 2;
pub const RTLD_GLOBAL: c_int = 8;
pub const RTLD_LOCAL: c_int = 4;

pub const AF_UNIX: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const IPV6_V6ONLY: c_int = 26;
pub const IP_TTL: c_int = 4;
pub const IP_MULTICAST_TTL: c_int = 10;
pub const TCP_NODELAY: c_int = 1;
pub const SOL_SOCKET: c_int = 65535;
pub const SO_REUSEADDR: c_int = 4;
pub const SO_BROADCAST: c_int = 32;
pub const SO_KEEPALIVE: c_int = 8;
pub const SO_LINGER: c_int = 128;
pub const SO_SNDBUF: c_int = 4097;
pub const SO_RCVBUF: c_int = 4098;
pub const SO_SNDTIMEO: c_int = 4101;
pub const SO_RCVTIMEO: c_int = 4102;
pub const SO_ERROR: c_int = 4103;
pub const SOMAXCONN: c_int = 5;
pub const MSG_PEEK: c_int = 2;
pub const MSG_NOSIGNAL: c_int = 16384;
pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const _SC_PAGESIZE: c_int = 8;
pub const _SC_OPEN_MAX: c_int = 4;
pub const _SC_CLK_TCK: c_int = 2;
pub const _SC_NPROCESSORS_ONLN: c_int = 10;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 70;

s! {
    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct sockaddr_storage {
        pub ss_len: c_uchar,
        pub ss_family: sa_family_t,
        __ss_pad1: [c_char; 6],
        __ss_align: i64,
        __ss_pad2: [c_char; 112],
    }
}

pub const EXIT_SUCCESS: c_int = 0;
pub const EXIT_FAILURE: c_int = 20;

pub const FIONBIO: c_ulong = 0x8004667e;
pub const FIOCLEX: c_ulong = 0x20006601;

pub const F_GETFD: c_int = 1;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;

pub const AT_FDCWD: c_int = -100;
pub const AT_REMOVEDIR: c_int = 512;
pub const AT_SYMLINK_NOFOLLOW: c_int = 256;

pub const O_ACCMODE: c_int = 3;
pub const O_DIRECTORY: c_int = 512;
pub const O_NOFOLLOW: c_int = 4096;

pub const POLLIN: c_short = 1;
pub const POLLOUT: c_short = 4;
pub const POLLERR: c_short = 8;
pub const POLLHUP: c_short = 16;
pub const POLLNVAL: c_short = 32;

pub const IP_MULTICAST_LOOP: c_int = 11;
pub const IP_ADD_MEMBERSHIP: c_int = 12;
pub const IP_DROP_MEMBERSHIP: c_int = 13;
pub const IPV6_MULTICAST_LOOP: c_int = 11;
pub const IPV6_ADD_MEMBERSHIP: c_int = 12;
pub const IPV6_DROP_MEMBERSHIP: c_int = 13;

pub const _SC_HOST_NAME_MAX: c_int = 65;

pub const S_IRUSR: mode_t = 256;
pub const S_IWUSR: mode_t = 128;
pub const S_IXUSR: mode_t = 64;
pub const S_IRGRP: mode_t = 32;
pub const S_IWGRP: mode_t = 16;
pub const S_IXGRP: mode_t = 8;
pub const S_IROTH: mode_t = 4;
pub const S_IWOTH: mode_t = 2;
pub const S_IXOTH: mode_t = 1;

pub const UTIME_OMIT: c_long = 1073741822;

pub const E2BIG: c_int = 7;
pub const EADDRINUSE: c_int = 48;
pub const EADDRNOTAVAIL: c_int = 49;
pub const EBUSY: c_int = 16;
pub const ECONNABORTED: c_int = 53;
pub const ECONNREFUSED: c_int = 61;
pub const ECONNRESET: c_int = 54;
pub const EDEADLK: c_int = 11;
pub const EDQUOT: c_int = 69;
pub const EFBIG: c_int = 27;
pub const EHOSTUNREACH: c_int = 65;
pub const EINPROGRESS: c_int = 36;
pub const EIO: c_int = 5;
pub const EISCONN: c_int = 56;
pub const ELOOP: c_int = 62;
pub const EMFILE: c_int = 24;
pub const EMLINK: c_int = 31;
pub const EMSGSIZE: c_int = 40;
pub const ENAMETOOLONG: c_int = 63;
pub const ENETDOWN: c_int = 50;
pub const ENETUNREACH: c_int = 51;
pub const ENFILE: c_int = 23;
pub const ENOSPC: c_int = 28;
pub const ENOTCONN: c_int = 57;
pub const ENOTSUP: c_int = 89;
pub const EOPNOTSUPP: c_int = 45;
pub const EPIPE: c_int = 32;
pub const ERANGE: c_int = 34;
pub const EROFS: c_int = 30;
pub const ESPIPE: c_int = 29;
pub const ESTALE: c_int = 70;
pub const ETIMEDOUT: c_int = 60;
pub const ETXTBSY: c_int = 26;

#[allow(non_snake_case)]
pub const fn WIFEXITED(status: c_int) -> bool {
    (status & 0x10000000) == 0
}

#[allow(non_snake_case)]
pub const fn WEXITSTATUS(status: c_int) -> c_int {
    status & 0x000000ff
}

#[allow(non_snake_case)]
pub const fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

#[allow(non_snake_case)]
pub const fn WIFSIGNALED(status: c_int) -> bool {
    (status & 0x7f) > 0 && (status & 0x7f) < 0x7f
}

#[allow(non_snake_case)]
pub const fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

#[allow(non_snake_case)]
pub const fn WSTOPSIG(status: c_int) -> c_int {
    status & 0x000000ff
}

#[allow(non_snake_case)]
pub const fn WIFCONTINUED(_status: c_int) -> bool {
    false
}

#[allow(non_snake_case)]
pub const fn WCOREDUMP(_status: c_int) -> bool {
    false
}

extern "C" {
    pub fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
    pub fn recvfrom(
        socket: c_int,
        buffer: *mut c_void,
        length: size_t,
        flags: c_int,
        address: *mut sockaddr,
        address_len: *mut socklen_t,
    ) -> ssize_t;
    pub fn ioctl(fildes: c_int, request: c_ulong, ...) -> c_int;
    pub fn setgroups(ngroups: c_int, gidset: *const gid_t) -> c_int;
    pub fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    pub fn futimens(fd: c_int, times: *const timespec) -> c_int;
    pub fn utimensat(fd: c_int, path: *const c_char, times: *const timespec, flags: c_int) -> c_int;
    pub fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn pthread_create(
        native: *mut pthread_t,
        attr: *const pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> c_int;
    pub fn pthread_condattr_setclock(attr: *mut pthread_condattr_t, clock_id: clockid_t) -> c_int;
    pub fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn dirfd(dirp: *mut DIR) -> c_int;
}

extern "C" {
    pub fn arc4random_buf(buf: *mut c_void, nbytes: size_t);
    // Returns the full path of the program, in a static buffer that the next call overwrites.
    pub fn getexecfullname() -> *const c_char;
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    // Provided by amiga/sdk shims on top of clib4 readv, writev, and dup2.
    pub fn preadv(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn pwritev(fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int;
}

pub const F_DUPFD: c_int = 0;

#[cfg_attr(target_feature = "crt-static", link(name = "pthread", kind = "static", modifiers = "-bundle"))]
#[cfg_attr(not(target_feature = "crt-static"), link(name = "pthread"))]
#[cfg_attr(target_feature = "crt-static", link(name = "m", kind = "static", modifiers = "-bundle"))]
#[cfg_attr(not(target_feature = "crt-static"), link(name = "m"))]
#[cfg_attr(target_feature = "crt-static", link(name = "rt", kind = "static", modifiers = "-bundle"))]
#[cfg_attr(not(target_feature = "crt-static"), link(name = "rt"))]
#[cfg_attr(target_feature = "crt-static", link(name = "c", kind = "static", modifiers = "-bundle"))]
#[cfg_attr(not(target_feature = "crt-static"), link(name = "c"))]
extern "C" {}

#[link(name = "gcc_eh", kind = "static", modifiers = "-bundle")]
#[link(name = "gcc", kind = "static", modifiers = "-bundle")]
extern "C" {}

extern "C" {
    pub fn spawnvpe(
        file: *const c_char,
        argv: *const *const c_char,
        deltaenv: *mut *mut c_char,
        dir: *const c_char,
        fhin: c_int,
        fhout: c_int,
        fherr: c_int,
    ) -> crate::pid_t;
}
// clib4 values (probed against its headers) and the Linux-standard baud, ioctl and fcntl values clib4 leaves out
pub const B0: speed_t = 0;
pub const B110: speed_t = 110;
pub const B115200: speed_t = 115200;
pub const B1152000: speed_t = 1152000;
pub const B1200: speed_t = 1200;
pub const B134: speed_t = 134;
pub const B150: speed_t = 150;
pub const B1800: speed_t = 1800;
pub const B19200: speed_t = 19200;
pub const B200: speed_t = 200;
pub const B230400: speed_t = 230400;
pub const B2400: speed_t = 2400;
pub const B300: speed_t = 300;
pub const B460800: speed_t = 460800;
pub const B4800: speed_t = 4800;
pub const B50: speed_t = 50;
pub const B57600: speed_t = 57600;
pub const B576000: speed_t = 576000;
pub const B600: speed_t = 600;
pub const B75: speed_t = 75;
pub const B9600: speed_t = 9600;
pub const BRKINT: tcflag_t = 2;
pub const CLOCAL: tcflag_t = 256;
pub const CMSPAR: tcflag_t = 1073741824;
pub const CREAD: tcflag_t = 16;
pub const CRTSCTS: tcflag_t = 2147483648;
pub const CS5: tcflag_t = 1;
pub const CS6: tcflag_t = 2;
pub const CS7: tcflag_t = 3;
pub const CS8: tcflag_t = 4;
pub const CSIZE: tcflag_t = 7;
pub const CSTOPB: tcflag_t = 8;
pub const EAFNOSUPPORT: c_int = 47;
pub const EALREADY: c_int = 37;
pub const EBADMSG: c_int = 88;
pub const ECANCELED: c_int = 87;
pub const ECHILD: c_int = 10;
pub const ECHOCTL: tcflag_t = 2048;
pub const ECHOE: tcflag_t = 8;
pub const ECHOK: tcflag_t = 16;
pub const ECHOKE: tcflag_t = 1024;
pub const ECHONL: tcflag_t = 32;
pub const EDESTADDRREQ: c_int = 39;
pub const EDOM: c_int = 33;
pub const EFAULT: c_int = 14;
pub const EHOSTDOWN: c_int = 64;
pub const EIDRM: c_int = 82;
pub const EILSEQ: c_int = 85;
pub const ENETRESET: c_int = 52;
pub const ENOBUFS: c_int = 55;
pub const ENODEV: c_int = 19;
pub const ENOEXEC: c_int = 8;
pub const ENOLCK: c_int = 77;
pub const ENOMSG: c_int = 83;
pub const ENOPROTOOPT: c_int = 42;
pub const ENOTBLK: c_int = 15;
pub const ENOTSOCK: c_int = 38;
pub const ENOTTY: c_int = 25;
pub const ENXIO: c_int = 6;
pub const EOVERFLOW: c_int = 84;
pub const EPFNOSUPPORT: c_int = 46;
pub const EPROTO: c_int = 86;
pub const EPROTONOSUPPORT: c_int = 43;
pub const EPROTOTYPE: c_int = 41;
pub const EREMOTE: c_int = 71;
pub const ESHUTDOWN: c_int = 58;
pub const ESOCKTNOSUPPORT: c_int = 44;
pub const ESRCH: c_int = 3;
pub const ETOOMANYREFS: c_int = 59;
pub const EUSERS: c_int = 68;
pub const FIONREAD: c_ulong = 1074030207;
pub const FLUSHO: tcflag_t = 512;
pub const F_SETFD: c_int = 2;
pub const HUPCL: tcflag_t = 128;
pub const ICRNL: tcflag_t = 256;
pub const IEXTEN: tcflag_t = 256;
pub const IGNBRK: tcflag_t = 1;
pub const IGNCR: tcflag_t = 128;
pub const IGNPAR: tcflag_t = 4;
pub const IMAXBEL: tcflag_t = 8;
pub const INLCR: tcflag_t = 64;
pub const INPCK: tcflag_t = 16;
pub const ISTRIP: tcflag_t = 32;
pub const IUTF8: tcflag_t = 131072;
pub const IXANY: tcflag_t = 32768;
pub const IXOFF: tcflag_t = 4096;
pub const IXON: tcflag_t = 1024;
pub const NOFLSH: tcflag_t = 64;
pub const OCRNL: tcflag_t = 4;
pub const ONLCR: tcflag_t = 2;
pub const ONLRET: tcflag_t = 16;
pub const ONOCR: tcflag_t = 8;
pub const OPOST: tcflag_t = 1;
pub const PARENB: tcflag_t = 32;
pub const PARMRK: tcflag_t = 65536;
pub const PARODD: tcflag_t = 64;
pub const TCIFLUSH: tcflag_t = 0;
pub const TCIOFF: tcflag_t = 2;
pub const TCIOFLUSH: tcflag_t = 2;
pub const TCION: tcflag_t = 3;
pub const TCOFLUSH: tcflag_t = 1;
pub const TCOOFF: tcflag_t = 0;
pub const TCOON: tcflag_t = 1;
pub const TIOCGWINSZ: c_ulong = 21523;
pub const TIOCSWINSZ: c_ulong = 21524;
pub const TOSTOP: tcflag_t = 128;
pub const VEOF: usize = 4;
pub const VEOL: usize = 6;
pub const VERASE: usize = 2;
pub const VINTR: usize = 0;
pub const VKILL: usize = 3;
pub const VQUIT: usize = 1;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VSUSP: usize = 10;
pub const VWERASE: usize = 11;
pub const B1000000: speed_t = 1000000;
pub const B1500000: speed_t = 1500000;
pub const B2000000: speed_t = 2000000;
pub const B2500000: speed_t = 2500000;
pub const B3000000: speed_t = 3000000;
pub const B3500000: speed_t = 3500000;
pub const B4000000: speed_t = 4000000;
pub const B500000: speed_t = 500000;
pub const B921600: speed_t = 921600;
pub const TIOCEXCL: c_ulong = 0x540C;
pub const TIOCNXCL: c_ulong = 0x540D;
pub const F_DUPFD_CLOEXEC: c_int = 1030;
