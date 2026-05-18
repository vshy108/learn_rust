// Section 30: Rust 1.90-1.92 latest stable features
//
// Rust 1.0 was the first stable release back in 2015. This repo now uses
// rustc 1.95 locally, so this lesson highlights recent stable APIs that landed
// in the Rust 1.90-1.92 window and are practical in day-to-day code.
//
// Run:
//   cargo run --example s30_rust_190_192_features
//   cargo test --example s30_rust_190_192_features

use std::collections::BTreeMap;
use std::num::NonZeroU32;

// Rust 1.90: unsigned integers gained signed subtraction helpers.
// These avoid awkward `if delta < 0 { add } else { sub }` branches when an
// offset can move either direction but the stored coordinate/counter is unsigned.
pub fn move_index_checked(index: u32, offset: i32) -> Option<u32> {
    index.checked_sub_signed(offset)
}

pub fn move_index_saturating(index: u32, offset: i32) -> u32 {
    index.saturating_sub_signed(offset)
}

pub fn move_index_wrapping(index: u32, offset: i32) -> u32 {
    index.wrapping_sub_signed(offset)
}

// Rust 1.92: NonZero integers gained div_ceil.
// Useful when zero is an invalid state and you want the type to encode that.
pub fn page_count(total_items: NonZeroU32, page_size: NonZeroU32) -> NonZeroU32 {
    total_items.div_ceil(page_size)
}

// Rust 1.92: BTreeMap Entry has insert_entry, returning an occupied entry after
// insertion. That lets you insert and immediately inspect/modify in one flow.
pub fn insert_or_replace_score<'name>(
    scores: &mut BTreeMap<&'name str, u32>,
    name: &'name str,
    score: u32,
) -> (String, u32) {
    let entry = scores.entry(name).insert_entry(score);
    // FIX: the map borrow and key lifetime must stay separate. Returning owned
    // data lets the mutable borrow end here so callers can read the map again.
    (entry.key().to_string(), *entry.get())
}

// Rust 1.92: slice rotate_left/rotate_right are const-callable.
// This lets small lookup tables be transformed at compile time instead of in
// startup code.
pub const fn rotate_left_once(mut values: [u8; 4]) -> [u8; 4] {
    values.rotate_left(1);
    values
}

pub const fn rotate_right_once(mut values: [u8; 4]) -> [u8; 4] {
    values.rotate_right(1);
    values
}

const ROTATED_LEFT: [u8; 4] = rotate_left_once([1, 2, 3, 4]);
const ROTATED_RIGHT: [u8; 4] = rotate_right_once([1, 2, 3, 4]);

fn main() {
    println!("Rust 1.90-1.92 latest stable feature samples");

    println!("checked index move: {:?}", move_index_checked(10, -5));
    println!("saturating index move: {}", move_index_saturating(3, 10));
    println!("wrapping index move: {}", move_index_wrapping(0, 1));

    let total = NonZeroU32::new(101).expect("101 is non-zero");
    let page_size = NonZeroU32::new(20).expect("20 is non-zero");
    println!("page count: {}", page_count(total, page_size));

    let mut scores = BTreeMap::new();
    println!(
        "inserted: {:?}",
        insert_or_replace_score(&mut scores, "Ada", 95)
    );

    println!("const rotate left:  {:?}", ROTATED_LEFT);
    println!("const rotate right: {:?}", ROTATED_RIGHT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_sub_signed_moves_backward_for_positive_offset() {
        assert_eq!(move_index_checked(10, 3), Some(7));
    }

    #[test]
    fn checked_sub_signed_moves_forward_for_negative_offset() {
        assert_eq!(move_index_checked(10, -5), Some(15));
    }

    #[test]
    fn checked_sub_signed_returns_none_on_underflow() {
        assert_eq!(move_index_checked(3, 10), None);
    }

    #[test]
    fn saturating_sub_signed_clamps_at_zero() {
        assert_eq!(move_index_saturating(3, 10), 0);
    }

    #[test]
    fn wrapping_sub_signed_wraps_like_unsigned_arithmetic() {
        assert_eq!(move_index_wrapping(0, 1), u32::MAX);
    }

    #[test]
    fn nonzero_div_ceil_rounds_up() {
        let total = NonZeroU32::new(101).unwrap();
        let page_size = NonZeroU32::new(20).unwrap();
        assert_eq!(page_count(total, page_size).get(), 6);
    }

    #[test]
    fn nonzero_div_ceil_keeps_exact_division_exact() {
        let total = NonZeroU32::new(100).unwrap();
        let page_size = NonZeroU32::new(20).unwrap();
        assert_eq!(page_count(total, page_size).get(), 5);
    }

    #[test]
    fn btree_insert_entry_returns_inserted_entry() {
        let mut scores = BTreeMap::new();
        assert_eq!(
            insert_or_replace_score(&mut scores, "Ada", 95),
            ("Ada".to_string(), 95)
        );
        assert_eq!(scores.get("Ada"), Some(&95));
    }

    #[test]
    fn btree_insert_entry_replaces_existing_value() {
        let mut scores = BTreeMap::from([("Ada", 80)]);
        assert_eq!(
            insert_or_replace_score(&mut scores, "Ada", 99),
            ("Ada".to_string(), 99)
        );
        assert_eq!(scores.get("Ada"), Some(&99));
    }

    #[test]
    fn const_slice_rotate_left_works_at_compile_time() {
        assert_eq!(ROTATED_LEFT, [2, 3, 4, 1]);
    }

    #[test]
    fn const_slice_rotate_right_works_at_compile_time() {
        assert_eq!(ROTATED_RIGHT, [4, 1, 2, 3]);
    }
}
