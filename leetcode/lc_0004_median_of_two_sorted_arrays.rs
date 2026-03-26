// LeetCode 4: Median of Two Sorted Arrays [Hard]
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
// --- Benchmark (100k iterations, input: two 250-element arrays) ---
//   My solution (binary search, f64 sentinels): 22.060ms avg (220ns/iter)
//   AI solution (binary search, i32 sentinels): 20.636ms avg (206ns/iter)
//   AI solution is 1.07x faster
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
        /// Binary search on the shorter array to find the correct partition
        /// point. At the partition, all elements on the left half are <=
        /// all elements on the right half across both arrays.
        ///
        /// # Arguments
        ///
        /// * `nums1` - First sorted array
        /// * `nums2` - Second sorted array
        ///
        /// # Returns
        ///
        /// The median value as f64
        ///
        /// # Complexity
        ///
        /// Time:  O(log(min(m, n))) — binary search on the shorter array
        /// Space: O(1)             — only a handful of scalar variables
        pub fn find_median_sorted_arrays(
            nums1: Vec<i32>,
            nums2: Vec<i32>,
        ) -> f64 {
            let nums1_len = nums1.len();
            let nums2_len = nums2.len();
            let larger_vec: Vec<i32>;
            let smaller_vec: Vec<i32>;
            let larger_len: usize;
            let smaller_len: usize;

            if nums1_len > nums2_len {
                larger_vec = nums1;
                smaller_vec = nums2;
                larger_len = nums1_len;
                smaller_len = nums2_len;
            } else {
                larger_vec = nums2;
                smaller_vec = nums1;
                larger_len = nums2_len;
                smaller_len = nums1_len;
            }

            // Use i32 instead of usize for left/right so that `right = i - 1`
            // is safe when i == 0. usize is unsigned — subtracting 1 from 0
            // wraps to usize::MAX (or panics in debug), causing an infinite
            // loop or out-of-bounds crash. i32 can go negative safely.
            let mut left: i32 = 0;
            let mut right: i32 = smaller_len as i32;

            while left <= right {
                // i32 arithmetic for safe binary search bounds.
                // Cast to usize only when indexing into the Vec.
                let i = (left + right) / 2;
                let j = (smaller_len as i32 + larger_len as i32 + 1) / 2 - i;

                // A: f64::MIN is negative smallest value in the f64
                let max_left_1  = if i == 0                  { f64::MIN } else { smaller_vec[(i - 1) as usize] as f64 };
                let min_right_1 = if i == smaller_len as i32 { f64::MAX } else { smaller_vec[i as usize] as f64 };
                let max_left_2  = if j == 0                  { f64::MIN } else { larger_vec[(j - 1) as usize] as f64 };
                let min_right_2 = if j == larger_len as i32  { f64::MAX } else { larger_vec[j as usize] as f64 };

                if max_left_1 <= min_right_2 && max_left_2 <= min_right_1 {
                    // A: check sum is odd
                    if (smaller_len + larger_len) % 2 != 0 {
                        return max_left_1.max(max_left_2);
                    } else {
                        return (max_left_1.max(max_left_2) + min_right_1.min(min_right_2)) / 2.0;
                    }
                } else if max_left_1 > min_right_2 {
                    // Took too many elements from smaller_vec — move left.
                    // Safe because i32: i can reach -1 without wrapping.
                    right = i - 1;
                } else {
                    left = i + 1;
                }
            }

            // The problem guarantees 1 <= m + n, so the binary search always
            // finds the partition and returns above. This is truly unreachable.
            unreachable!("binary search always finds the correct partition")
        }
    }
}

/// AI solution module.
///
/// Same binary search partitioning idea, but written more compactly:
/// - Borrows the arrays instead of moving them, using `&[i32]` slices.
/// - Uses `i32::MIN` / `i32::MAX` as sentinels so the partition check
///   stays entirely in the integer domain until the final division.
/// - Shorter variable names (`a`, `b`, `lo`, `hi`, `half`) reduce noise.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the median of two sorted arrays.
        ///
        /// Binary-searches for the partition index `i` on the shorter slice
        /// `a` (length `m`). The matching index `j = half - i` on `b` is
        /// derived so that the combined left half has `(m + n + 1) / 2`
        /// elements. When `a[i-1] <= b[j]` and `b[j-1] <= a[i]` both hold,
        /// the correct partition has been found.
        ///
        /// Using `i32::MIN` / `i32::MAX` as virtual boundary elements avoids
        /// special-casing the `i == 0` and `i == m` edges in the comparison.
        ///
        /// # Arguments
        ///
        /// * `nums1` - First sorted array
        /// * `nums2` - Second sorted array
        ///
        /// # Returns
        ///
        /// The median value as f64
        ///
        /// # Complexity
        ///
        /// Time:  O(log(min(m, n))) — binary search on the shorter array
        /// Space: O(1)             — only a handful of scalar variables
        pub fn find_median_sorted_arrays(
            nums1: Vec<i32>,
            nums2: Vec<i32>,
        ) -> f64 {
            // Always binary-search on the shorter slice to keep the search
            // space as small as possible.
            // A: use tuple to make a always the smaller vector
            //
            // .as_slice() converts Vec<i32> → &[i32] (a borrowed slice).
            // Without it the if/else arms would move the owned Vec into the
            // tuple, consuming the inputs. Since we only need to read elements
            // (for indexing), borrowing is correct and avoids an unnecessary
            // ownership transfer. Equivalent spellings: `&nums1[..]` or just
            // `&nums1` when the target type &[i32] is already known.
            let (a, b) = if nums1.len() <= nums2.len() {
                (nums1.as_slice(), nums2.as_slice())
            } else {
                (nums2.as_slice(), nums1.as_slice())
            };
            let (m, n) = (a.len() as i32, b.len() as i32);

            // `half` is the number of elements that belong in the left half
            // of the merged array. The +1 ensures correct rounding for odd
            // total lengths (the median lives in the left half).
            let half = (m + n + 1) / 2;

            // lo/hi are the valid range for the partition index `i` on `a`.
            let mut lo: i32 = 0;
            let mut hi: i32 = m;

            while lo <= hi {
                // `i` elements come from `a`; `j` elements from `b`.
                let i = (lo + hi) / 2;
                let j = half - i;

                // Virtual boundary elements: i32::MIN acts as -infinity
                // (always <= any real element) and i32::MAX as +infinity
                // (always >= any real element). This lets us skip the
                // out-of-bounds edge checks while keeping the logic uniform.
                let a_left  = if i == 0 { i32::MIN } else { a[(i - 1) as usize] };
                let a_right = if i == m { i32::MAX } else { a[i as usize] };
                let b_left  = if j == 0 { i32::MIN } else { b[(j - 1) as usize] };
                let b_right = if j == n { i32::MAX } else { b[j as usize] };

                if a_left <= b_right && b_left <= a_right {
                    // Correct partition found. The left half's maximum is the
                    // larger of the two left-boundary values.
                    if (m + n) % 2 != 0 {
                        // Odd total: median is the maximum left element.
                        return a_left.max(b_left) as f64;
                    } else {
                        // Even total: median is the mean of the boundary pair.
                        let max_left  = a_left.max(b_left) as f64;
                        let min_right = a_right.min(b_right) as f64;
                        return (max_left + min_right) / 2.0;
                    }
                } else if a_left > b_right {
                    // Took too many elements from `a` — shrink the window.
                    hi = i - 1;
                } else {
                    // Too few elements from `a` — grow the window.
                    lo = i + 1;
                }
            }

            // The problem guarantees 1 <= m + n, so the binary search always
            // finds the partition and returns above. This is truly unreachable.
            unreachable!("binary search always finds the correct partition")
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

    println!("--- AI Solution ---");
    for (nums1, nums2, expected) in &test_cases {
        let result = ai_solution::Solution::find_median_sorted_arrays(
            nums1.clone(),
            nums2.clone(),
        );
        println!(
            "  {nums1:?} + {nums2:?} -> {result} (expected {expected})"
        );
    }
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

    test_median!(ai_tests, crate::ai_solution::Solution);

    /// Compares the wall-clock time of both solutions over many iterations.
    ///
    /// Run with:
    ///   cargo test --example lc_0004_median_of_two_sorted_arrays bench -- --nocapture
    ///
    /// `std::hint::black_box` prevents the compiler from optimizing away
    /// the function call, ensuring we measure real execution time.
    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // Use a moderately sized pair of arrays to exercise the binary search
        // across a realistic input range.
        let nums1: Vec<i32> = (0..500).step_by(2).collect();   // 250 even numbers
        let nums2: Vec<i32> = (1..500).step_by(2).collect();   // 250 odd numbers

        // --- Benchmark my_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::find_median_sorted_arrays(
                    std::hint::black_box(nums1.clone()),
                    std::hint::black_box(nums2.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        // --- Benchmark ai_solution ---
        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::find_median_sorted_arrays(
                    std::hint::black_box(nums1.clone()),
                    std::hint::black_box(nums2.clone()),
                ),
            );
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
            let ratio = ai_duration.as_nanos() as f64
                / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64
                / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
