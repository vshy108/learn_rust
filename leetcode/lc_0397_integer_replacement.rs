// LeetCode 397: Integer Replacement [Medium]
// https://leetcode.com/problems/integer-replacement/
//
// Benchmark: 1,000,000 iterations (2026-04-08)
//   My solution (greedy bit ops):  avg 11ns
//   AI solution (recursive memo):  avg 3.08us
//   -> My solution is 266.61x faster
//
// Given a positive integer n, apply one of these operations:
//   - If n is even, replace n with n / 2
//   - If n is odd, replace n with n + 1 or n - 1
//
// Return the minimum number of operations to reach 1.
//
// Examples:
//   8 -> 3  (8 -> 4 -> 2 -> 1)
//   7 -> 4  (7 -> 8 -> 4 -> 2 -> 1  or  7 -> 6 -> 3 -> 2 -> 1)
//   4 -> 2  (4 -> 2 -> 1)
//
// Constraints:
//   - 1 <= n <= 2^31 - 1
//
// Run:  cargo run --example lc_0397_integer_replacement
// Test: cargo test --example lc_0397_integer_replacement
// Bench: cargo test --example lc_0397_integer_replacement bench -- --nocapture

/// My solution module — greedy bit manipulation.
///
/// Even: always divide by 2.
/// Odd: look at the last 2 bits.
///   - If `n & 3 == 3` and `n != 3`, adding 1 creates a trailing-zero run
///     (more future halvings). Otherwise subtract 1.
///
/// Use i64 to avoid overflow when n = i32::MAX and we do n+1.
///
/// Time:  O(log n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum operations to reduce n to 1.
        ///
        /// # Arguments
        ///
        /// * `n` - Positive integer
        ///
        /// # Returns
        ///
        /// Minimum number of operations
        pub fn integer_replacement(n: i32) -> i32 {
            // Use i64 because n+1 can overflow when n = i32::MAX
            let mut n = n as i64;
            let mut ops = 0;
            while n != 1 {
                if n & 1 == 0 {
                    n >>= 1;
                } else if n != 3 && (n & 3) == 3 {
                    // Bits ...11 — adding 1 creates more trailing zeros
                    n += 1;
                } else {
                    // Bits ...01 or n==3 — subtract 1
                    n -= 1;
                }
                ops += 1;
            }
            ops
        }
    }
}

/// AI solution module — recursive with memoization.
///
/// Straightforward recursive approach: even -> f(n/2)+1, odd -> min of
/// f(n+1)+1 and f(n-1)+1. Memoize to avoid redundant computation.
///
/// Time:  O(log n) — each value is computed once
/// Space: O(log n) — memo + recursion stack
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return minimum operations to reduce n to 1.
        ///
        /// # Arguments
        ///
        /// * `n` - Positive integer
        ///
        /// # Returns
        ///
        /// Minimum number of operations
        pub fn integer_replacement(n: i32) -> i32 {
            let mut memo = HashMap::new();
            Self::helper(n as i64, &mut memo)
        }

        /// Recursive helper with memoization.
        fn helper(n: i64, memo: &mut HashMap<i64, i32>) -> i32 {
            if n == 1 {
                return 0;
            }
            if let Some(&cached) = memo.get(&n) {
                return cached;
            }

            let result = if n % 2 == 0 {
                Self::helper(n / 2, memo) + 1
            } else {
                let add = Self::helper(n + 1, memo);
                let sub = Self::helper(n - 1, memo);
                add.min(sub) + 1
            };
            memo.insert(n, result);
            result
        }
    }
}

fn main() {
    let cases = [(8, 3), (7, 4), (4, 2), (1, 0)];

    println!("--- My Solution ---");
    for (n, expected) in &cases {
        let result = my_solution::Solution::integer_replacement(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (n, expected) in &cases {
        let result = ai_solution::Solution::integer_replacement(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_int_replace {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_8() {
                    assert_eq!(Solution::integer_replacement(8), 3);
                }

                #[test]
                fn test_7() {
                    assert_eq!(Solution::integer_replacement(7), 4);
                }

                #[test]
                fn test_4() {
                    assert_eq!(Solution::integer_replacement(4), 2);
                }

                #[test]
                fn test_1() {
                    assert_eq!(Solution::integer_replacement(1), 0);
                }

                #[test]
                fn test_3() {
                    // 3 -> 2 -> 1 (subtract is better than 3 -> 4 -> 2 -> 1)
                    assert_eq!(Solution::integer_replacement(3), 2);
                }

                #[test]
                fn test_max_i32() {
                    // Edge case: n = 2^31 - 1 = 2147483647
                    assert_eq!(
                        Solution::integer_replacement(2_147_483_647),
                        32
                    );
                }

                #[test]
                fn test_power_of_two() {
                    assert_eq!(Solution::integer_replacement(1024), 10);
                }
            }
        };
    }

    test_int_replace!(my_tests, crate::my_solution::Solution);
    test_int_replace!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 1_000_000_u32;
        let n = 2_147_483_647;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::integer_replacement(
                    std::hint::black_box(n),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::integer_replacement(
                    std::hint::black_box(n),
                ),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
