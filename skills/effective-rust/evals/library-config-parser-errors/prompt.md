---
tags: [effective-rust]
max_turns: 40
timeout_seconds: 900
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash]
---

The `kvconf` crate is a small library other teams will depend on. Right now it only has the `Config` struct. Implement `Config::parse(text: &str)` for our config file format:

- one `key = value` per line, whitespace around key and value is ignored
- lines starting with `#` and blank lines are skipped
- keys: `host` (string), `port` (must fit in a u16, must be non-zero), `workers` (positive integer); any other key is an error
- a duplicate key is an error
- `host` and `port` are required, `workers` defaults to 4

When parsing fails, callers need to know the line number and what went wrong so they can show it to the user. Add tests and make sure the crate is in good shape.

Input files: kvconf/Cargo.toml, kvconf/src/lib.rs
