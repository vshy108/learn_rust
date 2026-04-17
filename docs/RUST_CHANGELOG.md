# Rust Compiler Changelog

Curated changelog of Rust releases focusing on **language changes**, common
**breaking changes**, and **stabilized APIs** that matter in day-to-day coding.

Source: [releases.rs](https://releases.rs/)

> Versions covered: 1.85.0 (Edition 2024) through 1.95.0 (current as of April 2026).

## Table of Contents

- [1.95.0](#1950--2026-04-16)
- [1.94.1](#1941--2026-03-26-patch)
- [1.94.0](#1940--2026-03-05)
- [1.93.0](#1930--2026-01-22)
- [1.92.0](#1920--2025-12-11)
- [1.91.0](#1910--2025-10-30)
- [1.90.0](#1900--2025-09-18)
- [1.89.0](#1890--2025-08-07)
- [1.88.0](#1880--2025-06-26)
- [1.87.0](#1870--2025-05-15)
- [1.86.0](#1860--2025-04-03)
- [1.85.0](#1850--2025-02-20) (Edition 2024)
- [Edition Migration Quick Reference](#edition-migration-quick-reference)
- [How to Update](#how-to-update)

---

## 1.95.0 — 2026-04-16

### Language

- **Stabilize `if let` guards on match arms** — you can now write
  `match x { Some(v) if let Ok(n) = v.parse::<i32>() => ... }`
- `irrefutable_let_patterns` lint no longer fires on let-chains
- Support importing path-segment keywords with renaming
- Stabilize inline assembly for **PowerPC / PowerPC64**
- `const`-eval: padding is now handled more consistently during typed copies
- Const blocks are no longer evaluated to determine if expressions involving
  fallible operations can implicitly be constant-promoted
- Make operational semantics of pattern matching independent of crate and module

### Compiler

- Stabilize `--remap-path-scope` for controlling how paths are remapped in
  the resulting binary
- Apply patches for CVE-2026-6042 and CVE-2026-40200 to vendored musl

### Stabilized APIs (highlights)

- `bool::TryFrom<{integer}>` — convert integers to `bool`
- `AtomicBool::update` / `AtomicBool::try_update`
- `AtomicPtr::update` / `AtomicPtr::try_update`
- `Atomic{Isize,Usize}::update` / `Atomic{Isize,Usize}::try_update`
- `cfg_select!` macro
- `core::range::RangeInclusive` / `core::range::RangeInclusiveIter`
- `core::hint::cold_path` — hint for unlikely branches
- `<*const T>::as_ref_unchecked` / `<*mut T>::as_ref_unchecked` /
  `<*mut T>::as_mut_unchecked`
- `Vec::push_mut` — push and return `&mut T`
- `Layout::dangling_ptr` / `Layout::repeat` / `Layout::repeat_packed` /
  `Layout::extend_packed`
- `MaybeUninit<[T; N]>` conversions (`From`, `AsRef`, `AsMut`)
- `Cell<[T; N]>` / `Cell<[T]>` `AsRef` impls
- `fmt::from_fn` now const
- `ControlFlow::is_break` / `ControlFlow::is_continue` now const

### Rustdoc

- Unstable items now rank lower in search results
- New "hide deprecated items" setting

### Breaking / Compatibility

- Array coercions may now result in fewer inference constraints than before
- Importing `$crate` without renaming (`use $crate::{self}`) is no longer
  permitted
- Future-incompatibility lint `ambiguous_glob_imported_traits` now reported
  when using an ambiguously glob-imported trait
- Deprecate `Eq::assert_receiver_is_total_eq` with future-compat warnings on
  manual impls
- Matching on a `#[non_exhaustive]` enum now reads the discriminant even if
  the enum has only one variant (may affect closure captures)
- `mut ref` / `mut ref mut` patterns accidentally allowed on stable in struct
  field shorthand are now correctly feature-gated
- JSON target specs destabilized — now require `-Z unstable-options`
- Future-compat warning for derive helper attributes conflicting with built-in
  attributes

### Internal

- Update to **LLVM 22**

---

## 1.94.1 — 2026-03-26 (patch)

- Fix `std::thread::spawn` on `wasm32-wasip1-threads`
- Clippy: fix ICE in `match_same_arms`
- Cargo: update tar to 0.4.45 (CVE-2026-33055 and CVE-2026-33056)

---

## 1.94.0 — 2026-03-05

### Language

- Update to **Unicode 17**
- Stabilize 29 additional RISC-V target features
- `unused_visibilities` lint now warns on `pub const _ = ...` declarations

### Stabilized APIs (highlights)

- `<[T]>::array_windows` — sliding window iterator over slices
- `<[T]>::element_offset` — byte offset of an element in a slice
- `LazyCell::get` / `LazyCell::get_mut` / `LazyCell::force_mut`
- `LazyLock::get` / `LazyLock::get_mut` / `LazyLock::force_mut`
- `Peekable::next_if_map` / `Peekable::next_if_map_mut`
- `f32::consts::EULER_GAMMA`, `f64::consts::EULER_GAMMA`
- `f32::consts::GOLDEN_RATIO`, `f64::consts::GOLDEN_RATIO`
- `f32::mul_add` / `f64::mul_add` now const

### Cargo

- **Stabilize `include` config key** — load additional config files
- **Stabilize `pubtime` field** in registry index (publish timestamps)
- Cargo now parses **TOML v1.1** for manifests and config
- `CARGO_BIN_EXE_<crate>` now available at runtime

### Breaking / Compatibility

- Forbid freely casting lifetime bounds of `dyn` types
- Closure capturing changes around pattern matching (may cause new borrow
  checker errors or change `Drop` timing)
- Standard library macros now imported via prelude, not `#[macro_use]` —
  glob-imported macros with the same name (e.g. custom `matches!`) are now
  ambiguous
- Switch to `annotate-snippets` for error emission (visual changes to errors)

---

## 1.93.0 — 2026-01-22

### Language

- Stabilize `asm_cfg` — conditional assembly blocks
- `const_item_interior_mutations` lint (warn-by-default)
- `function_casts_as_integer` lint (warn-by-default)
- LUB coercions now correctly handle function item types with differing
  safeties

### Stabilized APIs (highlights)

- `String::into_raw_parts` / `Vec::into_raw_parts`
- `<[T]>::as_array` / `<[T]>::as_mut_array` — try to convert slice to array
  ref
- `VecDeque::pop_front_if` / `VecDeque::pop_back_if`
- `char::MAX_LEN_UTF8` / `char::MAX_LEN_UTF16`
- `std::fmt::from_fn` — create `Display` impl from a closure
- `Duration::from_nanos_u128`

### Cargo

- `cargo clean --workspace`
- `cargo tree` supports long forms for `--format` variables

### Breaking / Compatibility

- `#[test]` on structs / trait methods is now an error (was ignored)
- `deref_nullptr` lint upgraded to deny-by-default
- Internal `Copy` specialization removed — some std APIs may call
  `Clone::clone` instead of bitwise copy (potential minor perf regression)
- `offset_of!` now checks types are well-formed

---

## 1.92.0 — 2025-12-11

### Language

- `&raw [mut | const]` for union fields now allowed in safe code
- `never_type_fallback_flowing_into_unsafe` and
  `dependency_on_unit_never_type_fallback` are now **deny-by-default**
- `unused_must_use` no longer warns on `Result<(), !>` or similar uninhabited
  error types
- Allow multiple bounds for same associated item (except trait objects)

### Stabilized APIs (highlights)

- `RwLockWriteGuard::downgrade` — downgrade write lock to read lock
- `Box::new_zeroed` / `Rc::new_zeroed` / `Arc::new_zeroed`
- `btree_map::Entry::insert_entry`
- `NonZero<u{N}>::div_ceil`
- `Location::file_as_c_str`

### Breaking / Compatibility

- Backtrace generation with `-C panic=abort` on Linux now emits unwind tables
  by default (use `-C force-unwind-tables=no` to opt out)
- Minimum external LLVM updated to **20**
- `invalid_macro_export_arguments` is now deny-by-default
- `DerefMut for Pin<LocalType>` impls now prevented downstream
- Temporary lifetime extension changes for `pin!` and formatting macros

---

## 1.91.0 — 2025-10-30

### Language

- `semicolon_in_expressions_from_macros` upgraded from warn to **deny**
- `dangling_pointers_from_locals` lint added (warn-by-default)
- `integer_to_ptr_transmutes` lint added (warn-by-default)

### Stabilized APIs (highlights)

- `Path::file_prefix` — portion of filename before the first `.`
- `BTreeMap::extract_if` / `BTreeSet::extract_if`
- `str::ceil_char_boundary` / `str::floor_char_boundary`
- `Duration::from_mins` / `Duration::from_hours`
- `PathBuf::add_extension` / `PathBuf::with_added_extension`
- `core::iter::chain` — free function version of `.chain()`
- `Cell::as_array_of_cells`
- `{integer}::strict_*` methods (panic on overflow in all modes)
- `{integer}::carrying_add` / `borrowing_sub` / `carrying_mul`
- `PanicHookInfo::payload_as_str`
- `Path == str` / `Path == String` comparisons

### Cargo

- **Stabilize `build.build-dir`** — configurable intermediate artifact
  directory
- `--target "host-tuple"` literal support

### Platform

- `aarch64-pc-windows-msvc` promoted to **Tier 1**

### Internal

- Update to **LLVM 21**

### Breaking / Compatibility

- Thread ID now printed in panic messages
- Apple SDK root always passed when linking with `cc` — libraries in
  `/usr/local/lib` may no longer be linked automatically
- Closures marked `static` are now syntactically invalid
- Stricter attribute parsing (invalid `#[should_panic]`, `#[link]` now error)
- Fix drop checker being more permissive for let-else bindings

---

## 1.90.0 — 2025-09-18

### Language

- Allow volatile access to non-Rust memory, including address 0
- Diagnostic attribute lint split into 4 finer-grained lints

### Compiler

- **`lld` is now the default linker on `x86_64-unknown-linux-gnu`**

### Stabilized APIs (highlights)

- `u{N}::checked_sub_signed` / `overflowing_sub_signed` /
  `saturating_sub_signed` / `wrapping_sub_signed`
- CStr/CString/Cow\<CStr\> cross-comparisons
- `f32`/`f64` `floor`/`ceil`/`trunc`/`fract`/`round`/`round_ties_even` now
  const
- `<[T]>::reverse` now const

### Cargo

- **Stabilize multi-package publishing** (`cargo publish` for workspaces)

### Platform

- `x86_64-apple-darwin` **demoted to Tier 2** (with host tools)

### Breaking / Compatibility

- `lld` default on x86_64 Linux may change link behavior
- `Fuse::default()` now constructs `I::default()` internally (as documented)
- `home_dir()` uses fallback if `HOME` is empty on Unix
- Unsupported `extern "abi"` strings now consistently rejected

---

## 1.89.0 — 2025-08-07

### Language

- **Stabilize `#[repr(u128)]` / `#[repr(i128)]`**
- Stabilize generic arg infer (`feature(generic_arg_infer)`) — `[0u8; _]`
- `mismatched_lifetime_syntaxes` lint (warn-by-default) — replaces
  `elided_named_lifetimes`
- `dangerous_implicit_autorefs` lint now **deny-by-default**
- Stabilize AVX-512 target features
- Allow storing `format_args!()` in a variable

### Stabilized APIs (highlights)

- `File::lock` / `File::lock_shared` / `File::try_lock` / `File::unlock`
- `Result::flatten`
- `NonNull::from_ref` / `NonNull::from_mut`
- `OsString::leak` / `PathBuf::leak`
- `NonZero<char>`
- `DerefMut for Lazy{Cell,Lock}`
- `Default for array::IntoIter`
- `Clone for slice::ChunkBy`
- `io::Seek for io::Take`

### Cargo

- `cargo fix` / `cargo clippy --fix` now respect target selection (was
  applying to all targets)
- Stabilize doctest cross-compilation

### Breaking / Compatibility

- `missing_fragment_specifier` is now an unconditional error
- Sized Hierarchy Part I — small breaking change for `?Sized` bounds on
  recursive types with associated type projections
- `std::intrinsics::drop_in_place` removed
- `extern "C"` ABI on `wasm32-unknown-unknown` is now standards-compliant

---

## 1.88.0 — 2025-06-26

*(Release notes unavailable at fetch time — check
[releases.rs/docs/1.88.0](https://releases.rs/docs/1.88.0/) for details)*

---

## 1.87.0 — 2025-05-15

### Language

- Stabilize `asm_goto`
- `precise_capturing_in_traits` — `use<...>` bounds on RPIT in traits
- Open beginning ranges (`..EXPR`) allowed after `!`, `-`, `*`

### Compiler

- SSE2 now **required** for i686 targets

### Stabilized APIs (highlights)

- `Vec::extract_if` / `LinkedList::extract_if`
- `<[T]>::split_off` / `split_off_mut` / `split_off_first` /
  `split_off_last` (and `_mut` variants)
- `String::extend_from_within`
- `OsStr::display` / `OsString::display`
- `io::pipe` / `io::PipeReader` / `io::PipeWriter`
- `Box<MaybeUninit<T>>::write`
- `TryFrom<Vec<u8>> for String`
- `<uN>::cast_signed` / `<iN>::cast_unsigned`
- `<uN>::is_multiple_of`
- `<uN>::unbounded_shl` / `unbounded_shr`
- `<str>::from_utf8` (inherent, not via `std::str`)
- `ControlFlow` is now `#[must_use]`

### Internal

- Update to **LLVM 20**

### Breaking / Compatibility

- `ptr_cast_add_auto_to_object` is now a hard error
- Order-dependent trait objects warning is now a hard error
- `ControlFlow` is `#[must_use]` — unused `ControlFlow` values now warn
- Width/precision formatting options limited to 16 bits on all targets
- Windows: std no longer links `advapi32` (except on win7)
- `cfg(true)` attributes no longer visible to proc macros

---

## 1.86.0 — 2025-04-03

### Language

- **Stabilize trait object upcasting** to supertraits
- `#[target_feature]` on safe functions now allowed
- `missing_abi` lint now **warn-by-default**
- `double_negations` lint added (from clippy)

### Compiler

- `-O` now means `-C opt-level=3` (was 2), matching Cargo defaults
- Debug asserts on raw pointer null access

### Stabilized APIs (highlights)

- `{float}::next_down` / `{float}::next_up`
- `<[_]>::get_disjoint_mut` — multiple mutable borrows from a slice
- `HashMap::get_disjoint_mut`
- `Vec::pop_if`
- `sync::Once::wait` / `sync::OnceLock::wait`
- `NonZero::count_ones`

### Breaking / Compatibility

- `wasm_c_abi` warning is now a **hard error** — upgrade `wasm-bindgen` to
  >= 0.2.89
- `cenum_impl_drop_cast` is now a hard error
- SSE2 on i686 — disabling it causes warning (will become hard error)

### Internal

- AArch64 Linux compiler now built with ThinLTO + PGO (~30% faster)

---

## 1.85.0 — 2025-02-20

### Language

- **Rust Edition 2024 stabilized** — see
  [edition guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
- **Stabilize async closures** (RFC 3668)
- `#[diagnostic::do_not_recommend]` stabilized
- `unpredictable_function_pointer_comparisons` lint added

### Stabilized APIs (highlights)

- `ptr::fn_addr_eq`
- `io::ErrorKind::QuotaExceeded` / `io::ErrorKind::CrossesDevices`
- `{float}::midpoint` / `u{N}::midpoint` / `NonZeroU*::midpoint`
- `Waker::noop`
- `Extend` / `FromIterator` for tuples (arity 1-12)
- `mem::size_of_val` / `mem::align_of_val` now const
- `mem::swap` / `ptr::swap` now const
- `NonNull::new` now const
- `HashMap::with_hasher` / `HashSet::with_hasher` now const

### Cargo

- Stabilize higher precedence trailing flags
- `CARGO_CFG_FEATURE` passed to build scripts

### Breaking / Compatibility

- `c_char` signedness changed on many Tier 2/3 targets (Arm, RISC-V
  embedded) — update `libc` to >= 0.2.169
- `home_dir()` on Windows no longer reads non-standard `$HOME`
- `test` cfg no longer a well-known check-cfg (build systems must set it)

---

## Edition Migration Quick Reference

| Edition | Stabilized In | Key Changes |
|---------|--------------|-------------|
| 2024 | **1.85.0** | New borrow checker rules, `unsafe_op_in_unsafe_fn` warn-by-default, RPIT captures all lifetimes, `gen` keyword reserved, `impl Trait` in `dyn Trait` position, tail expression temporary scope changes |

---

## How to Update

```sh
# Update Rust toolchain
rustup update

# Check current version
rustc --version

# Update project dependencies
cargo update

# Migrate to a new edition (e.g., 2024)
cargo fix --edition
```

---

*Last updated: 2026-04-01*
