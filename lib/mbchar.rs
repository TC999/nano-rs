#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
extern "C" {
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn iswcntrl(__wc: wint_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint_least32_t = __uint32_t;
pub type char32_t = __uint_least32_t;
pub type wint_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const libc::c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: char32_t,
}
pub type mbchar_t = mbchar;
#[inline]
unsafe extern "C" fn c32iscntrl(mut wc: wint_t) -> libc::c_int {
    return iswcntrl(wc);
}
#[inline]
unsafe extern "C" fn c32width(mut wc: char32_t) -> libc::c_int {
    return wcwidth(wc as wchar_t);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_width_aux(mut wc: char32_t) -> libc::c_int {
    let mut w: libc::c_int = c32width(wc);
    return if w >= 0 as libc::c_int {
        w
    } else if c32iscntrl(wc) != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mb_copy(
    mut new_mbc: *mut mbchar_t,
    mut old_mbc: *const mbchar_t,
) {
    (*new_mbc).ptr = (*old_mbc).ptr;
    (*new_mbc).bytes = (*old_mbc).bytes;
    (*new_mbc).wc_valid = (*old_mbc).wc_valid;
    if (*new_mbc).wc_valid {
        (*new_mbc).wc = (*old_mbc).wc;
    }
}
