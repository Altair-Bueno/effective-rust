# Patterns

Each entry says what the pattern buys you and shows the minimal shape. Sources:
Effective Rust (items in brackets), Rust Design Patterns, Rust API Guidelines.

## Types

### Newtype [Item 6, C-NEWTYPE]

A wrapper struct turns a bare `u64` or `String` into a value the compiler can
tell apart from every other `u64`, and gives you one place to enforce
invariants.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Port(u16);

impl Port {
    pub fn new(raw: u16) -> Result<Self, InvalidPort> {
        if raw == 0 { return Err(InvalidPort(raw)); }
        Ok(Self(raw))
    }
    pub fn get(self) -> u16 { self.0 }
}

impl TryFrom<u16> for Port {
    type Error = InvalidPort;
    fn try_from(raw: u16) -> Result<Self, Self::Error> { Self::new(raw) }
}
```

Keep the field private. Add `Display`/`FromStr` when the value is parsed or
printed. Do not `impl Deref` to expose the inner type's methods; add the two or
three methods you need.

### Enums over flags [Item 1, C-CUSTOM-TYPE]

Replace `bool` and `Option<bool>` parameters and fields with an enum whose
variants name the meaning: `Overwrite::{Yes, No}`, `Mode::{Append, Truncate}`.
Callers read `open(path, Mode::Append)` without looking up the signature.

### Typestate

Encode lifecycle states as type parameters so the compiler rejects calls that
are only valid in another state. Use it for protocol handshakes, builders with
required fields, and resources with open/closed phases.

```rust
pub struct Connection<S> { inner: Inner, _state: PhantomData<S> }
pub struct Disconnected; pub struct Connected;

impl Connection<Disconnected> {
    pub fn connect(self) -> Result<Connection<Connected>, ConnectError> { .. }
}
impl Connection<Connected> {
    pub fn send(&mut self, msg: &Message) -> Result<(), SendError> { .. }
}
```

### Builder [Item 7, C-BUILDER]

Use a builder when a constructor would need more than about three optional
values, or when some fields are required and some are not. Consuming builders
(`self` by value) chain naturally and avoid cloning; borrowed builders
(`&mut self`) suit loops. If the project already depends on `bon` or
`typed-builder`, use that instead of hand-writing one.

```rust
#[derive(Debug, Default)]
pub struct ClientBuilder { timeout: Option<Duration>, retries: u8 }

impl ClientBuilder {
    pub fn timeout(mut self, t: Duration) -> Self { self.timeout = Some(t); self }
    pub fn retries(mut self, n: u8) -> Self { self.retries = n; self }
    pub fn build(self) -> Client { .. }
}
```

### Default and struct update syntax [C-COMMON-TRAITS]

`#[derive(Default)]` plus `Config { port: 8080, ..Default::default() }` beats
a constructor with every field. Implement `Default` by hand only when a
derived default would be a wrong value.

### Derive the common traits eagerly [Item 10, C-COMMON-TRAITS]

Public types derive `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`,
`Ord`, `Default` where they make sense, and `Copy` for small plain data.
Missing derives are a breaking change to add later for downstream users who
already wrote their own wrapper.

### Conversions through the standard traits [Item 5, C-CONV-TRAITS]

`From`/`Into` for infallible conversions, `TryFrom`/`TryInto` for fallible,
`AsRef`/`AsMut` for cheap reference conversions, `FromStr` with a matching
`Display`. Never `as` between numeric types unless the conversion is
provably lossless or you intend the wrap and say so with `wrapping_*`.

When you catch yourself writing `fn header_of(f: &Frame) -> Header`,
`fn parse_frame(bytes: &[u8]) -> Result<Frame, E>` or `Frame::from_bytes`,
the function is a conversion: one input, one output type, a body that moves
fields across. Express it as the trait so every caller gets `?`, `.into()`,
`.try_into()`, `.parse()` and `collect::<Result<Vec<_>, _>>()` for free,
and so the compiler can find every conversion site.

Assembling a value from several independent parts is construction, not
conversion. `Frame::new(header, payload) -> Result<Frame, E>` is the right
shape for that; `TryFrom<(Header, Payload)>` would only hide a constructor
behind a tuple. Use the trait when the pair is a real unit (a type of its
own), the constructor otherwise.

```rust
impl Frame {
    pub fn new(header: Header, payload: Payload) -> Result<Self, FrameError> {
        let len = u16::try_from(payload.body.len()).map_err(|_| FrameError::TooLong)?;
        Ok(Self { version: header.version, flags: header.flags, len, body: payload.body })
    }
}

impl From<&Frame> for Header {
    fn from(f: &Frame) -> Self { Self { version: f.version, flags: f.flags } }
}

impl TryFrom<&[u8]> for Frame { type Error = FrameError; /* parse */ }
```

Keep an inherent alias (`frame.header()`) only when discoverability
matters, and implement it by calling the trait so there is one body.

### Sealed traits and `#[non_exhaustive]` [C-SEALED, C-STRUCT-PRIVATE]

```rust
mod private { pub trait Sealed {} }
pub trait Codec: private::Sealed { .. }
impl private::Sealed for Json {}
```

Sealing lets you add required methods later. `#[non_exhaustive]` on enums and
structs lets you add variants and fields without a breaking change. Use both
on anything that a library exposes and expects to grow.

## Ownership and borrowing [Items 8, 14, 15]

### Borrowed parameter types

Accept `&str` not `&String`, `&[T]` not `&Vec<T>`, `&Path` not `&PathBuf`,
`&T` not `&Box<T>`. Every owned type derefs to the borrowed one, so callers
lose nothing and you accept more inputs. Take ownership (`String`, `Vec<T>`)
only when you store the value.

### `impl AsRef` and `impl Into` at the boundary

`fn open(path: impl AsRef<Path>)` is the std convention for paths.
`impl Into<String>` is worth it only for setters that store the string;
elsewhere it hides allocation and hurts inference.

### Move out without cloning

```rust
let items = std::mem::take(&mut self.items);        // leaves Vec::new()
let old = std::mem::replace(&mut self.state, State::Closed);
if let Some(conn) = self.conn.take() { conn.close(); }
```

These are the answer when the borrow checker rejects moving a field out of
`&mut self`.

### Split borrows

Borrow disjoint fields directly (`let (a, b) = (&mut self.a, &mut self.b)`),
use `split_at_mut`, `get_disjoint_mut`, `iter_mut`, or restructure so the
method takes the two fields it needs instead of `&mut self`.

### Lifetimes in signatures

Name a lifetime only when two references in the signature must be tied
together. Elision handles the rest. A struct holding `&'a T` is usually a
view type; if it ends up stored long-term, store an owned value or `Rc`/`Arc`.

### `Cow<'_, str>` for borrow-or-own returns

When a function returns the input unchanged most of the time and an allocated
version sometimes, return `Cow`. Callers that need `String` call
`into_owned()`.

### Shared immutable data

`Arc<str>`, `Arc<[T]>`, `Box<str>` and `Box<[T]>` are smaller than their
growable counterparts and signal "this will not change". Clone an `Arc` once
outside a loop, not inside it.

## Control flow [Items 3, 9]

### Iterator chains

```rust
let totals: BTreeMap<_, u64> = orders
    .iter()
    .filter(|o| o.status == Status::Paid)
    .map(|o| (o.customer, o.total))
    .fold(BTreeMap::new(), |mut m, (c, t)| { *m.entry(c).or_default() += t; m });
```

Iterators are lazy, bounds-checked once, and fuse with `collect` into any
`FromIterator` target. Reach for a `for` loop when the body has side effects
or early returns that read badly as a chain.

### Fail fast over a sequence

```rust
let ports: Vec<Port> = raw.iter().map(|r| Port::new(*r)).collect::<Result<_, _>>()?;
```

### `?` with `From` [Item 4]

Give your error type `impl From<io::Error>` (or `#[from]` with `thiserror`)
and plumbing becomes `let f = File::open(p)?;`. Add context at the layer that
knows it, not at every call.

### `let else`, `matches!`, let chains

```rust
let Some(header) = lines.next() else { return Err(ParseError::Empty) };
if matches!(tok, Token::Comma | Token::Semi) { .. }
if let Some(x) = a && x > 3 { .. }   // let chains, edition 2024
```

### `impl Trait` in return position

Return `impl Iterator<Item = &T> + '_` or `impl Future<Output = ..>` to hide
the concrete adapter type. Switch to `Box<dyn ..>` only when branches return
different types or the function is a trait method that must be object safe.

## Traits and generics [Items 2, 12, 13]

### Generic by default, `dyn` on purpose

Generics monomorphise: fast, inlinable, inference-friendly. Trait objects:
one code path, heterogeneous collections, smaller binaries, dynamic plugins.
A `Vec<Box<dyn Shape>>` is right; a `fn area(s: &dyn Shape)` that is only
ever called with one type is not.

### Enum instead of trait objects for a closed set

When the set of variants is known and owned by you, an `enum` with a `match`
is simpler, faster and exhaustively checked. Introduce a trait when downstream
crates must add implementations.

### Default methods minimise required ones [Item 13]

Require the one or two primitive methods; provide everything else as default
methods built on them, as `Iterator` and `Read` do.

### Extension traits

```rust
pub trait StrExt { fn is_blank(&self) -> bool; }
impl StrExt for str { fn is_blank(&self) -> bool { self.trim().is_empty() } }
```

Adds methods to foreign types without a wrapper struct. Name them `XxxExt`.

### Avoid stacking bounds: name them with a trait

```rust
pub trait Key: Eq + Hash + Clone + Send + Sync + 'static {}
impl<T: Eq + Hash + Clone + Send + Sync + 'static> Key for T {}
```

## Resources and lifecycle [Item 11]

### RAII guards

Anything that must be undone (a lock, a temp file, a transaction, a changed
setting) is released in `Drop`. Guards make early returns and `?` safe. A
`Drop` must never panic and must never block; offer an explicit `close()`
returning `Result` when failure matters.

### Lazy globals

```rust
static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+$").unwrap());
```

`OnceLock` when initialisation needs a runtime value. Both are `Sync` and
need no crate.

## Concurrency [Item 17]

### Scoped threads

`std::thread::scope(|s| { s.spawn(|| work(&data)); })` borrows locals and
joins automatically; no `Arc`, no `'static`.

### Message passing and owning tasks

Give mutable state to one thread or task and talk to it over channels
(`mpsc`, `crossbeam-channel`, `tokio::sync::mpsc`). Locks are for short,
non-blocking critical sections. See `async.md` for the actor shape.

### Atomics with explicit ordering

`Relaxed` for counters, `Acquire`/`Release` pairs for publishing data,
`SeqCst` only when you can say why. Read Rust Atomics and Locks before going
beyond counters.

## Modules and crates [Items 22–26]

### Crates are the unit of design and of compilation

A workspace of small crates compiles in parallel, recompiles only what
changed, and forces an explicit API at every boundary. When a new module
would hold a cohesive capability (a codec, a client, a parser, a domain
model), make it a crate under `crates/` and add it to the workspace
members. Modules are for code that only makes sense inside one crate.

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
edition = "2021"
rust-version = "1.81"

[workspace.dependencies]
telemetry-codec = { path = "crates/telemetry-codec" }
```

### Visibility: private by default, children see their ancestors

Rust privacy is lexical: an item is visible in the module that declares it
and in every descendant. So the parent module is the natural home for
helpers that several child modules share, with no visibility modifier at
all.

| Modifier | Use when |
|---|---|
| (none) | Default. The item is used here or by child modules |
| `pub(super)` | A child must expose one item to its parent |
| `pub(in crate::x)` | A deeper subtree needs it; rare |
| `pub(crate)` | Genuinely used across unrelated modules of the crate; a smell when frequent |
| `pub` | Part of the crate API, re-exported from `lib.rs` |

Consumers should see a handful of types and traits at the crate root; the
module tree itself stays private (`mod codec;` plus `pub use codec::Encode;`).

### `core` and `alloc` first

Import from `core` and `alloc` when the item exists there and from `std`
only for OS facilities. Nothing changes for `std` users, and a crate written
this way becomes `no_std` with a `#![cfg_attr(not(feature = "std"), no_std)]`
line instead of a rewrite.

| Prefer | Instead of |
|---|---|
| `core::fmt`, `core::error::Error`, `core::str::FromStr`, `core::num::NonZero`, `core::ffi::{CStr, c_char}`, `core::cell`, `core::mem`, `core::ops`, `core::iter` | the same paths under `std::` |
| `alloc::vec::Vec`, `alloc::string::String`, `alloc::boxed::Box`, `alloc::sync::Arc`, `alloc::collections::BTreeMap`, `alloc::ffi::CString`, `alloc::borrow::Cow` | the same paths under `std::` |
| `std::{fs, net, thread, env, process, io, time::SystemTime, collections::HashMap, sync::Mutex}` | these have no `core`/`alloc` home |

In a crate without `#![no_std]`, `extern crate alloc;` is still needed to
name `alloc::` paths.

### The rest

- One concept per module; `foo.rs` plus `foo/` subdirectory (not `foo/mod.rs`).
- Prelude modules only for traits users must import to call methods.
- Binaries: thin `main.rs` calling into `lib.rs`, so the logic is testable.
- Re-export dependency types that appear in your public API
  (`pub use bytes::Bytes;`).
- Features are additive and named after what they add (`serde`, `tokio`).
  Before adding one for optional behaviour, consider a companion crate
  (`foo-tokio`, `foo-serde`) or a refactor: a crate has its own tests, docs
  and semver, a feature is a global switch nobody can test in isolation.
  Keep features for small optional trait impls; use crates for whole
  integrations (a runtime, a backend, a wire format).
- Dependencies live once in `[workspace.dependencies]` with
  `default-features = false`; each crate adds `workspace = true` and the
  features it actually uses. A root that leaves defaults on cannot be
  overridden by members, so the root must be the strict one. Single crates
  apply the same rule in `[dependencies]`.

### Code conventions

- `Self` instead of the type name inside `impl` blocks, in return types,
  constructors and patterns (`Self { .. }`, `Self::Variant`). Renames become
  one-line diffs. Lint: `clippy::use_self`.
- Traits imported only for their methods come in as `use std::io::Write as _;`.
  The name stays out of scope, no collision with a local `Write`, and the
  import reads as "I need the methods". Lint: `clippy::unused_trait_names`.
- `core::` and `alloc::` paths; `foo.rs` plus `foo/` rather than `mod.rs`;
  `Arc::clone(&x)` rather than `x.clone()` on reference-counted pointers.

## Macros [Item 28]

Macros defeat the LSP, hide control flow and slow compilation, so they come
after traits, generics, blanket impls and const generics. The one case a
`macro_rules!` earns its place is repetition the type system cannot
express, such as implementing a trait for every integer type:

```rust
macro_rules! impl_encode_int {
    (20 20 12 61 79 80 81 701 33 98 100 204 250 395 398 399 400 702:ty),* 20 20 12 61 79 80 81 701 33 98 100 204 250 395 398 399 400 702,)?) => {20 20 12 61 79 80 81 701 33 98 100 204 250 395 398 399 400 702
        impl Encode for  {
            fn encode(&self, out: &mut Vec<u8>) { out.extend_from_slice(&self.to_be_bytes()); }
        }
    )*};
}
impl_encode_int!(u8, u16, u32, u64, i8, i16, i32, i64);
```

Keep the macro private to its module, keep its body a plain `impl`, and
expand it with `cargo expand` when debugging. Do not add a proc-macro crate
to a project: it adds a compilation unit that every dependent waits on and
code that nobody can jump into. Consuming derives from existing
dependencies (`serde`, `thiserror`, `clap`) is fine.

## Testing shapes [Item 30]

- Unit tests in a `#[cfg(test)] mod tests` beside private code.
- Integration tests in `tests/` for the public API, one file per area.
- Doctests on every public item with an example; they are the API's contract.
- `#[should_panic(expected = "..")]` with the message, never bare.
- Property tests (`proptest`) for parsers, codecs, arithmetic; snapshot tests
  (`insta`) for rendered output; fuzzing (`cargo fuzz`) for anything that
  parses untrusted bytes.
- Every test has this crate as its subject. Asserting that `u16::try_from`
  rejects large values, that `CString::new` rejects an interior NUL, or that
  `HashMap` returns what was inserted tests `std`, which has its own suite.
  Test the behaviour you built on top: the wrapper, the parser, the
  conversion, the error variant.
