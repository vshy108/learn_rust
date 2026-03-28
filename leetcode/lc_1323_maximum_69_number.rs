// LeetCode 1323: Maximum 69 Number [Easy]
// https://leetcode.com/problems/maximum-69-number/
//
// You are given a positive integer num consisting only of digits 6 and 9.
//
// Return the maximum number you can get by changing at most one digit
// (6 becomes 9, and 9 becomes 6).
//
// Examples:
//   num = 9669 -> 9969
//   num = 9996 -> 9999
//   num = 9999 -> 9999
//
// Constraints:
//   - 1 <= num <= 10^4
//   - num consists of only 6 and 9 digits
//
// Run:  cargo run --example lc_1323_maximum_69_number
// Test: cargo test --example lc_1323_maximum_69_number
// Bench: cargo test --example lc_1323_maximum_69_number bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum number by changing at most one digit.
        ///
        /// # Arguments
        ///
        /// * `num` - A positive integer consisting only of 6s and 9s
        ///
        /// # Returns
        ///
        /// The maximum number achievable with at most one digit change
        pub fn maximum69_number(num: i32) -> i32 {
            // digits = num.to_s.chars
            // digits.each_with_index do |d, i|
            //   if d == '6'
            //     digits[i] = '9'
            //     break
            //   end
            // end
            // digits.join.to_i
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
        /// Return the maximum number by changing at most one digit.
        ///
        /// # Arguments
        ///
        /// * `num` - A positive integer consisting only of 6s and 9s
        ///
        /// # Returns
        ///
        /// The maximum number achievable with at most one digit change
        pub fn maximum69_number(num: i32) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32)> = vec![(9669, 9969), (9996, 9999), (9999, 9999)];

    println!("--- My Solution ---");
    for (num, expected) in &test_cases {
        let result = my_solution::Solution::maximum69_number(*num);
        println!("  {num} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (num, expected) in &test_cases {
        let result = ai_solution::Solution::maximum69_number(*num);
        println!("  {num} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_maximum69_number {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::maximum69_number(9669), 9969);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::maximum69_number(9996), 9999);
                }

                #[test]
                fn test_all_nines() {
                    assert_eq!(Solution::maximum69_number(9999), 9999);
                }

                #[test]
                fn test_single_six() {
                    assert_eq!(Solution::maximum69_number(6), 9);
                }

                #[test]
                fn test_single_nine() {
                    assert_eq!(Solution::maximum69_number(9), 9);
                }

                #[test]
                fn test_leading_six() {
                    assert_eq!(Solution::maximum69_number(6699), 9699);
                }

                #[test]
                fn test_all_sixes() {
                    assert_eq!(Solution::maximum69_number(6666), 9666);
                }
            }
        };
    }

    test_maximum69_number!(my_tests, crate::my_solution::Solution);
    test_maximum69_number!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![9669, 9996, 9999, 6666, 6];

        let start = Instant::now();
        for _ in 0..iterations {
            for &num in &inputs {
                std::hint::black_box(crate::my_solution::Solution::maximum69_number(
                    std::hint::black_box(num),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &num in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::maximum69_number(
                    std::hint::black_box(num),
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
