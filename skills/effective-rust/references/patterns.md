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

- `pub(crate)` by default; `pub` is a promise.
- One concept per module; `foo.rs` plus `foo/` subdirectory (not `foo/mod.rs`).
- Prelude modules only for traits users must import to call methods.
- Binaries: thin `main.rs` calling into `lib.rs`, so the logic is testable.
- Re-export dependency types that appear in your public API
  (`pub use bytes::Bytes;`).
- Features are additive and named after what they add (`serde`, `tokio`).

## Testing shapes [Item 30]

- Unit tests in a `#[cfg(test)] mod tests` beside private code.
- Integration tests in `tests/` for the public API, one file per area.
- Doctests on every public item with an example; they are the API's contract.
- `#[should_panic(expected = "..")]` with the message, never bare.
- Property tests (`proptest`) for parsers, codecs, arithmetic; snapshot tests
  (`insta`) for rendered output; fuzzing (`cargo fuzz`) for anything that
  parses untrusted bytes.
