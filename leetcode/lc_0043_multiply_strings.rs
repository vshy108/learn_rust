// LeetCode 43: Multiply Strings [Medium]
// https://leetcode.com/problems/multiply-strings/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 188.329µs
//   AI solution avg: 189.023µs
//   Ratio: ~tied
//
// Given two non-negative integers num1 and num2 represented as strings,
// return their product, also represented as a string.
// Must not use built-in BigInteger or direct integer conversion.
//
// Example 1: num1 = "2", num2 = "3" -> "6"
// Example 2: num1 = "123", num2 = "456" -> "56088"
//
// Constraints:
//   1 <= num1.length, num2.length <= 200
//   num1 and num2 only contain digits
//   Neither has leading zeros except "0" itself
//
// Run:  cargo run --example lc_0043_multiply_strings
// Test: cargo test --example lc_0043_multiply_strings
// Bench: cargo test --example lc_0043_multiply_strings bench -- --nocapture

/// My solution — grade-school multiplication.
///
/// Multiply digit by digit and accumulate into a result array.
/// Position i*j contributes to result[i+j] and result[i+j+1].
///
/// Time: O(m*n) where m, n are the lengths of the two strings.
/// Space: O(m+n) for the result buffer.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Multiply two number strings.
        ///
        /// # Arguments
        /// * `num1` - first non-negative integer as string
        /// * `num2` - second non-negative integer as string
        ///
        /// # Returns
        /// Product as string.
        pub fn multiply(num1: String, num2: String) -> String {
            let n1 = num1.as_bytes();
            let n2 = num2.as_bytes();
            let mut result = vec![0u32; n1.len() + n2.len()];

            // Multiply each digit pair and accumulate.
            for i in (0..n1.len()).rev() {
                for j in (0..n2.len()).rev() {
                    let mul = (n1[i] - b'0') as u32 * (n2[j] - b'0') as u32;
                    let p1 = i + j; // tens position
                    let p2 = i + j + 1; // ones position
                    let sum = mul + result[p2];

                    result[p2] = sum % 10;
                    result[p1] += sum / 10;
                }
            }

            // Convert to string, skipping leading zeros.
            let s: String = result.iter().map(|&d| (d as u8 + b'0') as char).collect();
            let trimmed = s.trim_start_matches('0');

            if trimmed.is_empty() {
                "0".to_string()
            } else {
                trimmed.to_string()
            }
        }
    }
}

/// AI-optimized solution — same grade-school approach (already optimal for constraints).
///
/// Identical algorithm — grade-school multiplication is the standard approach
/// and is efficient enough for strings up to 200 digits.
///
/// Time: O(m*n).
/// Space: O(m+n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Multiply two number strings.
        pub fn multiply(num1: String, num2: String) -> String {
            let n1 = num1.as_bytes();
            let n2 = num2.as_bytes();
            let mut result = vec![0u32; n1.len() + n2.len()];

            for i in (0..n1.len()).rev() {
                for j in (0..n2.len()).rev() {
                    let mul = (n1[i] - b'0') as u32 * (n2[j] - b'0') as u32;
                    let p2 = i + j + 1;
                    let sum = mul + result[p2];
                    result[p2] = sum % 10;
                    result[i + j] += sum / 10;
                }
            }

            let s: String = result.iter().map(|&d| (d as u8 + b'0') as char).collect();
            let trimmed = s.trim_start_matches('0');
            if trimmed.is_empty() {
                "0".to_string()
            } else {
                trimmed.to_string()
            }
        }
    }
}

fn main() {
    let cases = [("2", "3"), ("123", "456"), ("0", "0")];
    for (a, b) in &cases {
        println!(
            "{a} * {b} = my:{}, ai:{}",
            my_solution::Solution::multiply(a.to_string(), b.to_string()),
            ai_solution::Solution::multiply(a.to_string(), b.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_multiply {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::multiply("2".into(), "3".into()), "6");
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::multiply("123".into(), "456".into()), "56088");
                }

                #[test]
                fn test_zero() {
                    assert_eq!(Solution::multiply("0".into(), "0".into()), "0");
                }

                #[test]
                fn test_one_zero() {
                    assert_eq!(Solution::multiply("999".into(), "0".into()), "0");
                }

                #[test]
                fn test_single_digits() {
                    assert_eq!(Solution::multiply("9".into(), "9".into()), "81");
                }

                #[test]
                fn test_large() {
                    assert_eq!(Solution::multiply("999".into(), "999".into()), "998001",);
                }

                #[test]
                fn test_one_times() {
                    assert_eq!(Solution::multiply("1".into(), "12345".into()), "12345");
                }
            }
        };
    }

    test_multiply!(my_tests, crate::my_solution::Solution);
    test_multiply!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let num1 = "9".repeat(100);
        let num2 = "9".repeat(100);

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::multiply(
                std::hint::black_box(num1.clone()),
                std::hint::black_box(num2.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::multiply(
                std::hint::black_box(num1.clone()),
                std::hint::black_box(num2.clone()),
            ));
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
