---
type: llm
focus: trace
weight: 1
---

Tests cover the three conversions including the error paths, and no test's subject is std behaviour (for example asserting that u16::try_from fails on big values in isolation).
