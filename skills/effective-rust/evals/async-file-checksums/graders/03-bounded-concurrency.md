---
type: llm
focus: trace
weight: 1
---

Files are processed concurrently with an explicit bound of 4 (tokio::sync::Semaphore, StreamExt::buffer_unordered(4), or a JoinSet kept at 4 in-flight), not sequentially awaited one by one and not spawned without limit.
