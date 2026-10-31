---
tags: [effective-rust]
max_turns: 40
timeout_seconds: 900
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash, LSP]
---

The `telemetry` workspace needs a binary codec. Add an `Encode` trait (`fn encode(&self, out: &mut Vec<u8>)`) and a `Decode` trait (`fn decode(input: &[u8]) -> Result<(Self, &[u8]), DecodeError>`) implemented for all the fixed-width integers, `bool`, `str`/`String` (u16 length prefix) and `Vec<T>` (u16 count prefix), then implement both for the existing `Sample` type. Other crates in this workspace and other teams will use the codec. Add tests.

Input files: telemetry/Cargo.toml, telemetry/crates/telemetry/Cargo.toml, telemetry/crates/telemetry/src/lib.rs, telemetry/crates/telemetry/src/sample.rs
