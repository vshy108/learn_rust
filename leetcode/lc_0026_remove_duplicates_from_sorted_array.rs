// LeetCode 26: Remove Duplicates from Sorted Array [Easy]
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
//
// Given an integer array nums sorted in non-decreasing order, remove the
// duplicates in-place such that each unique element appears only once.
// The relative order of the elements should be kept the same. Then return
// the number of unique elements in nums.
//
// Consider the number of unique elements of nums to be k, to get accepted,
// you need to do the following things:
//   - Change the array nums such that the first k elements of nums contain
//     the unique elements in the order they were present in nums initially.
//   - The remaining elements of nums are not important as well as the size
//     of nums.
//   - Return k.
//
// Examples:
//   nums = [1,1,2]                   -> 2, nums = [1,2,_]
//   nums = [0,0,1,1,1,2,2,3,3,4]    -> 5, nums = [0,1,2,3,4,_,_,_,_,_]
//
// Constraints:
//   - 1 <= nums.length <= 3 * 10^4
//   - -100 <= nums[i] <= 100
//   - nums is sorted in non-decreasing order
//
// Run:  cargo run --example lc_0026_remove_duplicates_from_sorted_array
// Test: cargo test --example lc_0026_remove_duplicates_from_sorted_array
// Bench: cargo test --example lc_0026_remove_duplicates_from_sorted_array bench -- --nocapture
//
// Benchmark (100_000 iterations, 2026-03-27):
//   My solution (two-pointer): 64.61ms (avg 646ns)
//   AI solution (Vec::dedup):  47.35ms (avg 473ns)
//   -> AI solution is 1.36x faster
//   Reason: Vec::dedup is implemented in optimized unsafe Rust in the standard library,
//   using ptr::copy for bulk moves instead of element-by-element assignment.
/// My solution module.
///
/// Time:  O(n) — single pass with two pointers
/// Space: O(1) — in-place, only two index variables
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Remove duplicates from a sorted array in-place.
        ///
        /// # Arguments
        ///
        /// * `nums` - A mutable reference to a sorted integer vector
        ///
        /// # Returns
        ///
        /// The number of unique elements (k). The first k elements
        /// of `nums` will contain the unique values in order.
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            if nums.is_empty() {
                return 0;
            };

            let mut write_index = 1;

            for read_index in 1..nums.len() {
                if nums[read_index] != nums[read_index - 1] {
                    nums[write_index] = nums[read_index];
                    write_index += 1;
                }
            }

            write_index as i32
        }
    }
}

/// AI solution module.
///
/// Time:  O(n) — Vec::dedup scans the entire array once
/// Space: O(1) — in-place deduplication, no extra allocation
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Remove duplicates from a sorted array in-place.
        ///
        /// # Arguments
        ///
        /// * `nums` - A mutable reference to a sorted integer vector
        ///
        /// # Returns
        ///
        /// The number of unique elements (k). The first k elements
        /// of `nums` will contain the unique values in order.
        pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
            // `dedup()` removes consecutive equal elements in-place.
            // Since `nums` is already sorted, all duplicates are consecutive,
            // making this a single-call O(n) solution.
            // Internally, dedup uses the same two-pointer pattern as my_solution,
            // but is implemented in optimized unsafe Rust in the standard library.
            nums.dedup();
            nums.len() as i32
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![1, 1, 2], 2, vec![1, 2]),
        (vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5, vec![0, 1, 2, 3, 4]),
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
    macro_rules! test_remove_dups {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn run(mut nums: Vec<i32>, expected_k: i32, expected_nums: &[i32]) {
                    let k = Solution::remove_duplicates(&mut nums);
                    assert_eq!(k, expected_k);
                    assert_eq!(&nums[..k as usize], expected_nums);
                }

                #[test]
                fn test_basic_two_elements() {
                    run(vec![1, 1, 2], 2, &[1, 2]);
                }

                #[test]
                fn test_longer_array() {
                    run(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4], 5, &[0, 1, 2, 3, 4]);
                }

                #[test]
                fn test_single_element() {
                    run(vec![1], 1, &[1]);
                }

                #[test]
                fn test_no_duplicates() {
                    run(vec![1, 2, 3, 4, 5], 5, &[1, 2, 3, 4, 5]);
                }

                #[test]
                fn test_all_same() {
                    run(vec![5, 5, 5, 5], 1, &[5]);
                }

                #[test]
                fn test_negative_values() {
                    run(vec![-3, -1, -1, 0, 0, 2], 4, &[-3, -1, 0, 2]);
                }

                #[test]
                fn test_two_elements_same() {
                    run(vec![1, 1], 1, &[1]);
                }
            }
        };
    }

    test_remove_dups!(my_tests, crate::my_solution::Solution);
    test_remove_dups!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let base_nums: Vec<Vec<i32>> = vec![
            vec![1, 1, 2],
            vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
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
