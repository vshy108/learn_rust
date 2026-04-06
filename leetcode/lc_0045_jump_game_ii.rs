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
//
// --- Benchmark (100_000 iterations, 2026-03-27) ---
//   My solution: 54.32ms (avg 543ns)
//   AI solution: 48.14ms (avg 481ns)
//   -> AI solution is 1.13x faster
//   Reason: AI solution uses usize natively (no i32 casts per iteration),
//           early-exits when the jump window covers the end, and enumerate()
//           avoids repeated index-to-usize conversions.
//
// --- Rust concepts used ---
//   - Greedy BFS: track current jump window end + farthest reachable
//   - enumerate() + iter(): idiomatic indexed iteration without manual casting
//   - .take(n - 1): limit iterator length instead of range with cast
//   (Previously covered: i32 vs usize arithmetic -> lc_0005,
//    `as usize` cast -> lc_0004, early break when goal reached -> lc_0007)

/// My solution module.
///
/// Time:  O(n) -- single pass through the array
/// Space: O(1) -- only three scalar variables
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
            let n = nums.len() as i32;
            if n <= 1 {
                return 0;
            };

            let mut jumps = 0;
            let mut current_end = 0;
            let mut farthest = 0;

            for i in 0..n - 1 {
                farthest = farthest.max(i + nums[i as usize]);

                if i == current_end {
                    jumps += 1;
                    current_end = farthest;
                }
            }

            jumps
        }
    }
}

/// AI solution module.
///
/// Same greedy BFS approach but uses idiomatic `usize` throughout,
/// avoiding `i32` casts. Uses `enumerate()` + `iter()` instead of
/// index-based access, and early-returns when `current_end` reaches
/// or passes the last index.
///
/// Time:  O(n) -- single pass through the array
/// Space: O(1) -- only three scalar variables
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
            let n = nums.len();
            if n <= 1 {
                return 0;
            }

            let mut jumps = 0;
            let mut current_end: usize = 0;
            let mut farthest: usize = 0;

            // Iterate with enumerate; stop before the last index (n - 1)
            // because we only need to *reach* it, not jump from it.
            for (i, &val) in nums.iter().enumerate().take(n - 1) {
                // val is i32 from LeetCode signature; cast to usize for arithmetic.
                farthest = farthest.max(i + val as usize);

                if i == current_end {
                    jumps += 1;
                    current_end = farthest;

                    // Early exit: if current jump window already covers the end,
                    // no need to continue scanning.
                    if current_end >= n - 1 {
                        break;
                    }
                }
            }

            jumps
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![(vec![2, 3, 1, 1, 4], 2), (vec![2, 3, 0, 1, 4], 2)];

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
                std::hint::black_box(crate::my_solution::Solution::jump(std::hint::black_box(
                    nums.clone(),
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::jump(std::hint::black_box(
                    nums.clone(),
                )));
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
