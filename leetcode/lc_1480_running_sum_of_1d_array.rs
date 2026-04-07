// LeetCode 1480: Running Sum of 1d Array [Easy]
// https://leetcode.com/problems/running-sum-of-1d-array/
//
// Given an array nums. We define a running sum of an array as
// runningSum[i] = sum(nums[0]..nums[i]).
//
// Return the running sum of nums.
//
// Examples:
//   nums = [1,2,3,4]     -> [1,3,6,10]
//   nums = [1,1,1,1,1]   -> [1,2,3,4,5]
//   nums = [3,1,2,10,1]  -> [3,4,6,16,17]
//
// Constraints:
//   - 1 <= nums.length <= 1000
//   - -10^6 <= nums[i] <= 10^6
//
// Run:  cargo run --example lc_1480_running_sum_of_1d_array
// Test: cargo test --example lc_1480_running_sum_of_1d_array
// Bench: cargo test --example lc_1480_running_sum_of_1d_array bench -- --nocapture
//
// --- Rust concepts used ---
// - `mut` parameter: `fn running_sum(mut nums: Vec<i32>)` takes
//   ownership and makes the Vec mutable for in-place prefix sum
// - `.scan(init, closure)`: stateful iterator adaptor that carries
//   an accumulator, yielding each intermediate sum
//   (Previously covered: .scan() -> lc_0238, in-place mutation -> lc_0027,
//    .collect() -> lc_0151)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  35.83ms (avg 358ns)
// AI solution:  78.45ms (avg 784ns)
// -> My solution is 2.19x faster
// Reason: in-place mutation avoids allocating a new Vec; the scan
//   iterator allocates and writes to fresh memory.

/// My solution module — in-place prefix sum.
///
/// Time:  O(n)
/// Space: O(1) (modifies input in place)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute the running sum of the array.
        ///
        /// # Arguments
        ///
        /// * `nums` - Input array of integers
        ///
        /// # Returns
        ///
        /// Array where each element is the prefix sum up to that index
        pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
            for i in 1..nums.len() {
                nums[i] += nums[i - 1];
            }
            nums
        }
    }
}

/// AI solution module — scan iterator.
///
/// Uses `.scan()` to accumulate prefix sums in a functional style.
///
/// Time:  O(n)
/// Space: O(n) for the new Vec
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compute the running sum of the array.
        ///
        /// # Arguments
        ///
        /// * `nums` - Input array of integers
        ///
        /// # Returns
        ///
        /// Array where each element is the prefix sum up to that index
        pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
            nums.iter()
                .scan(0, |acc, &x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
        (vec![1, 2, 3, 4], vec![1, 3, 6, 10]),
        (vec![1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5]),
        (vec![3, 1, 2, 10, 1], vec![3, 4, 6, 16, 17]),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::running_sum(nums.clone());
        println!("  {nums:?} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::running_sum(nums.clone());
        println!("  {nums:?} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_running_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::running_sum(vec![1, 1, 1, 1, 1]),
                        vec![1, 2, 3, 4, 5]
                    );
                }

                #[test]
                fn test_example_3() {
                    assert_eq!(
                        Solution::running_sum(vec![3, 1, 2, 10, 1]),
                        vec![3, 4, 6, 16, 17]
                    );
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::running_sum(vec![5]), vec![5]);
                }

                #[test]
                fn test_negatives() {
                    assert_eq!(Solution::running_sum(vec![-1, -2, -3]), vec![-1, -3, -6]);
                }

                #[test]
                fn test_mixed() {
                    assert_eq!(Solution::running_sum(vec![1, -1, 1, -1]), vec![1, 0, 1, 0]);
                }

                #[test]
                fn test_zeros() {
                    assert_eq!(Solution::running_sum(vec![0, 0, 0]), vec![0, 0, 0]);
                }
            }
        };
    }

    test_running_sum!(my_tests, crate::my_solution::Solution);
    test_running_sum!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> =
            vec![vec![1, 2, 3, 4], vec![1, 1, 1, 1, 1], vec![3, 1, 2, 10, 1]];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::running_sum(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::running_sum(
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
