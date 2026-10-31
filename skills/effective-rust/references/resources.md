# Resources

Every link was checked on 2026-10-31. Read the "When" column as the trigger.

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

## Effective Rust item index

Use these numbers when the skill says "Item N".

| # | Item |
|---|---|
| 1 | Use the type system to express your data structures |
| 2 | Use the type system to express common behavior |
| 3 | Prefer `Option` and `Result` transforms over explicit `match` |
| 4 | Prefer idiomatic `Error` types |
| 5 | Understand type conversions |
| 6 | Embrace the newtype pattern |
| 7 | Use builders for complex types |
| 8 | Familiarize yourself with reference and pointer types |
| 9 | Consider using iterator transforms instead of explicit loops |
| 10 | Familiarize yourself with standard traits |
| 11 | Implement the `Drop` trait for RAII patterns |
| 12 | Understand the trade-offs between generics and trait objects |
| 13 | Use default implementations to minimize required trait methods |
| 14 | Understand lifetimes |
| 15 | Understand the borrow checker |
| 16 | Avoid writing unsafe code |
| 17 | Be wary of shared-state parallelism |
| 18 | Don't panic |
| 19 | Avoid reflection |
| 20 | Avoid the temptation to over-optimize |
| 21 | Understand what semantic versioning promises |
| 22 | Minimize visibility |
| 23 | Avoid wildcard imports |
| 24 | Re-export dependencies whose types appear in your API |
| 25 | Manage your dependency graph |
| 26 | Be wary of feature creep |
| 27 | Document public interfaces |
| 28 | Use macros judiciously |
| 29 | Listen to Clippy |
| 30 | Write more than unit tests |
| 31 | Take advantage of the tooling ecosystem |
| 32 | Set up a continuous integration (CI) system |
| 33 | Consider making library code `no_std` compatible |
| 34 | Control what crosses FFI boundaries |
| 35 | Prefer bindgen to manual FFI mappings |

Item pages follow `https://effective-rust.com/<slug>.html`; the slugs are
`use-types`, `use-types-2`, `transform`, `errors`, `casts`, `newtype`,
`builders`, `references`, `iterators`, `std-traits`, `raii`, `generics`,
`default-impl`, `lifetimes`, `borrows`, `unsafe`, `deadlock`, `panic`,
`reflection`, `optimize`, `semver`, `visibility`, `wildcard`, `re-export`,
`dep-graph`, `features`, `documentation`, `macros`, `clippy`, `testing`,
`use-tools`, `ci`, `no-std`, `ffi`, `bindgen`.
