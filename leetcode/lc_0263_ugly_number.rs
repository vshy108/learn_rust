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
// Run:  cargo run --example lc_0263_ugly_number
// Test: cargo test --example lc_0263_ugly_number
// Bench: cargo test --example lc_0263_ugly_number bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
            todo!()
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
                std::hint::black_box(crate::my_solution::Solution::is_ugly(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::is_ugly(
                    std::hint::black_box(n),
                ));
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
