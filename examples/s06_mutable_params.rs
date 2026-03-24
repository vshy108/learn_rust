// Demonstrates mutable parameters in functions.
//
// Functions can take `&mut T` to modify values without taking ownership.
// The caller must declare the variable as `mut` and pass `&mut var`.
//
// Rules:
// - Only one `&mut` reference to a value at a time (exclusive access).
// - Cannot have `&T` and `&mut T` alive simultaneously.
// - The mutable borrow ends at its last use (Non-Lexical Lifetimes).
//
// Run: cargo run --example s06_mutable_params

/// Appends an exclamation mark to a String through a mutable reference.
///
/// # Arguments
///
/// * `s` - Exclusive mutable reference to a String
fn add_excitement(s: &mut String) {
    s.push('!');
}

/// Doubles every element in a vector through a mutable reference.
///
/// # Arguments
///
/// * `v` - Exclusive mutable reference to a Vec of i32
fn double_elements(v: &mut Vec<i32>) {
    for elem in v.iter_mut() {
        *elem *= 2;
    }
}

/// Swaps two values using mutable references.
///
/// # Arguments
///
/// * `a` - Mutable reference to first value
/// * `b` - Mutable reference to second value
fn swap_values(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}

fn main() {
    // --- 1. Basic &mut String ---
    let mut greeting = String::from("Hello");
    add_excitement(&mut greeting);
    println!("{greeting}"); // Hello!

    // Can call it again — each mutable borrow is short-lived.
    add_excitement(&mut greeting);
    println!("{greeting}"); // Hello!!

    // --- 2. Mutating a Vec ---
    let mut numbers = vec![1, 2, 3, 4];
    println!("Before: {numbers:?}");
    double_elements(&mut numbers);
    println!("After:  {numbers:?}"); // [2, 4, 6, 8]

    // --- 3. Swapping values ---
    let mut x: i32 = 10;
    let mut y: i32 = 20;
    swap_values(&mut x, &mut y);
    println!("x={x}, y={y}"); // x=20, y=10

    // --- 4. Only one &mut at a time ---
    let mut data = String::from("exclusive");
    let r1 = &mut data;
    // let r2 = &mut data; // error: second mutable borrow
    r1.push_str("!");
    println!("{data}");

    // --- 5. &mut ends at last use → new borrow OK ---
    let mut s = String::from("nll");
    let r1 = &mut s;
    r1.push_str("1"); // last use of r1
    // r1 is done — we can borrow again:
    let r2 = &mut s;
    r2.push_str("2");
    println!("{s}"); // nll12

    // --- 6. Pattern: modify and return info ---
    let mut v = vec![5, 3, 8, 1];
    let sum = sort_and_sum(&mut v);
    println!("Sorted: {v:?}, sum: {sum}");
}

/// Sorts a vector in place and returns the sum.
///
/// # Arguments
///
/// * `v` - Mutable reference to a Vec of i32
///
/// # Returns
///
/// Sum of all elements
fn sort_and_sum(v: &mut Vec<i32>) -> i32 {
    v.sort();
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_excitement() {
        let mut s = String::from("test");
        add_excitement(&mut s);
        assert_eq!(s, "test!");
    }

    #[test]
    fn test_double_elements() {
        let mut v = vec![1, 2, 3];
        double_elements(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_swap_values() {
        let mut a: i32 = 1;
        let mut b: i32 = 2;
        swap_values(&mut a, &mut b);
        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn test_sort_and_sum() {
        let mut v = vec![3, 1, 2];
        let sum = sort_and_sum(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
        assert_eq!(sum, 6);
    }
}
