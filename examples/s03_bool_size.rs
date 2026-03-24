// Demonstrates the size of `bool` in Rust.
//
// A `bool` in Rust occupies 1 byte (8 bits), even though it only needs 1 bit.
// This is for alignment and addressability — CPUs can't address individual bits.
//
// Python's `bool` is a subclass of `int` and takes 28 bytes!
//
// Run: cargo run --example s03_bool_size

fn main() {
    // --- 1. Size of bool ---
    println!("size_of::<bool>() = {} byte", std::mem::size_of::<bool>()); // 1

    // --- 2. Bool values ---
    let t = true;
    let f = false;
    println!("true = {t}, false = {f}");

    // --- 3. Bool arithmetic ---
    // Unlike Python, Rust bools are NOT integers.
    // `true + true` would fail in Rust.
    // You must cast explicitly:
    println!("true as u8 = {}", true as u8); // 1
    println!("false as u8 = {}", false as u8); // 0

    // --- 4. Bool in conditions ---
    // Rust requires an actual `bool` for `if` conditions.
    // No "truthy/falsy" like Python.
    // `if 1 { ... }` won't compile — must be `if true { ... }`.
    let condition = 5 > 3;
    if condition {
        println!("5 > 3 is true (type is bool, not int)");
    }

    // --- 5. Bool operators ---
    // Logical: && (AND), || (OR), ! (NOT) — short-circuit evaluation
    // Bitwise: & (AND), | (OR), ^ (XOR) — no short-circuit
    println!();
    println!("Logical: true && false = {}", true && false);
    println!("Logical: true || false = {}", true || false);
    println!("Logical: !true = {}", !true);
    println!("Bitwise: true & false = {}", true & false);
    println!("Bitwise: true ^ true = {}", true ^ true);

    // --- 6. Bool size in arrays ---
    // Each bool in an array takes 1 byte.
    let arr = [true, false, true, false];
    println!();
    println!("4 bools array size: {} bytes", std::mem::size_of_val(&arr));
    // 4 bytes (1 byte each)

    // For bit-level packing, use a crate like `bitvec` or manual bitflags.

    // --- 7. Bool in structs (alignment) ---
    // A struct with a bool might take more than 1 byte due to alignment.
    #[derive(Debug)]
    struct Flags {
        a: bool,
        b: bool,
    }
    let flags = Flags { a: true, b: false };
    println!(
        "Flags struct size: {} bytes (contains 2 bools)",
        std::mem::size_of::<Flags>()
    );
    println!("flags = {flags:?}");
    // 2 bytes — no padding needed since bool alignment is 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bool_size() {
        assert_eq!(std::mem::size_of::<bool>(), 1);
    }

    #[test]
    fn test_bool_as_integer() {
        assert_eq!(true as u8, 1);
        assert_eq!(false as u8, 0);
    }

    #[test]
    fn test_logical_operators() {
        assert_eq!(true && false, false);
        assert_eq!(true || false, true);
        assert_eq!(!true, false);
    }

    #[test]
    fn test_bitwise_operators() {
        assert_eq!(true & false, false);
        assert_eq!(true | false, true);
        assert_eq!(true ^ true, false);
    }

    #[test]
    fn test_bool_array_size() {
        let arr = [true; 8];
        assert_eq!(std::mem::size_of_val(&arr), 8); // 1 byte per bool
    }
}
