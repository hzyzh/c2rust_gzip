use ::libc;
#[no_mangle]
static exit_failure: libc::c_int = 1 as libc::c_int;
