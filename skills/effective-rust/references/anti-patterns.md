# Anti-patterns

Each entry: what it looks like, why it hurts, what to do instead. Clippy
catches a share of these (run it with `--all-targets`); the rest need a
human or this list.

## Reinventing the standard library

**Looks like:** a `fn split_key_value(s: &str)` char loop, a
`static mut CACHE: Option<..>` with an init function, a `dedup` that builds a
`HashSet` and a `Vec` by hand, a manual retry loop with `thread::sleep`, a
hand-written arg parser, an ad-hoc JSON emitter.

**Why:** every one is a subtle-bug magnet (UTF-8 boundaries, overflow,
ordering), is slower than the std version, and makes reviewers read code they
already know. See `already-exists.md`.

**Instead:** `split_once`, `LazyLock`, `sort_unstable` + `dedup`, `clap`,
`serde_json`. Run the lookup order from `SKILL.md` section 1.

## Clone to satisfy the borrow checker

**Looks like:** `.clone()` added until the error goes away; `Rc<RefCell<T>>`
or `Arc<Mutex<T>>` introduced for a single-owner value.

**Why:** hides the design problem the compiler found: two owners, or a
reference stored longer than its source. The clone becomes a hot-path cost
and the `RefCell` turns a compile error into a runtime panic.

**Instead:** restructure ownership (pass the field, not `&mut self`), use
`mem::take`/`Option::take`, split borrows, return indices or keys instead of
references, or compute the value before the mutable borrow. Clone
deliberately when the data is small or the clone is outside a loop, and say
so.

## Stringly-typed errors

**Looks like:** `Result<T, String>`, `Err(format!("bad input: {x}"))`,
`Box<dyn Error>` in a library's public signature, `map_err(|e| e.to_string())`.

**Why:** callers cannot match on the error, the `source()` chain is lost, and
every layer re-formats the message. `Box<dyn Error>` without `+ Send + Sync`
also blocks use across threads.

**Instead:** library: an enum with `thiserror`. Application: `anyhow` with
`.context()`. See `errors.md`.

## Panicking on recoverable conditions [Item 18]

**Looks like:** `unwrap()` on I/O, parsing, map lookups or user input in
library code; `expect("failed")` with no reason; `panic!` for validation;
`std::process::exit` inside a library.

**Why:** callers cannot recover, services crash on bad input, and the
message says nothing. Panics across FFI are undefined behaviour.

**Instead:** return `Result`. Reserve panics for programmer errors
(violated invariants) and write `expect("<why this is impossible>")`.
Index with `get(i)` when the index comes from outside.

## `as` casts

**Looks like:** `len as u32`, `x as usize`, `f as i64`.

**Why:** silent truncation and wrapping; `f64 as i64` saturates.

**Instead:** `u32::try_from(len)?`, `usize::from(x)` when lossless,
`checked_*`/`saturating_*` arithmetic, `to_le_bytes` for bit patterns. `as`
remains fine for `u8 as char`, enum discriminants with `#[repr]`, and
intentional truncation with a comment.

## Over-owned parameters

**Looks like:** `fn f(s: &String)`, `fn g(v: &Vec<T>)`, `fn h(name: String)`
where `name` is only read, `fn p(path: &PathBuf)`.

**Why:** rejects `&str`, slices, arrays and literals; forces callers to
allocate. Clippy: `ptr_arg`, `needless_pass_by_value`.

**Instead:** `&str`, `&[T]`, `&Path`, `impl AsRef<Path>`. Take ownership only
when storing.

## Boolean and optional flag parameters [C-CUSTOM-TYPE]

**Looks like:** `render(text, true, false)`, `fn connect(retry: Option<bool>)`.

**Instead:** small enums (`Wrap::Yes`), or a builder/config struct when there
are several.

## Deref polymorphism

**Looks like:** `impl Deref for Child { type Target = Parent; }` to "inherit"
methods.

**Why:** `Deref` is for smart pointers; method resolution becomes surprising
and the API Guidelines reject it (C-DEREF).

**Instead:** composition with explicit delegation, or a trait implemented by
both.

## `#![deny(warnings)]` in source

**Why:** a new toolchain adds a lint and every downstream build breaks.

**Instead:** `-D warnings` on the CI command line (`RUSTFLAGS` or
`cargo clippy -- -D warnings`), and per-lint levels in `[lints]`.

## `#[allow(..)]` to silence lints

**Why:** the lint was pointing at a real smell; the allow outlives the reason
and spreads by copy-paste.

**Instead:** fix the code. For real false positives:
`#[expect(clippy::too_many_arguments, reason = "mirrors the C signature")]`,
which fails when the lint stops firing.

## Wildcard imports [Item 23]

**Looks like:** `use crate::types::*;` outside tests or a prelude.

**Why:** name collisions appear when a dependency adds an item; readers cannot
tell where a name comes from.

## `static mut`, `transmute`, unexplained `unsafe` [Item 16]

**Why:** `static mut` references are denied in edition 2024 for a reason (data
races by construction). `transmute` bypasses every type-system check.
`unsafe` without a `// SAFETY:` comment is unreviewable.

**Instead:** `AtomicX`, `Mutex`, `OnceLock`, `LazyLock`; `from_ne_bytes`,
`bytemuck`/`zerocopy`; the gate in `unsafe-ffi.md`.

## Obsolete crates for things now in std

| Crate | std replacement (version) |
|---|---|
| `lazy_static`, `once_cell::sync::Lazy` | `std::sync::LazyLock` (1.80) |
| `once_cell::sync::OnceCell` | `std::sync::OnceLock` (1.70) |
| `async-trait` (non-`dyn` use) | native `async fn` in traits (1.75) |
| `num_cpus` | `std::thread::available_parallelism` |
| `atty` | `std::io::IsTerminal` (1.70) |
| `matches` | `matches!` macro |
| `hex`-free manual hex loops | `format!("{:02x}")` / `u8::from_str_radix` |

Keep the crate when the MSRV is older than the std version.

## Async mistakes

Full treatment in `async.md`.

- Blocking in a task: `std::fs`, `std::net`, `std::thread::sleep`,
  `reqwest::blocking`, tight CPU loops. Starves the runtime.
- `std::sync::MutexGuard` held across `.await`: deadlocks and non-`Send`
  futures.
- `Arc<Mutex<Vec<Job>>>` polled in a loop as a queue: use a channel.
- Sequential awaits in a loop when the operations are independent.
- `tokio::spawn` for CPU work: use `spawn_blocking` or `rayon`.
- `select!` on a non-cancellation-safe future, losing data on the other branch.
- `#[async_trait]` where native `async fn` compiles.

## Shape smells

- `for i in 0..v.len()`: use iterators (`needless_range_loop`).
- `.iter().map(|x| x.clone())`: `.cloned()`; `.len() == 0`: `.is_empty()`.
- `collect::<Vec<_>>()` followed by `.iter()`: keep the chain lazy.
- `match x { Some(v) => v, None => return .. }`: `let else`.
- `if x == true`, `return x;` as the tail expression, `&x[..]` for `as_slice`.
- `_ =>` on an enum you own: name the variants so the compiler flags new ones.
- `#[inline]` everywhere: let LLVM decide; add it only after a benchmark or
  for tiny cross-crate generics.
- `impl Into<String>` on every parameter: hides allocations, hurts inference.
- `Option<Option<T>>`, `Result<Option<T>, E>` juggled by hand: `flatten`,
  `transpose`.
- `println!` in a library: use `tracing`/`log`, return data, let the binary
  print.
- `.to_string()` / `.to_owned()` in a hot loop on data that could be borrowed.

## Premature abstraction

**Looks like:** a trait with one impl, a generic parameter never instantiated
with a second type, a `Box<dyn Strategy>` for a decision that never changes,
a `utils` module of one-line wrappers.

**Why:** abstractions are promises; unused ones cost compile time, reading
time and API surface, and Rust's abstractions are hard to remove later.

**Instead:** concrete types and free functions until the second real use
appears. Note the opposite rule for crates: splitting a workspace into small
crates is not premature abstraction, it is how Rust compiles in parallel
(Rust Design Patterns, "Prefer small crates").

## Ad-hoc conversion functions

**Looks like:** `fn header_of(f: &Frame) -> Header`, `Frame::from_bytes(..)`,
`fn parse_frame(bytes: &[u8]) -> Result<Frame, E>`, `fn to_dto(&self) ->
Dto` as the only way to convert one value into another type.

**Why:** the body is field shuffling; a name invented per pair hides that
from readers and from the compiler, and callers lose `?`, `.into()`,
`.parse()`, `collect::<Result<_, _>>()` and generic code bounded on
`TryFrom`.

**Instead:** `impl From<&Frame> for Header`, `impl TryFrom<&[u8]> for
Frame`, `impl FromStr for Frame`. Keep an inherent alias only when
discoverability matters, implemented via the trait. Assembling from several
independent parts is a constructor (`Frame::new(header, payload)`), not a
`TryFrom` on a tuple.

## Default features left on

**Looks like:** `tokio = "1"`, `reqwest = "0.12"` in a member crate; the
same dependency declared in three crates with three feature lists.

**Why:** default features drag in code nobody asked for (TLS backends,
runtimes, codecs), inflate build time and binary size, and three
declarations drift. Members cannot turn off a default the workspace root
enabled.

**Instead:** declare each dependency once in `[workspace.dependencies]`
with `default-features = false`; members use `workspace = true` plus the
features they need. Single crates: `default-features = false` and an
explicit feature list in `[dependencies]`. See
https://github.com/johnthagen/min-sized-rust for the rest of the size
toolkit.

## A feature where a crate would do

**Looks like:** `features = ["tokio"]`, `["async-std"]`, `["postgres"]`
switching whole integrations inside one crate, with `#[cfg(feature)]`
sprinkled through the modules.

**Why:** features are additive, global across the dependency graph and
not testable in isolation; a crate per integration (`foo-tokio`,
`foo-postgres`) has its own tests, docs, semver and dependency set.

**Instead:** a companion crate, or a refactor that makes the integration a
trait implemented in a separate crate. Keep features for small optional
trait impls such as `serde`.

## Spelling the type name, importing traits by name

**Looks like:** `impl Frame { fn new() -> Frame { Frame { .. } } }`;
`use std::io::Write;` when only `write_all` is called.

**Why:** a rename touches every line; a named trait import pollutes the
namespace and collides with local names. Clippy has `use_self` and
`unused_trait_names` for both.

**Instead:** `Self` everywhere inside the `impl`; `use std::io::Write as _;`.

## Hidden allocation and fallibility behind a prettier signature

**Looks like:**

```rust
pub fn new(name: &str) -> Result<Self, Error> {
    let name = CString::new(name)?;   // allocation + possible error, invisible to the caller
    // ...
}
```

Also `fn open(path: &str)` that builds a `PathBuf`, setters taking `&str`
that allocate a `String` per call in a hot loop.

**Why:** the signature promises a cheap borrow and delivers an allocation
and a failure mode the caller cannot avoid or amortise. The C side wants a
C string; the Rust type for that already exists.

**Instead:** take the boundary type: `&CStr` (or `impl AsRef<CStr>`),
`&Path`, the validated newtype. The caller converts once, sees the error,
and can reuse the buffer. Offer a `&str` convenience only as a separately
named method (`new_str`) that documents the allocation.

## `std::` paths for `core` and `alloc` items

**Looks like:** `use std::fmt;`, `use std::error::Error;`, `std::ffi::CStr`,
`std::collections::BTreeMap` in a crate that never touches the OS.

**Why:** the item lives in `core` or `alloc`; naming it through `std` ties
the crate to `std` for nothing and makes a later `no_std` port a diff of
every import. See the table in `patterns.md`.

**Instead:** `core::fmt`, `core::error::Error`, `core::ffi::CStr`,
`alloc::collections::BTreeMap`, with `extern crate alloc;` where needed.
`std` only for `fs`, `net`, `thread`, `env`, `process`, `io`, `HashMap`,
`Mutex`, `SystemTime`.

## A module where a crate would do, and `pub(crate)` everywhere

**Looks like:** `mod codec;` with 800 lines inside the application crate;
`pub(crate) fn helper()` on every function in a module that only its
parent and siblings use.

**Why:** a module recompiles with its crate and shares its dependency set;
a crate compiles in parallel and has a reviewed API. `pub(crate)` on
helpers is noise: child modules already see their ancestors, so the
helper belongs in the parent, private.

**Instead:** new capability, new crate under `crates/`. Private by
default; `pub(super)` for a child-to-parent export; `pub` only for the API
re-exported at the root.

## Macro abuse [Item 28]

**Looks like:** a proc-macro crate added for one derive; `macro_rules!`
generating whole modules, match arms or functions that a trait, generic
or const generic would express; macros to save typing a few lines.

**Why:** macros are opaque to rust-analyzer and to readers, every
dependent waits on the proc-macro crate, and errors point into expansions.

**Instead:** traits with blanket impls, generics, const generics,
`impl Trait`. A single private `macro_rules!` for "implement this trait for
each of these types" is the accepted exception. Derives from existing
dependencies are fine to consume.

## Tests that test the standard library

**Looks like:** `assert!(u16::try_from(70_000usize).is_err())`,
`assert!(CString::new("a\0b").is_err())`, a test that a `HashMap` returns
what was inserted, a test that `serde_json` round-trips a struct with only
derives on it.

**Why:** they pass regardless of your code and cost reading and CI time;
`std` and the dependency have their own suites.

**Instead:** test the behaviour this crate adds: the wrapper returns the
typed error on a C failure, the parser rejects trailing bytes, the
conversion maps the fields. If you cannot name what of yours a test
exercises, delete it.

## Feature misuse [Item 26]

- Non-additive features (`feature = "sync"` changing a type's shape).
- Negative features (`no-std`, `no-logging`): features can only be turned on
  across the dependency graph.
- Default features that pull in heavy optional deps for every user.
- Forgetting that `--all-features` and `--no-default-features` must both
  compile.
