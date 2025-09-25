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
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn vasnprintf(
        resultbuf: *mut libc::c_char,
        lengthp: *mut size_t,
        format: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn vsnzprintf(
    mut str: *mut libc::c_char,
    mut size: size_t,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> ptrdiff_t {
    let mut output: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut lenbuf: size_t = size;
    output = vasnprintf(str, &mut lenbuf, format, args.as_va_list());
    len = lenbuf;
    if output.is_null() {
        return -(1 as libc::c_int) as ptrdiff_t;
    }
    if output != str {
        if size != 0 {
            let mut pruned_len: size_t = if len < size {
                len
            } else {
                size.wrapping_sub(1 as libc::c_int as size_t)
            };
            memcpy(str as *mut libc::c_void, output as *const libc::c_void, pruned_len);
            *str.offset(pruned_len as isize) = '\0' as i32 as libc::c_char;
        }
        free(output as *mut libc::c_void);
    }
    if len > 9223372036854775807 as libc::c_long as size_t {
        *__errno_location() = 12 as libc::c_int;
        return -(1 as libc::c_int) as ptrdiff_t;
    }
    return len as ptrdiff_t;
}
