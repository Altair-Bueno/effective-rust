---
tags: [effective-rust]
max_turns: 40
timeout_seconds: 900
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash, LSP]
---

`stuff-sys` is the raw binding to our C library (the C side is stubbed in Rust here so it links). Write the safe wrapper crate `stuff` (currently empty) with a `Stuff` type: construct it from a name, read the name back, set a label, free on drop. Errors from the C side must surface as a proper error. Add tests.

Input files: stuff/Cargo.toml, stuff/crates/stuff-sys/Cargo.toml, stuff/crates/stuff-sys/src/lib.rs, stuff/crates/stuff/Cargo.toml, stuff/crates/stuff/src/lib.rs
