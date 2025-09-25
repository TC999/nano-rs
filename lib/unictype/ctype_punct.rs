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
    pub level2: [libc::c_short; 512],
    pub level3: [libc::c_uint; 1200],
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
static mut u_is_punct: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 17],
    level2: [0; 512],
    level3: [0; 1200],
};
#[no_mangle]
pub unsafe extern "C" fn uc_is_punct(mut uc: ucs4_t) -> bool {
    return bitmap_lookup(&u_is_punct as *const C2RustUnnamed as *const libc::c_void, uc)
        != 0;
}
unsafe extern "C" fn run_static_initializers() {
    u_is_punct = {
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
                -(1 as libc::c_int),
                -(1 as libc::c_int),
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
                (18 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(384 as libc::c_int as libc::c_ulong) as libc::c_int,
            ],
            level2: [
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
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
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(384 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(432 as libc::c_int as libc::c_ulong) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(560 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
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
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(816 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
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
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(880 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                (18 as libc::c_int as libc::c_ulong)
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
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
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
                        (512 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(1184 as libc::c_int as libc::c_ulong) as libc::c_short,
            ],
            level3: [
                0 as libc::c_uint,
                0xfc00fffe as libc::c_uint,
                0xf8000001 as libc::c_uint,
                0x78000001 as libc::c_uint,
                0 as libc::c_uint,
                0xfbdffbff as libc::c_uint,
                0x800000 as libc::c_uint,
                0x800000 as libc::c_uint,
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
                0xfffc003c as libc::c_uint,
                0xffffafe0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffdf as libc::c_uint,
                0x4020ffff as libc::c_uint,
                0xb0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x400000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3fc as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfc000000 as libc::c_uint,
                0 as libc::c_uint,
                0xfffee600 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xff as libc::c_uint,
                0x180000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0 as libc::c_uint,
                0xfffff800 as libc::c_uint,
                0x13c00 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffd00000 as libc::c_uint,
                0x60003f9f as libc::c_uint,
                0x2bfff as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x7ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1ffc0 as libc::c_uint,
                0 as libc::c_uint,
                0xe3cff800 as libc::c_uint,
                0xfbc00000 as libc::c_uint,
                0x7fff3eef as libc::c_uint,
                0x4e000000 as libc::c_uint,
                0 as libc::c_uint,
                0xff830100 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffc00 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf as libc::c_uint,
                0xdc000000 as libc::c_uint,
                0xfeffff as libc::c_uint,
                0x1003c as libc::c_uint,
                0xe as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0x80399f as libc::c_uint,
                0x6ffc000c as libc::c_uint,
                0xe as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0x23987 as libc::c_uint,
                0x630000 as libc::c_uint,
                0xe as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0x3bbf as libc::c_uint,
                0xfc03000c as libc::c_uint,
                0xe as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0xe0399f as libc::c_uint,
                0xfd000c as libc::c_uint,
                0x4 as libc::c_uint,
                0xc0000000 as libc::c_uint,
                0x803dc7 as libc::c_uint,
                0x7ff0000 as libc::c_uint,
                0x1f as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0x603ddf as libc::c_uint,
                0xff80000c as libc::c_uint,
                0x1e as libc::c_uint,
                0xd0000000 as libc::c_uint,
                0x603ddf as libc::c_uint,
                0x8000c as libc::c_uint,
                0xf as libc::c_uint,
                0xd8000000 as libc::c_uint,
                0x7f80bddf as libc::c_uint,
                0x3ff000c as libc::c_uint,
                0xe as libc::c_uint,
                0 as libc::c_uint,
                0xff5f8400 as libc::c_uint,
                0x1c0000 as libc::c_uint,
                0 as libc::c_uint,
                0x80008000 as libc::c_uint,
                0xc008040 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1ff20000 as libc::c_uint,
                0x7f00 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffffe as libc::c_uint,
                0xfffffc00 as libc::c_uint,
                0 as libc::c_uint,
                0xfffe0000 as libc::c_uint,
                0xfeffe0ff as libc::c_uint,
                0xdfffffff as libc::c_uint,
                0x7ffdfff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7ffff800 as libc::c_uint,
                0xc3c0fc00 as libc::c_uint,
                0x1e3f9d as libc::c_uint,
                0xfc00bffc as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x8000000 as libc::c_uint,
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
                0xe0000000 as libc::c_uint,
                0x1fffffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x6000 as libc::c_uint,
                0x18000000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3800 as libc::c_uint,
                0x3c0000 as libc::c_uint,
                0x7c0000 as libc::c_uint,
                0xc0000 as libc::c_uint,
                0xc0000 as libc::c_uint,
                0 as libc::c_uint,
                0xfff00000 as libc::c_uint,
                0x2f7fffff as libc::c_uint,
                0x3ff0000 as libc::c_uint,
                0xffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x60 as libc::c_uint,
                0x200 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfff0fff as libc::c_uint,
                0x31 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xc4000000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xcf800000 as libc::c_uint,
                0 as libc::c_uint,
                0x7fe00000 as libc::c_uint,
                0x9fffffff as libc::c_uint,
                0 as libc::c_uint,
                0xffff3f7f as libc::c_uint,
                0x7fff as libc::c_uint,
                0 as libc::c_uint,
                0x1f as libc::c_uint,
                0xfff00000 as libc::c_uint,
                0xfc00c01f as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7 as libc::c_uint,
                0x3ffe as libc::c_uint,
                0 as libc::c_uint,
                0xf00fffc0 as libc::c_uint,
                0 as libc::c_uint,
                0xf8fffff0 as libc::c_uint,
                0 as libc::c_uint,
                0xc0000000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff00ff as libc::c_uint,
                0x39021ff as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xa0000000 as libc::c_uint,
                0xe000e003 as libc::c_uint,
                0x6000e000 as libc::c_uint,
                0xfffff880 as libc::c_uint,
                0xfffffcff as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0x7ff1ffdf as libc::c_uint,
                0x7fff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff0001 as libc::c_uint,
                0x1ffff as libc::c_uint,
                0xc1d0037b as libc::c_uint,
                0xc0040af as libc::c_uint,
                0xffffbc1f as libc::c_uint,
                0 as libc::c_uint,
                0xffff0e00 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x3ff as libc::c_uint,
                0x7ff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffc00 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfe0387e0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x80010000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff7fff as libc::c_uint,
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
                0xffffff1e as libc::c_uint,
                0xe0c1fc01 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1e000000 as libc::c_uint,
                0x1 as libc::c_uint,
                0 as libc::c_uint,
                0x8000000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x803f as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7f as libc::c_uint,
                0xc0000000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xe000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fff8000 as libc::c_uint,
                0xc0000000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xff0000 as libc::c_uint,
                0x7fffff as libc::c_uint,
                0x3 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x600 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x844 as libc::c_uint,
                0x3ff1ff8 as libc::c_uint,
                0 as libc::c_uint,
                0xf00000 as libc::c_uint,
                0x3 as libc::c_uint,
                0xfff00000 as libc::c_uint,
                0xc03f as libc::c_uint,
                0x9703ffff as libc::c_uint,
                0 as libc::c_uint,
                0xffc0 as libc::c_uint,
                0x800fff80 as libc::c_uint,
                0 as libc::c_uint,
                0xf as libc::c_uint,
                0xfff80000 as libc::c_uint,
                0xc0003fff as libc::c_uint,
                0x20 as libc::c_uint,
                0 as libc::c_uint,
                0x7ffe00 as libc::c_uint,
                0xf0003008 as libc::c_uint,
                0x3b800000 as libc::c_uint,
                0 as libc::c_uint,
                0xc19d0000 as libc::c_uint,
                0xc0000002 as libc::c_uint,
                0x63f800 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x8000000 as libc::c_uint,
                0xc00 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3ff8 as libc::c_uint,
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
                0x40000000 as libc::c_uint,
                0x200 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffc0000 as libc::c_uint,
                0x7 as libc::c_uint,
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
                0xc0000000 as libc::c_uint,
                0xffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x8000 as libc::c_uint,
                0xf0000000 as libc::c_uint,
                0x3ffffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xfff7ffff as libc::c_uint,
                0xf7f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x80000000 as libc::c_uint,
                0xfc00fffe as libc::c_uint,
                0xf8000001 as libc::c_uint,
                0xf8000001 as libc::c_uint,
                0x3f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3e007f7f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffff87 as libc::c_uint,
                0xff8fffff as libc::c_uint,
                0 as libc::c_uint,
                0xffe00000 as libc::c_uint,
                0x1fff7fff as libc::c_uint,
                0x1 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3fffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffffff as libc::c_uint,
                0 as libc::c_uint,
                0xf as libc::c_uint,
                0 as libc::c_uint,
                0x7c00000 as libc::c_uint,
                0x80000000 as libc::c_uint,
                0 as libc::c_uint,
                0x10000 as libc::c_uint,
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
                0x8000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xff800000 as libc::c_uint,
                0xff800000 as libc::c_uint,
                0 as libc::c_uint,
                0xff80 as libc::c_uint,
                0 as libc::c_uint,
                0xf8000000 as libc::c_uint,
                0x8fc00000 as libc::c_uint,
                0x80000000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x30000000 as libc::c_uint,
                0xfffcffff as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xf06e as libc::c_uint,
                0x87000000 as libc::c_uint,
                0x1ff01ff as libc::c_uint,
                0xe0000000 as libc::c_uint,
                0xe0000000 as libc::c_uint,
                0 as libc::c_uint,
                0x100 as libc::c_uint,
                0x7ff860 as libc::c_uint,
                0 as libc::c_uint,
                0xfe000000 as libc::c_uint,
                0xff000000 as libc::c_uint,
                0xff000000 as libc::c_uint,
                0x1e000000 as libc::c_uint,
                0xfe00 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfc000000 as libc::c_uint,
                0 as libc::c_uint,
                0xf0 as libc::c_uint,
                0 as libc::c_uint,
                0x7e00 as libc::c_uint,
                0xc000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fffffff as libc::c_uint,
                0 as libc::c_uint,
                0x3800 as libc::c_uint,
                0 as libc::c_uint,
                0xf0000000 as libc::c_uint,
                0xe0000000 as libc::c_uint,
                0x7f as libc::c_uint,
                0x3ffffc0 as libc::c_uint,
                0 as libc::c_uint,
                0x3fc as libc::c_uint,
                0 as libc::c_uint,
                0xfe0 as libc::c_uint,
                0 as libc::c_uint,
                0x7 as libc::c_uint,
                0xff000000 as libc::c_uint,
                0xfffc3fff as libc::c_uint,
                0x8019003f as libc::c_uint,
                0x7 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x2007 as libc::c_uint,
                0 as libc::c_uint,
                0x7 as libc::c_uint,
                0x1fff80 as libc::c_uint,
                0x6f as libc::c_uint,
                0x380000 as libc::c_uint,
                0x7 as libc::c_uint,
                0xfff80000 as libc::c_uint,
                0xe800ffe1 as libc::c_uint,
                0x1ffffe as libc::c_uint,
                0 as libc::c_uint,
                0x7ffff000 as libc::c_uint,
                0x2 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x200 as libc::c_uint,
                0x80000000 as libc::c_uint,
                0x7ff as libc::c_uint,
                0xf as libc::c_uint,
                0xd8000000 as libc::c_uint,
                0x80399f as libc::c_uint,
                0x1f1fcc as libc::c_uint,
                0 as libc::c_uint,
                0xff000000 as libc::c_uint,
                0x1b5f7a5 as libc::c_uint,
                0x6 as libc::c_uint,
                0 as libc::c_uint,
                0xffe00000 as libc::c_uint,
                0x6c00f87f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x4f as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xff3f8000 as libc::c_uint,
                0x30ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xf as libc::c_uint,
                0x1fff as libc::c_uint,
                0 as libc::c_uint,
                0x2fff800 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xe0000000 as libc::c_uint,
                0xfc000fff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fc00 as libc::c_uint,
                0 as libc::c_uint,
                0x79bf0000 as libc::c_uint,
                0x7d as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfcfe0000 as libc::c_uint,
                0x15 as libc::c_uint,
                0x7fe as libc::c_uint,
                0xfbf80000 as libc::c_uint,
                0xffe00ff as libc::c_uint,
                0 as libc::c_uint,
                0xdffffc00 as libc::c_uint,
                0x7 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x2 as libc::c_uint,
                0 as libc::c_uint,
                0xff7f8000 as libc::c_uint,
                0xfc00003e as libc::c_uint,
                0x31fff as libc::c_uint,
                0xfffc0000 as libc::c_uint,
                0x7ffeff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xb47e0000 as libc::c_uint,
                0xbf as libc::c_uint,
                0 as libc::c_uint,
                0xfb7c00 as libc::c_uint,
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
                0x1f80000 as libc::c_uint,
                0xb as libc::c_uint,
                0xc7f00000 as libc::c_uint,
                0x400ffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x8003ffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x1f0000 as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x60000 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3fff81 as libc::c_uint,
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
                0xc0000000 as libc::c_uint,
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
                0xc000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x3f0000 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xf8000030 as libc::c_uint,
                0x3 as libc::c_uint,
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
                0xe000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xfffe8000 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x780ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x30014 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xf0000000 as libc::c_uint,
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
                0x3fffff as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x8000002 as libc::c_uint,
                0x8000000 as libc::c_uint,
                0x200000 as libc::c_uint,
                0x200000 as libc::c_uint,
                0x8000 as libc::c_uint,
                0x8000 as libc::c_uint,
                0x200 as libc::c_uint,
                0x200 as libc::c_uint,
                0x8 as libc::c_uint,
                0 as libc::c_uint,
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
                0xf9ffff7f as libc::c_uint,
                0x7db as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x8000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7f0000 as libc::c_uint,
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
                0x4000 as libc::c_uint,
                0 as libc::c_uint,
                0x8000f000 as libc::c_uint,
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
                0xf000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x8000c000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fff80 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xc00007f0 as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
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
                0xffff as libc::c_uint,
                0xe000 as libc::c_uint,
                0xfc00 as libc::c_uint,
                0xfc00 as libc::c_uint,
                0xfffff800 as libc::c_uint,
                0x3fdf as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
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
                0xffff as libc::c_uint,
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
