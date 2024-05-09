use ::libc;
extern "C" {
    fn fd_safer(_: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pipe_safer<'h0>(mut fd: &'h0 mut [(libc::c_int)]) -> libc::c_int {
    if pipe(fd) == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            *&mut (&mut (fd)[((i as isize) as usize) ..])[0] = fd_safer(*&(&(&*(fd))[((i as isize) as usize) ..])[0]);
            if *&(&(&*(fd))[((i as isize) as usize) ..])[0] < 0 as libc::c_int {
                let mut e: libc::c_int = (__errno_location()).get();
                close(*&(&(&*(fd))[(((1 as libc::c_int - i) as isize) as usize) ..])[0]);
                (__errno_location()).set((e));
                return -(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
