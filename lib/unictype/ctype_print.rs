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
    pub level1: [libc::c_int; 17],
    pub level2: [libc::c_short; 768],
    pub level3: [libc::c_uint; 1504],
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
static mut u_is_print: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 17],
    level2: [0; 768],
    level3: [0; 1504],
};
#[no_mangle]
pub unsafe extern "C" fn uc_is_print(mut uc: ucs4_t) -> bool {
    return bitmap_lookup(&u_is_print as *const C2RustUnnamed as *const libc::c_void, uc)
        != 0;
}
unsafe extern "C" fn run_static_initializers() {
    u_is_print = {
        let mut init = C2RustUnnamed {
            header: [17 as libc::c_int],
            level1: [
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_int,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(128 as libc::c_int as libc::c_ulong) as libc::c_int,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(256 as libc::c_int as libc::c_ulong) as libc::c_int,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(384 as libc::c_int as libc::c_ulong) as libc::c_int,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                -(1 as libc::c_int),
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(512 as libc::c_int as libc::c_ulong) as libc::c_int,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(640 as libc::c_int as libc::c_ulong) as libc::c_int,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(640 as libc::c_int as libc::c_ulong) as libc::c_int,
            ],
            level2: [
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(16 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(32 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(48 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(64 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(80 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(96 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(112 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(128 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(144 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(176 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(192 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(208 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(224 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(240 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(256 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(272 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(288 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(304 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(320 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(336 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(352 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(368 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(384 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(400 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(416 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(448 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(464 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(480 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(496 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(512 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(528 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(544 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(560 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(576 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(592 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(608 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(624 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(640 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(656 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(672 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(688 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(704 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(720 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(736 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(752 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(768 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(784 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(800 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(816 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(832 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(864 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(880 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(896 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(912 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(928 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(944 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(960 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(976 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1008 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1024 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1040 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1056 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1072 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1088 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1104 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1120 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1136 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1152 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1168 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1184 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1200 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1216 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1232 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1248 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1264 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1280 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1296 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1328 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1344 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1360 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1376 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1392 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1408 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1424 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1440 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1456 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1472 as libc::c_int as libc::c_ulong) as libc::c_short,
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (768 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1488 as libc::c_int as libc::c_ulong) as libc::c_short,
            ],
            level3: [
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfcffffff as libc::c_uint,
                0xffffd7f0 as libc::c_uint,
                0xfffffffb as libc::c_uint,
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
                0xfffeffff as libc::c_uint,
                0xfe7fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffee7ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0x1f87ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffbfff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffe7ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xe7ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fff3fff as libc::c_uint,
                0x4fffffff as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0xff837fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff99fef as libc::c_uint,
                0xf3c5fdff as libc::c_uint,
                0xb080799f as libc::c_uint,
                0x7fffffcf as libc::c_uint,
                0xfff987ee as libc::c_uint,
                0xd36dfdff as libc::c_uint,
                0x5e023987 as libc::c_uint,
                0x7fffc0 as libc::c_uint,
                0xfffbbfee as libc::c_uint,
                0xf3edfdff as libc::c_uint,
                0x13bbf as libc::c_uint,
                0xfe03ffcf as libc::c_uint,
                0xfff99fee as libc::c_uint,
                0xf3edfdff as libc::c_uint,
                0xb0e0399f as libc::c_uint,
                0xffffcf as libc::c_uint,
                0xd63dc7ec as libc::c_uint,
                0xc3ffc718 as libc::c_uint,
                0x813dc7 as libc::c_uint,
                0x7ffffc0 as libc::c_uint,
                0xfffddfff as libc::c_uint,
                0xf3fffdff as libc::c_uint,
                0x27603ddf as libc::c_uint,
                0xff80ffcf as libc::c_uint,
                0xfffddfff as libc::c_uint,
                0xf3effdff as libc::c_uint,
                0x60603ddf as libc::c_uint,
                0xeffcf as libc::c_uint,
                0xfffddfff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff0fddf as libc::c_uint,
                0xffffffcf as libc::c_uint,
                0xfc7fffee as libc::c_uint,
                0x2ffbffff as libc::c_uint,
                0xff5f847f as libc::c_uint,
                0x1cffc0 as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0x87ffffff as libc::c_uint,
                0xfffffff as libc::c_uint,
                0 as libc::c_uint,
                0xfffff7d6 as libc::c_uint,
                0x3fffffaf as libc::c_uint,
                0xf3ff7f5f as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffeff as libc::c_uint,
                0xfffe1fff as libc::c_uint,
                0xfeffffff as libc::c_uint,
                0xdfffffff as libc::c_uint,
                0x7ffdfff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff20bf as libc::c_uint,
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
                0x3d7f3dff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff3dff as libc::c_uint,
                0x7f3dffff as libc::c_uint,
                0xff7fff3d as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xff3dffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xe7ffffff as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3f3fffff as libc::c_uint,
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
                0x1fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0x803fffff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0xddfff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0x3ff03ff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xfff0fff as libc::c_uint,
                0xfffffff1 as libc::c_uint,
                0x1f3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0fff as libc::c_uint,
                0xc7ff03ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xcfffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0x9fffffff as libc::c_uint,
                0x3ff03ff as libc::c_uint,
                0xffff3fff as libc::c_uint,
                0x7fff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffdfff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf00fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf8ffffff as libc::c_uint,
                0xffffe3ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0xe7ffffff as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0x7ffffff as libc::c_uint,
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
                0x3f3fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xaaff3f3f as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffdfffff as libc::c_uint,
                0xefcfffdf as libc::c_uint,
                0x7fdcffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffcff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff3ffdf as libc::c_uint,
                0x1fff7fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0001 as libc::c_uint,
                0x1ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff as libc::c_uint,
                0x7ff as libc::c_uint,
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
                0xffcfffff as libc::c_uint,
                0xffbfffff as libc::c_uint,
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
                0xfe0fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff20bf as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x800180ff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0x7f7f7f7f as libc::c_uint,
                0x7f7f7f7f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0 as libc::c_uint,
                0xfbffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfe7fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffe0 as libc::c_uint,
                0xfffeffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff7fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff803f as libc::c_uint,
                0x7fffffff as libc::c_uint,
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
                0xffff1fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff007f as libc::c_uint,
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
                0xfff as libc::c_uint,
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
                0x1feb3fff as libc::c_uint,
                0xfffc0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff1fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffc03f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x800fffff as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xc3ffbfff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0xf3ff3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf8000007 as libc::c_uint,
                0x7fffff as libc::c_uint,
                0x7e7e7e as libc::c_uint,
                0xffff7f7f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff3fff as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0xe0f8007f as libc::c_uint,
                0x5f7fffff as libc::c_uint,
                0xffffffdb as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff80007 as libc::c_uint,
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
                0xfffcffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x80ff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff7ffff as libc::c_uint,
                0xffdf0f7f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x9fffffff as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0x1cfcfcfc as libc::c_uint,
                0x3e007f7f as libc::c_uint,
                0xffffefff as libc::c_uint,
                0xb7ffff7f as libc::c_uint,
                0x3fff3fff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0xffffff87 as libc::c_uint,
                0xff8fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fff7fff as libc::c_uint,
                0x1 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffff as libc::c_uint,
                0xfffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffe00f as libc::c_uint,
                0xffff07ff as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0xbfffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fff0f as libc::c_uint,
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
                0xf7ff800f as libc::c_uint,
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
                0xffbfffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xff80 as libc::c_uint,
                0 as libc::c_uint,
                0xf837ffff as libc::c_uint,
                0x8fffffff as libc::c_uint,
                0x83ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf0ffffff as libc::c_uint,
                0xfffcffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfeeff06f as libc::c_uint,
                0x873fffff as libc::c_uint,
                0x1ff01ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ff87f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfe3fffff as libc::c_uint,
                0xff3fffff as libc::c_uint,
                0xff07ffff as libc::c_uint,
                0x1e03ffff as libc::c_uint,
                0xfe00 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfc07ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff00ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffe3f as libc::c_uint,
                0xc03f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x33bff as libc::c_uint,
                0x1c as libc::c_uint,
                0xf0000000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3ff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xfff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffc3fff as libc::c_uint,
                0x803fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff2007 as libc::c_uint,
                0x3ff01ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffdfffff as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffe as libc::c_uint,
                0xfffbffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3 as libc::c_uint,
                0 as libc::c_uint,
                0xbfffbd7f as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff07ff as libc::c_uint,
                0xfff99fef as libc::c_uint,
                0xfbedfdff as libc::c_uint,
                0xe081399f as libc::c_uint,
                0x1f1fcf as libc::c_uint,
                0xffff4bff as libc::c_uint,
                0xffbfffff as libc::c_uint,
                0x1bff7a5 as libc::c_uint,
                0x6 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xefffffff as libc::c_uint,
                0x3 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff00ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xff3fffff as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff001f as libc::c_uint,
                0x1fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0xf as libc::c_uint,
                0xe7ffffff as libc::c_uint,
                0xffff0fff as libc::c_uint,
                0x7f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x8007ffff as libc::c_uint,
                0xff6ff27f as libc::c_uint,
                0xf9bfffff as libc::c_uint,
                0x3ff007f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffcff as libc::c_uint,
                0xfcffffff as libc::c_uint,
                0x1f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0007 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0x3ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff0003 as libc::c_uint,
                0xfffffdff as libc::c_uint,
                0xff7fffff as libc::c_uint,
                0xffff003f as libc::c_uint,
                0xffff1fff as libc::c_uint,
                0xfffcffff as libc::c_uint,
                0x7ffeff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffb7f as libc::c_uint,
                0xb47fffff as libc::c_uint,
                0x3ff00ff as libc::c_uint,
                0xfffffdbf as libc::c_uint,
                0x1fb7fff as libc::c_uint,
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
                0x1ffffff as libc::c_uint,
                0xfffdffff as libc::c_uint,
                0xc7ffffff as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x10000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x8003ffff as libc::c_uint,
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
                0x1f7fff as libc::c_uint,
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
                0x7ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffff as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xffffc3ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0xffff03ff as libc::c_uint,
                0x3f3fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfbff003f as libc::c_uint,
                0xe0fffffb as libc::c_uint,
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
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff87ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff80ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3001f as libc::c_uint,
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
                0xf3ff01ff as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff3fff as libc::c_uint,
                0xffff007f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffe7f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7fffff as libc::c_uint,
                0x1ffffff as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffcfff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf8000fff as libc::c_uint,
                0xfffe as libc::c_uint,
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
                0x7fffffff as libc::c_uint,
                0x7e0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xf9ffff7f as libc::c_uint,
                0xffff07db as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fff as libc::c_uint,
                0x8000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fff1fff as libc::c_uint,
                0xc3ff as libc::c_uint,
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
                0x7fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x83ffffff as libc::c_uint,
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
                0x3ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x87ffffff as libc::c_uint,
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
                0x7fff9f as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xc3ff0fff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffe0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x1fffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0 as libc::c_uint,
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
                0x30000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffff as libc::c_uint,
                0xfffe7fff as libc::c_uint,
                0xfffefffe as libc::c_uint,
                0x3fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3fff as libc::c_uint,
                0 as libc::c_uint,
                0xffffffc0 as libc::c_uint,
                0xffff0007 as libc::c_uint,
                0xfffffff as libc::c_uint,
                0x301ff as libc::c_uint,
                0x3f as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf0ffffff as libc::c_uint,
                0x1fff1fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf87fffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0x10fff as libc::c_uint,
                0xffff0fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff00ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0xfff3fff as libc::c_uint,
                0x3 as libc::c_uint,
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
                0xfffff as libc::c_uint,
                0x1fff3fff as libc::c_uint,
                0xffff83ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x9fffc07f as libc::c_uint,
                0x1ff03ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff7ffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ffffff as libc::c_uint,
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
                0x2 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
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
                0xffff as libc::c_uint,
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
                0x3fffffff as libc::c_uint,
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
