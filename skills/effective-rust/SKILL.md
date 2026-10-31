---
name: effective-rust
description: Idiomatic Rust guidance for writing, reviewing, refactoring and debugging Rust code. Use this skill whenever a task touches a .rs file, Cargo.toml, Cargo.lock, a cargo/clippy/rustfmt/rustc/miri command or its output, or any Rust crate, workspace, trait, lifetime, borrow-checker, async/tokio, unsafe/FFI, error-handling, testing or dependency question, even when the user never says "Rust". It stops agents from reinventing std and well-known crates, names the reference to read before each kind of task, and fixes the check/clippy/test/fmt workflow.
---

# Effective Rust

Rust rewards code that leans on the standard library, the type system and the
compiler's lints. Agents writing Rust tend to do the opposite: hand-roll
helpers that already exist in `std`, clone their way past the borrow checker,
return `String` errors, sprinkle `unwrap()` and `#[allow]`, and guess at crate
APIs. Every one of those is a bug or a review comment waiting to happen. This
skill exists to make the idiomatic choice the default one.

**Tradeoff:** the skill biases toward reading before writing. For a one-line
fix in a crate you already understand, use judgment and skip the gates that
do not apply.

**Precedence:** a project's own `CLAUDE.md`/`AGENTS.md`, `Cargo.toml`
`[lints]`, `clippy.toml`, `rustfmt.toml`, `deny.toml` and CI workflows are the
local law. When they disagree with this skill, follow the project.

## 1. The wheel already exists

Before writing any helper, type, trait, macro or algorithm, look for it in this
order and stop at the first hit:

1. **`std` / `core` / `alloc`.** Search docs.rs/std and the table in
   `references/already-exists.md`. The std version is tested, optimised,
   `const`-friendly and instantly recognisable to reviewers.
2. **This workspace and its existing dependencies.** Search with `rg` or
   `ast-grep --lang rust`, then read `Cargo.toml` and `cargo tree --depth 1`.
   A dependency such as `itertools`, `bytes`, `winnow` or `tokio` often already
   provides what you are about to write, and the crate may already have a
   private helper for it.
3. **The ecosystem.** Check https://blessed.rs/crates and `cargo info <crate>`.
   Adding a dependency is a visible decision: check maintenance, licence,
   MSRV, transitive size (`cargo tree -p <crate>`), and copy the project's
   `default-features = false` convention if it uses one. Say in your summary
   that you added it and why.
4. **Write it**, as small as possible, private unless it is part of the API,
   next to its only call site, with a test.

Treat "I will just write a quick version" as a signal to run the lookup, not
to skip it.

## 2. How Rust work is done

### Orient (first minutes in any Rust repo)

- Run `rustc --version` and read `rust-toolchain.toml`, the `edition` and
  `rust-version` (MSRV) fields. You may only use language and library features
  that exist at the MSRV. Do not change edition, MSRV or toolchain channel
  without being asked.
- Read `Cargo.toml` (workspace and crate): `[lints]`, `[features]`,
  dependencies and their feature flags. Read `clippy.toml`, `rustfmt.toml`,
  `deny.toml`, `.config/nextest.toml` and the CI workflow. They tell you which
  commands must pass and which lints are errors.
- Check `rust-analyzer --version`. When the LSP tool is available, use it for
  hover, go-to-definition, references and diagnostics instead of guessing
  signatures. `cargo check` is the fallback, not the first resort.
- Find how the crate already does the thing you are about to do (error type,
  logging, module layout, test style, builder style) and match it.

### Design before typing

- Make invalid states unrepresentable: newtypes for validated values, enums
  instead of `bool`/`Option` flags, typestate for protocols and lifecycles.
- Decide ownership at the boundary. Accept `&str`, `&[T]`, `&Path`,
  `impl AsRef<..>` or `impl Iterator`; return owned values; store owned values.
- Pick the error strategy for the crate kind: a `thiserror` enum for libraries,
  `anyhow`/`eyre` with context for binaries. See `references/errors.md`.
- Choose generics by default and `dyn Trait` when you need heterogeneous
  collections, plugin boundaries or smaller binaries.
- If the item is `pub` in a library, you are designing an API: read
  `references/api-design.md` first.
- A function whose whole job is turning `A` (and maybe `B`) into `C` is a
  conversion: implement `From` or `TryFrom` (on a tuple or a small input
  struct) instead of a free function. Callers get `?`, `into()` and
  `collect` for free, and the compiler finds every conversion site.
- Starting a new module? Consider a new crate in the workspace instead. A
  crate is the unit of parallel compilation and of API design: small crates
  compile faster and force you to decide what is public. Keep modules for
  code that only makes sense inside one crate.
- Visibility is private by default. Child modules already see everything in
  their ancestors, so shared helpers live in the parent, unmarked. Use
  `pub(super)` when a parent needs a child item, `pub(crate)` only for
  genuinely crate-wide items, and `pub` only for the few items that are the
  crate's API, re-exported from `lib.rs`.
- Traits, generics, blanket impls and const generics come before macros. A
  `macro_rules!` is justified for repetition the type system cannot express
  (implementing a trait for every integer type). Do not add proc-macro
  crates; using derive macros from existing dependencies is fine.

### Implement

- Prefer iterator chains to index loops, `?` to `match` on `Result`,
  combinators (`map`, `and_then`, `ok_or_else`, `unwrap_or_else`) to nested
  `match`, `let else` and `matches!` to boilerplate.
- Match exhaustively on enums you own. A `_ =>` arm hides the next variant's
  bug from the compiler.
- Import from `core` and `alloc` when the item lives there (`core::fmt`,
  `core::error::Error`, `core::ffi::CStr`, `alloc::vec::Vec`,
  `alloc::string::String`). Reach for `std` only for OS facilities: `fs`,
  `net`, `thread`, `env`, `process`, `io`, `SystemTime`. It costs nothing in
  a `std` crate and keeps the code usable from `no_std` users.
- Do not hide allocation or fallibility behind a prettier signature. At a
  boundary, take the well-formed type the callee needs (`&CStr`, `&Path`,
  `Port`) and let the caller convert. A `&str` parameter that runs
  `CString::new` inside hides both an allocation and an error.
- `unwrap()`, `expect()`, `panic!`, `todo!` and `unimplemented!` stay out of
  non-test code unless the invariant is locally provable; then write
  `expect("<why this cannot fail>")`.
- Never add `#[allow(..)]` to make a lint go away. Fix the cause. If it is a
  true false positive, use `#[expect(lint, reason = "..")]` so the attribute
  errors when it becomes stale.
- `unsafe` is not a tool for fighting the borrow checker or for speed without
  a benchmark. Pass the gate in `references/unsafe-ffi.md` first.
- Document `pub` items the way the crate already does, following
  RFC 1574 (summary line, `# Examples`, `# Errors`, `# Panics`, `# Safety`).

### Verify (the loop you iterate in)

```sh
cargo check --all-targets                      # fast; iterate here
cargo clippy --all-targets --all-features      # zero new warnings
cargo test            # or: cargo nextest run && cargo test --doc
cargo fmt             # cargo +nightly fmt if rustfmt.toml uses unstable options
cargo doc --no-deps   # when you touched public docs
```

- Use the project's exact flags from CI if they differ from the above.
- Fixing a bug: write the failing test first, then make it pass.
- Test your code, not your dependencies. A test asserting that
  `u16::try_from(70_000)` fails or that `CString::new` rejects interior NULs
  tests the standard library, which is already tested. Every test exercises
  a behaviour of this crate.
- `cargo fmt` and `cargo clippy --all-targets` run after the last edit, not
  only before it. Clippy finds the shape smells this skill lists; fmt keeps
  the diff reviewable.
- Crate has features: also run `--no-default-features` and `--all-features`
  for the touched crate (`cargo hack --feature-powerset` if installed).
- Workspace: scope with `-p <crate>` while iterating, run `--workspace` before
  you finish.
- `no_std` crates: check against the target CI uses, for example
  `cargo check --no-default-features --target thumbv7em-none-eabi`.
- Public API changed: run `cargo semver-checks` if installed and state the
  semver impact (patch, minor, breaking) in your summary.

### Deliver

Your summary names: dependencies added or removed, public API changes and
their semver impact, every new `unsafe` block, every `#[expect]`, and any
MSRV or feature implications. These are the things a Rust reviewer looks for
first.

## 3. Read before you do

These are gates, not suggestions. Each row names the cheapest source that
prevents the most common mistake for that task. Full URLs live in
`references/resources.md`.

| Before you... | Read first |
|---|---|
| Write a conversion between your own types | Conversions section of `references/patterns.md`; Effective Rust Item 5 |
| Design or change a `pub` API in a library | `references/api-design.md`; Rust API Guidelines checklist; RFC 1105 (API evolution), RFC 344 and 199 (naming); Effective Rust Items 1–13, 22–24 |
| Define an error type or choose an error strategy | `references/errors.md`; Effective Rust Item 4; `thiserror` and `anyhow` docs |
| Add a dependency | `references/already-exists.md`; blessed.rs; the crate's docs.rs page; Effective Rust Items 25–26 |
| Call a crate API you have not used in this session | Its docs.rs page at the version pinned in `Cargo.lock`. Never guess method names or signatures |
| Write a safe wrapper over C or OS APIs | Boundary types section of `references/unsafe-ffi.md` |
| Write `unsafe`, FFI, `#[repr]`, raw pointers or `transmute` | `references/unsafe-ffi.md`; the Rustonomicon; Unsafe Code Guidelines; then run Miri |
| Write async code, spawn tasks, use `select!`, pick a runtime | `references/async.md`; Tokio tutorial; "Async: What is blocking?"; `tokio::select!` cancellation-safety docs |
| Share state across threads, use atomics or locks | `std::sync` docs; Rust Atomics and Locks; Effective Rust Item 17 |
| Reach for `clone()`, `Rc<RefCell<_>>` or `unsafe` to satisfy the borrow checker | Effective Rust Items 14–15; Rust Design Patterns anti-pattern "Clone to satisfy the borrow checker"; ownership section of `references/patterns.md` |
| Optimise anything | Rust Performance Book; Effective Rust Item 20; a `criterion`/`divan` benchmark before and after |
| Write a macro | Effective Rust Item 28; The Little Book of Rust Macros; API Guidelines macros section |
| Add or change cargo features, bump a version, publish | Cargo features and SemVer references; Effective Rust Items 21, 26; `cargo semver-checks` |
| Change edition or MSRV | Edition Guide; RFC 3501 (edition 2024); Cargo `rust-version` reference |
| Write rustdoc | RFC 1574 (conventions) and RFC 1946 (intra-doc links); rustdoc book "What to include"; API Guidelines documentation section |
| Write tests, benches, fuzzers | `references/tooling.md`; Effective Rust Item 30; Book ch. 11; proptest, insta, cargo-fuzz books |
| Choose a collection type | `std::collections` "When should you use which collection?" |
| Make a library `no_std` | Effective Rust Item 33; Embedded Rust Book |
| Fail to fix a compiler error on the first try | `rustc --explain E0xxx`; the Rust Error Codes Index; RFC 1023 for orphan-rule errors (E0117, E0210) |
| Silence a lint | That lint's page in the Clippy lint list, to learn what it protects against |

## 4. Patterns to reach for

Details and snippets in `references/patterns.md`.

- **Newtype** around primitives that carry meaning (`UserId(u64)`, `Port(u16)`)
  with a validating constructor and `TryFrom`.
- **Typestate** for state machines and protocols so illegal transitions do not
  compile.
- **Builder** when a constructor would take more than three optional values.
  Use `bon` or `typed-builder` if the project already depends on one.
- **RAII guards** (`Drop`) for anything that must be released or restored.
- **`Default` derive** plus `..Default::default()` struct update syntax.
- **Borrowed argument types** (`&str`, `&[T]`, `&Path`) and `impl AsRef<T>`
  only where it buys real ergonomics.
- **`mem::take` / `mem::replace` / `Option::take`** to move out of `&mut`
  without cloning.
- **Entry API** (`map.entry(k).or_insert_with(..)`) instead of get-then-insert.
- **Iterator adapters** and `collect::<Result<Vec<_>, _>>()` to fail fast
  over a sequence.
- **`?` with `From`** conversions so error plumbing is one character.
- **`let else`**, `matches!`, `if let .. else`, let chains (edition 2024) for
  shallow control flow.
- **`impl Trait` in return position** to hide iterator and future types.
- **`From` / `TryFrom` for every conversion** between your own types, including
  `TryFrom<(A, B)>` when `C` is built from two inputs.
- **Sealed traits** and `#[non_exhaustive]` so the public API can grow.
- **`Cow<'_, str>`** when a function usually borrows but sometimes must own.
- **`Arc<str>` / `Box<str>` / `Box<[T]>`** for immutable shared data.
- **`std::thread::scope`** for borrowing across threads without `Arc`.
- **Channels plus an owning task** (actor) instead of `Arc<Mutex<_>>`
  for shared mutable state in async code.
- **`LazyLock` / `OnceLock`** for lazy globals.
- **Extension traits** to add methods to foreign types instead of wrapper
  structs.
- **Thin `main`**: binaries put logic in `lib.rs` and keep `main.rs` to
  argument parsing and `fn main() -> anyhow::Result<()>`.
- **Workspaces of small crates**: one crate per cohesive capability, private
  by default, `core`/`alloc` imports so they stay `no_std`-friendly.

## 5. Anti-patterns to refuse

Details and the replacement for each in `references/anti-patterns.md`.

- Hand-written versions of anything in `references/already-exists.md`
  (split/trim helpers, lazy init, dedup, chunking, retry loops, arg parsing,
  JSON, hashing, base64, random).
- `clone()` or `Rc<RefCell<_>>` as the first answer to a borrow error.
- `Result<T, String>`, `Box<dyn Error>` in a library's public API, errors that
  lose the `source()` chain, stringified errors.
- `unwrap()` / `expect()` / `panic!` on recoverable conditions in library code.
- `as` for numeric conversions that can truncate or wrap; use `From`,
  `TryFrom`, `checked_*`, `saturating_*`.
- `&String`, `&Vec<T>`, `&Box<T>` parameters; `String` parameters that are
  only read.
- `bool` and `Option<bool>` parameters, out-parameters, `Option<Option<T>>`.
- `impl Deref` to fake inheritance.
- `#![deny(warnings)]` in source (breaks on every toolchain update) and
  `#[allow(..)]` to silence lints.
- `use foo::*` outside tests and preludes.
- `static mut`, `transmute`, `unsafe` without a `// SAFETY:` comment.
- `lazy_static!` / `once_cell` when the MSRV has `std::sync::LazyLock`.
- `async_trait` when native `async fn` in traits works (keep it for
  `dyn Trait`).
- Blocking in async (`std::fs`, `std::thread::sleep`, CPU loops) and holding a
  `std::sync::MutexGuard` across `.await`.
- Sequential `.await`s that should be `join!`, `JoinSet` or
  `buffer_unordered`.
- `println!`, `eprintln!` or `std::process::exit` inside library code.
- `for i in 0..v.len()` index loops; `collect()` into a `Vec` only to iterate
  again.
- `_ =>` arms on enums you own; `#[inline]` sprinkled without a benchmark.
- Traits and generics with a single implementation, introduced "for
  flexibility".
- Ad-hoc `make_c(a, b)` or `C::from_parts(a, b)` functions where `From` or
  `TryFrom` is the whole story.
- `&str` parameters that silently allocate a `CString`, `PathBuf` or `String`
  inside; the caller should see the allocation and the error.
- `std::` paths for items that live in `core` or `alloc`.
- A new module where a new workspace crate would do; `pub(crate)` on helpers
  that only the parent and its children use.
- Proc macros, or `macro_rules!` for anything a trait or generic can express.
- Tests whose subject is `std` or a dependency rather than this crate.
- Non-additive cargo features and features that remove functionality
  (`no-std`, `no-serde`).

## 6. Reference index

Read the one that matches the task; each is self-contained.

| File | Read when |
|---|---|
| `references/already-exists.md` | About to write a helper, or about to add a dependency |
| `references/resources.md` | You need the URL for any book, guide or doc named in this skill |
| `references/patterns.md` | Implementing types, ownership, control flow, modules |
| `references/anti-patterns.md` | Reviewing code, or unsure whether a construct is idiomatic |
| `references/errors.md` | Defining, converting, reporting or testing errors |
| `references/api-design.md` | Any `pub` item in a library, semver, features, docs |
| `references/async.md` | tokio, futures, tasks, channels, cancellation, blocking |
| `references/unsafe-ffi.md` | `unsafe`, raw pointers, `#[repr]`, FFI, Miri |
| `references/tooling.md` | cargo, clippy configuration, tests, benches, fuzzing, coverage |
