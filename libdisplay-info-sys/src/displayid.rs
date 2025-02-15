/* automatically generated by rust-bindgen 0.68.1 */

#[doc = " DisplayID data structure."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Get the DisplayID version."]
    pub fn di_displayid_get_version(displayid: *const di_displayid) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Get the DisplayID revision."]
    pub fn di_displayid_get_revision(displayid: *const di_displayid) -> ::std::os::raw::c_int;
}
pub const di_displayid_product_type_DI_DISPLAYID_PRODUCT_TYPE_EXTENSION: di_displayid_product_type =
    0;
pub const di_displayid_product_type_DI_DISPLAYID_PRODUCT_TYPE_TEST: di_displayid_product_type = 1;
pub const di_displayid_product_type_DI_DISPLAYID_PRODUCT_TYPE_DISPLAY_PANEL:
    di_displayid_product_type = 2;
pub const di_displayid_product_type_DI_DISPLAYID_PRODUCT_TYPE_STANDALONE_DISPLAY:
    di_displayid_product_type = 3;
pub const di_displayid_product_type_DI_DISPLAYID_PRODUCT_TYPE_TV_RECEIVER:
    di_displayid_product_type = 4;
pub const di_displayid_product_type_DI_DISPLAYID_PRODUCT_TYPE_REPEATER: di_displayid_product_type =
    5;
pub const di_displayid_product_type_DI_DISPLAYID_PRODUCT_TYPE_DIRECT_DRIVE:
    di_displayid_product_type = 6;
#[doc = " Product type identifier, defined in section 2.3."]
pub type di_displayid_product_type = ::std::os::raw::c_uint;
extern "C" {
    #[doc = " Get the DisplayID product type."]
    pub fn di_displayid_get_product_type(
        displayid: *const di_displayid,
    ) -> di_displayid_product_type;
}
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_PRODUCT_ID:
    di_displayid_data_block_tag = 0;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_DISPLAY_PARAMS:
    di_displayid_data_block_tag = 1;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_COLOR_CHARACT:
    di_displayid_data_block_tag = 2;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TYPE_I_TIMING:
    di_displayid_data_block_tag = 3;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TYPE_II_TIMING:
    di_displayid_data_block_tag = 4;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TYPE_III_TIMING:
    di_displayid_data_block_tag = 5;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TYPE_IV_TIMING:
    di_displayid_data_block_tag = 6;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_VESA_TIMING:
    di_displayid_data_block_tag = 7;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_CEA_TIMING:
    di_displayid_data_block_tag = 8;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TIMING_RANGE_LIMITS:
    di_displayid_data_block_tag = 9;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_PRODUCT_SERIAL:
    di_displayid_data_block_tag = 10;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_ASCII_STRING:
    di_displayid_data_block_tag = 11;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_DISPLAY_DEVICE_DATA:
    di_displayid_data_block_tag = 12;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_INTERFACE_POWER_SEQ:
    di_displayid_data_block_tag = 13;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TRANSFER_CHARACT:
    di_displayid_data_block_tag = 14;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_DISPLAY_INTERFACE:
    di_displayid_data_block_tag = 15;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_STEREO_DISPLAY_INTERFACE:
    di_displayid_data_block_tag = 16;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TYPE_V_TIMING:
    di_displayid_data_block_tag = 17;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TILED_DISPLAY_TOPO:
    di_displayid_data_block_tag = 18;
pub const di_displayid_data_block_tag_DI_DISPLAYID_DATA_BLOCK_TYPE_VI_TIMING:
    di_displayid_data_block_tag = 19;
#[doc = " DisplayID data block tag."]
pub type di_displayid_data_block_tag = ::std::os::raw::c_uint;
#[doc = " A DisplayID data block."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid_data_block {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " Get a DisplayID data block tag."]
    pub fn di_displayid_data_block_get_tag(
        data_block: *const di_displayid_data_block,
    ) -> di_displayid_data_block_tag;
}
#[doc = " Display parameters feature support flags, defined in section 4.2.3."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid_display_params_features {
    pub audio: bool,
    pub separate_audio_inputs: bool,
    pub audio_input_override: bool,
    pub power_management: bool,
    pub fixed_timing: bool,
    pub fixed_pixel_format: bool,
    pub ai: bool,
    pub deinterlacing: bool,
}
#[test]
fn bindgen_test_layout_di_displayid_display_params_features() {
    const UNINIT: ::std::mem::MaybeUninit<di_displayid_display_params_features> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_displayid_display_params_features>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(di_displayid_display_params_features)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<di_displayid_display_params_features>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(di_displayid_display_params_features)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).audio) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(audio)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).separate_audio_inputs) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(separate_audio_inputs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).audio_input_override) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(audio_input_override)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).power_management) as usize - ptr as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(power_management)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fixed_timing) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(fixed_timing)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fixed_pixel_format) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(fixed_pixel_format)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ai) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(ai)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).deinterlacing) as usize - ptr as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params_features),
            "::",
            stringify!(deinterlacing)
        )
    );
}
#[doc = " Display parameters data block, defined in section 4.2."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid_display_params {
    pub horiz_image_mm: f32,
    pub vert_image_mm: f32,
    pub horiz_pixels: i32,
    pub vert_pixels: i32,
    pub features: *const di_displayid_display_params_features,
    pub gamma: f32,
    pub aspect_ratio: f32,
    pub bits_per_color_overall: i32,
    pub bits_per_color_native: i32,
}
#[test]
fn bindgen_test_layout_di_displayid_display_params() {
    const UNINIT: ::std::mem::MaybeUninit<di_displayid_display_params> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_displayid_display_params>(),
        40usize,
        concat!("Size of: ", stringify!(di_displayid_display_params))
    );
    assert_eq!(
        ::std::mem::align_of::<di_displayid_display_params>(),
        8usize,
        concat!("Alignment of ", stringify!(di_displayid_display_params))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_image_mm) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(horiz_image_mm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_image_mm) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(vert_image_mm)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_pixels) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(horiz_pixels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_pixels) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(vert_pixels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).features) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(features)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gamma) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(gamma)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aspect_ratio) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(aspect_ratio)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bits_per_color_overall) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(bits_per_color_overall)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bits_per_color_native) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_display_params),
            "::",
            stringify!(bits_per_color_native)
        )
    );
}
extern "C" {
    #[doc = " Get display parameters from a DisplayID data block.\n\n Returns NULL if the data block tag isn't\n DI_DISPLAYID_DATA_BLOCK_DISPLAY_PARAMS."]
    pub fn di_displayid_data_block_get_display_params(
        data_block: *const di_displayid_data_block,
    ) -> *const di_displayid_display_params;
}
pub const di_displayid_type_i_ii_vii_timing_stereo_3d_DI_DISPLAYID_TYPE_I_II_VII_TIMING_STEREO_3D_NEVER : di_displayid_type_i_ii_vii_timing_stereo_3d = 0 ;
pub const di_displayid_type_i_ii_vii_timing_stereo_3d_DI_DISPLAYID_TYPE_I_II_VII_TIMING_STEREO_3D_ALWAYS : di_displayid_type_i_ii_vii_timing_stereo_3d = 1 ;
pub const di_displayid_type_i_ii_vii_timing_stereo_3d_DI_DISPLAYID_TYPE_I_II_VII_TIMING_STEREO_3D_USER : di_displayid_type_i_ii_vii_timing_stereo_3d = 2 ;
pub type di_displayid_type_i_ii_vii_timing_stereo_3d = ::std::os::raw::c_uint;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_1_1:
    di_displayid_timing_aspect_ratio = 0;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_5_4:
    di_displayid_timing_aspect_ratio = 1;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_4_3:
    di_displayid_timing_aspect_ratio = 2;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_15_9:
    di_displayid_timing_aspect_ratio = 3;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_16_9:
    di_displayid_timing_aspect_ratio = 4;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_16_10:
    di_displayid_timing_aspect_ratio = 5;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_64_27:
    di_displayid_timing_aspect_ratio = 6;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_256_135:
    di_displayid_timing_aspect_ratio = 7;
pub const di_displayid_timing_aspect_ratio_DI_DISPLAYID_TIMING_ASPECT_RATIO_UNDEFINED:
    di_displayid_timing_aspect_ratio = 8;
pub type di_displayid_timing_aspect_ratio = ::std::os::raw::c_uint;
pub const di_displayid_type_i_ii_vii_timing_sync_polarity_DI_DISPLAYID_TYPE_I_II_VII_TIMING_SYNC_NEGATIVE : di_displayid_type_i_ii_vii_timing_sync_polarity = 0 ;
pub const di_displayid_type_i_ii_vii_timing_sync_polarity_DI_DISPLAYID_TYPE_I_II_VII_TIMING_SYNC_POSITIVE : di_displayid_type_i_ii_vii_timing_sync_polarity = 1 ;
pub type di_displayid_type_i_ii_vii_timing_sync_polarity = ::std::os::raw::c_uint;
#[doc = " Type I timing, defined in DisplayID 1.3 section 4.4.1 and\n Type II timing, defined in DisplayID 1.3 section 4.4.2 and\n Type VII timing, defined in DisplayID 2.0 section 4.3.1."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid_type_i_ii_vii_timing {
    pub pixel_clock_mhz: f64,
    pub preferred: bool,
    pub stereo_3d: di_displayid_type_i_ii_vii_timing_stereo_3d,
    pub interlaced: bool,
    pub aspect_ratio: di_displayid_timing_aspect_ratio,
    pub horiz_active: i32,
    pub vert_active: i32,
    pub horiz_blank: i32,
    pub vert_blank: i32,
    pub horiz_offset: i32,
    pub vert_offset: i32,
    pub horiz_sync_width: i32,
    pub vert_sync_width: i32,
    pub horiz_sync_polarity: di_displayid_type_i_ii_vii_timing_sync_polarity,
    pub vert_sync_polarity: di_displayid_type_i_ii_vii_timing_sync_polarity,
}
#[test]
fn bindgen_test_layout_di_displayid_type_i_ii_vii_timing() {
    const UNINIT: ::std::mem::MaybeUninit<di_displayid_type_i_ii_vii_timing> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_displayid_type_i_ii_vii_timing>(),
        64usize,
        concat!("Size of: ", stringify!(di_displayid_type_i_ii_vii_timing))
    );
    assert_eq!(
        ::std::mem::align_of::<di_displayid_type_i_ii_vii_timing>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(di_displayid_type_i_ii_vii_timing)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pixel_clock_mhz) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(pixel_clock_mhz)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).preferred) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(preferred)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).stereo_3d) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(stereo_3d)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interlaced) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(interlaced)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aspect_ratio) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(aspect_ratio)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_active) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(horiz_active)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_active) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(vert_active)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_blank) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(horiz_blank)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_blank) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(vert_blank)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_offset) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(horiz_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_offset) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(vert_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_sync_width) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(horiz_sync_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_sync_width) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(vert_sync_width)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_sync_polarity) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(horiz_sync_polarity)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_sync_polarity) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_i_ii_vii_timing),
            "::",
            stringify!(vert_sync_polarity)
        )
    );
}
extern "C" {
    #[doc = " Get type I timings from a DisplayID data block.\n\n The returned array is NULL-terminated.\n\n Returns NULL if the data block tag isn't\n DI_DISPLAYID_DATA_BLOCK_TYPE_I_TIMING."]
    pub fn di_displayid_data_block_get_type_i_timings(
        data_block: *const di_displayid_data_block,
    ) -> *const *const di_displayid_type_i_ii_vii_timing;
}
#[cfg(feature = "v0_2")]
extern "C" {
    #[doc = " Get type II timings from a DisplayID data block.\n\n The returned array is NULL-terminated.\n\n Returns NULL if the data block tag isn't\n DI_DISPLAYID_DATA_BLOCK_TYPE_II_TIMING."]
    pub fn di_displayid_data_block_get_type_ii_timings(
        data_block: *const di_displayid_data_block,
    ) -> *const *const di_displayid_type_i_ii_vii_timing;
}
pub const di_displayid_tiled_topo_missing_recv_behavior_DI_DISPLAYID_TILED_TOPO_MISSING_RECV_UNDEF : di_displayid_tiled_topo_missing_recv_behavior = 0 ;
pub const di_displayid_tiled_topo_missing_recv_behavior_DI_DISPLAYID_TILED_TOPO_MISSING_RECV_TILE_ONLY : di_displayid_tiled_topo_missing_recv_behavior = 1 ;
#[doc = " Behavior when more than 1 tile and less than total number of tiles are driven\n by the source."]
pub type di_displayid_tiled_topo_missing_recv_behavior = ::std::os::raw::c_uint;
pub const di_displayid_tiled_topo_single_recv_behavior_DI_DISPLAYID_TILED_TOPO_SINGLE_RECV_UNDEF:
    di_displayid_tiled_topo_single_recv_behavior = 0;
pub const di_displayid_tiled_topo_single_recv_behavior_DI_DISPLAYID_TILED_TOPO_SINGLE_RECV_TILE_ONLY : di_displayid_tiled_topo_single_recv_behavior = 1 ;
pub const di_displayid_tiled_topo_single_recv_behavior_DI_DISPLAYID_TILED_TOPO_SINGLE_RECV_SCALED : di_displayid_tiled_topo_single_recv_behavior = 2 ;
pub const di_displayid_tiled_topo_single_recv_behavior_DI_DISPLAYID_TILED_TOPO_SINGLE_RECV_CLONED : di_displayid_tiled_topo_single_recv_behavior = 3 ;
#[doc = " Behavior of this tile when it is the only tile receiving an image from the\n source."]
pub type di_displayid_tiled_topo_single_recv_behavior = ::std::os::raw::c_uint;
#[doc = " Tiled display capabilities."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid_tiled_topo_caps {
    pub single_enclosure: bool,
    pub missing_recv_behavior: di_displayid_tiled_topo_missing_recv_behavior,
    pub single_recv_behavior: di_displayid_tiled_topo_single_recv_behavior,
}
#[test]
fn bindgen_test_layout_di_displayid_tiled_topo_caps() {
    const UNINIT: ::std::mem::MaybeUninit<di_displayid_tiled_topo_caps> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_displayid_tiled_topo_caps>(),
        12usize,
        concat!("Size of: ", stringify!(di_displayid_tiled_topo_caps))
    );
    assert_eq!(
        ::std::mem::align_of::<di_displayid_tiled_topo_caps>(),
        4usize,
        concat!("Alignment of ", stringify!(di_displayid_tiled_topo_caps))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).single_enclosure) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo_caps),
            "::",
            stringify!(single_enclosure)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).missing_recv_behavior) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo_caps),
            "::",
            stringify!(missing_recv_behavior)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).single_recv_behavior) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo_caps),
            "::",
            stringify!(single_recv_behavior)
        )
    );
}
#[doc = " Tiled display bezel information.\n\n The lengths are measured in pixels, accurate to the tenths place."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid_tiled_topo_bezel {
    pub top_px: f32,
    pub bottom_px: f32,
    pub right_px: f32,
    pub left_px: f32,
}
#[test]
fn bindgen_test_layout_di_displayid_tiled_topo_bezel() {
    const UNINIT: ::std::mem::MaybeUninit<di_displayid_tiled_topo_bezel> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_displayid_tiled_topo_bezel>(),
        16usize,
        concat!("Size of: ", stringify!(di_displayid_tiled_topo_bezel))
    );
    assert_eq!(
        ::std::mem::align_of::<di_displayid_tiled_topo_bezel>(),
        4usize,
        concat!("Alignment of ", stringify!(di_displayid_tiled_topo_bezel))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).top_px) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo_bezel),
            "::",
            stringify!(top_px)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bottom_px) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo_bezel),
            "::",
            stringify!(bottom_px)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).right_px) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo_bezel),
            "::",
            stringify!(right_px)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).left_px) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo_bezel),
            "::",
            stringify!(left_px)
        )
    );
}
#[doc = " Tiled display topology, defined in section 4.14."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct di_displayid_tiled_topo {
    pub caps: *const di_displayid_tiled_topo_caps,
    pub total_horiz_tiles: i32,
    pub total_vert_tiles: i32,
    pub horiz_tile_location: i32,
    pub vert_tile_location: i32,
    pub horiz_tile_pixels: i32,
    pub vert_tile_lines: i32,
    pub bezel: *const di_displayid_tiled_topo_bezel,
    pub vendor_id: [::std::os::raw::c_char; 3usize],
    pub product_code: u16,
    pub serial_number: u32,
}
#[test]
fn bindgen_test_layout_di_displayid_tiled_topo() {
    const UNINIT: ::std::mem::MaybeUninit<di_displayid_tiled_topo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_displayid_tiled_topo>(),
        56usize,
        concat!("Size of: ", stringify!(di_displayid_tiled_topo))
    );
    assert_eq!(
        ::std::mem::align_of::<di_displayid_tiled_topo>(),
        8usize,
        concat!("Alignment of ", stringify!(di_displayid_tiled_topo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).caps) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(caps)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).total_horiz_tiles) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(total_horiz_tiles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).total_vert_tiles) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(total_vert_tiles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_tile_location) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(horiz_tile_location)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_tile_location) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(vert_tile_location)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_tile_pixels) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(horiz_tile_pixels)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vert_tile_lines) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(vert_tile_lines)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bezel) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(bezel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vendor_id) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(vendor_id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).product_code) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(product_code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).serial_number) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_tiled_topo),
            "::",
            stringify!(serial_number)
        )
    );
}
extern "C" {
    #[doc = " Get tiled display topology from a DisplayID data block.\n\n Returns NULL if the data block tag isn't\n DI_DISPLAYID_DATA_BLOCK_TILED_DISPLAY_TOPO."]
    pub fn di_displayid_data_block_get_tiled_topo(
        data_block: *const di_displayid_data_block,
    ) -> *const di_displayid_tiled_topo;
}
#[cfg(feature = "v0_2")]
pub const di_displayid_type_iii_timing_algo_DI_DISPLAYID_TYPE_III_TIMING_CVT_STANDARD_BLANKING:
    di_displayid_type_iii_timing_algo = 0;
#[cfg(feature = "v0_2")]
pub const di_displayid_type_iii_timing_algo_DI_DISPLAYID_TYPE_III_TIMING_CVT_REDUCED_BLANKING:
    di_displayid_type_iii_timing_algo = 1;
#[doc = " Formula/algorithm for type III timings."]
#[cfg(feature = "v0_2")]
pub type di_displayid_type_iii_timing_algo = ::std::os::raw::c_uint;
#[doc = " Type III timing, defined in section 4.4.3."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg(feature = "v0_2")]
pub struct di_displayid_type_iii_timing {
    pub preferred: bool,
    pub algo: di_displayid_type_iii_timing_algo,
    pub aspect_ratio: di_displayid_timing_aspect_ratio,
    pub horiz_active: i32,
    pub interlaced: bool,
    pub refresh_rate_hz: i32,
}
#[test]
#[cfg(feature = "v0_2")]
fn bindgen_test_layout_di_displayid_type_iii_timing() {
    const UNINIT: ::std::mem::MaybeUninit<di_displayid_type_iii_timing> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<di_displayid_type_iii_timing>(),
        24usize,
        concat!("Size of: ", stringify!(di_displayid_type_iii_timing))
    );
    assert_eq!(
        ::std::mem::align_of::<di_displayid_type_iii_timing>(),
        4usize,
        concat!("Alignment of ", stringify!(di_displayid_type_iii_timing))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).preferred) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_iii_timing),
            "::",
            stringify!(preferred)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).algo) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_iii_timing),
            "::",
            stringify!(algo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aspect_ratio) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_iii_timing),
            "::",
            stringify!(aspect_ratio)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).horiz_active) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_iii_timing),
            "::",
            stringify!(horiz_active)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).interlaced) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_iii_timing),
            "::",
            stringify!(interlaced)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).refresh_rate_hz) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(di_displayid_type_iii_timing),
            "::",
            stringify!(refresh_rate_hz)
        )
    );
}
#[cfg(feature = "v0_2")]
extern "C" {
    #[doc = " Get type III timings from a DisplayID data block.\n\n The returned array is NULL-terminated.\n\n Returns NULL if the data block tag isn't\n DI_DISPLAYID_DATA_BLOCK_TYPE_III_TIMING."]
    pub fn di_displayid_data_block_get_type_iii_timings(
        data_block: *const di_displayid_data_block,
    ) -> *const *const di_displayid_type_iii_timing;
}
extern "C" {
    #[doc = " Get DisplayID data blocks.\n\n The returned array is NULL-terminated."]
    pub fn di_displayid_get_data_blocks(
        displayid: *const di_displayid,
    ) -> *const *const di_displayid_data_block;
}
