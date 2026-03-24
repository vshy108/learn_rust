// Demonstrates augmented assignment operators in Rust.
//
// Augmented assignment operators combine an operation with assignment:
//   `x += 1` is equivalent to `x = x + 1`.
//
// Rust supports: +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=
//
// Unlike Python, Rust does NOT have `x++` or `x--` (increment/decrement).
// Unlike Python, the variable MUST be `mut`.
//
// Run: cargo run --example s03_augmented_assignment

fn main() {
    // --- 1. Arithmetic augmented assignment ---
    let mut x = 10;
    x += 5; // x = x + 5
    println!("after +=: {x}"); // 15
    x -= 3; // x = x - 3
    println!("after -=: {x}"); // 12
    x *= 2; // x = x * 2
    println!("after *=: {x}"); // 24
    x /= 4; // x = x / 4
    println!("after /=: {x}"); // 6
    x %= 4; // x = x % 4
    println!("after %=: {x}"); // 2

    // --- 2. Bitwise augmented assignment ---
    let mut bits: u8 = 0b1100;
    bits &= 0b1010; // AND
    println!("after &=: {:04b}", bits); // 1000
    bits |= 0b0011; // OR
    println!("after |=: {:04b}", bits); // 1011
    bits ^= 0b1111; // XOR
    println!("after ^=: {:04b}", bits); // 0100

    // --- 3. Shift augmented assignment ---
    let mut val: u32 = 1;
    val <<= 4; // left shift by 4
    println!("after <<=4: {val}"); // 16
    val >>= 2; // right shift by 2
    println!("after >>=2: {val}"); // 4

    // --- 4. No ++ or -- operators ---
    // Rust deliberately omits `++` and `--` to avoid ambiguity.
    // Use `x += 1` and `x -= 1` instead.
    let mut counter = 0;
    counter += 1; // instead of counter++
    counter += 1;
    println!("counter: {counter}"); // 2

    // --- 5. Must be mut ---
    // Augmented assignment requires the variable to be `mut`.
    // let y = 5;
    // y += 1;  // error[E0384]: cannot assign twice to immutable variable

    // --- 6. No chained assignment ---
    // Unlike Python's `a = b = c = 0`, Rust doesn't support chained assignment.
    // Each variable must be declared separately.
    let a = 0;
    let b = 0;
    let c = 0;
    println!("a={a}, b={b}, c={c}");

    // --- Gotcha: augmented assignment is NOT an expression ---
    // In Rust, `x += 1` returns `()`, not the new value.
    // You can't write `let y = (x += 1);` meaningfully.
    // Python's augmented assignment also has no useful return value,
    // so this gotcha isn't surprising.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_arithmetic_augmented() {
        let mut x = 10;
        x += 5;
        assert_eq!(x, 15);
        x -= 3;
        assert_eq!(x, 12);
        x *= 2;
        assert_eq!(x, 24);
        x /= 4;
        assert_eq!(x, 6);
        x %= 4;
        assert_eq!(x, 2);
    }

    #[test]
    fn test_bitwise_augmented() {
        let mut b: u8 = 0b1100;
        b &= 0b1010;
        assert_eq!(b, 0b1000);
        b |= 0b0011;
        assert_eq!(b, 0b1011);
        b ^= 0b1111;
        assert_eq!(b, 0b0100);
    }

    #[test]
    fn test_shift_augmented() {
        let mut v: u32 = 1;
        v <<= 4;
        assert_eq!(v, 16);
        v >>= 2;
        assert_eq!(v, 4);
    }

    #[test]
    fn test_augmented_returns_unit() {
        let mut x = 0;
        let result = {
            x += 1;
        };
        assert_eq!(result, ()); // returns unit
        assert_eq!(x, 1);
    }
}
