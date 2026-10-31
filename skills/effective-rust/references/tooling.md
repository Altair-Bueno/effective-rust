# Tooling and workflow

Commands and configuration the skill expects you to use. The project's CI
workflow overrides anything here.

## The inner loop

```sh
cargo check --all-targets                 # seconds; use while editing
cargo clippy --all-targets --all-features # before every "done"
cargo test                                # unit + integration + doctests
cargo nextest run && cargo test --doc     # if the project uses nextest
cargo fmt                                 # or cargo +nightly fmt (see below)
cargo doc --no-deps                       # when public docs changed
```

- `--all-targets` includes tests, benches and examples; clippy on `src/`
  only misses half the code.
- nextest does not run doctests; always pair it with `cargo test --doc`.
- Check `rustfmt.toml`: options like `imports_granularity`, `wrap_comments`,
  `group_imports` are nightly-only and the project will say
  `cargo +nightly fmt`. Do not reformat the repo with a different toolchain.
- Scope to one crate with `-p <name>` while iterating; finish with
  `--workspace`.
- `cargo build --timings` and `cargo check` instead of `cargo build` when
  you do not need the binary.

## Lint configuration

Put lint levels in `Cargo.toml`, not in source attributes, so they apply to
every target and are visible in one place.

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
unused_qualifications = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
module_name_repetitions = "allow"
unwrap_used = "warn"
expect_used = "warn"
dbg_macro = "warn"
todo = "warn"
print_stdout = "warn"   # libraries

# per crate:
[lints]
workspace = true
```

- Groups: `clippy::all` (default, correctness + suspicious + style + complexity
  + perf), `clippy::pedantic` (opinionated, worth enabling with a few allows),
  `clippy::nursery` (unstable, do not deny), `clippy::restriction`
  (opt-in single lints only, never the whole group), `clippy::cargo`.
- `clippy.toml` holds `msrv = "1.xx"` so clippy does not suggest APIs newer
  than the MSRV, plus thresholds such as `too-many-arguments-threshold`.
- `-D warnings` belongs on the CI command line, not in source.
- Prefer `#[expect(lint, reason = "..")]` over `#[allow]`; it errors once the
  lint no longer fires.
- Learn what a lint protects against before touching it:
  https://rust-lang.github.io/rust-clippy/master/index.html.
- `rustc --explain E0502` explains any compiler error code.

## Toolchain and MSRV

- `rust-toolchain.toml` pins the channel; respect it. Never run
  `rustup default` in a project.
- `rust-version` in `Cargo.toml` is the MSRV. With resolver 3 (edition 2024)
  cargo picks dependency versions compatible with it. Check with
  `cargo msrv verify` if the tool is installed, otherwise compile with
  `cargo +1.xx.0 check`.
- Check since when an API is stable before using it:
  https://doc.rust-lang.org/stable/releases.html or the "since" badge on
  docs.rs/std.

## Dependencies

```sh
cargo info <crate>                 # description, versions, MSRV, features, licence
cargo add <crate> --no-default-features --features x   # keep the feature set explicit
cargo tree -p <crate>              # what it pulls in
cargo tree -d                      # duplicated versions in the graph
cargo update -p <crate> --dry-run  # see what a bump would do
cargo deny check                   # licences, advisories, bans (if deny.toml exists)
cargo audit                        # RustSec advisories
cargo machete                      # unused dependencies
```

- Commit `Cargo.lock` for binaries and libraries alike; the Cargo FAQ now
  recommends it for both.
- Workspace dependencies live in `[workspace.dependencies]`; crates use
  `dep = { workspace = true }`.

## Testing

| Kind | Where | Run |
|---|---|---|
| Unit | `#[cfg(test)] mod tests` in the file | `cargo test -p crate module::` |
| Integration | `tests/*.rs` (each file is a crate; share helpers via `tests/common/mod.rs`) | `cargo test --test name` |
| Doctest | `///` examples | `cargo test --doc` |
| Property | `proptest!` in unit tests | same as unit |
| Snapshot | `insta::assert_snapshot!` | `cargo insta review` |
| Benchmark | `benches/` with `criterion` or `divan` | `cargo bench` |
| Fuzz | `fuzz/fuzz_targets` | `cargo +nightly fuzz run target` |
| UB check | existing tests | `cargo +nightly miri test` |
| Coverage | existing tests | `cargo llvm-cov --all-features --workspace` |
| Feature matrix | all | `cargo hack check --feature-powerset --no-dev-deps` |

Habits:

- Bug fix: failing test first, then the fix, in the same change.
- Name tests after behaviour: `rejects_duplicate_keys`, not `test1`.
- `#[should_panic(expected = "..")]` with the message.
- Use `rstest` for parameterised cases if the project depends on it;
  otherwise a loop over a `[(input, expected); N]` array.
- Integration tests exercise the public API only; if you need a private
  function, test it in a unit test or question the API.
- Keep tests deterministic: no wall-clock, no network, no shared temp paths
  (`tempfile::tempdir()`).

## Code navigation and search

- LSP (`rust-analyzer`): hover for types, go-to-definition, references,
  rename. Prefer it to grepping for identifiers.
- `ast-grep --lang rust -p 'fn $NAME($$$ARGS) -> Result<$T, String>'` for
  structural searches (find every function returning a `String` error).
- `rg` for text; `fd` for files.
- `cargo expand <path::to::item>` to see macro output.
- `cargo doc --open` for the dependency graph's docs offline; docs.rs for
  the pinned version.

## Release hygiene

```sh
cargo semver-checks            # public API diff vs last release
cargo publish --dry-run
cargo package --list           # what ships
```

Keep a changelog; `cargo release` or `release-plz` automate the version
bump, tag and publish.
