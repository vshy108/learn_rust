// LeetCode 279: Perfect Squares [Medium]
// https://leetcode.com/problems/perfect-squares/
//
// Given an integer n, return the least number of perfect square numbers
// that sum to n.
//
// A perfect square is an integer that is the square of an integer; in
// other words, it is the product of some integer with itself. For
// example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.
//
// Examples:
//   n = 12 -> 3  (12 = 4 + 4 + 4)
//   n = 13 -> 2  (13 = 4 + 9)
//
// Constraints:
//   - 1 <= n <= 10^4
//
// Run:  cargo run --example lc_0279_perfect_squares
// Test: cargo test --example lc_0279_perfect_squares
// Bench: cargo test --example lc_0279_perfect_squares bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the least number of perfect squares that sum to `n`.
        ///
        /// # Arguments
        ///
        /// * `n` - The target sum
        ///
        /// # Returns
        ///
        /// Minimum count of perfect squares summing to `n`
        pub fn num_squares(n: i32) -> i32 {
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
        /// Return the least number of perfect squares that sum to `n`.
        ///
        /// # Arguments
        ///
        /// * `n` - The target sum
        ///
        /// # Returns
        ///
        /// Minimum count of perfect squares summing to `n`
        pub fn num_squares(n: i32) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32)> = vec![(12, 3), (13, 2)];

    println!("--- My Solution ---");
    for &(n, expected) in &test_cases {
        let result = my_solution::Solution::num_squares(n);
        println!("  n={n} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for &(n, expected) in &test_cases {
        let result = ai_solution::Solution::num_squares(n);
        println!("  n={n} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_num_squares {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_twelve() {
                    assert_eq!(Solution::num_squares(12), 3); // 4+4+4
                }

                #[test]
                fn test_thirteen() {
                    assert_eq!(Solution::num_squares(13), 2); // 4+9
                }

                #[test]
                fn test_one() {
                    assert_eq!(Solution::num_squares(1), 1); // 1
                }

                #[test]
                fn test_perfect_square() {
                    assert_eq!(Solution::num_squares(16), 1); // 16
                }

                #[test]
                fn test_two_squares() {
                    assert_eq!(Solution::num_squares(5), 2); // 1+4
                }

                #[test]
                fn test_four_needed() {
                    // By Lagrange's four-square theorem, every natural number
                    // can be represented as the sum of at most 4 perfect squares.
                    assert_eq!(Solution::num_squares(7), 4); // 4+1+1+1
                }

                #[test]
                fn test_larger() {
                    assert_eq!(Solution::num_squares(100), 1); // 100 = 10^2
                }
            }
        };
    }

    test_num_squares!(my_tests, crate::my_solution::Solution);
    test_num_squares!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;
        let inputs: Vec<i32> = vec![12, 13, 1, 16, 100, 9999];

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::my_solution::Solution::num_squares(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::num_squares(
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
