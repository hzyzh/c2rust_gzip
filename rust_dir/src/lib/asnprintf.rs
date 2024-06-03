use ::libc;
extern "C" {
    fn vasnprintf(
        resultbuf: *mut libc::c_char,
        lengthp: *mut size_t,
        format: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut libc::c_char;
}
pub type __builtin_va_list<'h1, 'h2> = [__va_list_tag<'h1, 'h2>; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag<'h1,'h2> {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: &'h1 libc::c_void,
    pub reg_save_area: &'h2 libc::c_void,
}
pub type va_list<'h1, 'h2> = __builtin_va_list<'h1, 'h2>;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn asnprintf(
    mut resultbuf: *mut libc::c_char,
    mut lengthp: *mut size_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    args_0 = args.clone();
    result = vasnprintf(resultbuf, lengthp, format, args_0.as_va_list());
    return result;
}
