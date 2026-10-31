---
type: llm
focus: trace
weight: 1
---

Building a Frame from a Header and a Payload is implemented as TryFrom (for the tuple or a small input struct), not as an ad-hoc function such as make_frame or Frame::from_parts that is the only entry point.
