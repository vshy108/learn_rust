// Topic 130: Module as file.
//
// This file IS the `math` module. It's declared in lib.rs with:
//   pub mod math;
// The compiler looks for either `math.rs` or `math/mod.rs`.

/// Adds two numbers.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
///
/// The sum
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Computes a^n for non-negative integer exponents.
pub fn power(a: i64, n: u32) -> i64 {
    a.pow(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2, 10), 1024);
    }
}
