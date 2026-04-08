// LeetCode 9: Palindrome Number [Easy]
// https://leetcode.com/problems/palindrome-number/
//
// Given an integer x, return true if x is a palindrome integer.
//
// An integer is a palindrome when it reads the same forward and backward.
// For example, 121 is a palindrome while 123 is not.
//
// Examples:
//   x = 121   -> true   (reads as 121 from left to right and right to left)
//   x = -121  -> false  (reads as 121- from right to left, not a palindrome)
//   x = 10    -> false  (reads as 01 from right to left, not a palindrome)
//
// Constraints:
//   - -2^31 <= x <= 2^31 - 1
//
// Follow up: Could you solve it without converting the integer to a string?
//
// Run:  cargo run --example lc_0009_palindrome_number
// Test: cargo test --example lc_0009_palindrome_number
// Bench: cargo test --example lc_0009_palindrome_number bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-27) ---
//   My solution (reverse half):          17.92ms avg (179ns/iter)
//   AI solution (two-pointer on digits): 23.01ms avg (230ns/iter)
//   My solution is 1.28x faster
//   Reason: Reversing only the lower half halts as soon as reversed_half >= remaining, doing roughly
//   half as many divisions as the two-pointer approach which extracts every digit from both ends.
/// My solution module — reverse half of the number.
///
/// Only reverses half the digits and compares with the remaining
/// front half. Avoids overflow risk since the reversed portion
/// never exceeds the original number's magnitude.
///
/// Time:  O(log10(n)) — processes half the digits
/// Space: O(1) — only scalar variables
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine whether an integer is a palindrome.
        ///
        /// # Arguments
        ///
        /// * `x` - The integer to check
        ///
        /// # Returns
        ///
        /// `true` if `x` is a palindrome, `false` otherwise
        pub fn is_palindrome(x: i32) -> bool {
            // Negative numbers are never palindromes (the '-' sign).
            // Numbers ending in 0 (except 0 itself) can't be palindromes
            // since no number starts with 0.
            if x < 0 || (x % 10 == 0 && x != 0) {
                return false;
            };

            // Shadow `x` as mutable — we'll shrink it from the right.
            let mut x = x;
            let mut reversed_half = 0;
            let mut digit;

            // Build the reversed second half digit by digit.
            // Loop stops when reversed_half >= x, meaning we've
            // processed at least half the digits.
            // Example: 1221 -> x=12, reversed_half=12 (even length)
            // Example: 12321 -> x=12, reversed_half=123 (odd length)
            while x > reversed_half {
                digit = x % 10;
                reversed_half = reversed_half * 10 + digit;
                x /= 10
            }

            // Even length: x == reversed_half (e.g. 12 == 12)
            // Odd length: x == reversed_half / 10 (e.g. 12 == 123/10)
            // The middle digit doesn't affect palindrome check.
            x == reversed_half || x == reversed_half / 10
        }
    }
}

/// AI solution module — two-pointer on digit count.
///
/// Compares the most-significant and least-significant digits
/// directly using powers of 10, narrowing inward like a
/// two-pointer on an array. No reversal needed.
///
/// Time:  O(log10(n)) — processes half the digits
/// Space: O(1) — only scalar variables
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine whether an integer is a palindrome.
        ///
        /// Compares outermost digits inward using division and modulo.
        /// `divisor` isolates the leading digit; `% 10` gives the
        /// trailing digit. After each comparison, strip both ends.
        ///
        /// # Arguments
        ///
        /// * `x` - The integer to check
        ///
        /// # Returns
        ///
        /// `true` if `x` is a palindrome, `false` otherwise
        pub fn is_palindrome(x: i32) -> bool {
            // Negative or trailing-zero (except 0) -> not palindrome.
            if x < 0 || (x % 10 == 0 && x != 0) {
                return false;
            }

            let mut x = x;

            // Find the largest power of 10 <= x.
            // For 12321, divisor starts at 10_000.
            let mut divisor = 1;
            while x / divisor >= 10 {
                divisor *= 10;
            }

            // Compare leading and trailing digits, then strip both.
            while x > 0 {
                // Leading digit: e.g. 12321 / 10000 = 1
                let leading = x / divisor;
                // Trailing digit: e.g. 12321 % 10 = 1
                let trailing = x % 10;

                if leading != trailing {
                    return false;
                }

                // Strip both ends: 12321 % 10000 = 2321, then / 10 = 232
                x = (x % divisor) / 10;
                // Divisor shrinks by 100 (removed two digits).
                divisor /= 100;
            }

            true
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, bool)> = vec![(121, true), (-121, false), (10, false)];

    println!("--- My Solution ---");
    for (input, expected) in &test_cases {
        let result = my_solution::Solution::is_palindrome(*input);
        println!("  {input} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &test_cases {
        let result = ai_solution::Solution::is_palindrome(*input);
        println!("  {input} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_palindrome {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_positive_palindrome() {
                    assert!(Solution::is_palindrome(121));
                }

                #[test]
                fn test_negative_number() {
                    assert!(!Solution::is_palindrome(-121));
                }

                #[test]
                fn test_trailing_zero() {
                    assert!(!Solution::is_palindrome(10));
                }

                #[test]
                fn test_single_digit() {
                    assert!(Solution::is_palindrome(0));
                    assert!(Solution::is_palindrome(5));
                    assert!(Solution::is_palindrome(9));
                }

                #[test]
                fn test_two_digit_palindrome() {
                    assert!(Solution::is_palindrome(11));
                    assert!(!Solution::is_palindrome(12));
                }

                #[test]
                fn test_large_palindrome() {
                    assert!(Solution::is_palindrome(1_234_321));
                }

                #[test]
                fn test_large_non_palindrome() {
                    assert!(!Solution::is_palindrome(1_234_567));
                }

                #[test]
                fn test_even_length_palindrome() {
                    assert!(Solution::is_palindrome(1221));
                }

                #[test]
                fn test_max_int() {
                    // 2_147_483_647 is not a palindrome
                    assert!(!Solution::is_palindrome(i32::MAX));
                }

                #[test]
                fn test_min_int() {
                    // Negative -> always false
                    assert!(!Solution::is_palindrome(i32::MIN));
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_palindrome!(my_tests, crate::my_solution::Solution);
    test_palindrome!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0009_palindrome_number bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![
            121,
            -121,
            10,
            0,
            5,
            11,
            12,
            1_234_321,
            1_234_567,
            1221,
            i32::MAX,
            i32::MIN,
        ];

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &x in &inputs {
                std::hint::black_box(crate::my_solution::Solution::is_palindrome(
                    std::hint::black_box(x),
                ));
            }
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            for &x in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::is_palindrome(
                    std::hint::black_box(x),
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
