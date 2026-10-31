---
type: llm
focus: trace
weight: 1
---

No std::sync::MutexGuard or RefCell borrow is held across an .await, and no Arc<Mutex<Vec<_>>> is polled as a work queue; results flow back through join handles or channels.
