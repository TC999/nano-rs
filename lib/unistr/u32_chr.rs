#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn u32_chr(
    mut s: *const uint32_t,
    mut n: size_t,
    mut uc: ucs4_t,
) -> *mut uint32_t {
    while n > 0 as libc::c_int as size_t {
        if *s == uc {
            return s as *mut uint32_t;
        }
        s = s.offset(1);
        s;
        n = n.wrapping_sub(1);
        n;
    }
    return 0 as *mut uint32_t;
}
