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
//
// --- Rust concepts used ---
// - `let mut num = num` parameter rebinding: takes ownership of the
//   i32 param and rebinds as mutable for in-place modification
// - Bitwise `& 1` and `>>= 1`: test odd/even and divide-by-2 in a
//   single pass without branching on even/odd separately
//   (Previously covered: let mut x = x shadowing -> lc_0009,
//    >> 1 bit shift -> lc_0278, & bitwise ops -> lc_0222)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  17.25ms (avg 172ns)
// AI solution:  10.29ms (avg 102ns)
// -> AI solution is 1.68x faster
// Reason: bitwise approach processes all bits uniformly without the
//   branch misprediction cost of the if num%2==0 check.

/// My solution module.
/// My solution module — direct simulation.
///
/// Time:  O(log n)
/// Space: O(1)
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
            let mut num = num;
            let mut steps = 0;
            while num > 0 {
                if num % 2 == 0 {
                    num /= 2;
                } else {
                    num -= 1;
                }
                steps += 1;
            }
            steps
        }
    }
}

/// AI solution module — bitwise approach.
///
/// Each bit contributes: 1 step for shift-right (divide by 2), plus
/// 1 extra step if the bit is 1 (subtract 1). Special case: num == 0.
///
/// Time:  O(log n)
/// Space: O(1)
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
            if num == 0 {
                return 0;
            }
            let mut n = num;
            let mut steps = 0;
            while n > 0 {
                // Bitwise: & 1 checks odd (subtract step), >> 1 is divide step
                steps += (n & 1) + 1;
                n >>= 1;
            }
            // The last bit (when n becomes 1) only needs the subtract step,
            // not the extra divide step. Subtract 1 to correct.
            steps - 1
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
