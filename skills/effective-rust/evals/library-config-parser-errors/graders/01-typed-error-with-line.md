---
type: llm
focus: trace
weight: 1
---

kvconf exposes a dedicated error type that implements std::error::Error (via thiserror or by hand), with variants that carry the line number and the offending key or value; the public API does not use Result<_, String> or Box<dyn Error>.
