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
    pub level1: [libc::c_int; 4],
    pub level2: [libc::c_short; 512],
    pub level3: [libc::c_uint; 1360],
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
static mut u_is_alpha: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 4],
    level2: [0; 512],
    level3: [0; 1360],
};
#[no_mangle]
pub unsafe extern "C" fn uc_is_alpha(mut uc: ucs4_t) -> bool {
    return bitmap_lookup(&u_is_alpha as *const C2RustUnnamed as *const libc::c_void, uc)
        != 0;
}
unsafe extern "C" fn run_static_initializers() {
    u_is_alpha = {
        let mut init = C2RustUnnamed {
            header: [4 as libc::c_int],
            level1: [
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_int,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(128 as libc::c_int as libc::c_ulong) as libc::c_int,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(256 as libc::c_int as libc::c_ulong) as libc::c_int,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(384 as libc::c_int as libc::c_ulong) as libc::c_int,
            ],
            level2: [
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(16 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(32 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(48 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(64 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(80 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(96 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(112 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(128 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(144 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(176 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(192 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(208 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(224 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(240 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(256 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(272 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(288 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(304 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(320 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(352 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(368 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(384 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(400 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(416 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(432 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(448 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(464 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(480 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(496 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(512 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(528 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(544 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(560 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(576 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(592 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(608 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(624 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(640 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(656 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(672 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(688 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(704 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(720 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(736 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(752 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(768 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(784 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(800 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(816 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(832 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(848 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(864 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(880 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(896 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(912 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(928 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(944 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(960 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(976 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(992 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1008 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1024 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1040 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1056 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1072 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1088 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1104 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1120 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1136 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1152 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1168 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1184 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1200 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1216 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1232 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1248 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1264 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1280 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1296 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1312 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1328 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1344 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
            ],
            level3: [
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fffffe as libc::c_uint,
                0x7fffffe as libc::c_uint,
                0 as libc::c_uint,
                0x4200400 as libc::c_uint,
                0xff7fffff as libc::c_uint,
                0xff7fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffc3 as libc::c_uint,
                0x501f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x20 as libc::c_uint,
                0xbcdf0000 as libc::c_uint,
                0xffffd740 as libc::c_uint,
                0xfffffffb as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffbfffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffc03 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffeffff as libc::c_uint,
                0x27fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ff as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x787ff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ff as libc::c_uint,
                0xfffec3ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x2fffff as libc::c_uint,
                0x9fffc060 as libc::c_uint,
                0xfffd0000 as libc::c_uint,
                0xffff as libc::c_uint,
                0xffffe000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x2003f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x43007ff as libc::c_uint,
                0x43fffff as libc::c_uint,
                0x110 as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0x7eff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff as libc::c_uint,
                0 as libc::c_uint,
                0xfffffff0 as libc::c_uint,
                0x23ffffff as libc::c_uint,
                0xff010000 as libc::c_uint,
                0xfffeffc3 as libc::c_uint,
                0xfff99fe1 as libc::c_uint,
                0x23c5fdff as libc::c_uint,
                0xb0004000 as libc::c_uint,
                0x1003ffc3 as libc::c_uint,
                0xfff987e0 as libc::c_uint,
                0x36dfdff as libc::c_uint,
                0x5e000000 as libc::c_uint,
                0x1cffc0 as libc::c_uint,
                0xfffbbfe0 as libc::c_uint,
                0x23edfdff as libc::c_uint,
                0x10000 as libc::c_uint,
                0x200ffc3 as libc::c_uint,
                0xfff99fe0 as libc::c_uint,
                0x23edfdff as libc::c_uint,
                0xb0000000 as libc::c_uint,
                0x2ffc3 as libc::c_uint,
                0xd63dc7e8 as libc::c_uint,
                0x3ffc718 as libc::c_uint,
                0x10000 as libc::c_uint,
                0xffc0 as libc::c_uint,
                0xfffddfe0 as libc::c_uint,
                0x23fffdff as libc::c_uint,
                0x27000000 as libc::c_uint,
                0xffc3 as libc::c_uint,
                0xfffddfe1 as libc::c_uint,
                0x23effdff as libc::c_uint,
                0x60000000 as libc::c_uint,
                0x6ffc3 as libc::c_uint,
                0xfffddff0 as libc::c_uint,
                0x27ffffff as libc::c_uint,
                0x80704000 as libc::c_uint,
                0xfc00ffc3 as libc::c_uint,
                0xfc7fffe0 as libc::c_uint,
                0x2ffbffff as libc::c_uint,
                0x7f as libc::c_uint,
                0xffc0 as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0x7ff7fff as libc::c_uint,
                0x3ff7fbf as libc::c_uint,
                0 as libc::c_uint,
                0xfffff7d6 as libc::c_uint,
                0x200dffaf as libc::c_uint,
                0xf3ff005f as libc::c_uint,
                0 as libc::c_uint,
                0x1 as libc::c_uint,
                0x3ff as libc::c_uint,
                0xfffffeff as libc::c_uint,
                0x1fff as libc::c_uint,
                0x1f00 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x800007ff as libc::c_uint,
                0x3c3f03ff as libc::c_uint,
                0xffe1c062 as libc::c_uint,
                0x3ff4003 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff20bf as libc::c_uint,
                0xf7ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3d7f3dff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff3dff as libc::c_uint,
                0x7f3dffff as libc::c_uint,
                0xff7fff3d as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xff3dffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0 as libc::c_uint,
                0xffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3f3fffff as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff9fff as libc::c_uint,
                0x7fffffe as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffc7ff as libc::c_uint,
                0x8003ffff as libc::c_uint,
                0x3ffff as libc::c_uint,
                0x3ffff as libc::c_uint,
                0x1dfff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0x10800000 as libc::c_uint,
                0x3ff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0xffffff9f as libc::c_uint,
                0xffff05ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffc0 as libc::c_uint,
                0x1f3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0fff as libc::c_uint,
                0x3ff03ff as libc::c_uint,
                0 as libc::c_uint,
                0x7fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fffff as libc::c_uint,
                0 as libc::c_uint,
                0x3ff03ff as libc::c_uint,
                0x80 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffe0 as libc::c_uint,
                0xfffff as libc::c_uint,
                0x3ff1fe0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffff8 as libc::c_uint,
                0xffffc001 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf as libc::c_uint,
                0xffffe3ff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0xe7ffffff as libc::c_uint,
                0 as libc::c_uint,
                0x46fde00 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3f3fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xaaff3f3f as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x5fdfffff as libc::c_uint,
                0xfcf1fdc as libc::c_uint,
                0x1fdc1fff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x80020000 as libc::c_uint,
                0x1fff0000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3e2ffc84 as libc::c_uint,
                0xf3ffbf50 as libc::c_uint,
                0x43e0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xf0000000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xc781f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff20bf as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x80ff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0x7f7f7f7f as libc::c_uint,
                0x7f7f7f7f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x8000 as libc::c_uint,
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
                0xe0 as libc::c_uint,
                0x1f3e03fe as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xe07fffff as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf7ffffff as libc::c_uint,
                0xffffffe0 as libc::c_uint,
                0xfffeffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fff as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff1fff as libc::c_uint,
                0xfff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x80007fff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
                0xff800000 as libc::c_uint,
                0xfffffffc as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffff9ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1feb3fff as libc::c_uint,
                0xfffc0000 as libc::c_uint,
                0xfffff7bb as libc::c_uint,
                0x7 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0xfffffffc as libc::c_uint,
                0xfffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0x68fc0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff003f as libc::c_uint,
                0x7f as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0xfffffff0 as libc::c_uint,
                0x7ffff as libc::c_uint,
                0x3ff8000 as libc::c_uint,
                0x7fffffdf as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ff as libc::c_uint,
                0x3ff0ff7 as libc::c_uint,
                0xc47fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3e62ffff as libc::c_uint,
                0x38000005 as libc::c_uint,
                0x1c07ff as libc::c_uint,
                0x7e7e7e as libc::c_uint,
                0xffff7f7f as libc::c_uint,
                0xf7ffffff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff0007 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff000f as libc::c_uint,
                0xfffff87f as libc::c_uint,
                0xfffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0xa0f8007f as libc::c_uint,
                0x5f7ffdff as libc::c_uint,
                0xffffffdb as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffff as libc::c_uint,
                0xfff80000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffcffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xff as libc::c_uint,
                0xfff0000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffdf0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0x7fffffe as libc::c_uint,
                0x7fffffe as libc::c_uint,
                0xffffffc0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0x1cfcfcfc as libc::c_uint,
                0 as libc::c_uint,
                0xffffefff as libc::c_uint,
                0xb7ffff7f as libc::c_uint,
                0x3fff3fff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffe000 as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3eff0f as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0xff0fffff as libc::c_uint,
                0xfffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf7ff000f as libc::c_uint,
                0xffb7f7ff as libc::c_uint,
                0x1bfbfffb as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0xff as libc::c_uint,
                0xffffffbf as libc::c_uint,
                0x7fdffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffd3f as libc::c_uint,
                0x91bfffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x37ffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xc0ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfeef0001 as libc::c_uint,
                0x3fffff as libc::c_uint,
                0 as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0 as libc::c_uint,
                0xfffffeff as libc::c_uint,
                0x1f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x7ffff as libc::c_uint,
                0x3ffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff000f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff803f as libc::c_uint,
                0x3f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x303ff as libc::c_uint,
                0x1c as libc::c_uint,
                0 as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0xffff0080 as libc::c_uint,
                0x3f as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x1f as libc::c_uint,
                0x7fffff as libc::c_uint,
                0xfffffff8 as libc::c_uint,
                0xffffff as libc::c_uint,
                0 as libc::c_uint,
                0x26ffc0 as libc::c_uint,
                0xfffffff8 as libc::c_uint,
                0xffff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3ff01ff as libc::c_uint,
                0xfffffff8 as libc::c_uint,
                0xffc0007f as libc::c_uint,
                0xffff0090 as libc::c_uint,
                0x47ffff as libc::c_uint,
                0xfffffff8 as libc::c_uint,
                0x7ffff as libc::c_uint,
                0x17ff001e as libc::c_uint,
                0 as libc::c_uint,
                0xfffbffff as libc::c_uint,
                0x80000fff as libc::c_uint,
                0x1 as libc::c_uint,
                0 as libc::c_uint,
                0xbfffbd7f as libc::c_uint,
                0xffff01ff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0xfff99fe0 as libc::c_uint,
                0x23edfdff as libc::c_uint,
                0xe0010000 as libc::c_uint,
                0x3 as libc::c_uint,
                0xffff4bff as libc::c_uint,
                0xbfffff as libc::c_uint,
                0xa0000 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fffff as libc::c_uint,
                0x83ff0780 as libc::c_uint,
                0x3 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
                0x3ff00b0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fff as libc::c_uint,
                0xf000000 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
                0x3ff0010 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x10007ff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0xf as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0x7f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x800003ff as libc::c_uint,
                0xff6ff27f as libc::c_uint,
                0x8000ffff as libc::c_uint,
                0x3ff0002 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffcff as libc::c_uint,
                0x1ffff as libc::c_uint,
                0xa as libc::c_uint,
                0xfffff801 as libc::c_uint,
                0x407ffff as libc::c_uint,
                0xf0010000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x200003ff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff0001 as libc::c_uint,
                0xfffffdff as libc::c_uint,
                0x7fff as libc::c_uint,
                0x3ff0001 as libc::c_uint,
                0xfffc0000 as libc::c_uint,
                0xffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffb7f as libc::c_uint,
                0x1ffff as libc::c_uint,
                0x3ff0040 as libc::c_uint,
                0xfffffdbf as libc::c_uint,
                0x10003ff as libc::c_uint,
                0x3ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7ffff as libc::c_uint,
                0xfffdfff4 as libc::c_uint,
                0xfffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x10000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf as libc::c_uint,
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
                0xffff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
                0x7e as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7f as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0x3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
                0x3ff000f as libc::c_uint,
                0xe0fffff8 as libc::c_uint,
                0xffff as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0x3ff1fff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x107ff as libc::c_uint,
                0 as libc::c_uint,
                0xfff80000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xb as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x80000000 as libc::c_uint,
                0x1ff as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x6fef0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x40007 as libc::c_uint,
                0x270000 as libc::c_uint,
                0xffff00f0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fff07ff as libc::c_uint,
                0x3ff01ff as libc::c_uint,
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
                0xffc00000 as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffdfffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xdfffffff as libc::c_uint,
                0xebffde64 as libc::c_uint,
                0xffffffef as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xdfdfe7bf as libc::c_uint,
                0x7bffffff as libc::c_uint,
                0xfffdfc5f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffff3f as libc::c_uint,
                0xf7fffffd as libc::c_uint,
                0xf7ffffff as libc::c_uint,
                0xffdfffff as libc::c_uint,
                0xffdfffff as libc::c_uint,
                0xffff7fff as libc::c_uint,
                0xffff7fff as libc::c_uint,
                0xfffffdff as libc::c_uint,
                0xfffffdff as libc::c_uint,
                0xffffcff7 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0x7e0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3f801fff as libc::c_uint,
                0x43ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff0fff as libc::c_uint,
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
                0xffff0000 as libc::c_uint,
                0x3ff0fff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x7ff3fff as libc::c_uint,
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
                0x7fff6f7f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1f as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff080f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffef as libc::c_uint,
                0xaf7fe96 as libc::c_uint,
                0xaa96ea84 as libc::c_uint,
                0x5ef7f796 as libc::c_uint,
                0xffffbff as libc::c_uint,
                0xffffbee as libc::c_uint,
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
                0xffff0000 as libc::c_uint,
                0xffff1fff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0x7ff as libc::c_uint,
                0x20 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffc0 as libc::c_uint,
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
                0x3ff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0003 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0001 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
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
                0x3fffffff as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
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
