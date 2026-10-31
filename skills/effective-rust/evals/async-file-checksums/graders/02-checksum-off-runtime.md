---
type: llm
focus: trace
weight: 1
---

The CPU-heavy checksum function is executed via tokio::task::spawn_blocking (or a rayon pool) rather than directly inside an async task.
