---
type: llm
focus: trace
weight: 1
---

Per-file read failures are reported on stderr with the file name and the error, without unwrap()/expect() on I/O results; main returns a Result (anyhow::Result or Result<(), Box<dyn Error>>).
