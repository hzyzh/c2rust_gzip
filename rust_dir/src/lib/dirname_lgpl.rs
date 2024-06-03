use std::ptr;

use ::libc;
extern "C" {
    fn last_component(file: *const libc::c_char) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn dir_len(mut file: *const libc::c_char) -> size_t {
    let mut prefix_length: size_t = 0 as libc::c_int as size_t;
    let mut length: size_t = 0;
    prefix_length = (prefix_length as libc::c_ulong)
        .wrapping_add(
            (if prefix_length != 0 as libc::c_int as libc::c_ulong {
                (0 as libc::c_int != 0
                    && *file.offset(prefix_length as isize) as libc::c_int == '/' as i32)
                    as libc::c_int
            } else if *file.offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32
            {
                if 0 as libc::c_int != 0
                    && *file.offset(1 as libc::c_int as isize) as libc::c_int
                        == '/' as i32
                    && !(*file.offset(2 as libc::c_int as isize) as libc::c_int
                        == '/' as i32)
                {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                0 as libc::c_int
            }) as libc::c_ulong,
        ) as size_t as size_t;
    length = (last_component(file)).offset_from(file) as libc::c_long as size_t;
    while prefix_length < length {
        if !(*file
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32)
        {
            break;
        }
        length = length.wrapping_sub(1);
        length;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn mdir_name<'h0,'h1>(mut file: &'h0 [(libc::c_char)]) -> &'h1 core::cell::Cell<(libc::c_char)> {
    let mut length: size_t = dir_len(core::ptr::addr_of!(*&(file)[0]));
    let mut append_dot: bool = length == 0 as libc::c_int as libc::c_ulong
        || 0 as libc::c_int != 0 && length == 0 as libc::c_int as libc::c_ulong
            && *&(&(file)[((2 as libc::c_int as isize) as usize) ..])[0] as libc::c_int != '\0' as i32
            && !(*&(&(file)[((2 as libc::c_int as isize) as usize) ..])[0] as libc::c_int == '/' as i32);
    let mut dir: &[core::cell::Cell<(libc::c_char)>] = std::cell::Cell::from_mut((malloc(
        length
            .wrapping_add(append_dot as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    )));
    if &(dir)[0].is_null() {
        return 0 as *mut libc::c_char;
    }
    {
    let (dest, src, byte_len, ) = (((dir)), ((file)), (length), );
    let (n, ) = (byte_len as usize / 1, );
    {
        let this = &mut dest[..n];
        let src = &src[..n];
        // The panic code path was put into a cold function to not bloat the
        // call site.
        #[inline(never)]
        #[cold]
        #[track_caller]
        fn len_mismatch_fail(dst_len: usize, src_len: usize) -> ! {
            panic!(
                "source slice length ({}) does not match destination slice length ({})",
                src_len, dst_len,
            );
        }

        if this.len() != src.len() {
            len_mismatch_fail(this.len(), src.len());
        }

        // SAFETY: `self` is valid for `self.len()` elements by definition, and `src` was
        // checked to have the same length. The slices cannot overlap because
        // mutable references are exclusive.
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), this.as_mut_ptr(), this.len());
        }
    };
    dest
};
    if append_dot {
        let fresh0 = length;
        length = length.wrapping_add(1);
        (&(&(dir)[((fresh0 as isize) as usize) ..])[0]).set(('.' as i32 as libc::c_char));
    }
    (&(&(dir)[((length as isize) as usize) ..])[0]).set(('\0' as i32 as libc::c_char));
    return &(dir)[0];
}
