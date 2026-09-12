// Demonstrates memory layout inspection and allocation-friendly patterns.

use std::mem::{align_of, size_of};

#[repr(C)]
struct Packet {
    tag: u8,
    id: u32,
}

#[repr(transparent)]
struct UserId(u64);

fn hot_path(x: u64) -> u64 {
    x.wrapping_mul(2)
}

fn build_values() -> Vec<u32> {
    let mut values = Vec::with_capacity(4);
    values.extend([1_u32, 2, 3, 4]);
    values
}

fn main() {
    println!("size_of::<Packet>() = {}", size_of::<Packet>());
    println!("align_of::<Packet>() = {}", align_of::<Packet>());

    let values = build_values();
    println!("values = {values:?}");

    let user_id = UserId(7);
    println!("hot_path = {}", hot_path(user_id.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_layout_has_expected_alignment_contract() {
        assert_eq!(align_of::<Packet>(), align_of::<u32>());
        assert!(size_of::<Packet>() >= size_of::<u8>() + size_of::<u32>());
        assert_eq!(size_of::<Packet>() % align_of::<Packet>(), 0);
    }

    #[test]
    fn hot_path_uses_wrapping_math() {
        assert_eq!(hot_path(7), 14);
        assert_eq!(hot_path(u64::MAX), u64::MAX.wrapping_mul(2));
    }

    #[test]
    fn build_values_returns_expected_data() {
        let values = build_values();
        assert_eq!(values, vec![1, 2, 3, 4]);
        assert!(values.capacity() >= 4);
    }
}
