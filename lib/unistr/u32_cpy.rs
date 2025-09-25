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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn u32_cpy(
    mut dest: *mut uint32_t,
    mut src: *const uint32_t,
    mut n: size_t,
) -> *mut uint32_t {
    if n > 0 as libc::c_int as size_t {
        memcpy(
            dest as *mut libc::c_char as *mut libc::c_void,
            src as *const libc::c_char as *const libc::c_void,
            n.wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
        );
    }
    return dest;
}
