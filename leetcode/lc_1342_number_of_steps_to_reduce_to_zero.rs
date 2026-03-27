// LeetCode 1342: Number of Steps to Reduce a Number to Zero [Easy]
// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
//
// Given an integer num, return the number of steps to reduce it to zero.
//
// In one step, if the current number is even, you have to divide it by 2,
// otherwise, you have to subtract 1 from it.
//
// Examples:
//   num = 14  -> 6   (14 -> 7 -> 6 -> 3 -> 2 -> 1 -> 0)
//   num = 8   -> 4   (8 -> 4 -> 2 -> 1 -> 0)
//   num = 123 -> 12
//
// Constraints:
//   - 0 <= num <= 10^6
//
// Run:  cargo run --example lc_1342_number_of_steps_to_reduce_to_zero
// Test: cargo test --example lc_1342_number_of_steps_to_reduce_to_zero
// Bench: cargo test --example lc_1342_number_of_steps_to_reduce_to_zero bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count steps to reduce num to zero (even: divide by 2, odd: subtract 1).
        ///
        /// # Arguments
        ///
        /// * `num` - A non-negative integer
        ///
        /// # Returns
        ///
        /// Number of steps to reach zero
        pub fn number_of_steps(num: i32) -> i32 {
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
        /// Count steps to reduce num to zero (even: divide by 2, odd: subtract 1).
        ///
        /// # Arguments
        ///
        /// * `num` - A non-negative integer
        ///
        /// # Returns
        ///
        /// Number of steps to reach zero
        pub fn number_of_steps(num: i32) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32)> = vec![(14, 6), (8, 4), (123, 12)];

    println!("--- My Solution ---");
    for (num, expected) in &test_cases {
        let result = my_solution::Solution::number_of_steps(*num);
        println!("  {num} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (num, expected) in &test_cases {
        let result = ai_solution::Solution::number_of_steps(*num);
        println!("  {num} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_number_of_steps {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::number_of_steps(14), 6);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::number_of_steps(8), 4);
                }

                #[test]
                fn test_example_3() {
                    assert_eq!(Solution::number_of_steps(123), 12);
                }

                #[test]
                fn test_zero() {
                    assert_eq!(Solution::number_of_steps(0), 0);
                }

                #[test]
                fn test_one() {
                    assert_eq!(Solution::number_of_steps(1), 1);
                }

                #[test]
                fn test_power_of_two() {
                    assert_eq!(Solution::number_of_steps(16), 5);
                }

                #[test]
                fn test_large_number() {
                    assert_eq!(Solution::number_of_steps(1_000_000), 26);
                }
            }
        };
    }

    test_number_of_steps!(my_tests, crate::my_solution::Solution);
    test_number_of_steps!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![14, 8, 123, 0, 1, 1_000_000];

        let start = Instant::now();
        for _ in 0..iterations {
            for &num in &inputs {
                std::hint::black_box(crate::my_solution::Solution::number_of_steps(
                    std::hint::black_box(num),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &num in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::number_of_steps(
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
