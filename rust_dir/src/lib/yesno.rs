use ::libc;
extern "C" {
    fn getchar() -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yesno() -> bool {
    let mut yes: bool = false;
    let mut c: libc::c_int = getchar();
    yes = c == 'y' as i32 || c == 'Y' as i32;
    while c != '\n' as i32 && c != -(1 as libc::c_int) {
        c = getchar();
    }
    return yes;
}
