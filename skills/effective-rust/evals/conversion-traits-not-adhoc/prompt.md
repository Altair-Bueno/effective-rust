---
tags: [effective-rust]
max_turns: 40
timeout_seconds: 900
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash, LSP]
---

In the `wire` crate we need three things: build a `Frame` out of a `Header` and a `Payload` (it fails when the payload is longer than a u16 can describe), get the `Header` back out of a `Frame`, and parse a `Frame` from raw bytes (`version`, `flags`, big-endian `len`, then `len` body bytes; anything short or trailing is an error). Add tests.

Input files: wire/Cargo.toml, wire/src/lib.rs
