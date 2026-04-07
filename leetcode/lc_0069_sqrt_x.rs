// LeetCode 69: Sqrt(x) [Easy]
// https://leetcode.com/problems/sqrtx/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.161us
//   AI solution avg: 0.120us
//   Ratio: AI 1.34x faster
//
// Given a non-negative integer x, return the integer part of sqrt(x).
//
// Example 1: x=4 -> 2
// Example 2: x=8 -> 2 (sqrt(8) = 2.828...)
//
// Constraints: 0 <= x <= 2^31 - 1
//
// Run:  cargo run --example lc_0069_sqrt_x
// Test: cargo test --example lc_0069_sqrt_x
// Bench: cargo test --example lc_0069_sqrt_x bench -- --nocapture

/// My solution — binary search.
///
/// Search [0, x] for the largest m where m*m <= x.
/// Use i64 to avoid overflow on m*m.
///
/// Time: O(log x).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute integer square root.
        ///
        /// # Arguments
        /// * `x` - non-negative integer
        ///
        /// # Returns
        /// Floor of sqrt(x)
        pub fn my_sqrt(x: i32) -> i32 {
            if x < 2 {
                return x;
            }
            let x = x as i64;
            let (mut lo, mut hi) = (1i64, x);
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                let sq = mid * mid;
                if sq == x {
                    return mid as i32;
                } else if sq < x {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
            hi as i32
        }
    }
}

/// AI solution — Newton's method (integer version).
///
/// Start with x, iteratively compute r = (r + x/r) / 2 until convergence.
///
/// Time: O(log x) — quadratic convergence.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compute integer square root.
        ///
        /// # Arguments
        /// * `x` - non-negative integer
        ///
        /// # Returns
        /// Floor of sqrt(x)
        pub fn my_sqrt(x: i32) -> i32 {
            if x < 2 {
                return x;
            }
            let x = x as i64;
            let mut r = x;
            // Newton's method: r = (r + x/r) / 2
            while r * r > x {
                r = (r + x / r) / 2;
            }
            r as i32
        }
    }
}

fn main() {
    for x in [0, 1, 4, 8, 16, i32::MAX] {
        println!(
            "sqrt({x}) -> my:{}, ai:{}",
            my_solution::Solution::my_sqrt(x),
            ai_solution::Solution::my_sqrt(x),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_my_sqrt {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_4() {
                    assert_eq!(<$solution>::my_sqrt(4), 2);
                }

                #[test]
                fn test_8() {
                    assert_eq!(<$solution>::my_sqrt(8), 2);
                }

                #[test]
                fn test_0() {
                    assert_eq!(<$solution>::my_sqrt(0), 0);
                }

                #[test]
                fn test_1() {
                    assert_eq!(<$solution>::my_sqrt(1), 1);
                }

                #[test]
                fn test_perfect_square() {
                    assert_eq!(<$solution>::my_sqrt(100), 10);
                }

                #[test]
                fn test_large() {
                    assert_eq!(<$solution>::my_sqrt(i32::MAX), 46340);
                }
            }
        };
    }

    test_my_sqrt!(my, crate::my_solution::Solution);
    test_my_sqrt!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::my_sqrt(std::hint::black_box(i32::MAX)),
            );
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::my_sqrt(std::hint::black_box(i32::MAX)),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
