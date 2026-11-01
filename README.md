# effective-rust

A Claude Code skill (packaged as a plugin) that steers coding agents toward
idiomatic Rust: reuse `std` and well-known crates instead of reinventing them,
follow a fixed set of patterns and anti-patterns, read the right reference
before specific kinds of work, and run the check/clippy/test/fmt loop.

## Install

As a plugin from a marketplace:

```
/plugin marketplace add Altair-Bueno/effective-rust
/plugin install effective-rust@effective-rust
```

Or copy the skill directory alone:

```sh
cp -r skills/effective-rust ~/.claude/skills/effective-rust        # personal
cp -r skills/effective-rust <project>/.claude/skills/effective-rust # per project
```

## Layout

```
skills/effective-rust/
├── SKILL.md                 # lookup order, workflow, read-before-you-do gates, pattern and anti-pattern lists
├── references/
│   ├── already-exists.md    # std APIs and crates agents keep re-implementing
│   ├── resources.md         # every book, guide and doc the skill cites, with URLs
│   ├── patterns.md          # idioms and design patterns with snippets
│   ├── anti-patterns.md     # what to refuse and what to do instead
│   ├── errors.md            # error strategy per crate kind, thiserror/anyhow shapes
│   ├── api-design.md        # condensed API Guidelines, semver, features, docs
│   ├── async.md             # tokio, blocking, cancellation, actors
│   ├── unsafe-ffi.md        # the unsafe gate, Miri, FFI rules
│   └── tooling.md           # cargo commands, lint config, testing matrix
└── evals/                   # test cases runnable with `claude plugin eval`
```

The skill body stays under 250 lines; the reference files load only when a
task needs them.

## Evaluate

```sh
cd skills/effective-rust
claude plugin eval . --allow-tools Write,Edit,Bash --scaffold
```

Each case under `evals/` ships a small crate, a prompt and graders that check
for idiomatic output (no stringly-typed errors, no blocking in async, no
clone-to-please-the-borrow-checker, clippy clean, tests pass).

## Sources

The skill links to its sources and summarises them in its own words; it
does not reproduce their text. Effective Rust (David Drysdale, CC BY-NC-ND
4.0), the Rust API Guidelines and RFCs (MIT OR Apache-2.0), Rust Design
Patterns (MPL-2.0), the official Rust, Cargo and rustdoc books, blessed.rs
(crate names only), the Google Rust Style Guide (CC BY 3.0), the Tokio
tutorial and Alice Ryhl's async posts. The full
list with URLs and licences is in
`skills/effective-rust/references/resources.md`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
