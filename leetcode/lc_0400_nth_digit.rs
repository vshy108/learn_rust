// LeetCode 400: Nth Digit [Medium]
// https://leetcode.com/problems/nth-digit/
// Benchmark: 100000 iterations
//   My solution (math + string extraction):    avg 34ns
//   AI solution (math + arithmetic extraction): avg 8ns
//   -> AI solution is 4.08x faster
//
// Given an integer n, return the nth digit of the infinite integer sequence
// [1, 2, 3, ..., 9, 10, 11, ...].
//
// Examples:
//   n = 3  -> 3
//   n = 11 -> 0  (the '0' in "10")
//
// Constraints:
//   - 1 <= n <= 2^31 - 1
//
// Run:  cargo run --example lc_0400_nth_digit
// Test: cargo test --example lc_0400_nth_digit
// Bench: cargo test --example lc_0400_nth_digit bench -- --nocapture

/// My solution module — math: determine digit range, then locate.
///
/// 1-digit numbers: 1-9 = 9 numbers, 9 digits
/// 2-digit numbers: 10-99 = 90 numbers, 180 digits
/// k-digit numbers: 9 * 10^(k-1) numbers, k * 9 * 10^(k-1) digits
///
/// Subtract until we find which "digit band" n falls in, then compute
/// the exact number and digit within that number.
///
/// Time:  O(log n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the nth digit of the infinite sequence 1,2,3,...
        ///
        /// # Arguments
        ///
        /// * `n` - 1-based position in the digit sequence
        ///
        /// # Returns
        ///
        /// The digit at position n
        pub fn find_nth_digit(n: i32) -> i32 {
            let mut n = n as i64;
            let mut k: i64 = 1; // current digit length
            let mut cnt: i64 = 9; // count of numbers with `k` digits

            // Skip past complete digit bands
            while k * cnt < n {
                n -= k * cnt;
                k += 1;
                cnt *= 10;
            }

            // `n` is now the position within the k-digit band (1-based)
            // The target number = 10^(k-1) + (n-1)/k
            let num = 10_i64.pow((k - 1) as u32) + (n - 1) / k;
            // The digit index within that number
            let idx = ((n - 1) % k) as usize;
            // Extract the digit by converting to string
            let digits = num.to_string();
            (digits.as_bytes()[idx] - b'0') as i32
        }
    }
}

/// AI solution module — same math, digit extracted arithmetically.
///
/// Instead of converting to string, extracts the digit using
/// division and modulo by the appropriate power of 10.
///
/// Time:  O(log n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the nth digit of the infinite sequence 1,2,3,...
        ///
        /// # Arguments
        ///
        /// * `n` - 1-based position in the digit sequence
        ///
        /// # Returns
        ///
        /// The digit at position n
        pub fn find_nth_digit(n: i32) -> i32 {
            let mut n = n as i64;
            let mut k: i64 = 1;
            let mut cnt: i64 = 9;

            while k * cnt < n {
                n -= k * cnt;
                k += 1;
                cnt *= 10;
            }

            let num = 10_i64.pow((k - 1) as u32) + (n - 1) / k;
            let digit_pos = (n - 1) % k; // 0-based from left

            // Extract digit without string conversion:
            // Shift the target digit to the ones place
            let divisor = 10_i64.pow((k - 1 - digit_pos) as u32);
            ((num / divisor) % 10) as i32
        }
    }
}

fn main() {
    let cases = [(3, 3), (11, 0), (1, 1), (100, 5)];

    println!("--- My Solution ---");
    for (n, expected) in &cases {
        let result = my_solution::Solution::find_nth_digit(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (n, expected) in &cases {
        let result = ai_solution::Solution::find_nth_digit(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_nth_digit {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::find_nth_digit(3), 3);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::find_nth_digit(11), 0);
                }

                #[test]
                fn test_first_digit() {
                    assert_eq!(Solution::find_nth_digit(1), 1);
                }

                #[test]
                fn test_digit_9() {
                    assert_eq!(Solution::find_nth_digit(9), 9);
                }

                #[test]
                fn test_digit_10() {
                    // Position 10: first digit of "10" = 1
                    assert_eq!(Solution::find_nth_digit(10), 1);
                }

                #[test]
                fn test_digit_192() {
                    // 9 single-digit + 90*2=180 double-digit = digits 1-189.
                    // Position 189 is last digit of 99 = 9.
                    // Position 190 is first digit of 100 = 1.
                    // Position 191 = 0, Position 192 = 0.
                    assert_eq!(Solution::find_nth_digit(192), 0);
                }

                #[test]
                fn test_large() {
                    assert_eq!(Solution::find_nth_digit(1_000_000_000), 1);
                }
            }
        };
    }

    test_nth_digit!(my_tests, crate::my_solution::Solution);
    test_nth_digit!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 5_000_000_u32;
        let n = 1_000_000_000;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_nth_digit(
                std::hint::black_box(n),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_nth_digit(
                std::hint::black_box(n),
            ));
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
