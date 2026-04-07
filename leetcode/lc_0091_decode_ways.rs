// LeetCode 91: Decode Ways [Medium]
// https://leetcode.com/problems/decode-ways/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.626us
//   AI solution avg: 0.376us
//   Ratio: AI 1.67x faster
//
// A message encoded with A=1, B=2, ..., Z=26. Given a string of digits,
// return the number of ways to decode it.
//
// Constraints: 1 <= s.length <= 100, s contains only digits, s[0] != '0'
//
// Run:  cargo run --example lc_0091_decode_ways
// Test: cargo test --example lc_0091_decode_ways
// Bench: cargo test --example lc_0091_decode_ways bench -- --nocapture

/// My solution — DP with array.
///
/// dp[i] = number of ways to decode s[..i].
///
/// Time: O(n).
/// Space: O(n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of ways to decode the digit string.
        ///
        /// # Arguments
        /// * `s` - string of digits
        ///
        /// # Returns
        /// Number of valid decodings
        pub fn num_decodings(s: String) -> i32 {
            let bytes = s.as_bytes();
            let n = bytes.len();
            if n == 0 || bytes[0] == b'0' {
                return 0;
            }

            // dp[i] = ways to decode s[..i]
            let mut dp = vec![0i32; n + 1];
            dp[0] = 1; // empty string
            dp[1] = 1; // first char (already checked non-zero)

            for i in 2..=n {
                let one_digit = bytes[i - 1] - b'0';
                let two_digits = (bytes[i - 2] - b'0') * 10 + one_digit;

                if one_digit >= 1 {
                    dp[i] += dp[i - 1];
                }
                if (10..=26).contains(&two_digits) {
                    dp[i] += dp[i - 2];
                }
            }
            dp[n]
        }
    }
}

/// AI solution — space-optimized DP using two variables.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of ways to decode the digit string.
        ///
        /// # Arguments
        /// * `s` - string of digits
        ///
        /// # Returns
        /// Number of valid decodings
        pub fn num_decodings(s: String) -> i32 {
            let bytes = s.as_bytes();
            if bytes.is_empty() || bytes[0] == b'0' {
                return 0;
            }

            let mut prev2 = 1i32; // dp[i-2]
            let mut prev1 = 1i32; // dp[i-1]

            for i in 1..bytes.len() {
                let mut curr = 0;
                let one = bytes[i] - b'0';
                let two = (bytes[i - 1] - b'0') * 10 + one;

                if one >= 1 {
                    curr += prev1;
                }
                if (10..=26).contains(&two) {
                    curr += prev2;
                }
                prev2 = prev1;
                prev1 = curr;
            }
            prev1
        }
    }
}

fn main() {
    let cases = ["12", "226", "06"];
    for s in cases {
        let my = my_solution::Solution::num_decodings(s.to_string());
        let ai = ai_solution::Solution::num_decodings(s.to_string());
        println!("{s}: my={my}, ai={ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_decode {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_12() {
                    assert_eq!(<$solution>::num_decodings("12".to_string()), 2);
                }

                #[test]
                fn test_226() {
                    assert_eq!(<$solution>::num_decodings("226".to_string()), 3);
                }

                #[test]
                fn test_06() {
                    assert_eq!(<$solution>::num_decodings("06".to_string()), 0);
                }

                #[test]
                fn test_10() {
                    assert_eq!(<$solution>::num_decodings("10".to_string()), 1);
                }

                #[test]
                fn test_27() {
                    assert_eq!(<$solution>::num_decodings("27".to_string()), 1);
                }

                #[test]
                fn test_1111() {
                    // 1-1-1-1, 11-1-1, 1-11-1, 1-1-11, 11-11 = 5
                    assert_eq!(<$solution>::num_decodings("1111".to_string()), 5);
                }
            }
        };
    }

    test_decode!(my, crate::my_solution::Solution);
    test_decode!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "1234567891234567891234".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::num_decodings(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::num_decodings(
                std::hint::black_box(s.clone()),
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
