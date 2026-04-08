// LeetCode 457: Circular Array Loop [Medium]
// https://leetcode.com/problems/circular-array-loop/
//
// Given a circular array of non-zero integers nums, determine if there is
// a cycle. A cycle must:
//   - Have length > 1
//   - All elements in cycle must have same sign (all positive or all negative)
//   - Following nums[i] steps from index i (wrapping around)
//
// Examples:
//   [2,-1,1,2,2] -> true (cycle 0->2->3->0)
//   [-1,-2,-3,-4,-5,6] -> false (only size-1 cycle)
//   [1,-1,5,1,4] -> true (cycle 3->4->3)
//
// Constraints:
//   1 <= nums.length <= 5000
//   -1000 <= nums[i] <= 1000
//   nums[i] != 0
//
// Run:  cargo run --example lc_0457_circular_array_loop
// Test: cargo test --example lc_0457_circular_array_loop
// Bench: cargo test --example lc_0457_circular_array_loop bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 46ns  (fast/slow pointer)
//   AI solution: avg 45ns  (fast/slow pointer, inline next)
//   Ratio: ~1.0x

/// My solution module — fast/slow pointer approach.
///
/// For each starting index, use Floyd's cycle detection.
/// Mark visited nodes with 0 to skip later.
///
/// Time:  O(n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn circular_array_loop(nums: Vec<i32>) -> bool {
            let mut nums = nums;
            let n = nums.len() as i32;

            // Compute next index with wrapping
            let next = |i: i32, nums: &[i32]| -> i32 { ((i + nums[i as usize]) % n + n) % n };

            for i in 0..nums.len() {
                if nums[i] == 0 {
                    continue;
                }

                let mut slow = i as i32;
                let mut fast = next(i as i32, &nums);

                // Continue while direction is consistent
                while nums[slow as usize] * nums[fast as usize] > 0
                    && nums[slow as usize] * nums[next(fast, &nums) as usize] > 0
                {
                    if slow == fast {
                        // Check cycle length > 1
                        if slow != next(slow, &nums) {
                            return true;
                        }
                        break;
                    }
                    slow = next(slow, &nums);
                    fast = next(next(fast, &nums), &nums);
                }

                // Mark visited nodes with 0 to avoid re-processing
                let mut j = i as i32;
                while nums[j as usize] * nums[next(j, &nums) as usize] > 0 {
                    let tmp = next(j, &nums);
                    nums[j as usize] = 0;
                    j = tmp;
                }
            }
            false
        }
    }
}

/// AI solution module — same fast/slow pointer with cleanup.
///
/// Identical algorithm but structured with a helper closure for clarity.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn circular_array_loop(nums: Vec<i32>) -> bool {
            let mut nums = nums;
            let n = nums.len() as i32;

            // Inline next-index computation to avoid borrow conflicts
            let nxt = |idx: i32, nums: &[i32]| -> i32 { ((idx + nums[idx as usize]) % n + n) % n };

            for i in 0..n {
                if nums[i as usize] == 0 {
                    continue;
                }

                let (mut slow, mut fast) = (i, nxt(i, &nums));

                while nums[slow as usize] * nums[fast as usize] > 0
                    && nums[slow as usize] * nums[nxt(fast, &nums) as usize] > 0
                {
                    if slow == fast {
                        if slow != nxt(slow, &nums) {
                            return true;
                        }
                        break;
                    }
                    slow = nxt(slow, &nums);
                    fast = nxt(nxt(fast, &nums), &nums);
                }

                // Zero out the path that doesn't form a valid cycle
                let mut j = i;
                while nums[j as usize] * nums[nxt(j, &nums) as usize] > 0 {
                    let tmp = nxt(j, &nums);
                    nums[j as usize] = 0;
                    j = tmp;
                }
            }
            false
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_cycle_forward() {
            assert!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
        }
        #[test]
        fn test_no_cycle_all_negative() {
            assert!(!Solution::circular_array_loop(vec![-1, -2, -3, -4, -5, 6]));
        }
        #[test]
        fn test_mixed_direction_cycle() {
            // Cycle 3->4->3 exists (both positive)
            assert!(Solution::circular_array_loop(vec![1, -1, 5, 1, 4]));
        }
        #[test]
        fn test_all_negative_ones() {
            // 0->2->1->0, length 3, all negative: valid cycle
            assert!(Solution::circular_array_loop(vec![-1, -1, -1]));
        }
        #[test]
        fn test_self_loops() {
            // [2,2] with n=2: each index loops to itself (size 1), invalid
            assert!(!Solution::circular_array_loop(vec![2, 2]));
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>) -> bool,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(vec![2, -1, 1, 2, 2])));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::circular_array_loop);
    bench_solution("AI solution", ai_solution::Solution::circular_array_loop);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::circular_array_loop(vec![2, -1, 1, 2, 2])
    );
}
