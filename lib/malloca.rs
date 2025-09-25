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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type small_t = libc::c_uchar;
pub const sa_alignment_max: C2RustUnnamed = 16;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed = libc::c_uint;
pub const sa_alignment_longdouble: C2RustUnnamed = 16;
pub const sa_alignment_longlong: C2RustUnnamed = 8;
pub const sa_alignment_double: C2RustUnnamed = 8;
pub const sa_alignment_long: C2RustUnnamed = 8;
#[no_mangle]
pub unsafe extern "C" fn mmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut alignment2_mask: uintptr_t = (2 as libc::c_int
        * sa_alignment_max as libc::c_int - 1 as libc::c_int) as uintptr_t;
    let mut plus: libc::c_int = (::core::mem::size_of::<small_t>() as libc::c_ulong)
        .wrapping_add(alignment2_mask) as libc::c_int;
    let mut nplus: idx_t = 0;
    let (fresh0, fresh1) = n.overflowing_add(plus);
    *(&mut nplus as *mut idx_t) = fresh0;
    if !fresh1
        && !(1 as libc::c_int != 0 as libc::c_int
            && (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
                .wrapping_div(1 as libc::c_int as libc::c_ulong)
                < nplus as libc::c_ulong)
    {
        let mut mem: *mut libc::c_char = malloc(nplus as libc::c_ulong)
            as *mut libc::c_char;
        if !mem.is_null() {
            let mut umem: uintptr_t = mem as uintptr_t;
            let mut umemplus: uintptr_t = 0;
            let (fresh2, fresh3) = umem
                .overflowing_add(
                    (::core::mem::size_of::<small_t>() as libc::c_ulong)
                        .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            *(&mut umemplus as *mut uintptr_t) = fresh2;
            let mut offset: idx_t = umemplus
                .wrapping_sub(
                    umemplus
                        % (2 as libc::c_int * sa_alignment_max as libc::c_int)
                            as uintptr_t,
                )
                .wrapping_add(sa_alignment_max as libc::c_int as uintptr_t)
                .wrapping_sub(umem) as idx_t;
            let mut p: *mut libc::c_void = mem.offset(offset as isize)
                as *mut libc::c_void;
            let mut sp: *mut small_t = p as *mut small_t;
            *sp.offset(-(1 as libc::c_int) as isize) = offset as small_t;
            return p;
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn freea(mut p: *mut libc::c_void) {
    let mut u: uintptr_t = p as uintptr_t;
    if u & (sa_alignment_max as libc::c_int - 1 as libc::c_int) as uintptr_t != 0 {
        abort();
    }
    if u & sa_alignment_max as libc::c_int as uintptr_t != 0 {
        let mut cp: *mut libc::c_char = p as *mut libc::c_char;
        let mut sp: *mut small_t = p as *mut small_t;
        let mut mem: *mut libc::c_void = cp
            .offset(-(*sp.offset(-(1 as libc::c_int) as isize) as libc::c_int as isize))
            as *mut libc::c_void;
        free(mem);
    }
}
