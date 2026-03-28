// LeetCode 169: Majority Element [Easy]
// https://leetcode.com/problems/majority-element/
//
// Given an array nums of size n, return the majority element. The majority
// element is the element that appears more than n/2 times. You may assume
// that the majority element always exists in the array.
//
// Examples:
//   nums = [3,2,3]         -> 3
//   nums = [2,2,1,1,1,2,2] -> 2
//
// Constraints:
//   - n == nums.length
//   - 1 <= n <= 5 * 10^4
//   - -10^9 <= nums[i] <= 10^9
//
// Follow-up: Could you solve the problem in linear time and O(1) space?
//
// Run:  cargo run --example lc_0169_majority_element
// Test: cargo test --example lc_0169_majority_element
// Bench: cargo test --example lc_0169_majority_element bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the majority element (appears more than n/2 times).
        ///
        /// # Arguments
        ///
        /// * `nums` - Array of integers with a guaranteed majority element
        ///
        /// # Returns
        ///
        /// The majority element
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            // count = 0
            // candidate = nil
            // nums.each do |num|
            //   if count == 0
            //     candidate = num
            //   end
            //   count += (num == candidate ? 1 : -1)
            // end
            // candidate
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
        /// Find the majority element (appears more than n/2 times).
        ///
        /// # Arguments
        ///
        /// * `nums` - Array of integers with a guaranteed majority element
        ///
        /// # Returns
        ///
        /// The majority element
        pub fn majority_element(nums: Vec<i32>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::majority_element(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::majority_element(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_majority {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic_odd() {
                    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
                }

                #[test]
                fn test_basic_even() {
                    assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::majority_element(vec![1]), 1);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::majority_element(vec![1, 1]), 1);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::majority_element(vec![5, 5, 5, 5]), 5);
                }

                #[test]
                fn test_majority_at_start() {
                    assert_eq!(Solution::majority_element(vec![1, 1, 1, 2, 3]), 1);
                }

                #[test]
                fn test_negative_majority() {
                    assert_eq!(Solution::majority_element(vec![-1, -1, 2]), -1);
                }
            }
        };
    }

    test_majority!(my_tests, crate::my_solution::Solution);
    test_majority!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![3, 2, 3],
            vec![2, 2, 1, 1, 1, 2, 2],
            vec![1, 1, 1, 2, 3],
            vec![5, 5, 5, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::majority_element(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::majority_element(
                    std::hint::black_box(nums.clone()),
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
