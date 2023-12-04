use crate::ffi;

// A DMT timing.
#[derive(Debug, Copy, Clone)]
pub struct Timing {
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

impl From<ffi::dmt::di_dmt_timing> for Timing {
    fn from(value: ffi::dmt::di_dmt_timing) -> Self {
        Self {
            dmt_id: value.dmt_id,
            edid_std_id: value.edid_std_id,
            cvt_id: value.cvt_id,
            horiz_video: value.horiz_video,
            vert_video: value.vert_video,
            refresh_rate_hz: value.refresh_rate_hz,
            pixel_clock_hz: value.pixel_clock_hz,
            horiz_blank: value.horiz_blank,
            vert_blank: value.vert_blank,
            horiz_front_porch: value.horiz_front_porch,
            vert_front_porch: value.vert_front_porch,
            horiz_sync_pulse: value.horiz_sync_pulse,
            vert_sync_pulse: value.vert_sync_pulse,
            horiz_border: value.horiz_border,
            vert_border: value.vert_border,
            reduced_blanking: value.reduced_blanking,
        }
    }
}
