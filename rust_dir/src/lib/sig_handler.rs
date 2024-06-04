use ::libc;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval<'h60> {
    pub sival_int: libc::c_int,
    pub sival_ptr: &'h60 libc::c_void,
}
pub type __sigval_t<'a> = sigval<'a>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t<'h60,'h62,'h61,'h63,'h64> {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed<'h60,'h62,'h61,'h63,'h64>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed<'h60,'h62,'h61,'h63,'h64> {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7<'h60>,
    pub _rt: C2RustUnnamed_6<'h60>,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2<'h62,'h63,'h64>,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0<'h61>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0<'h61> {
    pub _call_addr: &'h61 libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2<'h62,'h63,'h64> {
    pub si_addr: &'h62 libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3<'h63,'h64>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3<'h63,'h64> {
    pub _addr_bnd: C2RustUnnamed_4<'h63,'h64>,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4<'h63,'h64> {
    pub _lower: &'h63 libc::c_void,
    pub _upper: &'h64 libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6<'h60> {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: sigval<'h60>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7<'h60> {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: sigval<'h60>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction<'h65,'h66,'h60,'h62,'h61,'h63,'h64> {
    pub __sigaction_handler: C2RustUnnamed_9<'h65,'h66,'h60,'h62,'h61,'h63,'h64>,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9<'h65,'h66,'h60,'h62,'h61,'h63,'h64> {
    pub sa_handler: std::option::Option<unsafe extern "C" fn(i32)>,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, &'h65 siginfo_t<'h60,'h62,'h61,'h63,'h64>, &'h66 libc::c_void) -> (),
    >,
}
pub type sa_handler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_handler<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7>(mut a: &'h0 sigaction<'h1,'h2,'h3,'h4,'h5,'h6,'h7>) -> std::option::Option<unsafe extern "C" fn(i32)> {
    return (*a).__sigaction_handler.sa_handler;
}
