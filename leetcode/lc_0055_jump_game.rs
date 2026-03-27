// LeetCode 55: Jump Game [Medium]
// https://leetcode.com/problems/jump-game/
//
// You are given an integer array nums. You are initially positioned at the
// array's first index, and each element in the array represents your maximum
// jump length at that position.
//
// Return true if you can reach the last index, or false otherwise.
//
// Examples:
//   nums = [2,3,1,1,4] -> true  (jump 1 to index 1, then 3 to last)
//   nums = [3,2,1,0,4] -> false (always arrive at index 3 with 0 jump)
//
// Constraints:
//   - 1 <= nums.length <= 10^4
//   - 0 <= nums[i] <= 10^5
//
// Run:  cargo run --example lc_0055_jump_game
// Test: cargo test --example lc_0055_jump_game
// Bench: cargo test --example lc_0055_jump_game bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-03-27) ---
//   My solution: 81.74ms (avg 817ns)
//   AI solution: 51.88ms (avg 518ns)
//   -> AI solution is 1.58x faster
//   Reason: Backward greedy does fewer comparisons on average -- it skips
//           positions that can't reach the current goal, while forward greedy
//           always computes max() at every index even after reaching the end.
//
// --- Rust concepts used ---
//   - Forward greedy: track max reachable index, early return on unreachable
//   - Backward greedy: collapse goal from end to start
//   - as usize cast: i32 -> usize for index arithmetic (safe when non-negative)
//   - (0..n).rev(): reverse range iterator for right-to-left scan
//   - usize::max(): Ord trait method for updating max reachable

/// My solution module.
///
/// Time:  O(n) -- single forward pass tracking max reachable index
/// Space: O(1) -- only two scalar variables
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if you can reach the last index.
        ///
        /// # Arguments
        ///
        /// * `nums` - A vector where each element is the max jump length
        ///   from that index
        ///
        /// # Returns
        ///
        /// `true` if the last index is reachable, `false` otherwise
        pub fn can_jump(nums: Vec<i32>) -> bool {
            let mut max_reach = 0;
            let n = nums.len();

            for i in 0..n {
                if i > max_reach {
                    return false;
                };

                // nums[i] is i32 (LeetCode signature), i is usize — Rust won't
                // auto-cast between integer types. `as usize` is safe here because
                // the constraint guarantees 0 <= nums[i] (no negative truncation).
                max_reach = max_reach.max(i + nums[i] as usize);
            }

            true
        }
    }
}

/// AI solution module.
///
/// Backward greedy: start `goal` at the last index and scan right-to-left.
/// If position `i` can reach `goal` (i.e. `i + nums[i] >= goal`), move
/// `goal` back to `i`. If `goal` reaches 0, the start can reach the end.
///
/// Time:  O(n) -- single backward pass
/// Space: O(1) -- one scalar variable
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if you can reach the last index.
        ///
        /// # Arguments
        ///
        /// * `nums` - A vector where each element is the max jump length
        ///   from that index
        ///
        /// # Returns
        ///
        /// `true` if the last index is reachable, `false` otherwise
        pub fn can_jump(nums: Vec<i32>) -> bool {
            // Start goal at the last index; work backwards.
            let mut goal = nums.len() - 1;

            // (0..goal) scanned in reverse: if position i can jump to goal,
            // then goal becomes i (we only need to reach i now).
            for i in (0..nums.len() - 1).rev() {
                if i + nums[i] as usize >= goal {
                    goal = i;
                }
            }

            // If goal collapsed to 0, the start can reach the end.
            goal == 0
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, bool)> =
        vec![(vec![2, 3, 1, 1, 4], true), (vec![3, 2, 1, 0, 4], false)];

    println!("--- My Solution ---");
    for (nums, expected) in &test_cases {
        let result = my_solution::Solution::can_jump(nums.clone());
        println!("  nums={nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &test_cases {
        let result = ai_solution::Solution::can_jump(nums.clone());
        println!("  nums={nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_can_jump {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_reachable() {
                    assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
                }

                #[test]
                fn test_not_reachable() {
                    assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
                }

                #[test]
                fn test_single_element() {
                    assert!(Solution::can_jump(vec![0]));
                }

                #[test]
                fn test_two_elements_reachable() {
                    assert!(Solution::can_jump(vec![1, 0]));
                }

                #[test]
                fn test_large_first_jump() {
                    assert!(Solution::can_jump(vec![10, 0, 0, 0, 1]));
                }

                #[test]
                fn test_all_zeros_except_first() {
                    assert!(!Solution::can_jump(vec![0, 1, 2]));
                }

                #[test]
                fn test_all_ones() {
                    assert!(Solution::can_jump(vec![1, 1, 1, 1, 1]));
                }
            }
        };
    }

    test_can_jump!(my_tests, crate::my_solution::Solution);
    test_can_jump!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![2, 3, 1, 1, 4],
            vec![3, 2, 1, 0, 4],
            vec![1, 1, 1, 1, 1],
            vec![10, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::my_solution::Solution::can_jump(
                    std::hint::black_box(nums.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::can_jump(
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
