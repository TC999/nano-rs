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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn rpl_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return realloc(ptr, if size != 0 { size } else { 1 as libc::c_int as size_t });
}
#[no_mangle]
pub unsafe extern "C" fn gl_dynarray_resize(
    mut list: *mut dynarray_header,
    mut size: size_t,
    mut scratch: *mut libc::c_void,
    mut element_size: size_t,
) -> bool {
    if size <= (*list).allocated {
        (*list).used = size;
        return 1 as libc::c_int != 0;
    }
    let mut new_size_bytes: size_t = 0;
    if if (0 as libc::c_int as size_t) < -(1 as libc::c_int) as size_t
        && (if 1 as libc::c_int != 0 { 0 as libc::c_int as size_t } else { size })
            .wrapping_sub(1 as libc::c_int as size_t) < 0 as libc::c_int as size_t
        && (if 1 as libc::c_int != 0 {
            0 as libc::c_int as size_t
        } else {
            element_size
        })
            .wrapping_sub(1 as libc::c_int as size_t) < 0 as libc::c_int as size_t
        && (if element_size < 0 as libc::c_int as size_t {
            (if size < 0 as libc::c_int as size_t {
                (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as size_t
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        -(1 as libc::c_int) as size_t
                    })
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as libc::c_int as size_t)
                    < 0 as libc::c_int as size_t
                {
                    (size < -(1 as libc::c_int) as size_t / element_size) as libc::c_int
                } else {
                    ((if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as libc::c_int as size_t)
                        < 0 as libc::c_int as size_t
                    {
                        !(((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            element_size
                        })
                            .wrapping_add(1 as libc::c_int as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as size_t)
                            * 2 as libc::c_int as size_t)
                            .wrapping_add(1 as libc::c_int as size_t)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as size_t)
                    }) < 0 as libc::c_int as size_t
                    {
                        (element_size
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as libc::c_int as size_t)
                                < 0 as libc::c_int as size_t
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as size_t
                                } else {
                                    element_size
                                })
                                    .wrapping_add(1 as libc::c_int as size_t)
                                    << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as size_t)
                                    * 2 as libc::c_int as size_t)
                                    .wrapping_add(1 as libc::c_int as size_t)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as size_t
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as libc::c_int as size_t)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as size_t) < element_size) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            element_size
                        })
                            .wrapping_add(-(1 as libc::c_int) as size_t)
                            >> (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) as size_t / element_size.wrapping_neg()
                    }) <= (-(1 as libc::c_int) as size_t).wrapping_sub(size))
                        as libc::c_int
                })
            } else {
                (if (if (if (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as size_t
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as size_t)
                })
                    .wrapping_sub(1 as libc::c_int as size_t)
                    < 0 as libc::c_int as size_t
                {
                    !(((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as size_t)
                    })
                        .wrapping_add(1 as libc::c_int as size_t)
                        << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as size_t)
                        * 2 as libc::c_int as size_t)
                        .wrapping_add(1 as libc::c_int as size_t)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as size_t)
                    })
                        .wrapping_add(0 as libc::c_int as size_t)
                }) < 0 as libc::c_int as size_t
                {
                    ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        element_size
                    })
                        .wrapping_add(0 as libc::c_int as size_t)
                        < (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                element_size
                            })
                                .wrapping_add(0 as libc::c_int as size_t)
                        })
                            .wrapping_sub(1 as libc::c_int as size_t)
                            < 0 as libc::c_int as size_t
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as size_t
                                } else {
                                    element_size
                                })
                                    .wrapping_add(0 as libc::c_int as size_t)
                            })
                                .wrapping_add(1 as libc::c_int as size_t)
                                << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as size_t)
                                * 2 as libc::c_int as size_t)
                                .wrapping_add(1 as libc::c_int as size_t)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as size_t
                                } else {
                                    element_size
                                })
                                    .wrapping_add(0 as libc::c_int as size_t)
                            })
                                .wrapping_sub(1 as libc::c_int as size_t)
                        })
                            .wrapping_neg()) as libc::c_int
                } else {
                    ((0 as libc::c_int as size_t)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            element_size
                        })
                            .wrapping_add(0 as libc::c_int as size_t)) as libc::c_int
                }) != 0 && element_size == -(1 as libc::c_int) as size_t
                {
                    (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        size
                    })
                        .wrapping_sub(1 as libc::c_int as size_t)
                        < 0 as libc::c_int as size_t
                    {
                        ((0 as libc::c_int as size_t)
                            < size.wrapping_add(0 as libc::c_int as size_t))
                            as libc::c_int
                    } else {
                        ((0 as libc::c_int as size_t) < size
                            && (-(1 as libc::c_int) as size_t)
                                .wrapping_sub(0 as libc::c_int as size_t)
                                < size.wrapping_sub(1 as libc::c_int as size_t))
                            as libc::c_int
                    })
                } else {
                    (0 as libc::c_int as size_t / element_size < size) as libc::c_int
                })
            })
        } else {
            (if element_size == 0 as libc::c_int as size_t {
                0 as libc::c_int
            } else {
                (if size < 0 as libc::c_int as size_t {
                    (if (if (if (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as size_t
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            size
                        })
                            .wrapping_add(0 as libc::c_int as size_t)
                    })
                        .wrapping_sub(1 as libc::c_int as size_t)
                        < 0 as libc::c_int as size_t
                    {
                        !(((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                size
                            })
                                .wrapping_add(0 as libc::c_int as size_t)
                        })
                            .wrapping_add(1 as libc::c_int as size_t)
                            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            .wrapping_sub(1 as libc::c_int as size_t)
                            * 2 as libc::c_int as size_t)
                            .wrapping_add(1 as libc::c_int as size_t)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                size
                            })
                                .wrapping_add(0 as libc::c_int as size_t)
                        })
                            .wrapping_add(0 as libc::c_int as size_t)
                    }) < 0 as libc::c_int as size_t
                    {
                        ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            size
                        })
                            .wrapping_add(0 as libc::c_int as size_t)
                            < (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as size_t
                                } else {
                                    size
                                })
                                    .wrapping_add(0 as libc::c_int as size_t)
                            })
                                .wrapping_sub(1 as libc::c_int as size_t)
                                < 0 as libc::c_int as size_t
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as size_t
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as size_t
                                    } else {
                                        size
                                    })
                                        .wrapping_add(0 as libc::c_int as size_t)
                                })
                                    .wrapping_add(1 as libc::c_int as size_t)
                                    << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as size_t)
                                    * 2 as libc::c_int as size_t)
                                    .wrapping_add(1 as libc::c_int as size_t)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as size_t
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as size_t
                                    } else {
                                        size
                                    })
                                        .wrapping_add(0 as libc::c_int as size_t)
                                })
                                    .wrapping_sub(1 as libc::c_int as size_t)
                            })
                                .wrapping_neg()) as libc::c_int
                    } else {
                        ((0 as libc::c_int as size_t)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as size_t
                            } else {
                                size
                            })
                                .wrapping_add(0 as libc::c_int as size_t)) as libc::c_int
                    }) != 0 && size == -(1 as libc::c_int) as size_t
                    {
                        (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as size_t
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as libc::c_int as size_t)
                            < 0 as libc::c_int as size_t
                        {
                            ((0 as libc::c_int as size_t)
                                < element_size.wrapping_add(0 as libc::c_int as size_t))
                                as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) as size_t)
                                .wrapping_sub(0 as libc::c_int as size_t)
                                < element_size.wrapping_sub(1 as libc::c_int as size_t))
                                as libc::c_int
                        })
                    } else {
                        (0 as libc::c_int as size_t / size < element_size) as libc::c_int
                    })
                } else {
                    (-(1 as libc::c_int) as size_t / element_size < size) as libc::c_int
                })
            })
        }) != 0
    {
        let (fresh4, fresh5) = size.overflowing_mul(element_size);
        *(&mut new_size_bytes as *mut size_t) = fresh4;
        1 as libc::c_int
    } else {
        let (fresh6, fresh7) = size.overflowing_mul(element_size);
        *(&mut new_size_bytes as *mut size_t) = fresh6;
        fresh7 as libc::c_int
    } != 0
    {
        *__errno_location() = 12 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    let mut new_array: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*list).array == scratch {
        new_array = malloc(new_size_bytes);
        if !new_array.is_null() && !((*list).array).is_null() {
            memcpy(new_array, (*list).array, (*list).used * element_size);
        }
    } else {
        new_array = rpl_realloc((*list).array, new_size_bytes);
    }
    if new_array.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*list).array = new_array;
    (*list).allocated = size;
    (*list).used = size;
    return 1 as libc::c_int != 0;
}
