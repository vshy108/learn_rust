// Demonstrates recursion in Rust.
//
// Rust supports recursion just like Python, but with key differences:
//   - No tail-call optimization (TCO) guaranteed by the compiler.
//   - Stack size is limited (~8 MB default on most platforms).
//   - Deep recursion can cause stack overflow (no automatic extension).
//
// For performance-critical code, prefer iteration or `stacker` crate.
//
// Run: cargo run --example s05_recursion

/// Classic factorial: n! = n * (n-1) * ... * 1, with 0! = 1.
///
/// # Panics
///
/// Panics if `n` is large enough to exceed u128::MAX.
fn factorial(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// Fibonacci using recursion (intentionally naive — O(2^n)).
///
/// Demonstrates the cost of naive recursion vs iteration.
fn fib_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n - 1) + fib_recursive(n - 2),
    }
}

/// Fibonacci using iteration — O(n) time, O(1) space.
///
/// This is how you'd actually compute Fibonacci in production.
fn fib_iterative(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}

/// Sum of a slice using recursion (divide and conquer style).
///
/// Shows how recursion processes collections. In practice, use `.iter().sum()`.
fn sum_slice(slice: &[i32]) -> i32 {
    match slice {
        [] => 0,
        [first, rest @ ..] => first + sum_slice(rest),
    }
}

/// Count down to zero, printing each step.
fn countdown(n: u32) {
    if n == 0 {
        println!("Go!");
        return;
    }
    println!("{n}...");
    countdown(n - 1);
}

fn main() {
    // --- 1. Factorial ---
    for n in 0..=10 {
        println!("{n}! = {}", factorial(n as u128));
    }

    // --- 2. Fibonacci ---
    println!("\nFibonacci (recursive + iterative):");
    for n in 0..=10 {
        let r = fib_recursive(n);
        let i = fib_iterative(n);
        assert_eq!(r, i);
        println!("fib({n}) = {r}");
    }

    // --- 3. Sum of a slice ---
    let data = [10, 20, 30, 40, 50];
    println!("\nsum of {data:?} = {}", sum_slice(&data));

    // --- 4. Countdown ---
    println!();
    countdown(3);

    // --- 5. Gotcha: stack overflow ---
    // Uncommenting the line below would crash:
    //   factorial(1_000_000);
    // Rust's default stack is ~8 MB. Deep recursion will overflow it.
    // Unlike Python (which raises RecursionError), Rust aborts the process.

    // --- 6. Gotcha: no guaranteed TCO ---
    // Some languages (Scheme, Haskell) guarantee tail-call optimization.
    // Rust does NOT. Even if your function is tail-recursive, the compiler
    // may or may not optimize it. For deep recursion, use iteration.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_base_cases() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_factorial_known_values() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    fn test_fib_known_values() {
        assert_eq!(fib_recursive(0), 0);
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(10), 55);
    }

    #[test]
    fn test_fib_recursive_matches_iterative() {
        for n in 0..=20 {
            assert_eq!(fib_recursive(n), fib_iterative(n), "mismatch at n={n}");
        }
    }

    #[test]
    fn test_sum_slice() {
        assert_eq!(sum_slice(&[]), 0);
        assert_eq!(sum_slice(&[1]), 1);
        assert_eq!(sum_slice(&[1, 2, 3, 4, 5]), 15);
    }
}
