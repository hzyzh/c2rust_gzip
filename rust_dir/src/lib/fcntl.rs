use ::libc;
extern "C" {
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list<'a> = [__va_list_tag<'a, 'a>; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag<'h3,'h4> {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: &'h3 libc::c_void,
    pub reg_save_area: &'h4 libc::c_void,
}
pub type va_list<'a> = [__va_list_tag<'a, 'a>; 1];
#[no_mangle]
pub unsafe extern "C" fn rpl_fcntl(
    mut fd: libc::c_int,
    mut action: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut arg: ::core::ffi::VaListImpl;
    let mut result: libc::c_int = -(1 as libc::c_int);
    arg = args.clone();
    match action {
        0 => {
            let mut target: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD(fd, target);
        }
        1030 => {
            let mut target_0: libc::c_int = arg.arg::<libc::c_int>();
            result = rpl_fcntl_DUPFD_CLOEXEC(fd, target_0);
        }
        _ => {
            let mut current_block_7: u64;
            match action {
                1 => {
                    current_block_7 = 14662553124558895045;
                }
                3 => {
                    current_block_7 = 14662553124558895045;
                }
                1025 => {
                    current_block_7 = 10116260006732865633;
                }
                9 => {
                    current_block_7 = 9076467077212446256;
                }
                1032 => {
                    current_block_7 = 14146614613186727711;
                }
                1034 => {
                    current_block_7 = 5492734852181678294;
                }
                11 => {
                    current_block_7 = 16168035904432745724;
                }
                1033 => {
                    current_block_7 = 5505061507103959253;
                }
                0 => {
                    current_block_7 = 5505061507103959253;
                }
                1030 => {
                    current_block_7 = 10539527634675305520;
                }
                1026 => {
                    current_block_7 = 14204516101666919947;
                }
                2 => {
                    current_block_7 = 1046931293691932351;
                }
                4 => {
                    current_block_7 = 14650795351653737306;
                }
                8 => {
                    current_block_7 = 188514164905784293;
                }
                1031 => {
                    current_block_7 = 188514164905784293;
                }
                1024 | 10 => {
                    current_block_7 = 577971193990838163;
                }
                _ => {
                    let mut p: *mut libc::c_void = arg.arg::<*mut libc::c_void>();
                    result = fcntl(fd, action, p);
                    current_block_7 = 7175849428784450219;
                }
            }
            match current_block_7 {
                14662553124558895045 => {
                    current_block_7 = 10116260006732865633;
                }
                5505061507103959253 => {
                    current_block_7 = 10539527634675305520;
                }
                188514164905784293 => {
                    current_block_7 = 577971193990838163;
                }
                _ => {}
            }
            match current_block_7 {
                10116260006732865633 => {
                    current_block_7 = 9076467077212446256;
                }
                10539527634675305520 => {
                    current_block_7 = 14204516101666919947;
                }
                _ => {}
            }
            match current_block_7 {
                9076467077212446256 => {
                    current_block_7 = 14146614613186727711;
                }
                14204516101666919947 => {
                    current_block_7 = 1046931293691932351;
                }
                _ => {}
            }
            match current_block_7 {
                14146614613186727711 => {
                    current_block_7 = 5492734852181678294;
                }
                1046931293691932351 => {
                    current_block_7 = 14650795351653737306;
                }
                _ => {}
            }
            match current_block_7 {
                5492734852181678294 => {
                    current_block_7 = 16168035904432745724;
                }
                14650795351653737306 => {
                    current_block_7 = 577971193990838163;
                }
                _ => {}
            }
            match current_block_7 {
                16168035904432745724 => {
                    result = fcntl(fd, action);
                }
                577971193990838163 => {
                    let mut x: libc::c_int = arg.arg::<libc::c_int>();
                    result = fcntl(fd, action, x);
                }
                _ => {}
            }
        }
    }
    return result;
}
unsafe extern "C" fn rpl_fcntl_DUPFD(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    result = fcntl(fd, 0 as libc::c_int, target);
    return result;
}
static mut have_dupfd_cloexec: libc::c_int = 0;
unsafe extern "C" fn rpl_fcntl_DUPFD_CLOEXEC(
    mut fd: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    if 0 as libc::c_int <= have_dupfd_cloexec {
        result = fcntl(fd, 1030 as libc::c_int, target);
        if 0 as libc::c_int <= result || *__errno_location() != 22 as libc::c_int {
            have_dupfd_cloexec = 1 as libc::c_int;
        } else {
            result = rpl_fcntl_DUPFD(fd, target);
            if result >= 0 as libc::c_int {
                have_dupfd_cloexec = -(1 as libc::c_int);
            }
        }
    } else {
        result = rpl_fcntl_DUPFD(fd, target);
    }
    if 0 as libc::c_int <= result && have_dupfd_cloexec == -(1 as libc::c_int) {
        let mut flags: libc::c_int = fcntl(result, 1 as libc::c_int);
        if flags < 0 as libc::c_int
            || fcntl(result, 2 as libc::c_int, flags | 1 as libc::c_int)
                == -(1 as libc::c_int)
        {
            let mut saved_errno: libc::c_int = *__errno_location();
            close(result);
            *__errno_location() = saved_errno;
            result = -(1 as libc::c_int);
        }
    }
    return result;
}
unsafe extern "C" fn run_static_initializers() {
    have_dupfd_cloexec = if 0 as libc::c_int != 0 {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
