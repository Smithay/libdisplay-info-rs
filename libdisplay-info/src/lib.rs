use std::{ffi::CStr, marker::PhantomData};

pub use libdisplay_info_sys as ffi;

pub mod cta;
pub mod cvt;
pub mod displayid;
pub mod dmt;
pub mod edid;
pub mod gtf;
pub mod info;

/// Get the [`String`] from an owned ffi ptr
///
/// This will automatically free the ptr
fn string_from_ffi_ptr(ptr: *const i8) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
    }
}

/// Get the [`String`] from an owned ffi ptr
///
/// This will automatically free the ptr
///
/// `None` is returned for NULL ptr
fn string_from_owned_ffi_ptr(ptr: *mut i8) -> Option<String> {
    let res = string_from_ffi_ptr(ptr);
    if res.is_some() {
        unsafe {
            libc::free(ptr as *mut _);
        }
    }
    res
}

struct FFIIter<'a, T, F> {
    ptr: *const *const F,
    t: PhantomData<T>,
    phantom: PhantomData<&'a ()>,
}

impl<'a, T, F> FFIIter<'a, T, F> {
    fn new(ptr: *const *const F) -> Self {
        Self {
            ptr,
            t: PhantomData,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, F> Iterator for FFIIter<'a, T, F>
where
    T: From<F>,
    F: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr.is_null() || unsafe { *self.ptr }.is_null() {
            None
        } else {
            let item = T::from(unsafe { *(*self.ptr) });
            self.ptr = self.ptr.wrapping_add(1);
            Some(item)
        }
    }
}
