---
type: llm
focus: trace
weight: 1
---

Imports use core::ffi::{CStr, c_char, c_int} and alloc::ffi::CString where those items exist, rather than std::ffi.
