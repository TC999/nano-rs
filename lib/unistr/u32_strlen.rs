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
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn u32_strlen(mut s: *const uint32_t) -> size_t {
    let mut ptr: *const uint32_t = 0 as *const uint32_t;
    ptr = s;
    while *ptr != 0 as libc::c_int as uint32_t {
        ptr = ptr.offset(1);
        ptr;
    }
    return ptr.offset_from(s) as libc::c_long as size_t;
}
