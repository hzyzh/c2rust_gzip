use ::libc;
extern "C" {
    fn xalloc_die();
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub const HAVE_GNU_CALLOC: C2RustUnnamed_0 = 1;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn x2nrealloc<'h0,'h1,'h2>(
    mut p: &'h0 (libc::c_void),
    mut pn: &'h1 mut (size_t),
    mut s: size_t,
) -> &'h2 (libc::c_void) {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc<'h0>(mut n: size_t) -> &'h0 [core::cell::Cell<(libc::c_void)>] {
    let mut p: &[core::cell::Cell<(libc::c_void)>] = malloc(n);
    if &(p)[0].is_null() && n != 0 as libc::c_int as libc::c_ulong {
        xalloc_die();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc<'h0,'h1>(
    mut p: &'h0 (libc::c_void),
    mut n: size_t,
) -> &'h1 (libc::c_void) {
    if n == 0 && !p.is_null() {
        free(p);
        return 0 as *mut libc::c_void;
    }
    p = realloc(p, n);
    if p.is_null() && n != 0 {
        xalloc_die();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn x2realloc<'h0,'h1,'h2>(
    mut p: &'h0 (libc::c_void),
    mut pn: &'h1 mut (size_t),
) -> &'h2 (libc::c_void) {
    return x2nrealloc(p, pn, 1 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn xzalloc<'h0>(mut s: size_t) -> &'h0 (libc::c_void) {
    return memset(xmalloc(s), 0 as libc::c_int, s);
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc<'h0>(mut n: size_t, mut s: size_t) -> &'h0 (libc::c_void) {
    let mut p: &(libc::c_void) = 0 as *mut libc::c_void;
    if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
        < 18446744073709551615 as libc::c_ulong
    {
        9223372036854775807 as libc::c_long as libc::c_ulong
    } else {
        (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    })
        .wrapping_div(s) < n
        || {
            p = calloc(n, s);
            p.is_null()
                && (HAVE_GNU_CALLOC as libc::c_int != 0
                    || n != 0 as libc::c_int as libc::c_ulong)
        }
    {
        xalloc_die();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xmemdup<'h0,'h1>(
    mut p: &'h0 [(libc::c_void)],
    mut s: size_t,
) -> &'h1 (libc::c_void) {
    return {
    let (dest, src, byte_len, ) = ((xmalloc(s)), (p), (s), );
    let (n, ) = (byte_len as usize / 1, );
    dest[..n].copy_from_slice(&src[..n]);
    dest
};
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup<'h0,'h1>(mut string: &'h0 [(libc::c_char)]) -> &'h1 (libc::c_char) {
    return xmemdup(
        string as *const libc::c_void,
        (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
}
