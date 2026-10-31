---
type: llm
focus: trace
weight: 1
---

Non-test code in inventory/src/lib.rs contains no .clone() or .to_vec() of Item or Vec<Item>; the only clones left, if any, are of small strings needed to store a new SKU.
