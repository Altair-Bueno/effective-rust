---
type: llm
focus: trace
weight: 1
---

There are no index loops of the form for i in 0..xs.len(); total_value is computed with an iterator (iter().map(..).sum()), and other loops use iter()/iter_mut()/enumerate().
