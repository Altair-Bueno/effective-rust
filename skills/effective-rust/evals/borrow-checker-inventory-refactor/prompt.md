---
tags: [effective-rust]
max_turns: 40
timeout_seconds: 900
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash]
---

A reviewer rejected `inventory/src/lib.rs` with the comment "this clones its way around the borrow checker and loops by index everywhere; `low_stock` allocates a whole Vec of copies just so the caller can iterate". Refactor the module so it is idiomatic Rust. Public behaviour must stay the same and the existing tests must keep passing (you may adjust a test only if a signature improvement forces it, and say so).

Input files: inventory/Cargo.toml, inventory/src/lib.rs
