---
type: llm
focus: trace
weight: 1
---

Building a Frame from a Header and a Payload is a constructor, Frame::new(header, payload) -> Result (a TryFrom on the tuple is also acceptable); it is not a free function such as make_frame and it does not appear in two competing forms.
