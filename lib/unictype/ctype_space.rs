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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub header: [libc::c_int; 1],
    pub level1: [libc::c_int; 1],
    pub level2: [libc::c_short; 128],
    pub level3: [libc::c_uint; 64],
}
#[inline]
unsafe extern "C" fn bitmap_lookup(
    mut table: *const libc::c_void,
    mut uc: ucs4_t,
) -> libc::c_int {
    let mut index1: libc::c_uint = uc >> 16 as libc::c_int;
    if index1
        < *(table as *const libc::c_int).offset(0 as libc::c_int as isize)
            as libc::c_uint
    {
        let mut lookup1: libc::c_int = *(table as *const libc::c_int)
            .offset((1 as libc::c_int as libc::c_uint).wrapping_add(index1) as isize);
        if lookup1 >= 0 as libc::c_int {
            let mut index2: libc::c_uint = uc >> 9 as libc::c_int
                & 127 as libc::c_int as ucs4_t;
            let mut lookup2: libc::c_int = *(table as *const libc::c_short)
                .offset((lookup1 as libc::c_uint).wrapping_add(index2) as isize)
                as libc::c_int;
            if lookup2 >= 0 as libc::c_int {
                let mut index3: libc::c_uint = uc >> 5 as libc::c_int
                    & 15 as libc::c_int as ucs4_t;
                let mut lookup3: libc::c_uint = *(table as *const libc::c_uint)
                    .offset((lookup2 as libc::c_uint).wrapping_add(index3) as isize);
                return (lookup3 >> (uc & 0x1f as libc::c_int as ucs4_t)
                    & 1 as libc::c_int as libc::c_uint) as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
static mut u_is_space: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 1],
    level2: [0; 128],
    level3: [0; 64],
};
#[no_mangle]
pub unsafe extern "C" fn uc_is_space(mut uc: ucs4_t) -> bool {
    return bitmap_lookup(&u_is_space as *const C2RustUnnamed as *const libc::c_void, uc)
        != 0;
}
unsafe extern "C" fn run_static_initializers() {
    u_is_space = {
        let mut init = C2RustUnnamed {
            header: [1 as libc::c_int],
            level1: [
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_int,
            ],
            level2: [
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (128 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (128 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(16 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (128 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(32 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (128 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(48 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
            ],
            level3: [
                0x3e00 as libc::c_uint,
                0x1 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x77f as libc::c_uint,
                0x300 as libc::c_uint,
                0x80000000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
            ],
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
