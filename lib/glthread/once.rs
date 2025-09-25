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
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
}
pub type pthread_once_t = libc::c_int;
static mut fresh_once: pthread_once_t = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn glthread_once_singlethreaded(
    mut once_control: *mut pthread_once_t,
) -> libc::c_int {
    let mut firstbyte: *mut libc::c_char = once_control as *mut libc::c_char;
    if *firstbyte as libc::c_int
        == *(&fresh_once as *const pthread_once_t as *const libc::c_char) as libc::c_int
    {
        *firstbyte = !(*(&fresh_once as *const pthread_once_t as *const libc::c_char)
            as libc::c_int) as libc::c_char;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn glthread_once_multithreaded(
    mut once_control: *mut pthread_once_t,
    mut init_function: Option::<unsafe extern "C" fn() -> ()>,
) -> libc::c_int {
    let mut err: libc::c_int = pthread_once(once_control, init_function);
    if err == 38 as libc::c_int {
        if glthread_once_singlethreaded(once_control) != 0 {
            init_function.expect("non-null function pointer")();
        }
        return 0 as libc::c_int;
    }
    return err;
}
