---
type: llm
focus: trace
weight: 1
---

Every unsafe block has a // SAFETY: comment stating the invariant; Stuff implements Drop calling stuff_free exactly once; the raw pointer is not Copy/Clone-exposed.
