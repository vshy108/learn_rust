// LeetCode 231: Power of Two [Easy]
// https://leetcode.com/problems/power-of-two/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.761us
//   AI solution avg: 0.585us
//   Ratio: AI 1.30x faster
//
// Given an integer n, return true if it is a power of two. Otherwise,
// return false. An integer n is a power of two if there exists an
// integer x such that n == 2^x.
//
// Constraints: -2^31 <= n <= 2^31 - 1
// Follow up: Could you solve it without loops/recursion?
//
// Run:  cargo run --example lc_0231_power_of_two
// Test: cargo test --example lc_0231_power_of_two
// Bench: cargo test --example lc_0231_power_of_two bench -- --nocapture

/// My solution — bit counting.
///
/// A positive power of two has exactly one bit set. Use `count_ones()`
/// to check.
///
/// Time: O(1).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if `n` is a power of two.
        pub fn is_power_of_two(n: i32) -> bool {
            // Must be positive, and exactly one bit set
            n > 0 && n.count_ones() == 1
        }
    }
}

/// AI solution — bit trick: n & (n - 1).
///
/// For a power of two, n has exactly one set bit. n - 1 flips that bit
/// and all lower bits, so n & (n - 1) == 0. This avoids even counting
/// bits — a single AND operation suffices.
///
/// Time: O(1).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if `n` is a power of two.
        pub fn is_power_of_two(n: i32) -> bool {
            // Must be positive, and clearing the lowest set bit gives 0
            n > 0 && (n & (n - 1)) == 0
        }
    }
}

fn main() {
    let cases = [(1, true), (16, true), (3, false), (0, false), (-4, false)];
    for (n, expected) in &cases {
        let my = my_solution::Solution::is_power_of_two(*n);
        let ai = ai_solution::Solution::is_power_of_two(*n);
        println!("n={n}: my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_pow2 {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_one() {
                    // 2^0 = 1
                    assert!(<$solution>::is_power_of_two(1));
                }

                #[test]
                fn test_sixteen() {
                    // 2^4 = 16
                    assert!(<$solution>::is_power_of_two(16));
                }

                #[test]
                fn test_three() {
                    assert!(!<$solution>::is_power_of_two(3));
                }

                #[test]
                fn test_zero() {
                    assert!(!<$solution>::is_power_of_two(0));
                }

                #[test]
                fn test_negative() {
                    assert!(!<$solution>::is_power_of_two(-4));
                }

                #[test]
                fn test_large_power() {
                    // 2^30 = 1073741824
                    assert!(<$solution>::is_power_of_two(1_073_741_824));
                }

                #[test]
                fn test_not_power_large() {
                    assert!(!<$solution>::is_power_of_two(1_073_741_823));
                }
            }
        };
    }

    test_pow2!(my, crate::my_solution::Solution);
    test_pow2!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let test_values: Vec<i32> = (0..1000).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &test_values {
                std::hint::black_box(crate::my_solution::Solution::is_power_of_two(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &test_values {
                std::hint::black_box(crate::ai_solution::Solution::is_power_of_two(
                    std::hint::black_box(n),
                ));
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
