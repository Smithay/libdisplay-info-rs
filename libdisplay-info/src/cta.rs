//! Low-level API for Consumer Technology Association standards.
//!
//! The library implements CTA-861-H, available at:
//! <https://shop.cta.tech/collections/standards/products/a-dtv-profile-for-uncompressed-high-speed-digital-interfaces-cta-861-h>
use std::marker::PhantomData;

use libdisplay_info_derive::FFIFrom;

use crate::{edid::ExtensionRef, ffi, FFIIter};

/// EDID CTA-861 extension block.
#[derive(Debug)]
pub struct CTA<'ext> {
    cta: *const ffi::cta::di_edid_cta,
    phantom: PhantomData<&'ext ()>,
}

impl<'ext> CTA<'ext> {
    /// Get a CTA-861 extension block.
    ///
    /// Returns `None` if the extension block tag is not [CEA](crate::edid::ExtensionTag::CEA).
    pub fn from_extension(extensions: &'ext ExtensionRef) -> Option<CTA<'ext>> {
        let cta = unsafe { ffi::edid::di_edid_ext_get_cta(extensions.as_ptr()) };

        if cta.is_null() {
            None
        } else {
            Some(Self {
                cta: cta as *const ffi::cta::di_edid_cta,
                phantom: PhantomData,
            })
        }
    }

    /// Get the CTA extension revision (also referred to as `version`` by the
    /// specification).
    pub fn revision(&self) -> i32 {
        unsafe { ffi::cta::di_edid_cta_get_revision(self.cta) }
    }

    /// Get miscellaneous CTA flags.
    pub fn flags(&self) -> Flags {
        let flags = unsafe { ffi::cta::di_edid_cta_get_flags(self.cta) };
        Flags::from(unsafe { *flags })
    }

    /// Get CTA data blocks.
    pub fn data_blocks(&self) -> &[DataBlockRef] {
        let data_blocks = unsafe { ffi::cta::di_edid_cta_get_data_blocks(self.cta) };

        let mut len = 0;
        while !unsafe { *data_blocks.offset(len) }.is_null() {
            len += 1;
        }

        unsafe { std::slice::from_raw_parts(data_blocks as *const DataBlockRef, len as usize) }
    }

    /// Get a list of EDID detailed timing definitions.
    pub fn detailed_timing_defs(&self) -> impl Iterator<Item = crate::edid::DetailedTimingDef> {
        FFIIter::new(unsafe {
            ffi::cta::di_edid_cta_get_detailed_timing_defs(self.cta)
                as *const *const ffi::edid::di_edid_detailed_timing_def
        })
    }
}

/// CTA video format picture aspect ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_video_format_picture_aspect_ratio)]
#[repr(u32)]
pub enum VideoFormatPictureAspectRatio {
    _4_3 = ffi::cta::di_cta_video_format_picture_aspect_ratio_DI_CTA_VIDEO_FORMAT_PICTURE_ASPECT_RATIO_4_3,
    _16_9 = ffi::cta::di_cta_video_format_picture_aspect_ratio_DI_CTA_VIDEO_FORMAT_PICTURE_ASPECT_RATIO_16_9,
    _64_27 = ffi::cta::di_cta_video_format_picture_aspect_ratio_DI_CTA_VIDEO_FORMAT_PICTURE_ASPECT_RATIO_64_27,
    _256_135 = ffi::cta::di_cta_video_format_picture_aspect_ratio_DI_CTA_VIDEO_FORMAT_PICTURE_ASPECT_RATIO_256_135,
}

/// CTA video format sync pulse polarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_video_format_sync_polarity)]
#[repr(u32)]
pub enum VideoFormatSyncPolarity {
    Negative = ffi::cta::di_cta_video_format_sync_polarity_DI_CTA_VIDEO_FORMAT_SYNC_NEGATIVE,
    Positive = ffi::cta::di_cta_video_format_sync_polarity_DI_CTA_VIDEO_FORMAT_SYNC_POSITIVE,
}

/// A CTA-861 video format, defined in section 4.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_video_format)]
pub struct VideoFormat {
    pub vic: u8,
    pub h_active: i32,
    pub v_active: i32,
    pub h_front: i32,
    pub v_front: i32,
    pub h_sync: i32,
    pub v_sync: i32,
    pub h_back: i32,
    pub v_back: i32,
    pub h_sync_polarity: VideoFormatSyncPolarity,
    pub v_sync_polarity: VideoFormatSyncPolarity,
    pub pixel_clock_hz: i64,
    pub interlaced: bool,
    pub picture_aspect_ratio: VideoFormatPictureAspectRatio,
}

impl VideoFormat {
    /// Get a CTA-861 video format from a VIC.
    ///
    /// Returns `None` if the VIC is unknown.
    pub fn from_vic(vic: u8) -> Option<VideoFormat> {
        let video_format = unsafe { ffi::cta::di_cta_video_format_from_vic(vic) };

        if video_format.is_null() {
            None
        } else {
            Some(VideoFormat::from(unsafe { *video_format }))
        }
    }
}

/// Miscellaneous EDID CTA flags, defined in section 7.3.3.
///
/// For CTA revision 1, all of the fields are zero.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_edid_cta_flags)]
pub struct Flags {
    pub it_underscan: bool,
    pub basic_audio: bool,
    pub ycc444: bool,
    pub ycc422: bool,
    pub native_dtds: i32,
}

/// CTA data block, defined in section 7.4.
#[repr(transparent)]
pub struct DataBlockRef(*const ffi::cta::di_cta_data_block);

impl DataBlockRef {
    /// Get the tag of the CTA data block.
    pub fn tag(&self) -> DataBlockTag {
        DataBlockTag::from(unsafe { ffi::cta::di_cta_data_block_get_tag(self.0) })
    }

    /// Get an array of short audio descriptors from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_AUDIO.
    pub fn sads(&self) -> impl Iterator<Item = Sad> {
        FFIIter::new(unsafe { ffi::cta::di_cta_data_block_get_sads(self.0) })
    }

    /// Get the speaker allocation from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_SPEAKER_ALLOC.
    pub fn speaker_alloc(&self) -> Option<SpeakerAllocBlock> {
        SpeakerAllocBlock::from_ptr(unsafe {
            ffi::cta::di_cta_data_block_get_speaker_alloc(self.0)
        })
    }

    /// Get the video capabilities from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_VIDEO_CAP.
    pub fn video_cap(&self) -> Option<VideoCapBlock> {
        VideoCapBlock::from_ptr(unsafe { ffi::cta::di_cta_data_block_get_video_cap(self.0) })
    }

    /// Get the VESA Display Device Data Block (DDDB) from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not
    /// DI_CTA_DATA_BLOCK_VESA_DISPLAY_DEVICE.
    pub fn vesa_dddb(&self) -> Option<VesaDddb> {
        VesaDddb::from_ptr(unsafe { ffi::cta::di_cta_data_block_get_vesa_dddb(self.0) })
    }

    /// Get the colorimetry data from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_COLORIMETRY.
    pub fn colorimetry(&self) -> Option<ColorimetryBlock> {
        ColorimetryBlock::from_ptr(unsafe { ffi::cta::di_cta_data_block_get_colorimetry(self.0) })
    }

    /// Get the HDR static metadata from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not
    /// DI_CTA_DATA_BLOCK_HDR_STATIC_METADATA.
    pub fn hdr_static_metadata(&self) -> Option<HdrStaticMetadataBlock> {
        HdrStaticMetadataBlock::from_ptr(unsafe {
            ffi::cta::di_cta_data_block_get_hdr_static_metadata(self.0)
        })
    }

    /// Get the HDR dynamic metadata from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not
    /// DI_CTA_DATA_BLOCK_HDR_DYNAMIC_METADATA.
    pub fn hdr_dynamic_metadata(&self) -> Option<HdrDynamicMetadataBlock> {
        HdrDynamicMetadataBlock::from_ptr(unsafe {
            ffi::cta::di_cta_data_block_get_hdr_dynamic_metadata(self.0)
        })
    }

    /// Get an array of short video descriptors from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_VIDEO.
    pub fn svds(&self) -> impl Iterator<Item = Svd> {
        FFIIter::new(unsafe { ffi::cta::di_cta_data_block_get_svds(self.0) })
    }

    /// Get an array of short video descriptors which only allow YCbCr 4:2:0 sampling
    /// mode from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_YCBCR420.
    pub fn ycbcr420_svds(&self) -> impl Iterator<Item = Svd> {
        FFIIter::new(unsafe { ffi::cta::di_cta_data_block_get_ycbcr420_svds(self.0) })
    }

    /// Get the Display Transfer Characteristic from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not
    /// DI_CTA_DATA_BLOCK_VESA_DISPLAY_TRANSFER_CHARACTERISTIC.
    ///
    /// Upstream is not aware of any EDID blob containing a Display Transfer
    /// Characteristic data block.
    /// If such a blob is found, please share it with upstream!
    pub fn vesa_transfer_characteristics(&self) -> Option<VesaTransferCharacteristics> {
        VesaTransferCharacteristics::from_ptr(unsafe {
            ffi::cta::di_cta_data_block_get_vesa_transfer_characteristics(self.0)
        })
    }

    /// Get the YCbCr 4:2:0 Capability Map from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_YCBCR420_CAP_MAP.
    pub fn ycbcr420_cap_map(&self) -> Option<Ycbcr420CapMapRef> {
        Ycbcr420CapMapRef::from_ptr(unsafe {
            ffi::cta::di_cta_data_block_get_ycbcr420_cap_map(self.0)
        })
    }

    /// Get the InfoFrame information from a CTA data block.
    ///
    /// Returns `None` if the data block tag is not DI_CTA_DATA_BLOCK_INFOFRAME.
    pub fn infoframe(&self) -> Option<InfoframeBlockRef> {
        InfoframeBlockRef::from_ptr(unsafe { ffi::cta::di_cta_data_block_get_infoframe(self.0) })
    }
}

/// CTA data block tag.
///
/// Note, the enum values don't match the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_data_block_tag)]
#[repr(u32)]
pub enum DataBlockTag {
    Audio = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_AUDIO,
    Video = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_VIDEO,
    SpeakerAlloc = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_SPEAKER_ALLOC,
    VesaDisplayTransferCharacteristic =
        ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_VESA_DISPLAY_TRANSFER_CHARACTERISTIC,
    VideoFormat = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_VIDEO_FORMAT,
    VideoCap = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_VIDEO_CAP,
    VesaDisplayDevice = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_VESA_DISPLAY_DEVICE,
    Colorimetry = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_COLORIMETRY,
    HdrStaticMetadata = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_HDR_STATIC_METADATA,
    HdrDynamicMetadata = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_HDR_DYNAMIC_METADATA,
    NativeVideoResolution =
        ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_NATIVE_VIDEO_RESOLUTION,
    VideoFormatPref = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_VIDEO_FORMAT_PREF,
    Ycbcr420 = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_YCBCR420,
    Ycbcr420CapMap = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_YCBCR420_CAP_MAP,
    HdmiAudio = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_HDMI_AUDIO,
    RoomConfig = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_ROOM_CONFIG,
    SpeakerLocation = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_SPEAKER_LOCATION,
    Infoframe = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_INFOFRAME,
    DisplayidVideoTimingVii =
        ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_DISPLAYID_VIDEO_TIMING_VII,
    DisplayidVideoTimingViii =
        ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_DISPLAYID_VIDEO_TIMING_VIII,
    DisplayidVideoTimingX =
        ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_DISPLAYID_VIDEO_TIMING_X,
    HdmiEdidExtOverride = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_HDMI_EDID_EXT_OVERRIDE,
    HdmiSinkCap = ffi::cta::di_cta_data_block_tag_DI_CTA_DATA_BLOCK_HDMI_SINK_CAP,
}

/// Audio formats, defined in tables 37 and 39.
///
/// Note, the enum values don't match the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_audio_format)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum AudioFormat {
    LPCM = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_LPCM,
    AC3 = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_AC3,
    MPEG1 = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEG1,
    MP3 = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MP3,
    MPEG2 = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEG2,
    AAC_LC = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_AAC_LC,
    DTS = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_DTS,
    ATRAC = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_ATRAC,
    ONE_BIT_AUDIO = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_ONE_BIT_AUDIO,
    ENHANCED_AC3 = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_ENHANCED_AC3,
    DTS_HD = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_DTS_HD,
    MAT = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MAT,
    DST = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_DST,
    WMA_PRO = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_WMA_PRO,
    MPEG4_HE_AAC = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEG4_HE_AAC,
    MPEG4_HE_AAC_V2 = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEG4_HE_AAC_V2,
    MPEG4_AAC_LC = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEG4_AAC_LC,
    DRA = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_DRA,
    MPEG4_HE_AAC_MPEG_SURROUND =
        ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEG4_HE_AAC_MPEG_SURROUND,
    MPEG4_AAC_LC_MPEG_SURROUND =
        ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEG4_AAC_LC_MPEG_SURROUND,
    MPEGH_3D = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_MPEGH_3D,
    AC4 = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_AC4,
    LPCM_3D = ffi::cta::di_cta_audio_format_DI_CTA_AUDIO_FORMAT_LPCM_3D,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_sample_rates)]
pub struct SadSampleRates {
    pub has_192_khz: bool,
    pub has_176_4_khz: bool,
    pub has_96_khz: bool,
    pub has_88_2_khz: bool,
    pub has_48_khz: bool,
    pub has_44_1_khz: bool,
    pub has_32_khz: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_mpegh_3d_level)]
#[repr(u32)]
pub enum SadMpegh3dLevel {
    Unspecified = ffi::cta::di_cta_sad_mpegh_3d_level_DI_CTA_SAD_MPEGH_3D_LEVEL_UNSPECIFIED,
    _1 = ffi::cta::di_cta_sad_mpegh_3d_level_DI_CTA_SAD_MPEGH_3D_LEVEL_1,
    _2 = ffi::cta::di_cta_sad_mpegh_3d_level_DI_CTA_SAD_MPEGH_3D_LEVEL_2,
    _3 = ffi::cta::di_cta_sad_mpegh_3d_level_DI_CTA_SAD_MPEGH_3D_LEVEL_3,
    _4 = ffi::cta::di_cta_sad_mpegh_3d_level_DI_CTA_SAD_MPEGH_3D_LEVEL_4,
    _5 = ffi::cta::di_cta_sad_mpegh_3d_level_DI_CTA_SAD_MPEGH_3D_LEVEL_5,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_mpegh_3d)]
pub struct SadMpegh3d {
    pub level: SadMpegh3dLevel,
    pub low_complexity_profile: bool,
    pub baseline_profile: bool,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_mpeg_aac)]
pub struct SadMpegAac {
    pub has_frame_length_960: bool,
    pub has_frame_length_1024: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_mpeg_surround_signaling)]
#[repr(u32)]
pub enum SadMpegSurroundSignaling {
    Implicit =
        ffi::cta::di_cta_sad_mpeg_surround_signaling_DI_CTA_SAD_MPEG_SURROUND_SIGNALING_IMPLICIT,
    ImplicitAndExplicit = ffi::cta::di_cta_sad_mpeg_surround_signaling_DI_CTA_SAD_MPEG_SURROUND_SIGNALING_IMPLICIT_AND_EXPLICIT,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_mpeg_surround)]
pub struct SadMpegSurround {
    pub signaling: SadMpegSurroundSignaling,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_mpeg_aac_le)]
pub struct SadMpegAacLe {
    pub supports_multichannel_sound: bool,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_lpcm)]
pub struct SadLpcm {
    pub has_sample_size_24_bits: bool,
    pub has_sample_size_20_bits: bool,
    pub has_sample_size_16_bits: bool,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_enhanced_ac3)]
#[allow(non_snake_case)]
pub struct SadEnhancedAc3 {
    pub supports_joint_object_coding: bool,
    pub supports_joint_object_coding_ACMOD28: bool,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_mat)]
pub struct SadMat {
    pub supports_object_audio_and_channel_based: bool,
    pub requires_hash_calculation: bool,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad_wma_pro)]
pub struct SadWmaPro {
    pub profile: ::std::os::raw::c_int,
}

/// A CTA short audio descriptor (SAD), defined in section 7.5.2.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_sad)]
pub struct Sad {
    pub format: AudioFormat,
    #[optional(0i32)]
    pub max_channels: Option<i32>,
    #[ptr_deref]
    pub supported_sample_rates: Option<SadSampleRates>,
    #[optional(0i32)]
    pub max_bitrate_kbs: Option<i32>,
    #[ptr_deref]
    pub lpcm: Option<SadLpcm>,
    #[ptr_deref]
    pub mpegh_3d: Option<SadMpegh3d>,
    #[ptr_deref]
    pub mpeg_aac: Option<SadMpegAac>,
    #[ptr_deref]
    pub mpeg_surround: Option<SadMpegSurround>,
    #[ptr_deref]
    pub mpeg_aac_le: Option<SadMpegAacLe>,
    #[ptr_deref]
    pub enhanced_ac3: Option<SadEnhancedAc3>,
    #[ptr_deref]
    pub mat: Option<SadMat>,
    #[ptr_deref]
    pub wma_pro: Option<SadWmaPro>,
}

/// Speaker allocation data block (SADB), defined in section 7.5.3.
///
/// This block indicates which speakers are present. See figure 6 for the meaning
/// of the fields.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_speaker_alloc_block)]
pub struct SpeakerAllocBlock {
    pub flw_frw: bool,
    pub flc_frc: bool,
    pub bc: bool,
    pub bl_br: bool,
    pub fc: bool,
    pub lfe1: bool,
    pub fl_fr: bool,
    pub tpsil_tpsir: bool,
    pub sil_sir: bool,
    pub tpbc: bool,
    pub lfe2: bool,
    pub ls_rs: bool,
    pub tpfc: bool,
    pub tpc: bool,
    pub tpfl_tpfr: bool,
    pub btfl_btfr: bool,
    pub btfc: bool,
    pub tpbl_tpbr: bool,
}

/// Over- and underscan capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_video_cap_over_underscan)]
#[repr(u32)]
pub enum VideoCapOverUnderscan {
    UnknownOverUnderscan =
        ffi::cta::di_cta_video_cap_over_underscan_DI_CTA_VIDEO_CAP_UNKNOWN_OVER_UNDERSCAN,
    AlwaysOverscan = ffi::cta::di_cta_video_cap_over_underscan_DI_CTA_VIDEO_CAP_ALWAYS_OVERSCAN,
    AlwaysUnderscan = ffi::cta::di_cta_video_cap_over_underscan_DI_CTA_VIDEO_CAP_ALWAYS_UNDERSCAN,
    BothOverUnderscan =
        ffi::cta::di_cta_video_cap_over_underscan_DI_CTA_VIDEO_CAP_BOTH_OVER_UNDERSCAN,
}

/// Video capability data block (VCDB), defined in section 7.5.6.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_video_cap_block)]
pub struct VideoCapBlock {
    pub selectable_ycc_quantization_range: bool,
    pub selectable_rgb_quantization_range: bool,
    pub pt_over_underscan: VideoCapOverUnderscan,
    pub it_over_underscan: VideoCapOverUnderscan,
    pub ce_over_underscan: VideoCapOverUnderscan,
}

/// Interface types, defined in VESA DDDB section 2.3.1 and 2.3.2.
///
/// Note, the enum values don't match the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_interface_type)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VesaDddbInterfaceType {
    VGA = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_VGA,
    NAVI_V = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_NAVI_V,
    NAVI_D = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_NAVI_D,
    LVDS = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_LVDS,
    RSDS = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_RSDS,
    DVI_D = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_DVI_D,
    DVI_I_ANALOG =
        ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_DVI_I_ANALOG,
    DVI_I_DIGITAL =
        ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_DVI_I_DIGITAL,
    HDMI_A = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_HDMI_A,
    HDMI_B = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_HDMI_B,
    MDDI = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_MDDI,
    DISPLAYPORT = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_DISPLAYPORT,
    IEEE_1394 = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_IEEE_1394,
    M1_ANALOG = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_M1_ANALOG,
    M1_DIGITAL = ffi::cta::di_cta_vesa_dddb_interface_type_DI_CTA_VESA_DDDB_INTERFACE_M1_DIGITAL,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_content_protection)]
#[repr(u32)]
pub enum VesaDddbContentProtection {
    None = ffi::cta::di_cta_vesa_dddb_content_protection_DI_CTA_VESA_DDDB_CONTENT_PROTECTION_NONE,
    HDCP = ffi::cta::di_cta_vesa_dddb_content_protection_DI_CTA_VESA_DDDB_CONTENT_PROTECTION_HDCP,
    DTCP = ffi::cta::di_cta_vesa_dddb_content_protection_DI_CTA_VESA_DDDB_CONTENT_PROTECTION_DTCP,
    DPCP = ffi::cta::di_cta_vesa_dddb_content_protection_DI_CTA_VESA_DDDB_CONTENT_PROTECTION_DPCP,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_default_orientation)]
#[repr(u32)]
pub enum VesaDddbDefaultOrientation {
    Landscape = ffi::cta::di_cta_vesa_dddb_default_orientation_DI_CTA_VESA_DDDB_DEFAULT_ORIENTATION_LANDSCAPE,
    Portrait =
        ffi::cta::di_cta_vesa_dddb_default_orientation_DI_CTA_VESA_DDDB_DEFAULT_ORIENTATION_PORTAIT,
    Unfixed =
        ffi::cta::di_cta_vesa_dddb_default_orientation_DI_CTA_VESA_DDDB_DEFAULT_ORIENTATION_UNFIXED,
    Undefined = ffi::cta::di_cta_vesa_dddb_default_orientation_DI_CTA_VESA_DDDB_DEFAULT_ORIENTATION_UNDEFINED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_rotation_cap)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VesaDddbRotationCap {
    None = ffi::cta::di_cta_vesa_dddb_rotation_cap_DI_CTA_VESA_DDDB_ROTATION_CAP_NONE,
    _90DEG_CLOCKWISE =
        ffi::cta::di_cta_vesa_dddb_rotation_cap_DI_CTA_VESA_DDDB_ROTATION_CAP_90DEG_CLOCKWISE,
    _90DEG_COUNTERCLOCKWISE = ffi::cta::di_cta_vesa_dddb_rotation_cap_DI_CTA_VESA_DDDB_ROTATION_CAP_90DEG_COUNTERCLOCKWISE,
    _90DEG_EITHER =
        ffi::cta::di_cta_vesa_dddb_rotation_cap_DI_CTA_VESA_DDDB_ROTATION_CAP_90DEG_EITHER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_zero_pixel_location)]
#[repr(u32)]
pub enum VesaDddbZeroPixelLocation {
    UpperLeft =
        ffi::cta::di_cta_vesa_dddb_zero_pixel_location_DI_CTA_VESA_DDDB_ZERO_PIXEL_UPPER_LEFT,
    UpperRight =
        ffi::cta::di_cta_vesa_dddb_zero_pixel_location_DI_CTA_VESA_DDDB_ZERO_PIXEL_UPPER_RIGHT,
    LowerLeft =
        ffi::cta::di_cta_vesa_dddb_zero_pixel_location_DI_CTA_VESA_DDDB_ZERO_PIXEL_LOWER_LEFT,
    LowerRight =
        ffi::cta::di_cta_vesa_dddb_zero_pixel_location_DI_CTA_VESA_DDDB_ZERO_PIXEL_LOWER_RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_scan_direction)]
#[repr(u32)]
pub enum VesaDddbScanDirection {
    Undefined = ffi::cta::di_cta_vesa_dddb_scan_direction_DI_CTA_VESA_DDDB_SCAN_DIRECTION_UNDEFINED,
    FastLongSlowShort = ffi::cta::di_cta_vesa_dddb_scan_direction_DI_CTA_VESA_DDDB_SCAN_DIRECTION_FAST_LONG_SLOW_SHORT,
    FastShortSlowLong = ffi::cta::di_cta_vesa_dddb_scan_direction_DI_CTA_VESA_DDDB_SCAN_DIRECTION_FAST_SHORT_SLOW_LONG,
}

/// Subpixel layout, defined in VESA DDDB section 2.9.
///
/// For layouts with more than 3 subpixels, the color coordinates of the
/// additional subpixels are defined in the additional primary chromaticities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_subpixel_layout)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum VesaDddbSubpixelLayout {
    Undefined = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_UNDEFINED,
    RGB_VERT = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_RGB_VERT,
    RGB_HORIZ = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_RGB_HORIZ,
    EDID_CHROM_VERT =
        ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_EDID_CHROM_VERT,
    EDID_CHROM_HORIZ =
        ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_EDID_CHROM_HORIZ,
    QUAD_RGGB = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_QUAD_RGGB,
    QUAD_GBRG = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_QUAD_GBRG,
    DELTA_RGB = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_DELTA_RGB,
    MOSAIC = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_MOSAIC,
    QUAD_ANY = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_QUAD_ANY,
    FIVE = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_FIVE,
    SIX = ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_SIX,
    CLAIRVOYANTE_PENTILE =
        ffi::cta::di_cta_vesa_dddb_subpixel_layout_DI_CTA_VESA_DDDB_SUBPIXEL_CLAIRVOYANTE_PENTILE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_dithering_type)]
#[repr(u32)]
pub enum VesaDddbDitheringType {
    None = ffi::cta::di_cta_vesa_dddb_dithering_type_DI_CTA_VESA_DDDB_DITHERING_NONE,
    Spacial = ffi::cta::di_cta_vesa_dddb_dithering_type_DI_CTA_VESA_DDDB_DITHERING_SPACIAL,
    Temporal = ffi::cta::di_cta_vesa_dddb_dithering_type_DI_CTA_VESA_DDDB_DITHERING_TEMPORAL,
    SpatialAndTemporal =
        ffi::cta::di_cta_vesa_dddb_dithering_type_DI_CTA_VESA_DDDB_DITHERING_SPATIAL_AND_TEMPORAL,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_additional_primary_chromaticity)]
pub struct VesaDddbAdditionalPrimaryChromaticity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_frame_rate_conversion)]
#[repr(u32)]
pub enum VesaDddbFrameRateConversion {
    None = ffi::cta::di_cta_vesa_dddb_frame_rate_conversion_DI_CTA_VESA_DDDB_FRAME_RATE_CONVERSION_NONE,
    SingleBuffering = ffi::cta::di_cta_vesa_dddb_frame_rate_conversion_DI_CTA_VESA_DDDB_FRAME_RATE_CONVERSION_SINGLE_BUFFERING,
    DoubleBuffering = ffi::cta::di_cta_vesa_dddb_frame_rate_conversion_DI_CTA_VESA_DDDB_FRAME_RATE_CONVERSION_DOUBLE_BUFFERING,
    Advanced = ffi::cta::di_cta_vesa_dddb_frame_rate_conversion_DI_CTA_VESA_DDDB_FRAME_RATE_CONVERSION_ADVANCED,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb_resp_time_transition)]
#[repr(u32)]
pub enum VesaDddbRespTimeTransition {
    BlackToWhite =
        ffi::cta::di_cta_vesa_dddb_resp_time_transition_DI_CTA_VESA_DDDB_RESP_TIME_BLACK_TO_WHITE,
    WhiteToBlack =
        ffi::cta::di_cta_vesa_dddb_resp_time_transition_DI_CTA_VESA_DDDB_RESP_TIME_WHITE_TO_BLACK,
}

/// VESA Display Device Data Block (DDDB), defined in VESA Display Device Data
/// Block (DDDB) Standard version 1.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_dddb)]
pub struct VesaDddb {
    pub interface_type: VesaDddbInterfaceType,
    #[optional(0i32)]
    pub num_channels: Option<i32>,
    pub interface_version: i32,
    pub interface_release: i32,
    pub content_protection: VesaDddbContentProtection,
    pub min_clock_freq_mhz: i32,
    pub max_clock_freq_mhz: i32,
    pub native_horiz_pixels: i32,
    pub native_vert_pixels: i32,
    pub aspect_ratio: f32,
    pub default_orientation: VesaDddbDefaultOrientation,
    pub rotation_cap: VesaDddbRotationCap,
    pub zero_pixel_location: VesaDddbZeroPixelLocation,
    pub scan_direction: VesaDddbScanDirection,
    pub subpixel_layout: VesaDddbSubpixelLayout,
    pub horiz_pitch_mm: f32,
    pub vert_pitch_mm: f32,
    pub dithering_type: VesaDddbDitheringType,
    pub direct_drive: bool,
    pub overdrive_not_recommended: bool,
    pub deinterlacing: bool,
    pub audio_support: bool,
    pub separate_audio_inputs: bool,
    pub audio_input_override: bool,
    pub audio_delay_provided: bool,
    pub audio_delay_ms: i32,
    pub frame_rate_conversion: VesaDddbFrameRateConversion,
    #[optional(0i32)]
    pub frame_rate_range_hz: Option<i32>,
    pub frame_rate_native_hz: i32,
    pub bit_depth_interface: i32,
    pub bit_depth_display: i32,
    pub additional_primary_chromaticities_len: usize,
    pub additional_primary_chromaticities: [VesaDddbAdditionalPrimaryChromaticity; 3usize],
    pub resp_time_transition: VesaDddbRespTimeTransition,
    pub resp_time_ms: i32,
    pub overscan_horiz_pct: i32,
    pub overscan_vert_pct: i32,
}

/// CTA colorimetry data block, defined in section 7.5.5.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_colorimetry_block)]
pub struct ColorimetryBlock {
    pub xvycc_601: bool,
    pub xvycc_709: bool,
    pub sycc_601: bool,
    pub opycc_601: bool,
    pub oprgb: bool,
    pub bt2020_cycc: bool,
    pub bt2020_ycc: bool,
    pub bt2020_rgb: bool,
    pub st2113_rgb: bool,
    pub ictcp: bool,
}

/// Supported Electro-Optical Transfer Functions for a CTA HDR static metadata
/// block.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_static_metadata_block_eotfs)]
pub struct HdrStaticMetadataBlockEotfs {
    pub traditional_sdr: bool,
    pub traditional_hdr: bool,
    pub pq: bool,
    pub hlg: bool,
}

/// Supported static metadata descriptors for a CTA HDR static metadata block.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_static_metadata_block_descriptors)]
pub struct HdrStaticMetadataBlockDescriptors {
    pub type1: bool,
}

/// CTA HDR static metadata block, defined in section 7.5.13.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_static_metadata_block)]
pub struct HdrStaticMetadataBlock {
    #[optional(0f32)]
    pub desired_content_max_luminance: Option<f32>,
    #[optional(0f32)]
    pub desired_content_max_frame_avg_luminance: Option<f32>,
    #[optional(0f32)]
    pub desired_content_min_luminance: Option<f32>,
    #[ptr_deref]
    pub eotfs: Option<HdrStaticMetadataBlockEotfs>,
    #[ptr_deref]
    pub descriptors: Option<HdrStaticMetadataBlockDescriptors>,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_dynamic_metadata_block_type1)]
pub struct HdrDynamicMetadataBlockType1 {
    pub type_1_hdr_metadata_version: u8,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_dynamic_metadata_block_type2)]
pub struct HdrDynamicMetadataBlockType2 {
    pub ts_103_433_spec_version: u8,
    pub ts_103_433_1_capable: bool,
    pub ts_103_433_2_capable: bool,
    pub ts_103_433_3_capable: bool,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_dynamic_metadata_block_type3)]
pub struct HdrDynamicMetadataBlockType3 {}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_dynamic_metadata_block_type4)]
pub struct HdrDynamicMetadataBlockType4 {
    pub type_4_hdr_metadata_version: u8,
}

#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_dynamic_metadata_block_type256)]
pub struct HdrDynamicMetadataBlockType256 {
    pub graphics_overlay_flag_version: u8,
}

#[doc = " CTA HDR dynamic metadata block, defined in section 7.5.14."]
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_hdr_dynamic_metadata_block)]
pub struct HdrDynamicMetadataBlock {
    #[ptr_deref]
    pub type1: Option<HdrDynamicMetadataBlockType1>,
    #[ptr_deref]
    pub type2: Option<HdrDynamicMetadataBlockType2>,
    #[ptr_deref]
    pub type3: Option<HdrDynamicMetadataBlockType3>,
    #[ptr_deref]
    pub type4: Option<HdrDynamicMetadataBlockType4>,
    #[ptr_deref]
    pub type256: Option<HdrDynamicMetadataBlockType256>,
}

/// A Short Video Descriptor (SVD).
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_svd)]
pub struct Svd {
    pub vic: u8,
    pub native: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_transfer_characteristics_usage)]
#[repr(u32)]
pub enum VesaTransferCharacteristicsUsage {
    White = ffi::cta::di_cta_vesa_transfer_characteristics_usage_DI_CTA_VESA_TRANSFER_CHARACTERISTIC_USAGE_WHITE,
    Red = ffi::cta::di_cta_vesa_transfer_characteristics_usage_DI_CTA_VESA_TRANSFER_CHARACTERISTIC_USAGE_RED,
    Green = ffi::cta::di_cta_vesa_transfer_characteristics_usage_DI_CTA_VESA_TRANSFER_CHARACTERISTIC_USAGE_GREEN,
    Blue = ffi::cta::di_cta_vesa_transfer_characteristics_usage_DI_CTA_VESA_TRANSFER_CHARACTERISTIC_USAGE_BLUE,
}

/// VESA Display Transfer Characteristic Data Block, defined in VESA Display
/// Transfer Characteristics Data Block Standard Version 1.0
///
/// Contains 8, 16 or 32 evenly distributed points on the input axis describing
/// the normalized relative luminance at that input. The first value includes the
/// relative black level luminance.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_vesa_transfer_characteristics)]
pub struct VesaTransferCharacteristics {
    pub usage: VesaTransferCharacteristicsUsage,
    pub points_len: u8,
    pub points: [f32; 32usize],
}

/// CTA YCbCr 4:2:0 Capability Map block, defined in section 7.5.11.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_ycbcr420_cap_map)]
#[wrap]
pub struct Ycbcr420CapMap {}

impl Ycbcr420CapMapRef {
    /// Returns true if the SVD in regular Video Data Blocks at index `svd_index`
    /// supports YCbCr 4:2:0 subsampling.
    pub fn di_cta_ycbcr420_cap_map_supported(&self, svd_index: usize) -> bool {
        unsafe { ffi::cta::di_cta_ycbcr420_cap_map_supported(self.0, svd_index) }
    }
}

/// InfoFrame types, defined in table 7.
///
/// Note, the enum values don't match the specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, FFIFrom)]
#[ffi(ffi::cta::di_cta_infoframe_type)]
#[repr(u32)]
pub enum InfoframeType {
    AuxiliaryVideoInformation =
        ffi::cta::di_cta_infoframe_type_DI_CTA_INFOFRAME_TYPE_AUXILIARY_VIDEO_INFORMATION,
    SourceProductDescription =
        ffi::cta::di_cta_infoframe_type_DI_CTA_INFOFRAME_TYPE_SOURCE_PRODUCT_DESCRIPTION,
    Audio = ffi::cta::di_cta_infoframe_type_DI_CTA_INFOFRAME_TYPE_AUDIO,
    MpegSource = ffi::cta::di_cta_infoframe_type_DI_CTA_INFOFRAME_TYPE_MPEG_SOURCE,
    NtscVbi = ffi::cta::di_cta_infoframe_type_DI_CTA_INFOFRAME_TYPE_NTSC_VBI,
    DynamicRangeAndMastering =
        ffi::cta::di_cta_infoframe_type_DI_CTA_INFOFRAME_TYPE_DYNAMIC_RANGE_AND_MASTERING,
}

/// CTA InfoFrame descriptor, defined in section 7.5.9.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_infoframe_descriptor)]
pub struct InfoframeDescriptor {
    pub type_: InfoframeType,
}

/// CTA InfoFrame processing, defined in section 7.5.9.
#[derive(Debug, Copy, Clone, FFIFrom)]
#[ffi(ffi::cta::di_cta_infoframe_block)]
#[wrap]
pub struct InfoframeBlock {
    pub num_simultaneous_vsifs: i32,
}

impl InfoframeBlockRef {
    pub fn infoframes(&self) -> impl Iterator<Item = InfoframeDescriptor> {
        FFIIter::new(unsafe { (*self.0).infoframes })
    }
}
