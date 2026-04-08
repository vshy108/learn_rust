// LeetCode 504: Base 7 [Easy]
// https://leetcode.com/problems/base-7/
//
// Given an integer num, return a string of its base 7 representation.
//
// Examples:
//   100 -> "202"
//   -7  -> "-10"
//
// Constraints:
//   -10^7 <= num <= 10^7
//
// Run:  cargo run --example lc_0504_base_7
// Test: cargo test --example lc_0504_base_7
// Bench: cargo test --example lc_0504_base_7 bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 151ns
//   AI solution: avg 134ns
//   AI is 1.12x faster
//   Reason: recursive approach builds the string in correct order directly via
//   format!, avoiding the final chars().rev().collect() reversal.

/// My solution — iterative division with remainder collection.
///
/// Adapted from doocs/leetcode Rust solution. Repeatedly take num % 7 and
/// num /= 7, collect digits in reverse, then reverse the string.
///
/// Time:  O(log_7(n))
/// Space: O(log_7(n))
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Convert num to its base 7 string representation.
        ///
        /// # Arguments
        /// * `num` - integer to convert
        ///
        /// # Returns
        /// Base 7 string
        pub fn convert_to_base7(mut num: i32) -> String {
            if num == 0 {
                return String::from("0");
            }
            let mut res = String::new();
            let is_negative = num < 0;
            if is_negative {
                num = -num;
            }
            while num != 0 {
                res.push_str(&(num % 7).to_string());
                num /= 7;
            }
            if is_negative {
                res.push('-');
            }
            // Digits were pushed in reverse order
            res.chars().rev().collect()
        }
    }
}

/// AI solution — recursive approach.
///
/// Handle sign separately, then recurse: convert_to_base7(num / 7) + (num % 7).
/// Base case: num < 7 returns the single digit.
///
/// Time:  O(log_7(n))
/// Space: O(log_7(n)) recursion stack
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Convert num to base 7 using recursion.
        ///
        /// # Arguments
        /// * `num` - integer to convert
        ///
        /// # Returns
        /// Base 7 string
        pub fn convert_to_base7(num: i32) -> String {
            if num < 0 {
                return format!("-{}", Self::convert_to_base7(-num));
            }
            if num < 7 {
                return num.to_string();
            }
            format!("{}{}", Self::convert_to_base7(num / 7), num % 7)
        }
    }
}

fn main() {
    println!("my: {}", my_solution::Solution::convert_to_base7(100));
    println!("ai: {}", ai_solution::Solution::convert_to_base7(-7));
}

#[cfg(test)]
mod tests {
    macro_rules! test_base7 {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::convert_to_base7(100), "202");
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::convert_to_base7(-7), "-10");
                }

                #[test]
                fn test_zero() {
                    assert_eq!(<$solution>::convert_to_base7(0), "0");
                }

                #[test]
                fn test_one() {
                    assert_eq!(<$solution>::convert_to_base7(1), "1");
                }

                #[test]
                fn test_seven() {
                    assert_eq!(<$solution>::convert_to_base7(7), "10");
                }

                #[test]
                fn test_negative() {
                    assert_eq!(<$solution>::convert_to_base7(-100), "-202");
                }
            }
        };
    }

    test_base7!(my, crate::my_solution::Solution);
    test_base7!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 500_000;
        let num = 100;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::convert_to_base7(
                std::hint::black_box(num),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::convert_to_base7(
                std::hint::black_box(num),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}ns avg",
            my_duration.as_secs_f64() * 1_000_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}ns avg",
            ai_duration.as_secs_f64() * 1_000_000_000.0 / iterations as f64,
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
