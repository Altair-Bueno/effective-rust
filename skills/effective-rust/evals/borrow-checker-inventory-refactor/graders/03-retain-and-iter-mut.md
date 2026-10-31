---
type: llm
focus: trace
weight: 1
---

remove_discontinued uses Vec::retain (or retain_mut); restock locates existing items with iter_mut().find(..) or a map/entry lookup and pushes new items without rebuilding the whole Vec.
