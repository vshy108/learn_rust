// LeetCode 704: Binary Search [Easy]
// https://leetcode.com/problems/binary-search/
//
// Given an array of integers `nums` which is sorted in ascending order, and
// an integer `target`, write a function to search `target` in `nums`. If
// `target` exists, then return its index. Otherwise, return -1.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Examples:
//   nums = [-1,0,3,5,9,12], target = 9 -> 4
//   nums = [-1,0,3,5,9,12], target = 2 -> -1
//
// Constraints:
//   - 1 <= nums.length <= 10^4
//   - -10^4 < nums[i], target < 10^4
//   - All the integers in nums are unique
//   - nums is sorted in ascending order
//
// Run:  cargo run --example lc_0704_binary_search
// Test: cargo test --example lc_0704_binary_search
// Bench: cargo test --example lc_0704_binary_search bench -- --nocapture
//
// --- Rust concepts used ---
// - `usize` exclusive upper bound pattern: `right = nums.len()` with
//   `left < right` and `right = mid` avoids underflow when right = 0
// - i32 inclusive bounds with `as usize` indexing: simpler loop but
//   requires casting mid to usize for array access
//   (Previously covered: binary search -> lc_0004, match .cmp() -> lc_0374,
//    as i32/usize casts -> lc_0004)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  34.35ms (avg 343ns)
// AI solution:  33.09ms (avg 330ns)
// -> AI solution is 1.04x faster
// Reason: usize arithmetic avoids the `as usize` cast per iteration;
//   both are effectively identical at this scale.

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
/// My solution module — classic binary search with inclusive bounds.
///
/// Time:  O(log n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Search for target in a sorted array, returning its index or -1.
        ///
        /// # Arguments
        ///
        /// * `nums` - Sorted integer array (ascending)
        /// * `target` - The value to search for
        ///
        /// # Returns
        ///
        /// Index of target, or -1 if not found
        pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            let mut left: i32 = 0;
            let mut right: i32 = nums.len() as i32 - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if nums[mid as usize] == target {
                    return mid;
                } else if nums[mid as usize] < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            -1
        }
    }
}

/// AI solution module — binary search using usize with exclusive upper bound.
///
/// Uses `left < right` loop with `right = mid` instead of `mid - 1`,
/// avoiding the i32 cast and underflow concerns.
///
/// Time:  O(log n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Search for target in a sorted array, returning its index or -1.
        ///
        /// # Arguments
        ///
        /// * `nums` - Sorted integer array (ascending)
        /// * `target` - The value to search for
        ///
        /// # Returns
        ///
        /// Index of target, or -1 if not found
        pub fn search(nums: Vec<i32>, target: i32) -> i32 {
            if nums.is_empty() {
                return -1;
            }
            let mut left: usize = 0;
            let mut right: usize = nums.len();
            while left < right {
                let mid = left + (right - left) / 2;
                match nums[mid].cmp(&target) {
                    std::cmp::Ordering::Equal => return mid as i32,
                    std::cmp::Ordering::Less => left = mid + 1,
                    std::cmp::Ordering::Greater => right = mid,
                }
            }
            -1
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
        (vec![-1, 0, 3, 5, 9, 12], 9, 4),
        (vec![-1, 0, 3, 5, 9, 12], 2, -1),
    ];

    println!("--- My Solution ---");
    for (nums, target, expected) in &test_cases {
        let result = my_solution::Solution::search(nums.clone(), *target);
        println!("  {nums:?}, target={target} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, target, expected) in &test_cases {
        let result = ai_solution::Solution::search(nums.clone(), *target);
        println!("  {nums:?}, target={target} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_search {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
                }

                #[test]
                fn test_single_element_found() {
                    assert_eq!(Solution::search(vec![5], 5), 0);
                }

                #[test]
                fn test_single_element_not_found() {
                    assert_eq!(Solution::search(vec![5], 3), -1);
                }

                #[test]
                fn test_first_element() {
                    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 1), 0);
                }

                #[test]
                fn test_last_element() {
                    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5], 5), 4);
                }

                #[test]
                fn test_negative_numbers() {
                    assert_eq!(Solution::search(vec![-10, -5, -3, 0, 2], -5), 1);
                }
            }
        };
    }

    test_search!(my_tests, crate::my_solution::Solution);
    test_search!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32)> = vec![
            (vec![-1, 0, 3, 5, 9, 12], 9),
            (vec![-1, 0, 3, 5, 9, 12], 2),
            (vec![1, 2, 3, 4, 5], 1),
            (vec![1, 2, 3, 4, 5], 5),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, target) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::search(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*target),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums, target) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::search(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(*target),
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
