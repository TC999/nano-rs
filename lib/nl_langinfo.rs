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
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
}
pub type nl_item = libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn rpl_nl_langinfo(mut item: nl_item) -> *mut libc::c_char {
    match item {
        _ => {}
    }
    return nl_langinfo(item);
}
