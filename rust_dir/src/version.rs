use ::libc;
#[no_mangle]
static mut Version: *const libc::c_char = b"1.10\0" as *const u8
    as *const libc::c_char;
