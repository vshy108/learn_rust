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

## Types
- **`as` casts**: `num_rows as usize`, `(b[j] as char)`. Explicit, no implicit coercion.
- **`i64` for intermediate overflow**: Accumulate in wider type, clamp at the end.
- **`with_capacity(n)`**: Pre-allocate `String`/`Vec` when size known. Avoids reallocations.

## Clippy Tips
- Use `.and_then()` instead of `.map_or(None, |x| ...)`
- Remove unnecessary `as usize` casts when already `usize`
- Don't chain `.into_iter()` after `.chars()` / `.iter()`
