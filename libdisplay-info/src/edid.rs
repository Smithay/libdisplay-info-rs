use std::marker::PhantomData;

use crate::ffi;

/// EDID data structure.
#[derive(Debug)]
pub struct Edid<'info> {
    edid: *const ffi::edid::di_edid,
    phantom: PhantomData<&'info ()>,
}

impl<'info> Edid<'info> {
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
