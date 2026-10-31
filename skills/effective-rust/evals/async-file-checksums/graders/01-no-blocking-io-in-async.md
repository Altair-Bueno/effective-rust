---
type: llm
focus: trace
weight: 1
---

No std::fs, std::io blocking reads, or std::thread::sleep are called inside an async fn or async block; file reading uses tokio::fs or is wrapped in tokio::task::spawn_blocking.
