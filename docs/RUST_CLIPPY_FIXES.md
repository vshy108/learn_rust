# Rust Clippy Fixes — Reference Guide

> Catalog of every clippy lint encountered in this repo, why it fires, and the idiomatic fix.
> Based on commit [`80a247f`](https://github.com/vshy108/learn_rust/commit/80a247fc2f4cae6ba00a460faf5ed1efb4abb57c) and subsequent fixes.

## Table of Contents

- [Auto-fixable Lints](#auto-fixable-lints)
  - [clippy::map_flatten / map_or(None, ..)](#clippymap_flatten--map_ornone-)
  - [clippy::needless_borrow](#clippyneedless_borrow)
  - [clippy::manual_flatten](#clippymanual_flatten)
  - [clippy::map_entry](#clippymap_entry)
  - [clippy::manual_case_insensitive_ascii_comparison](#clippymanual_case_insensitive_ascii_comparison)
  - [clippy::map_or_simplification (is_none_or)](#clippymap_or_simplification-is_none_or)
  - [clippy::manual_saturating_arithmetic](#clippymanual_saturating_arithmetic)
  - [clippy::manual_div_ceil](#clippymanual_div_ceil)
  - [clippy::manual_arithmetic_check](#clippymanual_arithmetic_check)
  - [clippy::redundant_into_iter](#clippyredundant_into_iter)
  - [clippy::identity_conversion (useless cast)](#clippyidentity_conversion-useless-cast)
- [Manual-fix Lints](#manual-fix-lints)
  - [Doc comment indentation](#doc-comment-indentation)
  - [Manual loop → copy_from_slice](#manual-loop--copy_from_slice)
  - [abs_diff instead of manual branching](#abs_diff-instead-of-manual-branching)
  - [div_ceil instead of manual ceiling division](#div_ceil-instead-of-manual-ceiling-division)
  - [/// on non-doc items → //](#-on-non-doc-items--)
- [Suppressed Lints (#\[allow\])](#suppressed-lints-allow)
  - [clippy::too_many_arguments](#clippytoo_many_arguments)
  - [clippy::only_used_in_recursion](#clippyonly_used_in_recursion)
  - [clippy::type_complexity](#clippytype_complexity)
  - [non_snake_case](#non_snake_case)
  - [unused_mut](#unused_mut)
  - [unused_variables](#unused_variables)
- [Dead Code Warnings](#dead-code-warnings)

---

## Auto-fixable Lints

These can be auto-fixed via `cargo clippy --fix`, but understanding *why* matters.

### clippy::map_flatten / map_or(None, ..)

**Why it fires:** `.map(f).flatten()` on `Option` is exactly what `.and_then(f)` does.
Similarly, `.map_or(None, |x| ...)` is just `.and_then(|x| ...)`.

**Bad:**
```rust
let next = node.next.as_mut().map(|n| n.next.take()).flatten();
let val = opt.map_or(None, |x| Some(x + 1));
```

**Good:**
```rust
let next = node.next.as_mut().and_then(|n| n.next.take());
let val = opt.and_then(|x| Some(x + 1));
```

**Why it matters:** `and_then` communicates intent more clearly — "chain this
fallible operation" — and avoids an intermediate `Option<Option<T>>`.

**Files:** `lc_0019` (map_flatten), `lc_0002` (map_or)

---

### clippy::needless_borrow

**Why it fires:** Adding `&` to an expression that already produces a reference
creates a double-reference (`&&T`). Rust auto-derefs for comparisons, so the
extra `&` is noise.

**Bad:**
```rust
if &haystack[i..i + m] == needle { ... }
sorted.sort_by(|a, b| b.1.cmp(&a.1));
if &a[i..] >= &b[j..] { ... }
```

**Good:**
```rust
if haystack[i..i + m] == needle { ... }     // [i..] already yields &str
sorted.sort_by(|a, b| b.1.cmp(a.1));       // a.1 is already &str
if a[i..] >= b[j..] { ... }                // slice PartialOrd works directly
```

**Why it matters:** The extra `&` doesn't change behavior (auto-deref handles it),
but it adds visual clutter and signals the author may not understand borrowing.

**Files:** `lc_0028`, `lc_0321`, `lc_0332`

---

### clippy::manual_flatten

**Why it fires:** Iterating over `Option` or `Result` items and immediately
pattern-matching with `if let Some(x) = item` is what `.flatten()` does.

**Bad:**
```rust
for bucket in &buckets {
    if let Some((lo, hi)) = bucket {
        max_gap = max_gap.max(lo - prev_max);
        prev_max = *hi;
    }
}
```

**Good:**
```rust
for (lo, hi) in buckets.iter().flatten() {
    max_gap = max_gap.max(lo - prev_max);
    prev_max = *hi;
}
```

**Why it matters:** `.flatten()` skips `None` and unwraps `Some` in one step.
Less nesting, clearer intent that we're processing only the present values.

**Files:** `lc_0164`, `lc_0589`

---

### clippy::map_entry

**Why it fires:** `if !map.contains_key(&k) { map.insert(k, v); }` does **two**
hash lookups — one for `contains_key`, one for `insert`. The entry API does a
single lookup and branches on `Vacant` vs `Occupied`.

**Bad:**
```rust
if !map.contains_key(&x) {
    map.insert(x, x);
}
```

**Good:**
```rust
// For simple "insert if absent":
map.entry(x).or_insert(x);

// When you need the branch to return early:
if let std::collections::hash_map::Entry::Vacant(e) = parent.entry(x) {
    e.insert(x);
    return x;
}
```

**Why it matters:** Single lookup is ~2x faster for hash-heavy code. The entry
API is idiomatic Rust and avoids the TOCTOU (time-of-check-time-of-use) gap.

**Files:** `lc_0290`, `lc_0947`

---

### clippy::manual_case_insensitive_ascii_comparison

**Why it fires:** Comparing `a.to_ascii_lowercase() == b.to_ascii_lowercase()`
creates two temporary converted values. `eq_ignore_ascii_case` does the
comparison in one call without temporaries.

**Bad:**
```rust
if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
```

**Good:**
```rust
if !bytes[left].eq_ignore_ascii_case(&bytes[right]) {
```

**Why it matters:** Avoids two temporary conversions. More readable intent: "are
these equal ignoring case?" rather than "convert both, then compare."

**Files:** `lc_0125`

---

### clippy::map_or_simplification (is_none_or)

**Why it fires:** `.map_or(true, |x| pred(x))` means "true if None, otherwise
check predicate." Rust 1.82+ has `.is_none_or(|x| pred(x))` which states that
intent directly.

**Bad:**
```rust
if result.last().map_or(true, |last| last[1] != max_h) {
```

**Good:**
```rust
if result.last().is_none_or(|last| last[1] != max_h) {
```

**Why it matters:** `is_none_or` reads like English: "is none, or satisfies this
predicate." `map_or(true, ...)` requires mental gymnastics to parse.

**Files:** `lc_0218` (two occurrences)

---

### clippy::manual_saturating_arithmetic

**Why it fires:** `if k > n { k - n } else { 0 }` is exactly what
`k.saturating_sub(n)` does — subtract with a floor of 0.

**Bad:**
```rust
let lo = if k > n { k - n } else { 0 };
```

**Good:**
```rust
let lo = k.saturating_sub(n);
```

**Why it matters:** `saturating_sub` is a single instruction on most CPUs, avoids
branching, and is impossible to get wrong (no accidental underflow on unsigned).

**Files:** `lc_0321` (two occurrences)

---

### clippy::manual_div_ceil

**Why it fires:** `(a + b + 1) / 2` is ceiling division. Rust 1.73+ provides
`.div_ceil()` which is clearer and handles potential overflow at the boundaries.

**Bad:**
```rust
let mid = (lo + hi + 1) / 2;
```

**Good:**
```rust
let mid = (lo + hi).div_ceil(2);
```

**Why it matters:** `div_ceil` is self-documenting, while `(a + b + 1) / 2`
requires knowing the ceiling-division trick. Also overflow-safe.

**Files:** `lc_0302` (two occurrences), `lc_0328`

---

### clippy::manual_arithmetic_check

**Why it fires:** `mid + 1 <= hi` is the same as `mid < hi` for unsigned types.
The simpler form avoids potential overflow if `mid + 1` wraps.

**Bad:**
```rust
if mid + 1 <= hi && nums[mid] > nums[mid + 1] {
```

**Good:**
```rust
if mid < hi && nums[mid] > nums[mid + 1] {
```

**Why it matters:** For `usize`, `mid + 1` could theoretically wrap to 0 on
`usize::MAX`. `mid < hi` is both simpler and safer.

**Files:** `lc_0153`

---

### clippy::redundant_into_iter

**Why it fires:** Calling `.into_iter()` on a type that already implements
`Iterator` (like `Chars` or `Range`) is redundant — it's a no-op.

**Bad:**
```rust
for c in s.chars().into_iter() { ... }
for i in (0..n).into_iter() { ... }
```

**Good:**
```rust
for c in s.chars() { ... }
for i in 0..n { ... }
```

**Files:** `lc_0006`, `lc_0238`

---

### clippy::identity_conversion (useless cast)

**Why it fires:** Casting a value to a type it already is (e.g. `x as usize`
when `x` is already `usize`) is a no-op that adds noise.

**Bad:**
```rust
let len = min_len as usize;  // min_len is already usize
```

**Good:**
```rust
let len = min_len;
```

**Files:** `lc_0076`, `lc_0003`

---

## Manual-fix Lints

These require human judgment or can't be auto-fixed.

### Doc comment indentation

**Why it fires:** Clippy (and rustdoc) expect consistent indentation in `///`
doc comments. A continuation line that's indented further than the list item
it belongs to triggers a warning.

**Bad:**
```rust
/// Transition:
///   - p[j-1] == '*' => dp[i][j] = dp[i][j-1] (star matches empty)
///                                 || dp[i-1][j] (star matches one more char)
```

**Good:**
```rust
/// Transition:
///   - p[j-1] == '*' => dp[i][j] = dp[i][j-1] (star matches empty)
///     || dp[i-1][j] (star matches one more char)
```

**Why it matters:** Rustdoc parses doc comments as markdown. Inconsistent
indentation can cause content to render as code blocks or break list structure.

**Files:** `lc_0001`, `lc_0005`, `lc_0044`, `lc_0239`, `lc_0299` (8 total)

---

### Manual loop → copy_from_slice

**Why it fires:** A `for i in 0..n { arr[i+1] = nums[i]; }` loop that copies
elements one-by-one can be replaced with `copy_from_slice`, which may use
`memcpy` internally — both clearer and faster.

**Bad:**
```rust
let mut arr = vec![1i32; n + 2];
for i in 0..n {
    arr[i + 1] = nums[i];
}
```

**Good:**
```rust
let mut arr = vec![1i32; n + 2];
arr[1..=n].copy_from_slice(&nums);
```

**Why it matters:** `copy_from_slice` is a single `memcpy` call for `Copy` types.
The compiler might optimize the loop too, but `copy_from_slice` is guaranteed
fast and communicates "bulk copy" intent.

**Files:** `lc_0312` (two occurrences)

---

### abs_diff instead of manual branching

**Why it fires:** `if a >= b { a - b } else { b - a }` is exactly `a.abs_diff(b)`,
available on all integer types since Rust 1.60.

**Bad:**
```rust
let diff = if pos >= ring_idx { pos - ring_idx } else { ring_idx - pos };
```

**Good:**
```rust
let diff = pos.abs_diff(ring_idx);
```

**Files:** `lc_0514`

---

### div_ceil instead of manual ceiling division

Same as the auto-fixable lint above, but sometimes the pattern is more subtle
and `cargo clippy --fix` doesn't catch it. See [clippy::manual_div_ceil](#clippymanual_div_ceil).

**Files:** `lc_0328`

---

### `///` on non-doc items → `//`

**Why it fires:** `///` is a doc comment that attaches to the *next item* (struct,
fn, etc.). Using `///` above a non-item (like a `#[derive]` attribute or a plain
statement) generates a warning because there's nothing to document.

**Bad:**
```rust
/// Linked list represented as Vec<i32> for benchmarking convenience.
#[derive(Debug, Clone)]
enum NestedInt { ... }
```

**Good:**
```rust
// Linked list represented as Vec<i32> for benchmarking convenience.
#[derive(Debug, Clone)]
enum NestedInt { ... }
```

**Why it matters:** `///` creates public documentation. If the comment is just a
note to the reader (not API docs), use `//` to avoid confusion.

**Files:** `lc_0328`, `lc_0341`, `lc_0355`, `lc_0364`

---

## Suppressed Lints (#[allow])

Sometimes the idiomatic fix doesn't apply. These cases justify suppression.

### clippy::too_many_arguments

**What it warns:** Functions with more than 7 parameters.

**When to suppress:** LeetCode-imposed signatures (can't change the param count)
or backtracking DFS where all params are needed and a context struct would
hurt readability.

```rust
// Clippy warns about >7 params, but DFS needs board, word chars, current
// index, row/col, dimensions, and visited state — no clean way to reduce
// without a context struct that would hurt readability for this pattern.
#[allow(clippy::too_many_arguments)]
fn dfs(board: &[Vec<char>], chars: &[char], idx: usize,
       row: usize, col: usize, rows: usize, cols: usize,
       visited: &mut Vec<Vec<bool>>) -> bool { ... }
```

**Files:** `lc_0079`, `lc_0212`, `lc_0223` (LeetCode signature), `lc_0301`

---

### clippy::only_used_in_recursion

**What it warns:** Parameters that are passed unchanged to every recursive call
and never otherwise read.

**When to suppress:** False positive — clippy doesn't track indexing/derefs
through the parameters. If you index into a slice (`key[key_idx]`, `positions[&target]`),
clippy doesn't see that as "using" the `key` parameter.

```rust
// Clippy flags `ring`, `key`, `n`, `positions` as "only used in recursion"
// because they're passed unchanged to recursive calls. But they ARE read
// in each call (key[key_idx], positions[&target], ring_idx.abs_diff, n - diff)
// — clippy's analysis doesn't track indexing into these slices/maps.
#[allow(clippy::only_used_in_recursion)]
fn dp(ring_idx: usize, key_idx: usize, ring: &str, key: &str,
      n: usize, positions: &HashMap<u8, Vec<usize>>,
      memo: &mut Vec<Vec<i32>>) -> i32 { ... }
```

**Files:** `lc_0514`

---

### clippy::type_complexity

**What it warns:** Type annotations that are very long/nested (e.g. deeply
generic tuples).

**When to suppress:** Test case data where a type alias would hurt readability
for a one-off declaration.

```rust
#[allow(clippy::type_complexity)]
let test_cases: Vec<(Vec<i32>, i32, Vec<i32>, i32, Vec<i32>)> = vec![...];
```

**Files:** `lc_0088`, `lc_0102`

---

### non_snake_case

**What it warns:** Module or function names not in `snake_case`.

**When to suppress:** When matching LeetCode's exact API naming (e.g. `guessNumber`).

```rust
#[allow(non_snake_case)]
mod my_solution { ... }
```

**Files:** `lc_0374`

---

### unused_mut

**What it warns:** Variable declared `mut` but never mutated.

**When to suppress:** In test macros shared across solutions where one solution
takes `&self` (no mut needed) but another takes `&mut self` (needs mut). The
macro must use `mut` to satisfy both.

```rust
// allow(unused_mut): my_solution takes &self so `mut` is unused,
// but ai_solution still takes &mut self and needs it.
#[allow(unused_mut)]
mod $mod_name { ... }
```

**Files:** `lc_0380`, `lc_0382`

---

### unused_variables

**What it warns:** Function parameter declared but never read.

**When to suppress:** When a LeetCode signature requires a parameter that the
algorithm doesn't use (e.g. `n` in `first_bad_version` where you only use binary search
and never reference `n` beyond the initial range).

**Files:** `lc_0278`

---

## Dead Code Warnings

Not clippy lints per se, but the compiler's `dead_code` analysis. These were
fixed in commit [`c38debd`](https://github.com/vshy108/learn_rust/commit/c38debd) (278 files).

### Problem: `main()` only calls one solution

If `main()` only demos `my_solution`, then `ai_solution::Solution` and all its
methods are dead code outside `#[cfg(test)]`.

**Fix:** `main()` must call **both** `my_solution` and `ai_solution`.

### Problem: Test-only helpers

Functions used only in `#[cfg(test)]` blocks are invisible to the compiler in
normal builds, so they trigger `dead_code` warnings.

**Fix:** Gate them with `#[cfg(test)]`:
```rust
#[cfg(test)]
fn is_valid(result: &[i32]) -> bool { ... }

// For trait-based helpers, gate the trait AND all impls
#[cfg(test)]
trait Api { ... }
#[cfg(test)]
impl Api for my_solution::Foo { ... }
```

### Problem: Unused struct fields

A field that's stored during construction but never read afterward triggers a
warning.

**Fix:** Remove the field, or add a reader method.

---

## Quick Reference: Running Clippy

```bash
# Check all examples
cargo clippy --examples

# Auto-fix what clippy can
cargo clippy --fix --examples --allow-dirty

# Check a specific example
cargo clippy --example lc_0001_two_sum

# Strict mode (treat warnings as errors)
cargo clippy -- -D warnings
```
