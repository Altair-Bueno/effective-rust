# Unsafe and FFI

The gate: before writing `unsafe`, raw pointers, `#[repr]`, `extern`
blocks, `transmute`, `MaybeUninit`, or `static mut`, read this file, the
relevant Rustonomicon chapter, and run the result under Miri. Sources:
Effective Rust Items 16, 34, 35; Rustonomicon; Unsafe Code Guidelines; Rust
Design Patterns "Contain unsafety in small modules" and FFI idioms.

## When `unsafe` is legitimate

1. FFI calls and callbacks.
2. Implementing a primitive that safe Rust cannot express (intrusive lists,
   lock-free structures, custom allocators), after confirming no crate
   already provides it (`crossbeam`, `parking_lot`, `slab`, `bumpalo`).
3. A `*_unchecked` fast path whose safe counterpart has measured overhead in a
   profile.
4. Hardware access (`volatile`, MMIO) in embedded code.

It is not legitimate for satisfying the borrow checker, for "performance"
without a benchmark, or for working around a missing `Send`/`Sync` impl you
do not understand.

Prefer `#![forbid(unsafe_code)]` in crates that need none; it documents the
guarantee.

## The rules of writing it

- **Small module, safe API.** Put the `unsafe` in one module with a safe
  public interface that upholds every invariant. The rest of the crate stays
  safe (Rust Design Patterns, "Contain unsafety in small modules").
- **`// SAFETY:` on every block**, stating the invariant that makes the
  operation sound and who guarantees it, not what the code does.
  ```rust
  // SAFETY: `idx < self.len` was checked by the caller of `get_unchecked`,
  // and `self.ptr` points to `self.len` initialised elements.
  unsafe { &*self.ptr.add(idx) }
  ```
- **`# Safety` section** in the docs of every `unsafe fn`, listing the
  caller's obligations (C-FAILURE).
- **`unsafe fn` bodies are not implicitly unsafe** in edition 2024
  (`unsafe_op_in_unsafe_fn`): wrap each operation in its own `unsafe {}` with
  its own comment.
- **Constructor convention:** `Type::new` validates and returns `Result`;
  `Type::new_unchecked` is `unsafe fn` and documents the invariant. Callers
  use the checked form unless they already proved the invariant.
- **No `static mut`.** Use atomics, `Mutex`, `OnceLock`, or `UnsafeCell`
  inside a type that enforces access.
- **No `transmute`** for things with a safe equivalent: `from_ne_bytes`,
  `to_bits`, `as` for pointer casts, `bytemuck::cast`/`zerocopy` for POD,
  `ptr::read_unaligned`, enum discriminants via `#[repr(u8)]` and `TryFrom`.
- **Aliasing:** never hold a `&mut T` while any other reference to the same
  memory is live; derive raw pointers with `&raw mut`/`&raw const`
  (1.82) or `addr_of_mut!` instead of going through a reference.
- **Uninitialised memory** is `MaybeUninit<T>`, never `mem::zeroed()` for
  non-zeroable types or `mem::uninitialized()`.
- **Pointer provenance:** derive every pointer from an allocation you own or
  were handed; do not fabricate from integers unless on bare metal.
- **Panics must not unwind into unsafe invariants or across FFI.** Use
  `catch_unwind` in `extern "C"` callbacks or declare `extern "C-unwind"`.

## Verification

```sh
cargo +nightly miri test            # detects UB in tests (ask before installing Miri)
cargo +nightly miri test -- --many-seeds   # for concurrency
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test --target <host>
cargo fuzz run <target>             # for anything parsing untrusted bytes
```

Miri is slow but non-negotiable for new `unsafe`. If the component or
nightly is missing, ask the user before installing anything; toolchain
changes are theirs to make. Write tests that exercise
the boundaries (empty, max len, overlapping ranges, zero-sized types).

## Boundary types: do not hide the conversion [Item 34]

The safe wrapper's job is to make the C contract explicit, not to make it
look like idiomatic Rust at the cost of hidden work. If the C function takes
a C string, the safe signature takes `&CStr`:

```rust
impl Stuff {
    pub fn new(name: &CStr) -> Result<Self, Error> {
        // SAFETY: `name` is a valid NUL-terminated string for the duration of the call.
        let raw = unsafe { sys::stuff_new(name.as_ptr()) };
        NonNull::new(raw).map(|raw| Self { raw }).ok_or(Error::Rejected)
    }
}

// caller
let name = CString::new(name)?;      // the allocation and its error are visible here
let stuff = Stuff::new(&name)?;
```

A `&str` parameter that calls `CString::new` inside hides an allocation and
an error path in a signature that promises a borrow. The same applies to
`&Path` versus `&str`, `&[u8]` versus `&str`, and validated newtypes versus
raw integers. If a `&str` convenience is wanted, add it as a separately
named method that documents the allocation, next to the `&CStr` one, never
instead of it. Use `core::ffi::{CStr, c_char, c_int}` and
`alloc::ffi::CString`; they do not need `std`.

## FFI [Items 34, 35]

- Generate bindings with `bindgen` (C to Rust) and `cbindgen` (Rust to C);
  hand-written `extern` declarations drift from the headers.
- Edition 2024: `unsafe extern "C" { .. }` blocks; mark individual items
  `safe fn` only when they truly are.
- Types crossing the boundary are `#[repr(C)]` structs, `#[repr(u8)]`
  enums with explicit discriminants, raw pointers, `c_int` and friends from
  `std::ffi`/`core::ffi`, never `String`, `Vec`, `Box<dyn ..>`, `Option<T>`
  (except `Option<extern "C" fn>` and `Option<&T>`/`Option<NonNull>` which
  are guaranteed nullable-pointer optimised).
- Strings: accept `*const c_char` and convert with `CStr::from_ptr` (check
  null first); return `CString::into_raw` with a matching `free` function;
  never let C free Rust memory or vice versa.
- Ownership: expose opaque handles (`Box::into_raw`) with `new`/`free` pairs;
  document which side owns what (Rust Design Patterns, "Object-based APIs").
- Errors: return status codes plus an out-param, or a thread-local last-error,
  never panics (Rust Design Patterns, FFI "Idiomatic errors").
- `extern "C" fn` callbacks: catch panics, keep them `unwind`-free, and pass a
  `*mut c_void` user-data pointer rather than capturing state.
- Build scripts link with the `cc` crate or `pkg-config`; emit
  `cargo:rerun-if-changed` lines.

## Checklist

- [ ] A safe alternative (crate or std) was searched for and ruled out.
- [ ] `unsafe` confined to one module with a safe interface.
- [ ] Every block has a `// SAFETY:` comment naming the invariant.
- [ ] Every `unsafe fn` has a `# Safety` doc section.
- [ ] Tests run clean under Miri.
- [ ] No `static mut`, no `transmute` with a safe alternative, no
      `mem::zeroed` on non-zeroable types.
- [ ] FFI types are `#[repr(C)]`; no panics cross the boundary.
