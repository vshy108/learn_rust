// Demonstrates enum memory layout.
//
// Rust enums use a tag (discriminant) + the largest variant's data:
//
//   Size of enum = size of tag + size of largest variant (aligned)
//
// The tag tells Rust which variant is currently stored.
//
// Special optimization: Option<&T> and Option<Box<T>> use the null
// pointer as None, so they're the same size as a raw pointer!
//
// Run: cargo run --example s10_enum_memory

use std::mem::size_of;

#[derive(Debug)]
enum Small {
    A,     // no data
    B(u8), // 1 byte
}

#[derive(Debug)]
enum Medium {
    X(u8),  // 1 byte
    Y(u32), // 4 bytes
    Z(u64), // 8 bytes
}

#[derive(Debug)]
enum Large {
    Data([u8; 100]), // 100 bytes
    Empty,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // 4 bytes
    V6(String),         // 24 bytes (String = ptr + len + capacity)
}

fn main() {
    // --- 1. Size of simple enums ---
    println!("Size of Small: {} bytes", size_of::<Small>());
    println!("  (tag + max variant data, aligned)");

    println!("Size of Medium: {} bytes", size_of::<Medium>());
    println!("  (tag + 8 bytes for u64 variant, with alignment padding)");

    println!("Size of Large: {} bytes", size_of::<Large>());
    println!("  (must hold the 100-byte array variant)");

    // --- 2. Enum with String ---
    println!("Size of IpAddr: {} bytes", size_of::<IpAddr>());
    println!("Size of String: {} bytes", size_of::<String>());

    // --- 3. Null pointer optimization ---
    // Option<&T> is the same size as &T because Rust uses null for None.
    println!("\nNull pointer optimization:");
    println!("  &u64:          {} bytes", size_of::<&u64>());
    println!("  Option<&u64>:  {} bytes", size_of::<Option<&u64>>());
    println!("  Box<u64>:      {} bytes", size_of::<Box<u64>>());
    println!(
        "  Option<Box<u64>>: {} bytes",
        size_of::<Option<Box<u64>>>()
    );
    // Both are same size — no overhead for Option on references/Box!

    // --- 4. C-like enum (just integers) ---
    #[derive(Debug)]
    #[repr(u8)] // force the discriminant to be u8
    enum Color {
        Red = 0,
        Green = 1,
        Blue = 2,
    }
    println!("\nSize of Color (repr u8): {} bytes", size_of::<Color>());

    // --- 5. Discriminant values ---
    println!("Color::Red   = {}", Color::Red as u8);
    println!("Color::Green = {}", Color::Green as u8);
    println!("Color::Blue  = {}", Color::Blue as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_pointer_opt() {
        // Option<&T> should be the same size as &T
        assert_eq!(size_of::<Option<&u64>>(), size_of::<&u64>());
        assert_eq!(size_of::<Option<Box<u64>>>(), size_of::<Box<u64>>());
    }

    #[test]
    fn test_small_size() {
        // Small has variants: no-data and u8. Tag + u8 ≤ 2 bytes.
        assert!(size_of::<Small>() <= 2);
    }

    #[test]
    fn test_large_holds_array() {
        // Large must be at least 100 bytes for the array variant.
        assert!(size_of::<Large>() >= 100);
    }
}
