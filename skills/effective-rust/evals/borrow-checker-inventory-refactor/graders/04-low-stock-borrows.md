---
type: llm
focus: trace
weight: 1
---

low_stock no longer returns owned clones: it returns impl Iterator<Item = &Item> + '_ or Vec<&Item> (or an equivalent borrowed form).
