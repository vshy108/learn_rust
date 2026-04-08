// LeetCode 29: Divide Two Integers [Medium]
// https://leetcode.com/problems/divide-two-integers/
//
// First benchmark: 100_000 iterations x 5 cases, 2025-07-08
//   My solution avg: 1.507µs
//   AI solution avg: 1.071µs
//   Ratio: AI solution is 1.41x faster
//
// Given two integers dividend and divisor, divide them without using
// multiplication, division, or mod operator. Truncate toward zero.
//
// If the quotient overflows 32-bit signed range, clamp to i32::MIN or i32::MAX.
//
// Example 1: dividend = 10, divisor = 3   -> 3
// Example 2: dividend = 7,  divisor = -3  -> -2
//
// Constraints:
//   -2^31 <= dividend, divisor <= 2^31 - 1
//   divisor != 0
//
// Run:  cargo run --example lc_0029_divide_two_integers
// Test: cargo test --example lc_0029_divide_two_integers
// Bench: cargo test --example lc_0029_divide_two_integers bench -- --nocapture

/// My solution — repeated subtraction with doubling.
///
/// Work in negative space (i32::MIN has larger magnitude than i32::MAX)
/// to avoid overflow. Double the divisor while it fits, then subtract
/// and accumulate the quotient.
///
/// Time: O(log^2 n) — outer loop O(log n) bits, inner doubling O(log n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Divide without using *, /, or %.
        ///
        /// # Arguments
        /// * `dividend` - the number to divide
        /// * `divisor` - the number to divide by (non-zero)
        ///
        /// # Returns
        /// Truncated quotient, clamped to [i32::MIN, i32::MAX].
        pub fn divide(dividend: i32, divisor: i32) -> i32 {
            // Special overflow case: -2^31 / -1 = 2^31 which overflows i32.
            if dividend == i32::MIN && divisor == -1 {
                return i32::MAX;
            }

            // Determine the sign of the result.
            let negative = (dividend < 0) != (divisor < 0);

            // Work with negative numbers to avoid overflow
            // (i32::MIN cannot be negated to positive i32).
            let mut dvd = if dividend > 0 { -dividend } else { dividend };
            let dvs = if divisor > 0 { -divisor } else { divisor };

            let mut quotient: i32 = 0;

            // Subtract doubled divisors from dividend.
            while dvd <= dvs {
                let mut temp = dvs;
                let mut multiple = -1; // negative to stay in negative space

                // Double temp while it doesn't overflow past dvd.
                // `temp >= (i32::MIN >> 1)` prevents overflow on doubling.
                while temp >= (i32::MIN >> 1) && (temp + temp) >= dvd {
                    temp += temp; // double the divisor
                    multiple += multiple; // double the count (negative)
                }

                dvd -= temp; // subtract the chunk
                quotient += multiple; // accumulate (negative count)
            }

            // quotient is negative; negate if result should be positive.
            if negative { quotient } else { -quotient }
        }
    }
}

/// AI-optimized solution — bit-shift approach.
///
/// Same idea but uses explicit bit positions: find the highest power of 2
/// such that divisor << power fits in dividend, subtract, repeat.
/// Avoids the inner doubling loop by scanning bits top-down.
///
/// Time: O(log n) — single pass over 32 bit positions.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Divide without using *, /, or %.
        ///
        /// # Arguments
        /// * `dividend` - the number to divide
        /// * `divisor` - the number to divide by (non-zero)
        ///
        /// # Returns
        /// Truncated quotient, clamped to [i32::MIN, i32::MAX].
        pub fn divide(dividend: i32, divisor: i32) -> i32 {
            if dividend == i32::MIN && divisor == -1 {
                return i32::MAX;
            }
            if dividend == i32::MIN && divisor == 1 {
                return i32::MIN;
            }

            let negative = (dividend < 0) != (divisor < 0);

            // Use i64 to safely take absolute values (i32::MIN abs overflows i32).
            let mut dvd = (dividend as i64).abs();
            let dvs = (divisor as i64).abs();

            let mut quotient: i32 = 0;

            // Scan from the highest possible shift down to 0.
            // At each bit position, check if divisor << i fits into remainder.
            for i in (0..32).rev() {
                if (dvs << i) <= dvd {
                    dvd -= dvs << i;
                    quotient += 1 << i;
                }
            }

            if negative { -quotient } else { quotient }
        }
    }
}

fn main() {
    let cases = [(10, 3), (7, -3), (i32::MIN, -1), (0, 1)];
    for (a, b) in &cases {
        println!(
            "{a} / {b} = my:{}, ai:{}",
            my_solution::Solution::divide(*a, *b),
            ai_solution::Solution::divide(*a, *b),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_divide {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic() {
                    assert_eq!(Solution::divide(10, 3), 3);
                }

                #[test]
                fn test_negative_divisor() {
                    assert_eq!(Solution::divide(7, -3), -2);
                }

                #[test]
                fn test_overflow_clamp() {
                    // -2^31 / -1 would be 2^31, but that overflows i32.
                    assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
                }

                #[test]
                fn test_min_div_1() {
                    assert_eq!(Solution::divide(i32::MIN, 1), i32::MIN);
                }

                #[test]
                fn test_zero_dividend() {
                    assert_eq!(Solution::divide(0, 5), 0);
                }

                #[test]
                fn test_same_sign() {
                    assert_eq!(Solution::divide(-10, -3), 3);
                }

                #[test]
                fn test_exact_division() {
                    assert_eq!(Solution::divide(12, 4), 3);
                }

                #[test]
                fn test_one_divisor() {
                    assert_eq!(Solution::divide(42, 1), 42);
                }

                #[test]
                fn test_dividend_less_than_divisor() {
                    assert_eq!(Solution::divide(2, 3), 0);
                }

                #[test]
                fn test_negative_dividend() {
                    assert_eq!(Solution::divide(-10, 3), -3);
                }

                #[test]
                fn test_min_div_2() {
                    // -2^31 / 2 = -2^30
                    assert_eq!(Solution::divide(i32::MIN, 2), i32::MIN / 2);
                }
            }
        };
    }

    test_divide!(my_tests, crate::my_solution::Solution);
    test_divide!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let cases = [
            (i32::MAX, 1),
            (i32::MIN, 2),
            (100_000, 3),
            (-999_999, 7),
            (1, 1),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(a, b) in &cases {
                std::hint::black_box(crate::my_solution::Solution::divide(
                    std::hint::black_box(a),
                    std::hint::black_box(b),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(a, b) in &cases {
                std::hint::black_box(crate::ai_solution::Solution::divide(
                    std::hint::black_box(a),
                    std::hint::black_box(b),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!(
            "\n=== Benchmark ({iterations} iterations x {} cases) ===",
            cases.len()
        );
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
