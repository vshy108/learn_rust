// LeetCode 4: Median of Two Sorted Arrays
// https://leetcode.com/problems/median-of-two-sorted-arrays/
//
// Given two sorted arrays `nums1` and `nums2` of size m and n respectively,
// return the median of the two sorted arrays.
//
// The overall run time complexity should be O(log(m + n)).
//
// Examples:
//   nums1 = [1, 3],    nums2 = [2]       -> 2.0
//     (merged = [1, 2, 3], median = 2.0)
//   nums1 = [1, 2],    nums2 = [3, 4]    -> 2.5
//     (merged = [1, 2, 3, 4], median = (2 + 3) / 2.0 = 2.5)
//
// Constraints:
//   - nums1.len() == m, nums2.len() == n
//   - 0 <= m <= 1000
//   - 0 <= n <= 1000
//   - 1 <= m + n <= 2000
//   - -10^6 <= nums1[i], nums2[i] <= 10^6
//
// Run:  cargo run --example lc_0004_median_of_two_sorted_arrays
// Test: cargo test --example lc_0004_median_of_two_sorted_arrays
// Bench: cargo test --example lc_0004_median_of_two_sorted_arrays bench -- --nocapture
//
// --- Rust concepts used ---
//
// Binary search:
//   The O(log(min(m, n))) solution uses binary search on the shorter array
//   to find a partition where all left elements <= all right elements.
//   This is a classic divide-and-conquer approach.
//
// Floating-point division:
//   Integer division (`/`) in Rust truncates: `5 / 2 == 2`.
//   For the median of two elements, cast to `f64` first:
//   `(left + right) as f64 / 2.0`.
//
// Slice operations:
//   `nums.len()` returns usize.
//   `nums[i]` indexes into the slice (panics if out of bounds).
//   Slices are already sorted per the problem guarantee.
//
// i32::MIN and i32::MAX:
//   Used as sentinel values for empty partitions in the binary search
//   approach. `i32::MIN` represents negative infinity (left side),
//   `i32::MAX` represents positive infinity (right side).

/// My solution module — implement your solution here.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the median of two sorted arrays.
        ///
        /// # Arguments
        ///
        /// * `nums1` - First sorted array
        /// * `nums2` - Second sorted array
        ///
        /// # Returns
        ///
        /// The median value as f64
        pub fn find_median_sorted_arrays(
            nums1: Vec<i32>,
            nums2: Vec<i32>,
        ) -> f64 {
            todo!("Implement your solution here")
        }
    }
}

/// AI solution module — will be filled after you solve it yourself.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the median of two sorted arrays.
        ///
        /// # Arguments
        ///
        /// * `nums1` - First sorted array
        /// * `nums2` - Second sorted array
        ///
        /// # Returns
        ///
        /// The median value as f64
        pub fn find_median_sorted_arrays(
            nums1: Vec<i32>,
            nums2: Vec<i32>,
        ) -> f64 {
            todo!("AI solution will be provided after you complete yours")
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>, f64)> = vec![
        (vec![1, 3], vec![2], 2.0),
        (vec![1, 2], vec![3, 4], 2.5),
        (vec![], vec![1], 1.0),
        (vec![2], vec![], 2.0),
    ];

    println!("--- My Solution ---");
    for (nums1, nums2, expected) in &test_cases {
        let result = my_solution::Solution::find_median_sorted_arrays(
            nums1.clone(),
            nums2.clone(),
        );
        println!(
            "  {nums1:?} + {nums2:?} -> {result} (expected {expected})"
        );
    }

    // Uncomment after AI solution is filled in:
    // println!("--- AI Solution ---");
    // for (nums1, nums2, expected) in &test_cases {
    //     let result = ai_solution::Solution::find_median_sorted_arrays(
    //         nums1.clone(),
    //         nums2.clone(),
    //     );
    //     println!(
    //         "  {nums1:?} + {nums2:?} -> {result} (expected {expected})"
    //     );
    // }
}

#[cfg(test)]
mod tests {
    /// Macro that generates the same set of tests for any Solution type.
    macro_rules! test_median {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_odd_total() {
                    // merged = [1, 2, 3], median = 2.0
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![1, 3],
                            vec![2],
                        ),
                        2.0
                    );
                }

                #[test]
                fn test_even_total() {
                    // merged = [1, 2, 3, 4], median = (2 + 3) / 2 = 2.5
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![1, 2],
                            vec![3, 4],
                        ),
                        2.5
                    );
                }

                #[test]
                fn test_one_empty_left() {
                    // Only nums2 has elements
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![],
                            vec![1],
                        ),
                        1.0
                    );
                }

                #[test]
                fn test_one_empty_right() {
                    // Only nums1 has elements
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![2],
                            vec![],
                        ),
                        2.0
                    );
                }

                #[test]
                fn test_single_elements() {
                    // merged = [1, 2], median = 1.5
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![1],
                            vec![2],
                        ),
                        1.5
                    );
                }

                #[test]
                fn test_different_lengths() {
                    // merged = [1, 2, 3, 4, 5, 6], median = (3 + 4) / 2 = 3.5
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![1, 3, 5],
                            vec![2, 4, 6],
                        ),
                        3.5
                    );
                }

                #[test]
                fn test_no_overlap() {
                    // merged = [1, 2, 3, 4, 5, 6], median = 3.5
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![1, 2, 3],
                            vec![4, 5, 6],
                        ),
                        3.5
                    );
                }

                #[test]
                fn test_duplicate_values() {
                    // merged = [1, 1, 1, 1], median = 1.0
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![1, 1],
                            vec![1, 1],
                        ),
                        1.0
                    );
                }

                #[test]
                fn test_negative_numbers() {
                    // merged = [-3, -2, -1, 0], median = (-2 + -1) / 2 = -1.5
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![-3, -1],
                            vec![-2, 0],
                        ),
                        -1.5
                    );
                }

                #[test]
                fn test_large_difference_in_size() {
                    // merged = [1, 2, 3, 4, 5], median = 3.0
                    assert_eq!(
                        Solution::find_median_sorted_arrays(
                            vec![1],
                            vec![2, 3, 4, 5],
                        ),
                        3.0
                    );
                }
            }
        };
    }

    // Run the SAME tests against both solutions.
    test_median!(my_tests, crate::my_solution::Solution);

    // Uncomment after AI solution is filled in:
    // test_median!(ai_tests, crate::ai_solution::Solution);
}
