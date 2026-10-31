//! Raw bindings to libstuff. In the real build these come from bindgen; here the
//! C side is stubbed in Rust so the crate links without the library.
#![allow(non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::{c_char, c_int, CStr, CString};

#[repr(C)]
pub struct stuff_t {
    name: CString,
    label: Option<CString>,
}

/// Returns a null pointer when `name` is empty.
#[no_mangle]
pub unsafe extern "C" fn stuff_new(name: *const c_char) -> *mut stuff_t {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    let name = CStr::from_ptr(name).to_owned();
    if name.as_bytes().is_empty() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(stuff_t { name, label: None }))
}

/// Valid until the next `stuff_free`.
#[no_mangle]
pub unsafe extern "C" fn stuff_name(s: *const stuff_t) -> *const c_char {
    (*s).name.as_ptr()
}

/// Returns 0 on success, -1 if the label is longer than 16 bytes.
#[no_mangle]
pub unsafe extern "C" fn stuff_set_label(s: *mut stuff_t, label: *const c_char) -> c_int {
    let label = CStr::from_ptr(label);
    if label.to_bytes().len() > 16 {
        return -1;
    }
    (*s).label = Some(label.to_owned());
    0
}

#[no_mangle]
pub unsafe extern "C" fn stuff_free(s: *mut stuff_t) {
    if !s.is_null() {
        drop(Box::from_raw(s));
    }
}
