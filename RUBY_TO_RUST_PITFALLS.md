# Ruby to Rust Pitfalls (LeetCode)

Common gotchas when converting Ruby solutions to Rust.

## Strings
- **No `s[i]` indexing**: `String`/`&str` can't be indexed by `usize` (UTF-8 is multi-byte). Use `s.as_bytes()[i]` for byte access or `s.chars().nth(i)` (O(n)).
- **Byte literals**: Compare with `b' '`, `b'0'`, `b'-'` (type `u8`), not `' '` (type `char`). Ruby: `' '.ord`.
- **No `.ord`**: Ruby `s[i].ord - '0'.ord` → Rust `(s[i] - b'0') as i32`.
- **`.is_ascii_digit()`**: Works on both `u8` and `char`. Equivalent to `>= b'0' && <= b'9'`.
- **`push` vs `<<`**: Ruby `<<` → Rust `.push(char)` or `.push_str(&str)`.
- **`String` vs `&str`**: `String` is owned/growable, `&str` is a borrowed slice. `vec![""; n]` gives `&str` (can't push). Use `vec![String::new(); n]`.

## Numbers
- **`usize` can't go negative**: `curr_row -= 1` at 0 panics. Guard with conditionals.
- **`i32::MIN.abs()` panics**: `|-2_147_483_648|` = `2_147_483_648` > `i32::MAX`. Guard with `if x == i32::MIN`.
- **`%` preserves sign**: Rust `-123 % 10 == -3` (not 7 like some languages). Works for sign-preserving reversal.
- **No implicit overflow**: Debug mode panics on overflow. Use `checked_mul`/`checked_add` or `saturating_*` for safe arithmetic.
- **Overflow check rearrangement**: `result * 10 + digit > MAX` → `result > (MAX - digit) / 10` to avoid overflow during the check itself.

## Control Flow
- **No ternary `?:`**: Ruby `x ? a : b` → Rust `if x { a } else { b }` (if is an expression).
- **No trailing `if`**: Ruby `return 0 if x < 0` → Rust `if x < 0 { return 0; }`.
- **`let = if` expression**: `let val = if cond { a } else { b };` — assign from if.
- **`match` for multi-branch**: Replaces Ruby `case/when`. Must be exhaustive.

## Option / Nil
- **No `nil`**: Rust has no null. Use `Option<T>` for "maybe absent" values.
- **`None == None` is true**: `Option::get()` returns `Option` — comparing two `None`s is true. Watch out in expansion loops.
- **`.unwrap()` panics**: Use `match`, `if let`, or `?` operator instead.
- **`and_then` = Ruby's `&.` (safe navigation)**: Chains `Option`-returning operations. If the first is `None`, short-circuits — second call never happens.
  ```rust
  // Ruby: fast.next&.next       (nil if fast.next is nil)
  // Rust:  next(fast).and_then(|f| next(f))  (None if first hop is None)
  ```
  - `map` vs `and_then`: `map` wraps the result in `Some`; `and_then` expects the closure to return `Option` (avoids `Some(Some(...))` nesting).
  - Also known as `flat_map` / `flatmap` in other languages.

## Iterators
- **`.into_iter()` on `.chars()` is redundant**: `.chars()` already returns an iterator.
- **`iter()` vs `into_iter()`**: Ruby `.each` borrows implicitly. Rust has three:
  - `v.iter()` → `&T` (like Ruby `.each`, vec still usable)
  - `v.into_iter()` → `T` (consumes vec — no Ruby equivalent, vec is gone)
  - `v.iter_mut()` → `&mut T` (like Ruby `.map!`, mutate in place)
  - **Gotcha**: `v.iter().sum()` needs turbofish `.sum::<i32>()` (iterates `&i32`). `v.into_iter().sum()` infers the type automatically.
- **`enumerate()` order is `(index, element)`**: Opposite of Ruby's `each_with_index { |element, index| }`. Easy to mix up.
- **`peekable()`**: Look ahead without consuming — useful for sign detection.
- **`take_while` + `fold`**: Replace manual while loops. `fold` = Ruby's `inject`.
- **`clamp(min, max)`**: Built-in on numeric types. Replaces Ruby `[[val, max].min, min].max`.
- **Don't chain `.into_iter()` on things that are already iterators**:
  - Already iterators: `0..n` (Range), `.chars()`, `.bytes()`, `.iter()`, `.enumerate()`, `.rev()`, `.zip()`
  - NOT iterators (need `.iter()` / `.into_iter()`): `Vec<T>`, `&[T]`, `String`, `HashMap`, `HashSet`
  - Rule: if it *returns* an iterator, don't add `.into_iter()` — it's redundant and clippy warns.
  - Example: `(0..n).into_iter().rev()` → just `(0..n).rev()`

## Types
- **`as` casts**: `num_rows as usize`, `(b[j] as char)`. Explicit, no implicit coercion.
- **`i64` for intermediate overflow**: Accumulate in wider type, clamp at the end.
- **`with_capacity(n)`**: Pre-allocate `String`/`Vec` when size known. Avoids reallocations.

## Collections: HashSet vs HashMap
- **`HashSet<T>`**: Stores keys only. Ruby's `Set`. Use for "does this exist?" checks.
- **`HashMap<K,V>`**: Stores key-value pairs. Ruby's `Hash`. Use for "what's associated with this key?".
- **`HashSet` is a `HashMap<T, ()>` internally**: Same performance, just no value stored.
- **Lookup/remove take `&T`, not `T`**: Ruby `set.include?(val)` / `hash[key]` → Rust `set.contains(&val)` / `map.get(&key)`. Only needs to compare, no ownership transfer.
- **Insert return values differ**:
  - `HashSet::insert(val)` → `bool` (`false` = already existed). Ruby's `Set.add?(val)` returns `nil` on duplicate.
  - `HashMap::insert(key, val)` → `Option<V>` (previous value if any). Ruby's `hash[key] = val` returns the new value.
- **`with_capacity(n)`**: Pre-allocate to avoid rehashing. `HashSet::new()` starts at capacity 0.

## Smart Pointers: Rc, RefCell, and Tree Nodes
- **Why `Option<Rc<RefCell<TreeNode>>>`**: Ruby objects are GC'd and freely shared/mutated. Rust needs explicit layers:
  - `Option<...>` — might be nil (`None`)
  - `Rc<...>` — shared ownership (multiple pointers to same node, reference counted). Ruby: implicit via GC.
  - `RefCell<...>` — interior mutability (runtime borrow check). Ruby: not needed, all objects are always mutable.
  - `TreeNode` — the actual data
- **`Rc<T>` only gives `&T`**: You can't mutate through `Rc` alone. `RefCell` lets you get `&mut T` at runtime.
- **`.borrow()` / `.borrow_mut()`**: RefCell's runtime borrow check — panics if you mix read+write borrows on the same node simultaneously.
- **`.clone()` on `Rc` is cheap**: Just bumps a reference count (like Ruby's object references). No deep copy.
- **`.clone()` on `Option<Rc<...>>` is needed** to reuse a node reference later — can't move out of a borrowed `RefCell`.
- **`while let Some(n) = node`**: Replaces Ruby's `while node` — Rust has no truthy nil, must pattern-match `Option`.
- **Accessing children**: `n.borrow().left.clone()` — `.borrow()` for RefCell, `.clone()` because you can't move a field out of a borrow.

## Clippy Tips
- Use `.and_then()` instead of `.map_or(None, |x| ...)`
- Remove unnecessary `as usize` casts when already `usize`
- Don't chain `.into_iter()` after `.chars()` / `.iter()`
