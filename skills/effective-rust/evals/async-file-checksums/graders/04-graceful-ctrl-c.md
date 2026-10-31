---
type: llm
focus: trace
weight: 1
---

Ctrl-C is handled with tokio::signal::ctrl_c and a cancellation signal (tokio_util CancellationToken, watch channel, or equivalent); after the signal no new files start, in-flight tasks are awaited (JoinSet::join_next / JoinHandle awaited), results are printed, and the process exits with status 0.
