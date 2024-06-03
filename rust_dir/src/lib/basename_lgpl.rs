use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub extern "C" fn last_component<'h0,'h1>(
    mut name: &'h0 [(libc::c_char)],
) -> &'h1 (libc::c_char) {
    let mut base: &[(libc::c_char)] = &(name)[((0 as libc::c_int as isize) as usize) ..];
    let mut p: &[(libc::c_char)] = &[0 as libc::c_char];
    let mut saw_slash: bool = 0 as libc::c_int != 0;
    while base[0] as libc::c_int == '/' as i32 {
        base = &(base)[((1) as usize) ..];
        &(base)[0];
    }
    p = base;
    while p[0] != 0 {
        if p[0] as libc::c_int == '/' as i32 {
            saw_slash = 1 as libc::c_int != 0;
        } else if saw_slash {
            base = p;
            saw_slash = 0 as libc::c_int != 0;
        }
        p = &(p)[((1) as usize) ..];
        &(p)[0];
    }
    return &(base)[0];
}
#[no_mangle]
pub unsafe extern "C" fn base_len<'h0>(mut name: &'h0 [(libc::c_char)]) -> size_t {
    let mut len: size_t = 0;
    let mut prefix_len: size_t = 0 as libc::c_int as size_t;
    len = strlen(name.as_ptr());
    while (1 as libc::c_int as libc::c_ulong) < len
        && *&(&(name)[((len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) as usize) ..])[0]
            as libc::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
        len;
    }
    if 0 as libc::c_int != 0 && len == 1 as libc::c_int as libc::c_ulong
        && *&(&(name)[((0 as libc::c_int as isize) as usize) ..])[0] as libc::c_int == '/' as i32
        && *&(&(name)[((1 as libc::c_int as isize) as usize) ..])[0] as libc::c_int == '/' as i32
        && *&(&(name)[((2 as libc::c_int as isize) as usize) ..])[0] == 0
    {
        return 2 as libc::c_int as size_t;
    }
    if 0 as libc::c_int != 0 && prefix_len != 0 && len == prefix_len
        && *&(&(name)[((prefix_len as isize) as usize) ..])[0] as libc::c_int == '/' as i32
    {
        return prefix_len.wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    return len;
}
