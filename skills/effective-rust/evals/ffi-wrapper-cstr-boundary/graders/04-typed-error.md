---
type: llm
focus: trace
weight: 1
---

The non-zero status code from stuff_set_label and a null pointer from stuff_new map to a dedicated error type implementing core::error::Error / std::error::Error, not to a String or a panic.
