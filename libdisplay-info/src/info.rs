//! High-level API.
use crate::{edid::Edid, ffi, string_from_owned_ffi_ptr};

/// Information about a display device.
///
/// This includes at least one EDID or DisplayID blob.
///
/// Use [`Info::parse_edid`](Info::parse_edid) to create a [`Info`] from an EDID blob.
/// DisplayID blobs are not yet supported.
#[derive(Debug)]
pub struct Info(*mut ffi::info::di_info);

/// Parsing the EDID blob failed
#[derive(Debug, thiserror::Error)]
#[error("Parsing the EDID blob failed")]
pub struct ParseFailed;

impl Info {
    /// Parse an EDID blob.
    pub fn parse_edid(data: &[u8]) -> Result<Self, ParseFailed> {
        let info = unsafe {
            ffi::info::di_info_parse_edid(data.as_ptr() as *const std::ffi::c_void, data.len())
        };

        if info.is_null() {
            return Err(ParseFailed);
        }

        Ok(Self(info))
    }

    /// Get the failure messages for this blob.
    ///
    /// `None` is returned if the blob conforms to the relevant specifications.
    pub fn failure_msg(&self) -> Option<&std::ffi::CStr> {
        let failure_msg = unsafe { ffi::info::di_info_get_failure_msg(self.0) };

        if failure_msg.is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(failure_msg) })
        }
    }

    /// Returns the EDID the display device information was constructed with.
    ///
    /// The returned [`Edid`] can be used to query low-level EDID information,
    /// see [`edid`](crate::edid) module level docs. Users should prefer the high-level API if
    /// possible.
    ///
    /// `None` is returned if the [`Info`] doesn't contain an EDID.
    pub fn edid(&self) -> Option<Edid<'_>> {
        Edid::from_ptr(unsafe { ffi::info::di_info_get_edid(self.0) as *const ffi::edid::di_edid })
    }

    /// Get the make of the display device.
    ///
    /// This is the manufacturer name, either company name or PNP ID.
    /// This string is informational and not meant to be used in programmatic
    /// decisions, configuration keys, etc.
    ///
    /// The string is in UTF-8 and may contain any characters except ASCII control
    /// codes.
    ///
    /// `None` is returned if the information is not available.
    pub fn make(&self) -> Option<String> {
        string_from_owned_ffi_ptr(unsafe { ffi::info::di_info_get_make(self.0) })
    }

    /// Get the model of the display device.
    ///
    /// This is the product name/model string or product number.
    /// This string is informational and not meant to be used in programmatic
    /// decisions, configuration keys, etc.
    ///
    /// The string is in UTF-8 and may contain any characters except ASCII control
    /// codes.
    ///
    /// `None` is returned if the information is not available.
    pub fn model(&self) -> Option<String> {
        string_from_owned_ffi_ptr(unsafe { ffi::info::di_info_get_model(self.0) })
    }

    /// Get the serial of the display device.
    ///
    /// This is the product serial string or the serial number.
    /// This string is informational and not meant to be used in programmatic
    /// decisions, configuration keys, etc.
    ///
    /// The string is in UTF-8 and may contain any characters except ASCII control
    /// codes.
    ///
    /// `None` is returned if the information is not available.
    pub fn serial(&self) -> Option<String> {
        string_from_owned_ffi_ptr(unsafe { ffi::info::di_info_get_serial(self.0) })
    }
}

impl Drop for Info {
    fn drop(&mut self) {
        unsafe {
            ffi::info::di_info_destroy(self.0);
        }
    }
}
