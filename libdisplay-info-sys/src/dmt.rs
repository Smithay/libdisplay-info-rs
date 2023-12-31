/* automatically generated by rust-bindgen 0.68.1 */

#[doc = " A DMT timing."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_dmt_timing {
    pub dmt_id: u8,
    pub edid_std_id: u16,
    pub cvt_id: u32,
    pub horiz_video: i32,
    pub vert_video: i32,
    pub refresh_rate_hz: f32,
    pub pixel_clock_hz: i32,
    pub horiz_blank: i32,
    pub vert_blank: i32,
    pub horiz_front_porch: i32,
    pub vert_front_porch: i32,
    pub horiz_sync_pulse: i32,
    pub vert_sync_pulse: i32,
    pub horiz_border: i32,
    pub vert_border: i32,
    pub reduced_blanking: bool,
}
#[test]
fn bindgen_test_layout_di_dmt_timing() {
    const UNINIT: ::std::mem::MaybeUninit<di_dmt_timing> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_dmt_timing>(),
        60usize,
        concat!("Size of: ", stringify!(di_dmt_timing))
    );
    assert_eq!(
        ::std::mem::align_of::<di_dmt_timing>(),
        4usize,
        concat!("Alignment of ", stringify!(di_dmt_timing))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dmt_id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(dmt_id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).edid_std_id) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(edid_std_id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cvt_id) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(cvt_id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_video) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(horiz_video)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_video) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(vert_video)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refresh_rate_hz) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(refresh_rate_hz)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixel_clock_hz) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(pixel_clock_hz)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_blank) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(horiz_blank)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_blank) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(vert_blank)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_front_porch) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(horiz_front_porch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_front_porch) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(vert_front_porch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_sync_pulse) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(horiz_sync_pulse)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_sync_pulse) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(vert_sync_pulse)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_border) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(horiz_border)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_border) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(vert_border)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reduced_blanking) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(di_dmt_timing),
            "::",
            stringify!(reduced_blanking)
        )
    );
}
