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

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
            todo!()
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
                    run(
                        vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
                        7,
                        &[0, 0, 1, 1, 2, 3, 3],
                    );
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
