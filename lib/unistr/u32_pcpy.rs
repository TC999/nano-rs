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
    fn u32_cpy(dest: *mut uint32_t, src: *const uint32_t, n: size_t) -> *mut uint32_t;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn u32_pcpy(
    mut dest: *mut uint32_t,
    mut src: *const uint32_t,
    mut n: size_t,
) -> *mut uint32_t {
    return (u32_cpy(dest, src, n)).offset(n as isize);
}
