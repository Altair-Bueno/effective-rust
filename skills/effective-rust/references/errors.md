# Error handling

Sources: Effective Rust Item 4 and 18, API Guidelines C-GOOD-ERR, nrc's Error
Docs, BurntSushi's "Error Handling in Rust", `std::error` docs.

## Decide by crate kind

| Crate kind | Error type | Why |
|---|---|---|
| Library | One enum per module or per fallible operation, derived with `thiserror` | Callers match on variants; `source()` chain preserved; no allocation per error |
| Binary / service | `anyhow::Result<T>` (or `color-eyre`) with `.context()` | Nobody matches on these; they get logged or shown |
| Library wrapping many sub-errors | `struct Error { kind: ErrorKind, source: Option<Box<dyn Error + Send + Sync>> }` with `#[non_exhaustive] enum ErrorKind` | Lets you add variants and inner sources without breaking callers |

Never put `anyhow::Error` in a library's public signature; it forces the
dependency on every user and removes matchability.

## The `Error` trait contract

An error type is `Debug + Display + Error + Send + Sync + 'static`. `Display`
is lowercase, has no trailing period, and does not repeat its source's
message (the chain prints it). `source()` returns the underlying cause.
`Debug` is derived. Error types for public APIs also get `#[non_exhaustive]`.

## Library shape with `thiserror`

```rust
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ConfigError {
    #[error("cannot read {path}")]
    Read { path: PathBuf, #[source] source: std::io::Error },

    #[error("line {line}: expected `key = value`")]
    Syntax { line: usize },

    #[error("line {line}: duplicate key `{key}`")]
    DuplicateKey { line: usize, key: String },

    #[error("line {line}: invalid port")]
    Port { line: usize, #[source] source: std::num::ParseIntError },
}
```

- `#[source]` (or a field literally named `source`) keeps the chain.
- `#[from]` generates `From<Inner>` so `?` works, but it also makes
  the inner type part of your API; use it only for errors that need no extra
  context (`io::Error` usually needs the path, so prefer `#[source]` and
  `map_err`).
- Add the context the caller cannot reconstruct: paths, line numbers, keys,
  identifiers. Error messages must let a user find the problem without a
  debugger.

## Application shape with `anyhow`

```rust
use anyhow::{Context, bail, ensure};

fn main() -> anyhow::Result<()> {
    let cfg = load(&path).with_context(|| format!("loading config {}", path.display()))?;
    ensure!(cfg.workers > 0, "workers must be positive");
    if cfg.dry_run { bail!("dry run requested"); }
    Ok(())
}
```

- `with_context(|| ..)` when building the message allocates; `context("..")`
  for static text.
- Returning `anyhow::Result<()>` from `main` prints the whole chain on exit.
- Enable the `backtrace` feature (or `RUST_BACKTRACE=1`) for unexplained
  failures.

## Panics versus Results [Item 18]

| Situation | Do |
|---|---|
| Bad input, missing file, network, parse failure | `Result` |
| Violated internal invariant (a bug) | `panic!`/`expect("<why impossible>")`, `debug_assert!` |
| Prototype or test code | `unwrap()` is fine; keep it out of `src/` of libraries |
| Infallible by construction (`"literal".parse::<i32>()`) | `expect("literal is valid")` with the reason |

`expect` messages describe the expectation ("env var HOME is set"), not the
failure. Annotate functions that panic on caller misuse with
`#[track_caller]` so the panic location points at the caller.

## Conversions and layering

- Convert at module boundaries: the `net` module returns `NetError`, the
  `app` layer maps it into `AppError::Net`. Do not leak `reqwest::Error` from
  a crate whose users never see `reqwest`.
- `?` converts through `From`. One `From` per inner type; if two variants
  wrap the same inner type you need `map_err` with the right variant.
- `Box<dyn Error + Send + Sync + 'static>` is acceptable in application
  code and in `main`, not in library APIs.
- `impl From<ConfigError> for io::Error` is a smell: errors flow upward, not
  into std types.

## Documenting errors [C-FAILURE]

Every fallible public function has an `# Errors` section listing the variants
it can return and when. Functions that can panic have `# Panics`.

```rust
/// Parses `text` into a [`Config`].
///
/// # Errors
///
/// Returns [`ConfigError::Syntax`] for a line without `=` and
/// [`ConfigError::DuplicateKey`] when a key repeats.
pub fn parse(text: &str) -> Result<Config, ConfigError> { .. }
```

## Testing errors

```rust
let err = parse("port = x").unwrap_err();
assert!(matches!(err, ConfigError::Port { line: 1, .. }), "got {err:?}");
assert_eq!(err.to_string(), "line 1: invalid port");
```

Test the variant, not just `is_err()`. Test the `Display` text when users see
it. For `anyhow` chains, assert on `err.chain().any(|c| ..)` or
`downcast_ref::<ConfigError>()`.

## Checklist before you commit an error type

- [ ] Enum variants carry the context a caller needs (ids, paths, positions).
- [ ] `source()` is wired for wrapped errors.
- [ ] `Display` messages are lowercase, no trailing period, no source text.
- [ ] `#[non_exhaustive]` on public error enums.
- [ ] `Send + Sync + 'static` holds (no `Rc`, no `RefCell`, no borrowed data).
- [ ] No `unwrap`/`expect` on recoverable paths in `src/`.
- [ ] `# Errors` section on every fallible public fn.
