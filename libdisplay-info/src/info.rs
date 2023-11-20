use crate::{edid::Edid, ffi};

#[derive(Debug)]
pub struct Info(*mut ffi::info::di_info);

#[derive(Debug)]
pub struct ParseFailed {
    pub failure_msg: Option<String>,
}

impl Info {
    /// Parse an EDID blob.
    pub fn parse(data: &[u8]) -> Result<Self, ParseFailed> {
        let info = unsafe {
            ffi::info::di_info_parse_edid(data.as_ptr() as *const std::ffi::c_void, data.len())
        };

        if info.is_null() {
            return Err(ParseFailed { failure_msg: None });
        }

        let failure_msg = unsafe { ffi::info::di_info_get_failure_msg(info) };

        if !failure_msg.is_null() {
            let failure_msg = unsafe { std::ffi::CStr::from_ptr(failure_msg) };
            Err(ParseFailed {
                failure_msg: Some(failure_msg.to_string_lossy().to_string()),
            })
        } else {
            Ok(Self(info))
        }
    }

    /// Returns the EDID the display device information was constructed with.
    ///
    /// The returned struct di_edid can be used to query low-level EDID information,
    /// see [Edid](crate::edid). Users should prefer the high-level API if\n possible.
    ///
    /// `None` is returned if the struct [`Info`] doesn't contain an EDID.
    pub fn edid(&self) -> Option<Edid<'_>> {
        let edid = unsafe { ffi::info::di_info_get_edid(self.0) };

        if edid.is_null() {
            None
        } else {
            Some(unsafe { Edid::from_ptr(edid as *const ffi::edid::di_edid) })
        }
    }
}

impl Drop for Info {
    fn drop(&mut self) {
        unsafe {
            ffi::info::di_info_destroy(self.0);
        }
    }
}
