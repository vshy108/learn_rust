// LeetCode 80: Remove Duplicates from Sorted Array II [Medium]
// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
//
// Given an integer array nums sorted in non-decreasing order, remove some
// duplicates in-place such that each unique element appears at most twice.
// The relative order of the elements should be kept the same.
//
// Since it is impossible to change the length of the array in some languages,
// you must instead have the result be placed in the first part of the array
// nums. More formally, if there are k elements after removing the duplicates,
// then the first k elements of nums should hold the final result.
//
// Return k after placing the final result in the first k slots of nums.
//
// Examples:
//   nums = [1,1,1,2,2,3]          -> 5, nums = [1,1,2,2,3,_]
//   nums = [0,0,1,1,1,1,2,3,3]   -> 7, nums = [0,0,1,1,2,3,3,_,_]
//
// Constraints:
//   - 1 <= nums.length <= 3 * 10^4
//   - -10^4 <= nums[i] <= 10^4
//   - nums is sorted in non-decreasing order
//
// Run:  cargo run --example lc_0080_remove_duplicates_from_sorted_array_ii
// Test: cargo test --example lc_0080_remove_duplicates_from_sorted_array_ii
// Bench: cargo test --example lc_0080_remove_duplicates_from_sorted_array_ii bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-04-04) ---
//   My solution:  72.81ms total (avg 728ns)
//   AI solution:  55.03ms total (avg 550ns)
//   -> AI solution is 1.32x faster
//   Reason: AI caches the previous two values in local vars (CPU registers),
//     avoiding repeated array loads + bounds checks in `nums[w - 2]`.
//     `continue` on 3rd+ duplicates skips the write entirely (no dead store).
//
// --- Rust concepts used ---
//   - Index-based loop: `for i in 0..nums.len()` avoids borrow conflict
//     that `for num in nums` would cause (can't iterate and mutate simultaneously)
//   - In-place two-pointer: write_index trails read index, overwriting duplicates
//   - Register-cached sliding window: local vars (`n2`, `n1`) avoid array reads
//   - `continue` to skip writes for 3rd+ duplicates (no dead store to array)
//   - `as i32`: usize to i32 cast for return type matching LeetCode signature
//   - `&mut Vec<i32>`: mutable reference allows in-place modification

/// My solution module — index-based two-pointer.
///
/// Time:  O(n) — single pass through the array
/// Space: O(1) — in-place, only a write pointer
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Remove duplicates so each element appears at most twice.
        ///
        /// # Arguments
        ///
        /// * `nums` - A mutable reference to a sorted integer vector
        ///
        /// # Returns
        ///
        /// The number of elements remaining (k). The first k elements
        /// of `nums` will be the result.
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.is_empty() {
                return 0;
            };

            let mut write_index = 0;

            // Ruby: `for num in nums` can read & write the same array.
            // Rust: can't iterate (`for num in nums`) and mutate (`nums[w] = num`)
            // simultaneously — borrow conflict. Use index-based loop instead.
            for i in 0..nums.len() {
                if write_index < 2 || nums[i] > nums[write_index - 2] {
                    nums[write_index] = nums[i];
                    write_index += 1;
                }
            }

            write_index as i32
        }
    }
}

/// AI solution module — register-cached sliding window.
///
/// Tracks the previous two elements (`n2`, `n1`) in local variables
/// instead of reading `nums[w - 2]` from the array each iteration.
/// The compiler can keep `n2`/`n1` in CPU registers, eliminating
/// repeated array loads and bounds checks on every comparison.
///
/// Why faster than index-based two-pointer:
/// - `nums[w - 2]` requires: bounds check + memory load each iteration
/// - `n2`/`n1` in registers: zero-cost comparison, no memory access
/// - `continue` skips the write entirely for duplicates (no dead store)
///
/// Time:  O(n) — single pass through the array
/// Space: O(1) — in-place, two cached variables + write pointer
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Remove duplicates so each element appears at most twice.
        ///
        /// Keeps a two-element sliding window (`n2`, `n1`) in local
        /// variables. If the current element equals both, it's a 3rd+
        /// duplicate and is skipped. Otherwise, it's written and the
        /// window slides forward.
        ///
        /// # Arguments
        ///
        /// * `nums` - A mutable reference to a sorted integer vector
        ///
        /// # Returns
        ///
        /// The number of elements remaining (k). The first k elements
        /// of `nums` will be the result.
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.len() < 3 {
                // 0, 1, or 2 elements can never exceed the "at most 2" rule
                return nums.len() as i32;
            }

            // Cache the first two elements in local vars (likely registers).
            // These track the sliding window of the last two *written* values.
            let mut n2 = nums[0]; // two positions back
            let mut n1 = nums[1]; // one position back
            let mut idx = 2;      // next write position

            for i in 2..nums.len() {
                let cur = nums[i];
                // If cur == n2 == n1, we already have two copies — skip
                if cur == n2 && cur == n1 {
                    continue;
                }
                // Write and slide the window forward
                nums[idx] = cur;
                n2 = n1;
                n1 = cur;
                idx += 1;
            }

            idx as i32
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![1, 1, 1, 2, 2, 3], 5, vec![1, 1, 2, 2, 3]),
        (
            vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
            7,
            vec![0, 0, 1, 1, 2, 3, 3],
        ),
    ];

    println!("--- My Solution ---");
    for (nums, expected_k, expected_nums) in &test_cases {
        let mut nums = nums.clone();
        let k = my_solution::Solution::remove_duplicates(&mut nums);
        println!(
            "  k={k}, nums={:?} (expected k={expected_k}, nums={expected_nums:?})",
            &nums[..k as usize]
        );
    }

    println!("--- AI Solution ---");
    for (nums, expected_k, expected_nums) in &test_cases {
        let mut nums = nums.clone();
        let k = ai_solution::Solution::remove_duplicates(&mut nums);
        println!(
            "  k={k}, nums={:?} (expected k={expected_k}, nums={expected_nums:?})",
            &nums[..k as usize]
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_remove_dups_ii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn run(mut nums: Vec<i32>, expected_k: i32, expected_nums: &[i32]) {
                    let k = Solution::remove_duplicates(&mut nums);
                    assert_eq!(k, expected_k);
                    assert_eq!(&nums[..k as usize], expected_nums);
                }

                #[test]
                fn test_example_1() {
                    run(vec![1, 1, 1, 2, 2, 3], 5, &[1, 1, 2, 2, 3]);
                }

                #[test]
                fn test_example_2() {
                    run(vec![0, 0, 1, 1, 1, 1, 2, 3, 3], 7, &[0, 0, 1, 1, 2, 3, 3]);
                }

                #[test]
                fn test_single_element() {
                    run(vec![1], 1, &[1]);
                }

                #[test]
                fn test_two_same() {
                    run(vec![1, 1], 2, &[1, 1]);
                }

                #[test]
                fn test_three_same() {
                    run(vec![1, 1, 1], 2, &[1, 1]);
                }

                #[test]
                fn test_no_duplicates() {
                    run(vec![1, 2, 3, 4], 4, &[1, 2, 3, 4]);
                }

                #[test]
                fn test_all_same_many() {
                    run(vec![5, 5, 5, 5, 5], 2, &[5, 5]);
                }
            }
        };
    }

    test_remove_dups_ii!(my_tests, crate::my_solution::Solution);
    test_remove_dups_ii!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let base_nums: Vec<Vec<i32>> = vec![
            vec![1, 1, 1, 2, 2, 3],
            vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
            vec![1, 2, 3, 4, 5],
            vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &base_nums {
                let mut nums = nums.clone();
                std::hint::black_box(crate::my_solution::Solution::remove_duplicates(
                    std::hint::black_box(&mut nums),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for nums in &base_nums {
                let mut nums = nums.clone();
                std::hint::black_box(crate::ai_solution::Solution::remove_duplicates(
                    std::hint::black_box(&mut nums),
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
