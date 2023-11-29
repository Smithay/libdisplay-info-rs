use std::ffi::CStr;

pub use libdisplay_info_sys as ffi;

pub mod cta;
pub mod edid;
pub mod info;

/// Get the [`String`] from an owned ffi ptr
///
/// This will automatically free the ptr
///
/// `None` is returned for NULL ptr
fn string_from_owned_ffi_ptr(ptr: *mut i8) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        let result = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
        unsafe {
            libc::free(ptr as *mut _);
        }
        Some(result)
    }
}
