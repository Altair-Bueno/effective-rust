# API design for libraries

Read this before adding or changing any `pub` item. It condenses the Rust
API Guidelines checklist (https://rust-lang.github.io/api-guidelines/checklist.html)
and Effective Rust Items 21–27. Codes like C-CASE link to that checklist.

## Naming (RFC 430)

| Item | Convention |
|---|---|
| Crates, modules, functions, methods, locals, macros | `snake_case` |
| Types, traits, enum variants, type params | `UpperCamelCase` |
| Constants, statics | `SCREAMING_SNAKE_CASE` |
| Lifetimes | short lowercase: `'a`, `'de`, `'src` |
| Features | what they add, no `use-`/`with-` prefix (C-FEATURE) |

Conversions (C-CONV): `as_` borrowed→borrowed and free, `to_` expensive or
owned→owned, `into_` consuming. Getters are the field name without `get_`
(C-GETTER); `get` alone is for keyed lookups. Iterator methods are `iter`,
`iter_mut`, `into_iter` and the iterator types `Iter`, `IterMut`, `IntoIter`
(C-ITER, C-ITER-TY). Keep word order consistent: `ParseAddrError`,
`ParseIntError`, not `AddrParseError`.

## Interoperability

- Derive eagerly: `Debug` on everything public (C-DEBUG), plus `Clone`,
  `PartialEq`, `Eq`, `Hash`, `Ord`, `Default`, `Copy` where valid
  (C-COMMON-TRAITS).
- Conversions via `From`, `TryFrom`, `AsRef`, `AsMut` (C-CONV-TRAITS).
- Collections implement `FromIterator`, `Extend`, `IntoIterator` for `T`, `&T`
  and `&mut T` (C-COLLECT).
- Data types implement `serde::Serialize`/`Deserialize` behind a `serde`
  feature (C-SERDE).
- Types are `Send + Sync` where possible; add a compile-time assertion in
  tests so a future field does not silently remove it (C-SEND-SYNC):
  `fn assert_send<T: Send>() {}` then `assert_send::<Client>();`.
- Reader/writer parameters are `R: Read`, `W: Write` by value (C-RW-VALUE).
- Binary number types get `Hex`, `Octal`, `Binary` formatting (C-NUM-FMT).

## Predictability

- Constructors are inherent static methods: `new`, `with_capacity`,
  `from_parts` (C-CTOR). Prefer `Default` for zero-arg constructors.
- Functions with a clear receiver are methods (C-METHOD); no out-parameters
  (C-NO-OUT): return a tuple or struct.
- Only smart pointers implement `Deref` (C-DEREF); smart pointers add no
  inherent methods (C-SMART-PTR): use `Arc::clone(&x)` style associated fns.
- Operator overloads keep the operator's meaning (C-OVERLOAD).
- Conversions live on the most specific type (C-CONV-SPECIFIC).

## Flexibility

- Generic parameters minimise assumptions: `impl AsRef<Path>`,
  `impl IntoIterator<Item = T>`, `impl Read` (C-GENERIC). Do not generalise
  what has one caller type.
- Expose intermediate results to avoid duplicate work (C-INTERMEDIATE): a
  `parse` that returns position as well as value.
- Caller decides where data lives (C-CALLER-CONTROL): take `&mut [u8]` or
  `impl Write` instead of returning a fresh `Vec` when buffers are reused.
- Traits meant for `dyn` use must be object safe (C-OBJECT): no generic
  methods, no `Self: Sized` returns, provide `where Self: Sized` escapes.

## Type safety

- Newtypes for distinct meanings (C-NEWTYPE); enums instead of `bool`
  (C-CUSTOM-TYPE); `bitflags` for flag sets (C-BITFLAG); builders for
  complex construction (C-BUILDER).

## Dependability

- Validate arguments and document the rules (C-VALIDATE).
- Destructors never fail and never block; offer `close()`/`flush()` returning
  `Result` when the user must know (C-DTOR-FAIL, C-DTOR-BLOCK).

## Future proofing

- Sealed traits for traits you do not want implemented downstream (C-SEALED).
- Private fields on structs, with accessors (C-STRUCT-PRIVATE). `pub` fields
  are a promise that the representation never changes.
- `#[non_exhaustive]` on public enums and structs that may grow.
- Do not repeat derived bounds on type definitions (C-STRUCT-BOUNDS): put
  `T: Clone` on the `impl`, not on `struct Foo<T>`.
- `#[must_use]` on builders, guards, iterators, and `Result`-like types.
- `#[doc(hidden)]` for items that must be `pub` for macros but are not API
  (C-HIDDEN).

## Documentation [Item 27, C-CRATE-DOC, C-EXAMPLE, C-FAILURE, C-LINK]

- Crate-level docs (`//!` in `lib.rs`) explain purpose and show one complete
  example.
- Every public item has a one-line summary, then details, then sections in
  this order: `# Examples`, `# Errors`, `# Panics`, `# Safety`.
- Examples use `?` not `unwrap` (C-QUESTION-MARK); mark non-runnable ones
  `no_run`, never `ignore`.
- Link related items with intra-doc links: `[`Config`]`, `[`parse`]`.
- Build docs in CI with `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` so
  broken links fail.
- `Cargo.toml` carries `description`, `license`, `repository`, `keywords`,
  `categories`, `rust-version` (C-METADATA).

## SemVer [Item 21, Cargo SemVer reference]

Breaking (major, or minor while `0.x`):

- Removing or renaming any public item, field or variant.
- Adding a field to a public struct with public fields, or a variant to a
  non-`#[non_exhaustive]` enum.
- Adding a required trait method, or a bound to a public type/fn.
- Changing a function signature, including making a generic parameter more
  specific.
- Removing a trait impl or an auto trait (`Send`, `Sync`, `Unpin`) from a
  public type.
- Raising MSRV (policy varies; say so in the changelog).
- Changing a default feature set.

Not breaking: adding items, adding defaulted trait methods, loosening
bounds, adding variants to `#[non_exhaustive]` enums, adding fields to
structs with private fields. Run `cargo semver-checks` before a release and
read the Cargo reference for the long tail of cases.

## Features [Item 26, Cargo features reference]

- Additive only: enabling a feature never removes or changes behaviour.
- Named after what they add: `serde`, `tokio`, `std` (for `no_std` crates the
  convention is a `std` feature that is on by default).
- Optional dependencies become implicit features; use `dep:` syntax to hide
  them and expose a named feature instead.
- Test the matrix: `--no-default-features`, default, `--all-features`
  (`cargo hack --feature-powerset` when the set is small).
- Re-export dependency types that appear in your API and bump your major
  version when that dependency bumps its major (Item 24).

## Visibility [Item 22]

`pub(crate)` unless an external user needs it. Module paths that users write
should be short: re-export the handful of main types at the crate root
(`pub use config::Config;`) and keep the module tree private.
