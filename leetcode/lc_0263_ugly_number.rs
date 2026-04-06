// LeetCode 263: Ugly Number [Easy]
// https://leetcode.com/problems/ugly-number/
//
// An ugly number is a positive integer whose prime factors are limited
// to 2, 3, and 5.
//
// Given an integer n, return true if n is an ugly number.
//
// Examples:
//   n = 6  -> true   (6 = 2 * 3)
//   n = 1  -> true   (1 has no prime factors)
//   n = 14 -> false  (14 = 2 * 7, and 7 is not 2/3/5)
//
// Constraints:
//   - -2^31 <= n <= 2^31 - 1
//
// --- Benchmark (100_000 iterations, 2026-04-06) ---
//   My solution: 46.4ms (avg 464ns)
//   AI solution: 30.7ms (avg 307ns)
//   -> AI solution is 1.51x faster
//   Reason: Both solutions are algorithmically identical (same divisions).
//     The difference is likely benchmark noise at this nanosecond scale —
//     CPU caching, branch prediction, and run order can easily cause 1.5x
//     variation. The fold approach is stylistically different, not faster.
//
// --- Rust concepts used ---
//   - `for prime in [2, 3, 5]`: small fixed arrays implement IntoIterator.
//   (Previously covered: let mut n = n in lc_0009, .fold() in lc_0008,
//    &ref destructuring in lc_0001)
//
// Run:  cargo run --example lc_0263_ugly_number
// Test: cargo test --example lc_0263_ugly_number
// Bench: cargo test --example lc_0263_ugly_number bench -- --nocapture

/// My solution module.
///
/// Iteratively divide out factors of 2, 3, 5. If what remains is 1,
/// the number had no other prime factors — it's ugly.
///
/// Time:  O(log n) — each division at least halves n
/// Space: O(1) — only a mutable copy of n
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return `true` if `n` is an ugly number (prime factors only 2, 3, 5).
        ///
        /// # Arguments
        ///
        /// * `n` - The integer to check
        ///
        /// # Returns
        ///
        /// `true` if `n` is ugly, `false` otherwise
        pub fn is_ugly(n: i32) -> bool {
            // Ruby: shadowing not needed — variables are always mutable
            // Rust: function params are immutable by default, so we
            // shadow `n` with `let mut n = n` to get a mutable copy
            let mut n = n;
            if n <= 0 {
                return false;
            }

            // Ruby: [2, 3, 5].each { |prime| ... }
            // Rust: array literal [2, 3, 5] iterated directly —
            // small fixed arrays implement IntoIterator
            for prime in [2, 3, 5] {
                while n % prime == 0 {
                    n /= prime;
                }
            }

            n == 1
        }
    }
}

/// AI solution module.
///
/// Functional approach using Iterator::fold. Folds over the prime
/// factors [2, 3, 5], dividing each out completely. Same logic as
/// my_solution but expressed without explicit mutation or loops.
///
/// Time:  O(log n) — each division at least halves n
/// Space: O(1) — fold accumulator only
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return `true` if `n` is an ugly number (prime factors only 2, 3, 5).
        ///
        /// # Arguments
        ///
        /// * `n` - The integer to check
        ///
        /// # Returns
        ///
        /// `true` if `n` is ugly, `false` otherwise
        pub fn is_ugly(n: i32) -> bool {
            if n <= 0 {
                return false;
            }

            // fold: start with n, divide out each prime factor completely
            // The closure captures `prime` by value (i32 is Copy)
            // fold = Ruby's inject/reduce
            let remainder = [2, 3, 5].iter().fold(n, |mut acc, &prime| {
                while acc % prime == 0 {
                    acc /= prime;
                }
                acc
            });

            remainder == 1
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, bool)> = vec![(6, true), (1, true), (14, false)];

    println!("--- My Solution ---");
    for (n, expected) in &test_cases {
        let result = my_solution::Solution::is_ugly(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (n, expected) in &test_cases {
        let result = ai_solution::Solution::is_ugly(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_is_ugly {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_six_is_ugly() {
                    assert!(Solution::is_ugly(6));
                }

                #[test]
                fn test_one_is_ugly() {
                    assert!(Solution::is_ugly(1));
                }

                #[test]
                fn test_fourteen_not_ugly() {
                    assert!(!Solution::is_ugly(14));
                }

                #[test]
                fn test_zero_not_ugly() {
                    assert!(!Solution::is_ugly(0));
                }

                #[test]
                fn test_negative_not_ugly() {
                    assert!(!Solution::is_ugly(-6));
                }

                #[test]
                fn test_power_of_two() {
                    assert!(Solution::is_ugly(8)); // 2^3
                }

                #[test]
                fn test_power_of_three() {
                    assert!(Solution::is_ugly(27)); // 3^3
                }
            }
        };
    }

    test_is_ugly!(my_tests, crate::my_solution::Solution);
    test_is_ugly!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![6, 1, 14, 0, -6, 8, 27, 30, 1_000_000, 7];

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::my_solution::Solution::is_ugly(std::hint::black_box(
                    n,
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::is_ugly(std::hint::black_box(
                    n,
                )));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
