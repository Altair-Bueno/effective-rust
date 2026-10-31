---
type: llm
focus: trace
weight: 1
---

Parsing raw bytes is implemented as TryFrom<&[u8]> for Frame, with a typed error enum (implementing core::error::Error) distinguishing short input, trailing bytes and bad length.
