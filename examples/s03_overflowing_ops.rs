// Demonstrates Rust's overflow-aware integer arithmetic methods.
//
// In debug mode, Rust panics on integer overflow (e.g., `250u8 + 10` panics).
// In release mode (`--release`), overflow silently wraps (two's complement).
//
// To control overflow behavior explicitly, Rust provides four families
// of arithmetic methods on every integer type:
//
//   | Method family    | On overflow...                          |
//   |------------------|-----------------------------------------|
//   | `wrapping_*`     | Wraps (two's complement)                |
//   | `checked_*`      | Returns `None`                          |
//   | `saturating_*`   | Clamps to the type's min/max            |
//   | `overflowing_*`  | Wraps AND returns a bool (did overflow?) |
//
// These methods make overflow handling explicit — no silent bugs.

fn main() {
    // --- overflowing_add ---
    // Returns a tuple: (wrapped_result, did_overflow)
    // 250 + 10 = 260, but u8 max is 255, so it wraps: 260 - 256 = 4.
    // Use `{:?}` (Debug format) because tuples don't implement `Display`.
    let (result, overflowed) = 250u8.overflowing_add(10);
    println!("overflowing_add: 250u8 + 10 = ({result}, {overflowed})");
    // => overflowing_add: 250u8 + 10 = (4, true)

    // No overflow case: the bool is `false`.
    let (result, overflowed) = 100u8.overflowing_add(50);
    println!("overflowing_add: 100u8 + 50 = ({result}, {overflowed})");
    // => overflowing_add: 100u8 + 50 = (150, false)

    // --- overflowing_sub ---
    // 10 - 20 on u8 underflows: wraps to 10 + 256 - 20 = 246.
    let (result, overflowed) = 10u8.overflowing_sub(20);
    println!("overflowing_sub: 10u8 - 20 = ({result}, {overflowed})");
    // => overflowing_sub: 10u8 - 20 = (246, true)

    // --- overflowing_mul ---
    // 200 * 2 = 400, wraps to 400 - 256 = 144.
    let (result, overflowed) = 200u8.overflowing_mul(2);
    println!("overflowing_mul: 200u8 * 2 = ({result}, {overflowed})");
    // => overflowing_mul: 200u8 * 2 = (144, true)

    println!();

    // --- wrapping_* (silently wraps, no bool) ---
    let val = 250u8.wrapping_add(10);
    println!("wrapping_add: 250u8 + 10 = {val}");
    // => wrapping_add: 250u8 + 10 = 4

    // --- checked_* (returns Option: None on overflow) ---
    let safe = 100u8.checked_add(50);
    let overflow = 250u8.checked_add(10);
    println!("checked_add: 100u8 + 50 = {safe:?}");
    // => checked_add: 100u8 + 50 = Some(150)
    println!("checked_add: 250u8 + 10 = {overflow:?}");
    // => checked_add: 250u8 + 10 = None

    // --- saturating_* (clamps at min/max, never wraps) ---
    let val = 250u8.saturating_add(10);
    println!("saturating_add: 250u8 + 10 = {val}");
    // => saturating_add: 250u8 + 10 = 255

    let val = 5u8.saturating_sub(20);
    println!("saturating_sub: 5u8 - 20 = {val}");
    // => saturating_sub: 5u8 - 20 = 0

    println!();

    // --- Signed overflow: i8 range is -128..=127 ---
    let (result, overflowed) = 120i8.overflowing_add(20);
    println!("overflowing_add: 120i8 + 20 = ({result}, {overflowed})");
    // => overflowing_add: 120i8 + 20 = (-116, true)

    // Gotcha: `overflowing_neg` on i8::MIN overflows because
    // -(-128) = 128, which exceeds i8::MAX (127).
    let (result, overflowed) = i8::MIN.overflowing_neg();
    println!("overflowing_neg: -(-128i8) = ({result}, {overflowed})");
    // => overflowing_neg: -(-128i8) = (-128, true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_overflowing_add_overflow() {
        let (result, overflowed) = 250u8.overflowing_add(10);
        assert_eq!(result, 4);
        assert!(overflowed);
    }

    #[test]
    fn test_overflowing_add_no_overflow() {
        let (result, overflowed) = 100u8.overflowing_add(50);
        assert_eq!(result, 150);
        assert!(!overflowed);
    }

    #[test]
    fn test_checked_add_returns_none_on_overflow() {
        assert_eq!(250u8.checked_add(10), None);
    }

    #[test]
    fn test_saturating_add_clamps_at_max() {
        assert_eq!(250u8.saturating_add(10), u8::MAX);
    }

    #[test]
    fn test_signed_overflowing_neg_min() {
        // -(-128) wraps back to -128 for i8.
        let (result, overflowed) = i8::MIN.overflowing_neg();
        assert_eq!(result, i8::MIN);
        assert!(overflowed);
    }
}
