# Resources

Every link was checked on 2026-10-31. Read the "When" column as the trigger.
This file is a locator: it tells you where to read, it does not reproduce
the sources (see the licence note at the end).

## Books and guides

| Resource | URL | When |
|---|---|---|
| Effective Rust (David Drysdale, 35 items) | https://effective-rust.com/ | The backbone of this skill; item numbers below refer to it |
| Rust API Guidelines, checklist | https://rust-lang.github.io/api-guidelines/checklist.html | Any `pub` item in a library |
| Rust Design Patterns (idioms, patterns, anti-patterns) | https://rust-unofficial.github.io/patterns/ | Choosing a pattern; reviewing for anti-patterns |
| The Rust Programming Language (the Book) | https://doc.rust-lang.org/book/ | Fundamentals; ch. 10 generics and lifetimes, ch. 11 testing, the Unsafe Rust chapter |
| The Rust Reference | https://doc.rust-lang.org/reference/ | Exact language semantics, attributes, items |
| The Rustonomicon | https://doc.rust-lang.org/nomicon/ | Before any `unsafe` |
| Unsafe Code Guidelines | https://rust-lang.github.io/unsafe-code-guidelines/ | Aliasing, validity, layout rules for `unsafe` |
| Rust Atomics and Locks (Mara Bos) | https://marabos.nl/atomics/ | Atomics, memory ordering, building sync primitives |
| Asynchronous Programming in Rust | https://rust-lang.github.io/async-book/ | Async fundamentals |
| Tokio tutorial | https://tokio.rs/tokio/tutorial | Any tokio code |
| Async: What is blocking? (Alice Ryhl) | https://ryhl.io/blog/async-what-is-blocking/ | Deciding `spawn_blocking` vs async |
| Actors with Tokio (Alice Ryhl) | https://ryhl.io/blog/actors-with-tokio/ | Replacing `Arc<Mutex<_>>` with an owning task |
| `tokio::select!` docs, cancellation safety | https://docs.rs/tokio/latest/tokio/macro.select.html | Before every `select!` |
| The Rust Performance Book | https://nnethercote.github.io/perf-book/ | Before optimising |
| Error Handling in Rust (nrc) | https://nrc.github.io/error-docs/ | Error strategy |
| Error Handling in Rust (BurntSushi) | https://blog.burntsushi.net/rust-error-handling/ | Deep dive on error design |
| The Little Book of Rust Macros | https://veykril.github.io/tlborm/ | Writing `macro_rules!` |
| Learn Rust With Entirely Too Many Linked Lists | https://rust-unofficial.github.io/too-many-lists/ | `Rc`, `RefCell`, raw pointers in data structures |
| Rust Cookbook | https://rust-lang-nursery.github.io/rust-cookbook/ | Quick recipe for common tasks with the right crate |
| Comprehensive Rust (Google) | https://google.github.io/comprehensive-rust/ | Course-style refresher incl. async, bare-metal, concurrency |
| Secure Rust Guidelines (ANSSI) | https://anssi-fr.github.io/rust-guide/ | Security-sensitive code |
| The Embedded Rust Book | https://docs.rust-embedded.org/book/ | `no_std`, embedded targets |
| Rust by Example | https://doc.rust-lang.org/rust-by-example/ | Syntax reminders |
| Rust Language Cheat Sheet | https://cheats.rs/ | One-page lookup of syntax, traits, idioms |

## Official references

| Resource | URL | When |
|---|---|---|
| Standard library docs | https://doc.rust-lang.org/std/ | Before writing any helper |
| `std::collections` guide | https://doc.rust-lang.org/std/collections/index.html | Choosing a collection |
| `std::iter` | https://doc.rust-lang.org/std/iter/index.html | Iterator adapters |
| `std::sync` | https://doc.rust-lang.org/std/sync/index.html | Locks, atomics, channels, `LazyLock` |
| `std::error` | https://doc.rust-lang.org/std/error/index.html | `Error` trait contract |
| `std::pin` | https://doc.rust-lang.org/std/pin/index.html | Pinning, self-referential futures |
| The Cargo Book | https://doc.rust-lang.org/cargo/ | Anything in `Cargo.toml` |
| Cargo: SemVer compatibility | https://doc.rust-lang.org/cargo/reference/semver.html | What counts as a breaking change |
| Cargo: features | https://doc.rust-lang.org/cargo/reference/features.html | Adding or changing features |
| Cargo: workspaces | https://doc.rust-lang.org/cargo/reference/workspaces.html | Multi-crate repos |
| Cargo: `[lints]` table | https://doc.rust-lang.org/cargo/reference/lints.html | Configuring clippy/rustc lints per crate or workspace |
| Cargo: `rust-version` (MSRV) | https://doc.rust-lang.org/cargo/reference/rust-version.html | MSRV policy, resolver behaviour |
| Cargo: manifest | https://doc.rust-lang.org/cargo/reference/manifest.html | Metadata fields, targets |
| Cargo: `cargo info` | https://doc.rust-lang.org/cargo/commands/cargo-info.html | Inspect a crate before adding it |
| The rustdoc Book | https://doc.rust-lang.org/rustdoc/ | Writing docs, doctests |
| rustdoc: What to include | https://doc.rust-lang.org/rustdoc/write-documentation/what-to-include.html | Doc sections |
| rustdoc: Documentation tests | https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html | Doctest attributes (`no_run`, `should_panic`, `ignore`) |
| RFC 1574: API documentation conventions | https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html | Doc comment shape |
| RFC 430: naming conventions | https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html | Casing, `as_`/`to_`/`into_` |
| Clippy lint list | https://rust-lang.github.io/rust-clippy/master/index.html | Understanding a lint before touching it |
| The Clippy Book | https://doc.rust-lang.org/clippy/ | Configuration, `clippy.toml`, lint groups |
| Clippy usage and lint groups | https://doc.rust-lang.org/clippy/usage.html | What `all`, `pedantic`, `nursery`, `restriction`, `cargo` contain and how to enable them |
| Clippy lint configuration | https://doc.rust-lang.org/clippy/lint_configuration.html | Every `clippy.toml` key (`msrv`, `allow-unwrap-in-tests`, thresholds) |
| rustc lint groups | https://doc.rust-lang.org/rustc/lints/groups.html | `warnings`, `unused`, `rust-2018-idioms`, `future-incompatible` |
| rustc lints | https://doc.rust-lang.org/rustc/lints/index.html | Compiler lint groups and levels |
| Diagnostic attributes (`#[expect]`, `#[diagnostic::..]`) | https://doc.rust-lang.org/reference/attributes/diagnostics.html | Lint attributes |
| Rust Error Codes Index | https://doc.rust-lang.org/error_codes/ | `rustc --explain E0xxx` online |
| Edition Guide (incl. 2024) | https://doc.rust-lang.org/edition-guide/ | Migrations, edition-specific semantics |
| Rust release notes | https://doc.rust-lang.org/stable/releases.html | Checking since which version an API is stable |
| rustup book | https://rust-lang.github.io/rustup/ | Toolchains, components, `rust-toolchain.toml` |
| Miri | https://github.com/rust-lang/miri | Running tests under the UB checker |
| Rust Fuzz Book | https://rust-fuzz.github.io/book/ | `cargo fuzz` setup |
| proptest book | https://proptest-rs.github.io/proptest/ | Property-based tests |
| criterion book | https://bheisler.github.io/criterion.rs/book/ | Benchmarks |
| cargo-nextest | https://nexte.st/ | Faster test runner (does not run doctests) |
| min-sized-rust (johnthagen, MIT) | https://github.com/johnthagen/min-sized-rust | Shrinking binaries: profiles, strip, LTO, panic=abort, no_std; useful even when size is secondary |
| Embark Studios lint baseline | https://github.com/EmbarkStudios/rust-ecosystem/blob/main/lints.rs | A widely copied community set of clippy lint levels to compare `lints.md` against |
| insta | https://insta.rs/ | Snapshot tests |
| cargo-deny | https://embarkstudios.github.io/cargo-deny/ | Licence, advisory and duplicate checks |
| cargo-semver-checks | https://github.com/obi1kenobi/cargo-semver-checks | Detect breaking changes before release |
| bindgen user guide | https://rust-lang.github.io/rust-bindgen/ | C bindings |

## Crate discovery

| Resource | URL | When |
|---|---|---|
| blessed.rs | https://blessed.rs/crates | First stop for "which crate for X" |
| docs.rs | https://docs.rs/ | Read the exact API before calling it |
| lib.rs | https://lib.rs/ | Compare alternatives, see maintenance and download trends |
| crates.io | https://crates.io/ | Versions, owners, repository links |

## RFCs

The RFC book renders each RFC at `https://rust-lang.github.io/rfcs/<file>.html`;
the same text lives in the repository at
`https://github.com/rust-lang/rfcs/blob/main/text/<file>.md`.

| RFC | URL | When |
|---|---|---|
| 1574: API documentation conventions | https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html | Writing any doc comment: summary line, sections, examples |
| 1946: intra-rustdoc links | https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html | Linking items in docs with `[`Name`]` |
| 430: naming conventions | https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html | Casing of crates, types, functions, constants |
| 344: conventions galore | https://rust-lang.github.io/rfcs/0344-conventions-galore.html | Getter/setter names, `iter`/`iter_mut`/`into_iter`, trait naming, `_mut` suffixes |
| 199: ownership variants | https://rust-lang.github.io/rfcs/0199-ownership-variants.html | The `as_`/`to_`/`into_` prefixes and `_mut` conventions |
| 1105: API evolution | https://rust-lang.github.io/rfcs/1105-api-evolution.html | What counts as a breaking change; the basis of the Cargo SemVer chapter |
| 1122: language semver | https://rust-lang.github.io/rfcs/1122-language-semver.html | Why a toolchain update can break a build and what the project promises |
| 2008: `#[non_exhaustive]` | https://rust-lang.github.io/rfcs/2008-non-exhaustive.html | Designing enums and structs that can grow |
| 1023: rebalancing coherence | https://rust-lang.github.io/rfcs/1023-rebalancing-coherence.html | Hitting the orphan rule (E0117, E0210) or overlapping impls |
| 2126: path clarity | https://rust-lang.github.io/rfcs/2126-path-clarity.html | Module layout (`foo.rs` + `foo/`), `crate::` paths, 2018 module system |
| 2229: disjoint closure captures | https://rust-lang.github.io/rfcs/2229-capture-disjoint-fields.html | Understanding what a closure borrows since edition 2021 |
| 3389: `[lints]` manifest table | https://rust-lang.github.io/rfcs/3389-manifest-lint.html | Configuring lints in `Cargo.toml` instead of source attributes |
| 2906: workspace dependency inheritance | https://rust-lang.github.io/rfcs/2906-cargo-workspace-deduplicate.html | `[workspace.dependencies]` and `workspace = true` |
| 3501: edition 2024 | https://rust-lang.github.io/rfcs/3501-edition-2024.html | Migrating to or writing for edition 2024 |

## Effective Rust item locator

The book is CC BY-NC-ND 4.0, so this skill links to it and never reproduces
it. The table maps the item numbers used throughout this skill to the page
to open; the topic column is a two-word hint in this skill's own words, not
the book's title.

| # | Topic | URL |
|---|---|---|
| 1 | data via types | https://effective-rust.com/use-types.html |
| 2 | behaviour via types | https://effective-rust.com/use-types-2.html |
| 3 | Option/Result transforms | https://effective-rust.com/transform.html |
| 4 | error types | https://effective-rust.com/errors.html |
| 5 | conversions | https://effective-rust.com/casts.html |
| 6 | newtypes | https://effective-rust.com/newtype.html |
| 7 | builders | https://effective-rust.com/builders.html |
| 8 | references, pointers | https://effective-rust.com/references.html |
| 9 | iterators | https://effective-rust.com/iterators.html |
| 10 | std traits | https://effective-rust.com/std-traits.html |
| 11 | RAII, Drop | https://effective-rust.com/raii.html |
| 12 | generics vs trait objects | https://effective-rust.com/generics.html |
| 13 | default trait methods | https://effective-rust.com/default-impl.html |
| 14 | lifetimes | https://effective-rust.com/lifetimes.html |
| 15 | borrow checker | https://effective-rust.com/borrows.html |
| 16 | unsafe | https://effective-rust.com/unsafe.html |
| 17 | shared-state parallelism | https://effective-rust.com/deadlock.html |
| 18 | panics | https://effective-rust.com/panic.html |
| 19 | reflection | https://effective-rust.com/reflection.html |
| 20 | optimisation | https://effective-rust.com/optimize.html |
| 21 | semver | https://effective-rust.com/semver.html |
| 22 | visibility | https://effective-rust.com/visibility.html |
| 23 | wildcard imports | https://effective-rust.com/wildcard.html |
| 24 | re-exports | https://effective-rust.com/re-export.html |
| 25 | dependency graph | https://effective-rust.com/dep-graph.html |
| 26 | features | https://effective-rust.com/features.html |
| 27 | documentation | https://effective-rust.com/documentation.html |
| 28 | macros | https://effective-rust.com/macros.html |
| 29 | clippy | https://effective-rust.com/clippy.html |
| 30 | testing | https://effective-rust.com/testing.html |
| 31 | tooling | https://effective-rust.com/use-tools.html |
| 32 | CI | https://effective-rust.com/ci.html |
| 33 | no_std | https://effective-rust.com/no-std.html |
| 34 | FFI boundaries | https://effective-rust.com/ffi.html |
| 35 | bindgen | https://effective-rust.com/bindgen.html |

## Licences of the sources

This skill links to its sources and summarises them in its own words. For
the record: Effective Rust is CC BY-NC-ND 4.0; the Rust API Guidelines, the
RFCs and the official Rust books are MIT OR Apache-2.0; Rust Design Patterns
is MPL-2.0; blessed.rs declares no licence, so only crate names (facts) are
taken from it. Do not paste passages from any of them into this skill or
into code comments; link instead.
