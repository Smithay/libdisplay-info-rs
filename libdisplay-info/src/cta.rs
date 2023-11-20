use std::marker::PhantomData;

use crate::{edid::Extension, ffi};

#[derive(Debug)]
pub struct CTA<'ext> {
    cta: *const ffi::cta::di_edid_cta,
    phantom: PhantomData<&'ext ()>,
}

impl<'ext> CTA<'ext> {
    /// Get a CTA-861 extension block.
    ///
    /// Returns `None` if the extension block tag is not [CEA](crate::edid::ExtensionTag::CEA).
    pub fn from_extension(extensions: &'ext Extension) -> Option<CTA<'ext>> {
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

    /// Get miscellaneous CTA flags.
    pub fn flags(&self) -> Flags {
        let flags = unsafe { ffi::cta::di_edid_cta_get_flags(self.cta) };
        Flags::from(unsafe { *flags })
    }
}

#[derive(Debug)]
pub struct Flags {
    pub it_underscan: bool,
    pub basic_audio: bool,
    pub ycc444: bool,
    pub ycc422: bool,
    pub native_dtds: i32,
}

impl From<ffi::cta::di_edid_cta_flags> for Flags {
    fn from(value: ffi::cta::di_edid_cta_flags) -> Self {
        Self {
            it_underscan: value.it_underscan,
            basic_audio: value.basic_audio,
            ycc444: value.ycc444,
            ycc422: value.ycc422,
            native_dtds: value.native_dtds,
        }
    }
}
