# Lint baseline

There is no official "lints every Rust project must enable". Clippy ships
groups (`clippy::all` is on by default; `pedantic`, `nursery`,
`restriction` and `cargo` are opt-in) and rustc has its own allow-by-default
lints. Everything beyond `clippy::all` is project policy. This file is the
policy this skill expects, because the lints below mechanically enforce
half of its rules: `core` before `std`, `Self`, `as _` trait imports, no
`unwrap` outside tests, no `allow`, documented `unsafe`, no `mod.rs`.

Names and groups were checked against clippy 0.1.99 (Rust 1.99) and the
block below was compiled with it. A lint
that does not exist on the project's toolchain produces an
`unknown_lints` warning, not an error; remove it or raise the toolchain.

## When to apply

- At Orient, if the workspace (or single crate) has no `[lints]` table:
  add this baseline to the root `Cargo.toml`, add `[lints] workspace = true`
  to every member, add the `clippy.toml`, run clippy, and fix what it
  reports in the code you touch. Say in your summary that you did this.
  Pre-existing warnings elsewhere are the user's call; list the counts.
- If a `[lints]` table exists, it wins. Propose missing entries from this
  list in your summary instead of editing the table.
- `-D warnings` stays on the CI command line, never in source.

## Baseline `Cargo.toml`

```toml
[workspace.lints.rust]
unsafe_code = "warn"                 # "forbid" in crates that need none
unsafe_op_in_unsafe_fn = "warn"      # already warn in edition 2024
missing_docs = "warn"
missing_debug_implementations = "warn"
unreachable_pub = "warn"             # pub items not reachable from the crate root
unused_qualifications = "warn"
trivial_casts = "warn"
trivial_numeric_casts = "warn"
unused_import_braces = "warn"
unused_lifetimes = "warn"
unused_macro_rules = "warn"
single_use_lifetimes = "warn"
elided_lifetimes_in_paths = "warn"
explicit_outlives_requirements = "warn"
meta_variable_misuse = "warn"
unnameable_types = "warn"
redundant_lifetimes = "warn"
let_underscore_drop = "warn"
non_ascii_idents = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
cargo = { level = "warn", priority = -1 }
multiple_crate_versions = "allow"    # rarely fixable from one crate; cargo deny handles it
# cargo_common_metadata skips crates with `publish = false`; set that on internal crates

# core and alloc before std
std_instead_of_core = "warn"
std_instead_of_alloc = "warn"
alloc_instead_of_core = "warn"

# conventions
use_self = "warn"                    # nursery: Self inside impl blocks
unused_trait_names = "warn"          # use Trait as _ when only methods are used
redundant_pub_crate = "warn"         # nursery: pub(crate) where private would do
pub_without_shorthand = "warn"
mod_module_files = "warn"            # foo.rs + foo/, not foo/mod.rs
unnecessary_self_imports = "warn"
rest_pat_in_fully_bound_structs = "warn"
unneeded_field_pattern = "warn"
str_to_string = "warn"
rc_buffer = "warn"
clone_on_ref_ptr = "warn"            # Arc::clone(&x), not x.clone()
wildcard_enum_match_arm = "warn"     # name the variants; see note below

# no panics or stray output in library code
unwrap_used = "warn"
expect_used = "warn"
panic = "warn"
todo = "warn"
unimplemented = "warn"
dbg_macro = "warn"
print_stdout = "warn"
print_stderr = "warn"
exit = "warn"
mem_forget = "warn"

# casts
as_conversions = "warn"              # From/TryFrom/checked_* instead

# unsafe hygiene
undocumented_unsafe_blocks = "warn"  # every unsafe block needs // SAFETY:
multiple_unsafe_ops_per_block = "warn"
unnecessary_safety_doc = "warn"

# lint hygiene
allow_attributes = "warn"            # use #[expect(..)]
allow_attributes_without_reason = "warn"
```

Every member crate:

```toml
[lints]
workspace = true
```

## Baseline `clippy.toml`

```toml
msrv = "1.81"                       # match rust-version so clippy never suggests newer APIs
allow-unwrap-in-tests = true
allow-expect-in-tests = true
allow-panic-in-tests = true
allow-dbg-in-tests = true
allow-print-in-tests = true
```

## Per-crate overrides

Binaries legitimately print and exit:

```toml
[lints]
workspace = true

[lints.clippy]
print_stdout = "allow"
print_stderr = "allow"
exit = "allow"
```

Keep overrides in `Cargo.toml`, one place, never as source attributes.

## Notes on the stricter entries

- `wildcard_enum_match_arm` also fires on foreign enums where a wildcard is
  the only option; use `#[expect(clippy::wildcard_enum_match_arm, reason =
  "..")]` on that one match. For your own enums, name the variants.
- `as_conversions` fires on every `as`, including `u8 as char` and pointer
  casts. Prefer `From`/`TryFrom`/`char::from`; `expect` the rest with a
  reason.
- `redundant_pub_crate` and `use_self` are nursery lints: occasional false
  positives, no false negatives. Fix or `expect` with a reason.
- `missing_docs` makes every `pub` item need a doc comment. That is the
  point: an item that is not worth documenting is not worth exporting.
- `unused_crate_dependencies` is deliberately absent: it misfires on
  dev-dependencies used only by some targets. `string_to_string` is absent
  because clippy removed it in favour of `implicit_clone` (in `pedantic`).
- `cargo_common_metadata` wants `description`, `license`, `repository`,
  `readme`, `keywords` and `categories`. It skips crates marked
  `publish = false`, the right marker for internal workspace members anyway.
- `indexing_slicing` and `string_slice` are good candidates for parsers
  and codecs; add them per crate.

## Reading a lint before touching it

Each lint's page explains what it catches and why, with examples:
https://rust-lang.github.io/rust-clippy/master/index.html (search the
name). `cargo clippy -- -W help` lists every lint with its default level on
the installed toolchain. A community baseline worth comparing against is
Embark Studios' `lints.rs` (see `resources.md`).
