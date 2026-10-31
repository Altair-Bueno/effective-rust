---
type: llm
focus: trace
weight: 1
---

Stuff::new and set_label take &CStr (or impl AsRef<CStr>), not &str with a hidden CString::new inside; any &str convenience, if present, is a separate clearly-named method and the CStr form is the primary API.
