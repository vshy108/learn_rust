// LeetCode 45: Jump Game II [Medium]
// https://leetcode.com/problems/jump-game-ii/
//
// You are given a 0-indexed array of integers nums of length n. You are
// initially positioned at nums[0].
//
// Each element nums[i] represents the maximum length of a forward jump
// from index i. In other words, if you are at nums[i], you can jump to
// any nums[i + j] where:
//   - 0 <= j <= nums[i] and
//   - i + j < n
//
// Return the minimum number of jumps to reach nums[n - 1]. The test cases
// are generated such that you can reach nums[n - 1].
//
// Examples:
//   nums = [2,3,1,1,4] -> 2  (jump 1 step from 0 to 1, then 3 steps to 4)
//   nums = [2,3,0,1,4] -> 2
//
// Constraints:
//   - 1 <= nums.length <= 10^4
//   - 0 <= nums[i] <= 1000
//   - It's guaranteed that you can reach nums[n - 1]
//
// Run:  cargo run --example lc_0045_jump_game_ii
// Test: cargo test --example lc_0045_jump_game_ii
// Bench: cargo test --example lc_0045_jump_game_ii bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the minimum number of jumps to reach the last index.
        ///
        /// # Arguments
        ///
        /// * `nums` - A vector where each element is the max jump length
        ///   from that index
        ///
        /// # Returns
        ///
        /// The minimum number of jumps to reach the last index
        pub fn jump(nums: Vec<i32>) -> i32 {
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
        /// Return the minimum number of jumps to reach the last index.
        ///
        /// # Arguments
        ///
        /// * `nums` - A vector where each element is the max jump length
        ///   from that index
        ///
        /// # Returns
        ///
        /// The minimum number of jumps to reach the last index
        pub fn jump(nums: Vec<i32>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![2, 3, 1, 1, 4], 2),
        (vec![2, 3, 0, 1, 4], 2),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::jump(nums.clone());
        println!("  nums={nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::jump(nums.clone());
        println!("  nums={nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_jump {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::jump(vec![0]), 0);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::jump(vec![1, 1]), 1);
                }

                #[test]
                fn test_large_first_jump() {
                    assert_eq!(Solution::jump(vec![10, 1, 1, 1, 1]), 1);
                }

                #[test]
                fn test_all_ones() {
                    assert_eq!(Solution::jump(vec![1, 1, 1, 1, 1]), 4);
                }

                #[test]
                fn test_decreasing() {
                    assert_eq!(Solution::jump(vec![4, 3, 2, 1, 0]), 1);
                }
            }
        };
    }

    test_jump!(my_tests, crate::my_solution::Solution);
    test_jump!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![2, 3, 1, 1, 4],
            vec![2, 3, 0, 1, 4],
            vec![1, 1, 1, 1, 1],
            vec![10, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::jump(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::jump(
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
