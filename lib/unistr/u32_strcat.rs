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
    fn u32_strlen(s: *const uint32_t) -> size_t;
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn u32_strcat(
    mut dest: *mut uint32_t,
    mut src: *const uint32_t,
) -> *mut uint32_t {
    let mut destptr: *mut uint32_t = dest.offset(u32_strlen(dest) as isize);
    loop {
        *destptr = *src;
        if !(*destptr != 0 as libc::c_int as uint32_t) {
            break;
        }
        src = src.offset(1);
        src;
        destptr = destptr.offset(1);
        destptr;
    }
    return dest;
}
