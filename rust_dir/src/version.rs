use ::libc;
#[no_mangle]
static Version: *const libc::c_char = b"1.10\0" as *const u8
    as *const libc::c_char;
