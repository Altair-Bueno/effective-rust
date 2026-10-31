# It already exists

Use this file whenever you are about to write a helper or add a dependency.
Column one is the thing agents keep hand-writing; column two is what to use
instead. Version numbers mark items newer than Rust 1.70, so check the
project's `rust-version` before using them.

## Standard library

### Strings and text

| About to write... | Use instead |
|---|---|
| Split on first `=`/`:` | `str::split_once`, `rsplit_once` |
| Strip a known prefix/suffix | `str::strip_prefix`, `strip_suffix` (return `Option`, no double check) |
| Trim whitespace or chars | `trim`, `trim_start_matches`, `trim_end_matches` |
| Iterate lines, words | `str::lines`, `split_whitespace`, `split_terminator` |
| Case-insensitive compare | `eq_ignore_ascii_case`, `to_ascii_lowercase` |
| Check char classes | `char::is_ascii_digit`, `is_alphanumeric`, `is_whitespace` |
| Parse a number | `str::parse::<T>()` via `FromStr` |
| Build a string in a loop | `String::with_capacity` + `push_str`, or `write!(s, ..)` via `std::fmt::Write` |
| Join pieces | `[..].join(", ")`, `Iterator::collect::<String>()`, `format!` |
| Repeat a string | `str::repeat` |
| Replace substrings | `str::replace`, `replacen` |
| Bytes <-> str | `str::from_utf8`, `String::from_utf8_lossy`, `str::as_bytes` |
| Char positions | `char_indices`, `str::find`, `rfind`, `match_indices` |
| Pad or align output | `format!("{:>8}")`, `{:<8}`, `{:^8}`, `{:08.3}` |
| Rarely-owned string parameter | `Cow<'_, str>` |

### Iterators and collections

| About to write... | Use instead |
|---|---|
| `for i in 0..v.len()` | `iter()`, `iter_mut()`, `enumerate()` |
| Loop with manual index of neighbours | `windows(n)`, `chunks(n)`, `chunks_exact`, `chunk_by` (1.77), `array_windows` (nightly) |
| Loop collecting into Vec | `map(..).collect()`, `filter_map`, `flat_map`, `extend` |
| Loop that stops early | `take_while`, `find`, `position`, `any`, `all`, `try_fold`, `try_for_each` |
| Sum / product / min / max | `sum`, `product`, `min_by_key`, `max_by`, `f64::total_cmp` for floats |
| Group / partition | `partition`, `fold` into a `HashMap`, `chunk_by` on sorted input |
| Zip / unzip | `zip`, `std::iter::zip`, `unzip` |
| Peek ahead | `peekable()` |
| Reverse / step / skip | `rev`, `step_by`, `skip`, `skip_while` |
| Deduplicate | `Vec::dedup`, `dedup_by_key` (after sort), or `HashSet` |
| Sort | `sort_unstable_by_key`, `sort_by(|a, b| a.x.total_cmp(&b.x))`, `std::cmp::Reverse`, `Ordering::then_with` |
| Binary search | `binary_search_by_key`, `partition_point` |
| Remove while iterating | `retain`, `retain_mut`, `extract_if` (1.87), `drain(..)` |
| Insert or update a map entry | `entry(k).or_insert_with(..)`, `and_modify` |
| Get first/last safely | `first`, `last`, `get(i)`, `split_first`, `split_last` |
| Swap, take, replace a field | `mem::swap`, `mem::take`, `mem::replace`, `Option::take`, `Option::replace` |
| Infinite or generated sequences | `iter::successors`, `iter::from_fn`, `iter::repeat_n` (1.82), `(0..).step_by` |
| Fixed arrays from closures | `std::array::from_fn`, `array::map`, `each_ref` |
| Multiple `&mut` into a slice | `split_at_mut`, `get_disjoint_mut` (1.86), `chunks_mut` |
| Is sorted / contains | `is_sorted` (1.82), `contains`, `starts_with` on slices |
| Concatenate slices | `concat`, `join` on `&[Vec<T>]`/`&[&str]` |
| Pop conditionally | `Vec::pop_if` (1.86) |
| Priority queue / ordered map / dedupe set | `BinaryHeap`, `BTreeMap`, `BTreeSet`, `HashSet`, `VecDeque` for FIFO |

### Option and Result

| About to write... | Use instead |
|---|---|
| `match` returning the value or a default | `unwrap_or`, `unwrap_or_else`, `unwrap_or_default` |
| `match` to transform the inner value | `map`, `map_err`, `and_then`, `ok_or_else` |
| `if x.is_some() { x.unwrap() }` | `if let Some(x) = x`, `let Some(x) = x else { .. }` |
| `Option<Result<T, E>>` juggling | `transpose` |
| `Option<Option<T>>` | `flatten` |
| Condition to `Option` | `bool::then`, `then_some` |
| Early return on `None`/`Err` | `?` (works on `Option` in functions returning `Option`) |
| Check predicate on inner | `is_some_and`, `is_none_or` (1.82), `is_ok_and` |
| Collect iterator of `Result` | `collect::<Result<Vec<_>, _>>()`, `collect::<Option<Vec<_>>>()` |
| Lazily compute a default into an `Option` field | `get_or_insert_with` |
| Take a value out, leaving `None` | `take`, `take_if` (1.80) |
| Iterate zero-or-one values | `Option::iter`, `into_iter` (chains into `flat_map`) |

### Numbers

| About to write... | Use instead |
|---|---|
| `x as u16` on a value that might not fit | `u16::try_from(x)?`, `From` when lossless |
| Overflow-safe arithmetic | `checked_add`, `saturating_sub`, `wrapping_mul`, `overflowing_*`, `Wrapping<T>` |
| Round up division | `div_ceil` (1.73), `next_multiple_of` (1.73) |
| Absolute difference | `abs_diff` |
| Clamp | `clamp`, `f64::clamp` |
| Log2 / power of two | `ilog2`, `ilog10`, `is_power_of_two`, `next_power_of_two` |
| Bit manipulation | `count_ones`, `leading_zeros`, `rotate_left`, `reverse_bits`, `to_le_bytes`, `from_be_bytes` |
| Non-zero IDs or sizes | `NonZero<u32>` (1.79), niche-optimised `Option<NonZero<_>>` |
| Float ordering | `total_cmp`, or the `ordered-float` crate |

### Memory, ownership, smart pointers

| About to write... | Use instead |
|---|---|
| Shared immutable string | `Arc<str>`, `Rc<str>`, `Box<str>` |
| Shared immutable slice | `Arc<[T]>`, `Box<[T]>` |
| Interior mutability (single thread) | `Cell<T>` for `Copy`, `RefCell<T>` otherwise, `OnceCell` |
| Interior mutability (threads) | `Mutex`, `RwLock`, atomics, `OnceLock` |
| Lazy global | `LazyLock` (1.80), `OnceLock` (1.70); `std::sync::Once` for side effects |
| Thread-local | `thread_local!` with `LocalKey` |
| Borrow-or-own | `Cow<'_, T>` |
| Pinning self-referential futures | `std::pin::pin!`, `Box::pin` |
| Type-erased value | `Box<dyn Any>`, `Any::downcast_ref` |
| Zero-sized marker | `PhantomData<T>` |
| Uninitialised buffer | `MaybeUninit<T>` (only with the unsafe gate) |

### Concurrency

| About to write... | Use instead |
|---|---|
| Spawn threads that borrow locals | `std::thread::scope` |
| Thread name / stack size | `thread::Builder` |
| Queue between threads | `std::sync::mpsc::channel`, `sync_channel` |
| Wait for a condition | `Condvar`, `Barrier` |
| Counters and flags | `AtomicUsize`, `AtomicBool` with explicit `Ordering` |
| Number of CPUs | `thread::available_parallelism` |
| Data-parallel loops | `rayon` crate (`par_iter`) |

### I/O, filesystem, process, environment

| About to write... | Use instead |
|---|---|
| Read a whole file | `fs::read_to_string`, `fs::read` |
| Write a whole file | `fs::write` |
| Read lines | `BufReader::new(f).lines()` |
| Buffered writes | `BufWriter`, `writeln!` |
| Copy streams | `io::copy` |
| Path manipulation | `Path`/`PathBuf` methods: `join`, `extension`, `file_stem`, `parent`, `with_extension`, `components` |
| Create nested dirs | `fs::create_dir_all` |
| Walk a directory | `fs::read_dir`; recursive with the `walkdir` or `ignore` crate |
| Run a command | `std::process::Command`, `output()`, `status()` |
| Environment | `env::var`, `env::var_os`, `env::args_os`, `env::current_dir` |
| Temporary files | `tempfile` crate |
| Stdin/stdout locking in loops | `io::stdin().lock()`, `io::stdout().lock()` |

### Time

| About to write... | Use instead |
|---|---|
| Measure elapsed time | `Instant::now()`, `elapsed()` |
| Durations | `Duration::from_millis`, `as_secs_f64`, `Duration::ZERO` |
| Wall-clock timestamps | `SystemTime`, `UNIX_EPOCH` |
| Calendar dates, timezones, formatting | `jiff` or `time` crate (`chrono` is legacy) |

### Traits you forgot std has

`Default`, `From`/`Into`, `TryFrom`/`TryInto`, `AsRef`/`AsMut`, `Borrow`,
`FromStr`/`Display` (parse/print pairs), `FromIterator`/`Extend`/`IntoIterator`,
`Iterator`/`DoubleEndedIterator`/`ExactSizeIterator`, `PartialOrd`/`Ord`,
`Hash`, `Error`, `Read`/`Write`/`BufRead`/`Seek`, `fmt::Write`, `Deref` (only
for smart pointers), `Drop`, `ops::{Add, Index, ..}`, `Send`/`Sync`, `Any`,
`std::error::Error::source`.

## Well-known crates by need

Check `Cargo.toml` first: a dependency that is already there wins over a
better-looking new one. Confirm maintenance and MSRV with `cargo info <crate>`
and docs.rs. Source: https://blessed.rs/crates (keep it open when choosing).

| Need | Reach for | Notes |
|---|---|---|
| Library error types | `thiserror` | Enum with `#[error]`, `#[from]`, `#[source]` |
| Application errors | `anyhow` or `color-eyre` | `.context("..")`, `bail!`, `ensure!` |
| Logging / tracing | `tracing` (+ `tracing-subscriber`) | `log` only for tiny sync crates |
| Serialization | `serde` + `serde_json` / `toml` / `postcard` | Derive, do not hand-parse |
| CLI arguments | `clap` (derive) | `bpaf`, `lexopt`, `pico-args` for faster builds |
| Iterator extras | `itertools` | Only what std lacks: `group_by`, `join`, `multi_cartesian_product` |
| Async runtime | `tokio` | `smol` for small footprints; do not mix runtimes |
| HTTP client | `reqwest` (async) / `ureq` (sync) | |
| HTTP server | `axum` | `actix-web` for raw throughput |
| gRPC | `tonic` + `prost` | |
| Regex | `regex` | Compile once in a `LazyLock` |
| Random | `rand`; `fastrand` for non-crypto | |
| UUID | `uuid` | |
| Dates | `jiff` or `time` | |
| Temporary files | `tempfile` | |
| Insertion-ordered map | `indexmap` | |
| Small/stack vectors | `smallvec`, `arrayvec`, `tinyvec` | Measure first |
| Bitflags | `bitflags` | API Guidelines C-BITFLAG |
| Builders | `bon`, `typed-builder` | Hand-write when the project has neither |
| Enum string names, iteration | `strum` | |
| Bytes buffers | `bytes` | Network code, zero-copy slices |
| Parsing | `winnow` or `nom` | Do not write recursive-descent by hand for non-trivial grammars |
| Hashing (fast, non-crypto) | `ahash`, `rustc-hash` | Only when the hasher shows in a profile |
| Hashing (crypto) | `sha2`, `blake3` | |
| Password hashing | `argon2` | |
| TLS | `rustls` | |
| Concurrency primitives | `parking_lot`, `crossbeam-channel`, `flume`, `dashmap`, `arc-swap` | |
| Data parallelism | `rayon` | |
| Databases | `sqlx`, `diesel`, `rusqlite`, `tokio-postgres` | |
| Directory walking | `walkdir`, `ignore`, `globset` | |
| Platform dirs | `dirs`, `directories` | |
| Terminal UI | `ratatui` + `crossterm`, `indicatif`, `inquire` | |
| Unix / Windows APIs | `rustix` or `nix`; `windows` | Prefer over raw `libc` |
| Safe casts between plain-old-data | `bytemuck`, `zerocopy` | Instead of `transmute` |
| Zeroing secrets | `zeroize`, `subtle` | |
| Testing | `cargo-nextest`, `insta`, `proptest`, `rstest`, `criterion`/`divan`, `cargo-fuzz` | |
| Cross-language | `bindgen`, `cbindgen`, `cxx`, `pyo3`, `napi`, `uniffi` | |
