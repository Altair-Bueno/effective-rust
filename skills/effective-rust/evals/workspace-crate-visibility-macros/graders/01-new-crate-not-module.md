---
type: llm
focus: trace
weight: 1
---

The codec lives in a new workspace crate (e.g. crates/telemetry-codec with its own Cargo.toml, added to the workspace members) that telemetry depends on, not in a new module inside the telemetry crate.
