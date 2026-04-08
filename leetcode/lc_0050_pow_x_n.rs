// LeetCode 50: Pow(x, n) [Medium]
// https://leetcode.com/problems/powx-n/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.139us
//   AI solution avg: 0.072us
//   Ratio: AI 1.93x faster
//
// Implement pow(x, n), which calculates x raised to the power n.
//
// Example 1: x = 2.00000, n = 10 -> 1024.00000
// Example 2: x = 2.10000, n = 3 -> 9.26100
// Example 3: x = 2.00000, n = -2 -> 0.25000
//
// Constraints:
//   -100.0 < x < 100.0
//   -2^31 <= n <= 2^31 - 1
//   n is an integer
//   Either x is not zero or n > 0
//   -10^4 <= x^n <= 10^4
//
// Run:  cargo run --example lc_0050_pow_x_n
// Test: cargo test --example lc_0050_pow_x_n
// Bench: cargo test --example lc_0050_pow_x_n bench -- --nocapture

/// My solution — recursive binary exponentiation.
///
/// If n is even: x^n = (x^(n/2))^2
/// If n is odd:  x^n = x * x^(n-1)
/// Handle negative n by computing 1/x^|n|.
///
/// Time: O(log n) — halve n each step.
/// Space: O(log n) — recursion depth.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute x raised to the power n.
        ///
        /// # Arguments
        /// * `x` - base value
        /// * `n` - exponent (can be negative)
        ///
        /// # Returns
        /// x^n as f64
        pub fn my_pow(x: f64, n: i32) -> f64 {
            // Convert to i64 to handle i32::MIN safely (|-2^31| overflows i32)
            let n_long = n as i64;
            if n_long < 0 {
                1.0 / Self::helper(x, -n_long)
            } else {
                Self::helper(x, n_long)
            }
        }

        fn helper(x: f64, n: i64) -> f64 {
            if n == 0 {
                return 1.0;
            }
            let half = Self::helper(x, n / 2);
            if n % 2 == 0 {
                half * half
            } else {
                half * half * x
            }
        }
    }
}

/// AI solution — iterative binary exponentiation.
///
/// Process bits of n from LSB to MSB. Square the base each step;
/// multiply into result when the current bit is set.
///
/// Time: O(log n) — iterate through bits of n.
/// Space: O(1) — no recursion.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compute x raised to the power n.
        ///
        /// # Arguments
        /// * `x` - base value
        /// * `n` - exponent (can be negative)
        ///
        /// # Returns
        /// x^n as f64
        pub fn my_pow(x: f64, n: i32) -> f64 {
            let mut exp = (n as i64).unsigned_abs();
            let mut base = x;
            let mut result = 1.0;
            // Iterative fast exponentiation via bit scanning
            while exp > 0 {
                if exp & 1 == 1 {
                    result *= base;
                }
                base *= base;
                exp >>= 1;
            }
            if n < 0 { 1.0 / result } else { result }
        }
    }
}

fn main() {
    let cases = [(2.0, 10), (2.1, 3), (2.0, -2)];
    for (x, n) in &cases {
        println!(
            "pow({x}, {n}) = my:{:.5}, ai:{:.5}",
            my_solution::Solution::my_pow(*x, *n),
            ai_solution::Solution::my_pow(*x, *n),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_my_pow {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                fn approx_eq(a: f64, b: f64) -> bool {
                    (a - b).abs() < 1e-5
                }

                #[test]
                fn test_positive_exp() {
                    assert!(approx_eq(<$solution>::my_pow(2.0, 10), 1024.0));
                }

                #[test]
                fn test_fractional_base() {
                    assert!(approx_eq(<$solution>::my_pow(2.1, 3), 9.261));
                }

                #[test]
                fn test_negative_exp() {
                    assert!(approx_eq(<$solution>::my_pow(2.0, -2), 0.25));
                }

                #[test]
                fn test_zero_exp() {
                    assert!(approx_eq(<$solution>::my_pow(2.0, 0), 1.0));
                }

                #[test]
                fn test_one_exp() {
                    assert!(approx_eq(<$solution>::my_pow(5.0, 1), 5.0));
                }

                #[test]
                fn test_negative_base() {
                    assert!(approx_eq(<$solution>::my_pow(-2.0, 3), -8.0));
                }

                #[test]
                fn test_min_exponent() {
                    // n = i32::MIN; x = 1.0 so result = 1.0
                    assert!(approx_eq(<$solution>::my_pow(1.0, i32::MIN), 1.0));
                }

                #[test]
                fn test_zero_base() {
                    assert!(approx_eq(<$solution>::my_pow(0.0, 5), 0.0));
                }
            }
        };
    }

    test_my_pow!(my, crate::my_solution::Solution);
    test_my_pow!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let x = 1.00001;
        let n = 1_000_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::my_pow(
                std::hint::black_box(x),
                std::hint::black_box(n),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::my_pow(
                std::hint::black_box(x),
                std::hint::black_box(n),
            ));
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
