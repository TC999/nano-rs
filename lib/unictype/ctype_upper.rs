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
    pub level1: [libc::c_int; 2],
    pub level2: [libc::c_short; 256],
    pub level3: [libc::c_uint; 272],
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
static mut u_is_upper: C2RustUnnamed = C2RustUnnamed {
    header: [0; 1],
    level1: [0; 2],
    level2: [0; 256],
    level3: [0; 272],
};
#[no_mangle]
pub unsafe extern "C" fn uc_is_upper(mut uc: ucs4_t) -> bool {
    return bitmap_lookup(&u_is_upper as *const C2RustUnnamed as *const libc::c_void, uc)
        != 0;
}
unsafe extern "C" fn run_static_initializers() {
    u_is_upper = {
        let mut init = C2RustUnnamed {
            header: [2 as libc::c_int],
            level1: [
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_int,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    )
                    .wrapping_add(128 as libc::c_int as libc::c_ulong) as libc::c_int,
            ],
            level2: [
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(0 as libc::c_int as libc::c_ulong) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(16 as libc::c_int as libc::c_ulong) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
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
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(48 as libc::c_int as libc::c_ulong) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(64 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(80 as libc::c_int as libc::c_ulong) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(96 as libc::c_int as libc::c_ulong) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(112 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(128 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(144 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(160 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(176 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(192 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(208 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(224 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(240 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong,
                            )
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(256 as libc::c_int as libc::c_ulong) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
                -(1 as libc::c_int) as libc::c_short,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7f7fffff as libc::c_uint,
                0 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0xaa555555 as libc::c_uint,
                0x555554aa as libc::c_uint,
                0x2b555555 as libc::c_uint,
                0xb1dbced6 as libc::c_uint,
                0x11aed2d5 as libc::c_uint,
                0x4aaaadb0 as libc::c_uint,
                0x55d65555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x6c055555 as libc::c_uint,
                0x557a as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x80450000 as libc::c_uint,
                0xfffed740 as libc::c_uint,
                0xffb as libc::c_uint,
                0x55008000 as libc::c_uint,
                0xe6905555 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
                0 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555401 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55552aab as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0xfffe5555 as libc::c_uint,
                0x7fffff as libc::c_uint,
                0 as libc::c_uint,
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
                0x20bf as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
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
                0x3fffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0200 as libc::c_uint,
                0xe7ffffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x40155555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x3f00ff00 as libc::c_uint,
                0xff00ff00 as libc::c_uint,
                0xaa003f00 as libc::c_uint,
                0xff00 as libc::c_uint,
                0xff00ff00 as libc::c_uint,
                0x1f00ff00 as libc::c_uint,
                0xf001f00 as libc::c_uint,
                0x1f001f00 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x40c40 as libc::c_uint,
                0 as libc::c_uint,
                0xffff as libc::c_uint,
                0x8 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffc00000 as libc::c_uint,
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
                0xffffffff as libc::c_uint,
                0xffff as libc::c_uint,
                0 as libc::c_uint,
                0xc025ea9d as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x42805 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x1555 as libc::c_uint,
                0x5555555 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x55545554 as libc::c_uint,
                0x55555555 as libc::c_uint,
                0x6a005555 as libc::c_uint,
                0x55452855 as libc::c_uint,
                0x555f7d55 as libc::c_uint,
                0x15411af5 as libc::c_uint,
                0x200000 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0x7fffffe as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0xff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0xfffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xf7ff0000 as libc::c_uint,
                0x37f7ff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffffffff as libc::c_uint,
                0x7ffff as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0xffff0000 as libc::c_uint,
                0x3f as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
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
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
                0 as libc::c_uint,
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
                0x3 as libc::c_uint,
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
