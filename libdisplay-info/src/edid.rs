use std::marker::PhantomData;

use crate::ffi;

/// EDID data structure.
#[derive(Debug)]
pub struct Edid<'info> {
    edid: *const ffi::edid::di_edid,
    phantom: PhantomData<&'info ()>,
}

impl<'info> Edid<'info> {
    /// Get the EDID version.
    pub fn version(&self) -> i32 {
        unsafe { ffi::edid::di_edid_get_version(self.edid) }
    }

    /// Get the EDID revision.
    pub fn revision(&self) -> i32 {
        unsafe { ffi::edid::di_edid_get_revision(self.edid) }
    }

    /// Get the EDID vendor product
    pub fn vendor_product(&self) -> VendorProduct {
        VendorProduct::from(unsafe { *ffi::edid::di_edid_get_vendor_product(self.edid) })
    }

    /// Get the analog video input basic information.
    ///
    /// Returns `None` if this isn't an analog display.
    pub fn video_input_analog(&self) -> Option<VideoInputAnalog> {
        let video_input_analog = unsafe { ffi::edid::di_edid_get_video_input_analog(self.edid) };

        if video_input_analog.is_null() {
            None
        } else {
            Some(VideoInputAnalog::from(unsafe { *video_input_analog }))
        }
    }

    /// Get the digital video input basic information.
    ///
    /// Returns `None` if this isn't a digital display.
    pub fn video_input_digital(&self) -> Option<VideoInputDigital> {
        let video_input_digital = unsafe { ffi::edid::di_edid_get_video_input_digital(self.edid) };

        if video_input_digital.is_null() {
            None
        } else {
            Some(VideoInputDigital::from(unsafe { *video_input_digital }))
        }
    }

    /// Get the screen size.
    pub fn screen_size(&self) -> ScreenSize {
        ScreenSize::from(unsafe { *ffi::edid::di_edid_get_screen_size(self.edid) })
    }

    /// Get the display transfer characteristics from the basic EDID parameters, also
    /// known as \"gamma\".
    ///
    /// Returns `None` if unset (ie, stored in an extension block).
    pub fn basic_gamma(&self) -> Option<f32> {
        let basic_gamma = unsafe { ffi::edid::di_edid_get_basic_gamma(self.edid) };

        if basic_gamma == 0f32 {
            None
        } else {
            Some(basic_gamma)
        }
    }

    /// Get the set of supported legacy DPMS states.
    pub fn dpms(&self) -> Dpms {
        Dpms::from(unsafe { *ffi::edid::di_edid_get_dpms(self.edid) })
    }

    /// Get the display color type.
    ///
    /// For digital displays using EDID 1.4 and later, [`DisplayColorType::Undefined`]
    /// is always returned.
    pub fn display_color_type(&self) -> DisplayColorType {
        DisplayColorType::from(unsafe { ffi::edid::di_edid_get_display_color_type(self.edid) })
    }

    /// Get the set of supported color encoding formats.
    ///
    /// Returns `None` if the display is analog or if the color encoding formats are
    /// not specified.
    pub fn color_encoding_formats(&self) -> Option<ColorEncodingFormats> {
        let color_encoding_formats =
            unsafe { ffi::edid::di_edid_get_color_encoding_formats(self.edid) };

        if color_encoding_formats.is_null() {
            None
        } else {
            Some(ColorEncodingFormats::from(unsafe {
                *color_encoding_formats
            }))
        }
    }

    /// Get the set of miscellaneous basic features.
    pub fn misc_features(&self) -> MiscFeatures {
        MiscFeatures::from(unsafe { *ffi::edid::di_edid_get_misc_features(self.edid) })
    }

    /// Get chromaticity coordinates.
    pub fn chromaticity_coords(&self) -> ChromaticityCoords {
        ChromaticityCoords::from(unsafe { *ffi::edid::di_edid_get_chromaticity_coords(self.edid) })
    }

    /// Get a list of EDID extensions.
    pub fn extensions(&self) -> &[Extension] {
        let extensions = unsafe { ffi::edid::di_edid_get_extensions(self.edid) };

        let mut len = 0;
        while !unsafe { *extensions.offset(len) }.is_null() {
            len += 1;
        }

        unsafe { std::slice::from_raw_parts(extensions as *const Extension, len as usize) }
    }

    pub(crate) unsafe fn from_ptr(edid: *const ffi::edid::di_edid) -> Self {
        Self {
            edid,
            phantom: PhantomData,
        }
    }
}

/// EDID vendor & product identification.
#[derive(Debug, Copy, Clone)]
pub struct VendorProduct {
    pub manufacturer: [char; 3usize],
    pub product: u16,
    pub serial: u32,
    pub manufacture_week: i32,
    pub manufacture_year: i32,
    pub model_year: i32,
}

impl From<ffi::edid::di_edid_vendor_product> for VendorProduct {
    fn from(value: ffi::edid::di_edid_vendor_product) -> Self {
        VendorProduct {
            manufacturer: [
                value.manufacturer[0] as u8 as char,
                value.manufacturer[1] as u8 as char,
                value.manufacturer[2] as u8 as char,
            ],
            product: value.product,
            serial: value.serial,
            manufacture_week: value.manufacture_week,
            manufacture_year: value.manufacture_year,
            model_year: value.model_year,
        }
    }
}

/// EDID analog signal level standard.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum VideoInputAnalogSignalLevelStandard {
    Level0 = ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_0,
    Level1 = ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_1,
    Level2 = ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_2,
    Level3 = ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_3,
}

impl From<ffi::edid::di_edid_video_input_analog_signal_level_std>
    for VideoInputAnalogSignalLevelStandard
{
    fn from(value: ffi::edid::di_edid_video_input_analog_signal_level_std) -> Self {
        use VideoInputAnalogSignalLevelStandard::*;
        match value {
            ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_0 => Level0,
            ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_1 => Level1,
            ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_2 => Level2,
            ffi::edid::di_edid_video_input_analog_signal_level_std_DI_EDID_VIDEO_INPUT_ANALOG_SIGNAL_LEVEL_3 => Level3,
            _ => unreachable!()
        }
    }
}

/// EDID analog video setup.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum VideoInputAnalogVideoSetup {
    BlankLevelEqBlack = ffi::edid::di_edid_video_input_analog_video_setup_DI_EDID_VIDEO_INPUT_ANALOG_BLANK_LEVEL_EQ_BLACK,
    BlankToBlackSetupPedestal = ffi::edid::di_edid_video_input_analog_video_setup_DI_EDID_VIDEO_INPUT_ANALOG_BLANK_TO_BLACK_SETUP_PEDESTAL,
}

impl From<ffi::edid::di_edid_video_input_analog_video_setup> for VideoInputAnalogVideoSetup {
    fn from(value: ffi::edid::di_edid_video_input_analog_video_setup) -> Self {
        use VideoInputAnalogVideoSetup::*;
        match value {
            ffi::edid::di_edid_video_input_analog_video_setup_DI_EDID_VIDEO_INPUT_ANALOG_BLANK_LEVEL_EQ_BLACK => BlankLevelEqBlack,
            ffi::edid::di_edid_video_input_analog_video_setup_DI_EDID_VIDEO_INPUT_ANALOG_BLANK_TO_BLACK_SETUP_PEDESTAL => BlankToBlackSetupPedestal,
            _ => unreachable!()
        }
    }
}

/// EDID analog video input basic information, defined in section 3.6.1.
#[derive(Debug, Copy, Clone)]
pub struct VideoInputAnalog {
    pub signal_level_std: VideoInputAnalogSignalLevelStandard,
    pub video_setup: VideoInputAnalogVideoSetup,
    pub sync_separate: bool,
    pub sync_composite: bool,
    pub sync_on_green: bool,
    pub sync_serrations: bool,
}

impl From<ffi::edid::di_edid_video_input_analog> for VideoInputAnalog {
    fn from(value: ffi::edid::di_edid_video_input_analog) -> Self {
        Self {
            signal_level_std: VideoInputAnalogSignalLevelStandard::from(value.signal_level_std),
            video_setup: VideoInputAnalogVideoSetup::from(value.video_setup),
            sync_separate: value.sync_separate,
            sync_composite: value.sync_composite,
            sync_on_green: value.sync_on_green,
            sync_serrations: value.sync_serrations,
        }
    }
}

/// Digital video input interface standard.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum VideoInputDigitalInterface {
    Undefined =
        ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_UNDEFINED,
    DVI = ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_DVI,
    HDMIA = ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_HDMI_A,
    HDMIB = ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_HDMI_B,
    MDDI = ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_MDDI,
    DisplayPort =
        ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_DISPLAYPORT,
}

impl From<ffi::edid::di_edid_video_input_digital_interface> for VideoInputDigitalInterface {
    fn from(value: ffi::edid::di_edid_video_input_digital_interface) -> Self {
        use VideoInputDigitalInterface::*;
        match value {
            ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_UNDEFINED => Undefined,
            ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_DVI => DVI,
            ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_HDMI_A => HDMIA,
            ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_HDMI_B => HDMIB,
            ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_MDDI => MDDI,
            ffi::edid::di_edid_video_input_digital_interface_DI_EDID_VIDEO_INPUT_DIGITAL_DISPLAYPORT => DisplayPort,
            _ => unreachable!()
        }
    }
}

/// EDID digital video input basic information, defined in section 3.6.1.
#[derive(Debug, Copy, Clone)]
pub struct VideoInputDigital {
    pub dfp1: bool,
    pub color_bit_depth: i32,
    pub interface: VideoInputDigitalInterface,
}

impl From<ffi::edid::di_edid_video_input_digital> for VideoInputDigital {
    fn from(value: ffi::edid::di_edid_video_input_digital) -> Self {
        Self {
            dfp1: value.dfp1,
            color_bit_depth: value.color_bit_depth,
            interface: VideoInputDigitalInterface::from(value.interface),
        }
    }
}

/// Screen size
#[derive(Debug, Copy, Clone)]
pub struct ScreenSize {
    pub width_cm: Option<i32>,
    pub height_cm: Option<i32>,
    pub landscape_aspect_ratio: Option<f32>,
    pub portait_aspect_ratio: Option<f32>,
}

impl From<ffi::edid::di_edid_screen_size> for ScreenSize {
    fn from(value: ffi::edid::di_edid_screen_size) -> Self {
        let width_cm = if value.width_cm != 0 {
            Some(value.width_cm)
        } else {
            None
        };

        let height_cm = if value.height_cm != 0 {
            Some(value.height_cm)
        } else {
            None
        };

        let landscape_aspect_ratio = if value.landscape_aspect_ratio != 0f32 {
            Some(value.landscape_aspect_ratio)
        } else {
            None
        };

        let portait_aspect_ratio = if value.portait_aspect_ratio != 0f32 {
            Some(value.portait_aspect_ratio)
        } else {
            None
        };
        Self {
            width_cm,
            height_cm,
            landscape_aspect_ratio,
            portait_aspect_ratio,
        }
    }
}

/// Supported legacy Display Power Management Signaling (DPMS) states, defined in
/// section 3.6.4.
///
/// Display Power Management (DPM) compliant displays only support \"off\"."]
#[derive(Debug, Copy, Clone)]
pub struct Dpms {
    pub standby: bool,
    pub suspend: bool,
    pub off: bool,
}

impl From<ffi::edid::di_edid_dpms> for Dpms {
    fn from(value: ffi::edid::di_edid_dpms) -> Self {
        Self {
            standby: value.standby,
            suspend: value.suspend,
            off: value.off,
        }
    }
}

/// Display color type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum DisplayColorType {
    Monochrome = ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_MONOCHROME,
    RGB = ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_RGB,
    NonRGB = ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_NON_RGB,
    Undefined = ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_UNDEFINED,
}

impl From<ffi::edid::di_edid_display_color_type> for DisplayColorType {
    fn from(value: ffi::edid::di_edid_display_color_type) -> Self {
        use DisplayColorType::*;
        match value {
            ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_MONOCHROME => Monochrome,
            ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_RGB => RGB,
            ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_NON_RGB => NonRGB,
            ffi::edid::di_edid_display_color_type_DI_EDID_DISPLAY_COLOR_UNDEFINED => Undefined,
            _ => unreachable!(),
        }
    }
}

/// Basic color encoding formats, defined in section 3.6.4."
#[derive(Debug, Copy, Clone)]
pub struct ColorEncodingFormats {
    pub rgb444: bool,
    pub ycrcb444: bool,
    pub ycrcb422: bool,
}

impl From<ffi::edid::di_edid_color_encoding_formats> for ColorEncodingFormats {
    fn from(value: ffi::edid::di_edid_color_encoding_formats) -> Self {
        Self {
            rgb444: value.rgb444,
            ycrcb444: value.ycrcb444,
            ycrcb422: value.ycrcb422,
        }
    }
}

/// Miscellaneous basic features, defined in section 3.6.4.
///
/// Note, the enum values don't match the specification.
#[derive(Debug, Copy, Clone)]
pub struct MiscFeatures {
    /// First detailed timing is the preferred timing.
    ///
    /// Always set for EDID 1.4 and later.
    pub has_preferred_timing: bool,
    /// GTF using the default parameters is supported.
    ///
    /// Never set for EDID 1.4 and later.
    pub default_gtf: bool,
    /// sRGB standard default color space is primary color space.
    pub srgb_is_primary: bool,
    /// Preferred timing mode includes native pixel format and rate.
    ///
    /// Never set for EDID 1.3 and earlier.
    pub preferred_timing_is_native: bool,
    /// GTF or CVT generated timings within the display's range limits are
    /// accepted.
    ///
    /// Never set for EDID 1.3 and earlier.
    pub continuous_freq: bool,
}

impl From<ffi::edid::di_edid_misc_features> for MiscFeatures {
    fn from(value: ffi::edid::di_edid_misc_features) -> Self {
        Self {
            has_preferred_timing: value.has_preferred_timing,
            default_gtf: value.default_gtf,
            srgb_is_primary: value.srgb_is_primary,
            preferred_timing_is_native: value.preferred_timing_is_native,
            continuous_freq: value.continuous_freq,
        }
    }
}

/// EDID display chromaticity coordinates, defined in section 3.7.
///
/// The values are accurate to the thousandth place. The red, green and blue
/// values are zero for monochrome displays.
#[derive(Debug, Copy, Clone)]
pub struct ChromaticityCoords {
    pub red_x: f32,
    pub red_y: f32,
    pub green_x: f32,
    pub green_y: f32,
    pub blue_x: f32,
    pub blue_y: f32,
    pub white_x: f32,
    pub white_y: f32,
}

impl From<ffi::edid::di_edid_chromaticity_coords> for ChromaticityCoords {
    fn from(value: ffi::edid::di_edid_chromaticity_coords) -> Self {
        Self {
            red_x: value.red_x,
            red_y: value.red_y,
            green_x: value.green_x,
            green_y: value.green_y,
            blue_x: value.blue_x,
            blue_y: value.blue_y,
            white_x: value.white_x,
            white_y: value.white_y,
        }
    }
}

/// EDID extension block tags, defined in section 2.2.4.
#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ExtensionTag {
    CEA = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_CEA,
    VTB = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_VTB,
    DI = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_DI,
    LS = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_LS,
    DPVL = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_DPVL,
    BlockMap = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_BLOCK_MAP,
    Vendor = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_VENDOR,
    DisplayId = ffi::edid::di_edid_ext_tag_DI_EDID_EXT_DISPLAYID,
    Unknown(u32),
}

impl From<u32> for ExtensionTag {
    fn from(value: u32) -> Self {
        match value {
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_CEA => Self::CEA,
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_VTB => Self::VTB,
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_DI => Self::DI,
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_LS => Self::LS,
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_DPVL => Self::DPVL,
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_BLOCK_MAP => Self::BlockMap,
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_VENDOR => Self::Vendor,
            ffi::edid::di_edid_ext_tag_DI_EDID_EXT_DISPLAYID => Self::DisplayId,
            _ => Self::Unknown(value),
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Extension(*const ffi::edid::di_edid_ext);

impl Extension {
    /// Get the tag of an EDID extension block.
    pub fn tag(&self) -> ExtensionTag {
        unsafe { ffi::edid::di_edid_ext_get_tag(self.0) }.into()
    }

    pub(crate) fn as_ptr(&self) -> *const ffi::edid::di_edid_ext {
        self.0
    }
}
