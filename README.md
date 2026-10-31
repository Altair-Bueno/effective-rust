# effective-rust

A Claude Code skill (packaged as a plugin) that steers coding agents toward
idiomatic Rust: reuse `std` and well-known crates instead of reinventing them,
follow a fixed set of patterns and anti-patterns, read the right reference
before specific kinds of work, and run the check/clippy/test/fmt loop.

## Install

As a plugin from a marketplace:

```
/plugin marketplace add <github-user>/effective-rust
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

Effective Rust (David Drysdale), the Rust API Guidelines, Rust Design
Patterns, the Rust Book/Reference/Rustonomicon, the Cargo and rustdoc books,
blessed.rs, the Tokio tutorial and Alice Ryhl's async posts. See
`skills/effective-rust/references/resources.md`.
