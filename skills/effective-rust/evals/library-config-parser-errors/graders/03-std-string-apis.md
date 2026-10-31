---
type: llm
focus: trace
weight: 1
---

Parsing uses standard library string APIs such as lines(), split_once('='), trim(), starts_with('#') or strip_prefix, and parse::<u16>()/parse::<usize>(), rather than hand-written character-index loops; the port is never produced with an `as` cast.
