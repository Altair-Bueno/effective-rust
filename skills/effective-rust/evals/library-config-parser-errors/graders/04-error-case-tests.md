---
type: llm
focus: trace
weight: 1
---

Tests exist and pass (cargo test succeeds) covering at least: duplicate key, unknown key, invalid or zero port, missing required key, and that comments and blank lines are skipped; at least one test asserts the error variant or line number, not merely is_err().
