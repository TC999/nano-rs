#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn rpl_mbrtoc32(
        pc: *mut char32_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type mbchar_t = mbchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: char32_t,
}
pub type char32_t = __uint_least32_t;
pub type __uint_least32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type mbif_state_t = mbif_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbif_state {
    pub in_shift: bool,
    pub state: mbstate_t,
}
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
#[inline]
unsafe extern "C" fn mbszero(mut ps: *mut mbstate_t) {
    memset(
        ps as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
}
#[inline(always)]
unsafe extern "C" fn mbiterf_next(
    mut ps: *mut mbif_state,
    mut iter: *const libc::c_char,
    mut endptr: *const libc::c_char,
) -> mbchar_t {
    if !(*ps).in_shift {
        if (*iter as libc::c_uchar as libc::c_int) < 0x80 as libc::c_int {
            return {
                let mut init = mbchar {
                    ptr: iter,
                    bytes: 1 as libc::c_int as size_t,
                    wc_valid: 1 as libc::c_int != 0,
                    wc: *iter as char32_t,
                };
                init
            }
        } else {
            (*ps).in_shift = 1 as libc::c_int != 0;
        }
    }
    let mut bytes: size_t = 0;
    let mut wc: char32_t = 0;
    bytes = rpl_mbrtoc32(
        &mut wc,
        iter,
        endptr.offset_from(iter) as libc::c_long as size_t,
        &mut (*ps).state,
    );
    if bytes == -(1 as libc::c_int) as size_t {
        (*ps).in_shift = 0 as libc::c_int != 0;
        mbszero(&mut (*ps).state);
        return {
            let mut init = mbchar {
                ptr: iter,
                bytes: 1 as libc::c_int as size_t,
                wc_valid: 0 as libc::c_int != 0,
                wc: 0,
            };
            init
        };
    } else if bytes == -(2 as libc::c_int) as size_t {
        (*ps).in_shift = 0 as libc::c_int != 0;
        return {
            let mut init = mbchar {
                ptr: iter,
                bytes: endptr.offset_from(iter) as libc::c_long as size_t,
                wc_valid: 0 as libc::c_int != 0,
                wc: 0,
            };
            init
        };
    } else {
        if bytes == 0 as libc::c_int as size_t {
            bytes = 1 as libc::c_int as size_t;
        } else if bytes == -(3 as libc::c_int) as size_t {
            bytes = 0 as libc::c_int as size_t;
        }
        if mbsinit(&mut (*ps).state) != 0 {
            (*ps).in_shift = 0 as libc::c_int != 0;
        }
        return {
            let mut init = mbchar {
                ptr: iter,
                bytes: bytes,
                wc_valid: 1 as libc::c_int != 0,
                wc: wc,
            };
            init
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn mbsnlen(
    mut string: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    if __ctype_get_mb_cur_max() > 1 as libc::c_int as size_t {
        let mut count: size_t = 0 as libc::c_int as size_t;
        let mut string_end: *const libc::c_char = string.offset(len as isize);
        let mut state: mbif_state_t = mbif_state {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
        };
        let mut iter: *const libc::c_char = 0 as *const libc::c_char;
        state.in_shift = 0 as libc::c_int != 0;
        mbszero(&mut state.state);
        iter = string;
        while state.in_shift as libc::c_int != 0 || iter < string_end {
            let mut cur: mbchar_t = mbiterf_next(&mut state, iter, string_end);
            count = count.wrapping_add(1);
            count;
            iter = iter.offset(cur.bytes as isize);
        }
        return count;
    } else {
        return len
    };
}
