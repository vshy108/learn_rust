// LeetCode 88: Merge Sorted Array [Easy]
// https://leetcode.com/problems/merge-sorted-array/
//
// You are given two integer arrays nums1 and nums2, sorted in non-decreasing
// order, and two integers m and n, representing the number of elements in
// nums1 and nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead
// be stored inside the array nums1. To accommodate this, nums1 has a length
// of m + n, where the first m elements denote the elements that should be
// merged, and the last n elements are set to 0 and should be ignored.
// nums2 has a length of n.
//
// Examples:
//   nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//     -> nums1 = [1,2,2,3,5,6]
//   nums1 = [1], m = 1, nums2 = [], n = 0
//     -> nums1 = [1]
//   nums1 = [0], m = 0, nums2 = [1], n = 1
//     -> nums1 = [1]
//
// Constraints:
//   - nums1.length == m + n
//   - nums2.length == n
//   - 0 <= m, n <= 200
//   - 1 <= m + n <= 200
//   - -10^9 <= nums1[i], nums2[j] <= 10^9
//
// Follow up: Can you come up with an algorithm that runs in O(m + n) time?
//
// Run:  cargo run --example lc_0088_merge_sorted_array
// Test: cargo test --example lc_0088_merge_sorted_array
// Bench: cargo test --example lc_0088_merge_sorted_array bench -- --nocapture
//
// --- Benchmark (100,000 iterations, 2026-03-29) ---
//   My solution:  75.21ms total (avg 752ns)
//   AI solution:  80.81ms total (avg 808ns)
//   -> My solution is 1.07x faster
//   Reason: nearly identical algorithms; my_solution's manual while loop
//     has marginally less overhead than copy_from_slice on tiny arrays.
//     On larger inputs copy_from_slice's memcpy would win.
//
// --- Rust concepts used ---
//   - copy_from_slice: bulk memcpy for remaining elements (AI solution)
//   - Tuple destructuring: `let (mut i, mut j, mut k) = (...)` for compact init
//   (Previously covered: usize indexing lc_0001, > 0 guard pattern lc_0005,
//    &mut Vec<i32> lc_0027, as usize lc_0004)

/// My solution module — reverse two-pointer merge.
///
/// Time:  O(m + n) — single reverse pass through both arrays
/// Space: O(1) — in-place, no extra allocations
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Merge `nums2` into `nums1` in-place, maintaining sorted order.
        ///
        /// # Arguments
        ///
        /// * `nums1` - First sorted array with m elements followed by n zeros
        /// * `m` - Number of real elements in nums1
        /// * `nums2` - Second sorted array with n elements
        /// * `n` - Number of elements in nums2
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            // Use usize for indexing — Rust Vecs can only be indexed by usize, not i32.
            // Use > 0 and decrement before indexing so usize never goes negative (would panic).
            let mut i = m as usize;
            let mut j = n as usize;
            let mut k = (m + n) as usize;

            while i > 0 && j > 0 {
                if nums1[i - 1] > nums2[j - 1] {
                    k -= 1;
                    i -= 1;
                    nums1[k] = nums1[i];
                } else {
                    k -= 1;
                    j -= 1;
                    nums1[k] = nums2[j];
                }
            }

            // Copy remaining nums2 elements — no need to copy nums1 leftovers
            // since they're already in place
            while j > 0 {
                k -= 1;
                j -= 1;
                nums1[k] = nums2[j];
            }
        }
    }
}

/// AI solution module — slice `copy_within` + `copy_from_slice`.
///
/// Uses `copy_from_slice` to bulk-copy remaining nums2 elements
/// instead of a manual while loop. Same algorithmic approach but
/// leverages Rust's optimized memcpy for the tail copy.
///
/// Time:  O(m + n) — single reverse pass through both arrays
/// Space: O(1) — in-place, no extra allocations
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Merge `nums2` into `nums1` in-place, maintaining sorted order.
        ///
        /// # Arguments
        ///
        /// * `nums1` - First sorted array with m elements followed by n zeros
        /// * `m` - Number of real elements in nums1
        /// * `nums2` - Second sorted array with n elements
        /// * `n` - Number of elements in nums2
        pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
            let (mut i, mut j, mut k) = (m as usize, n as usize, (m + n) as usize);

            while i > 0 && j > 0 {
                if nums1[i - 1] > nums2[j - 1] {
                    i -= 1;
                    k -= 1;
                    nums1[k] = nums1[i];
                } else {
                    j -= 1;
                    k -= 1;
                    nums1[k] = nums2[j];
                }
            }

            // Bulk-copy remaining nums2 elements via copy_from_slice —
            // uses optimized memcpy instead of element-by-element loop
            if j > 0 {
                nums1[..j].copy_from_slice(&nums2[..j]);
            }
        }
    }
}

fn main() {
    #[allow(clippy::type_complexity)]
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>, i32, Vec<i32>)> = vec![
        (
            vec![1, 2, 3, 0, 0, 0],
            3,
            vec![2, 5, 6],
            3,
            vec![1, 2, 2, 3, 5, 6],
        ),
        (vec![1], 1, vec![], 0, vec![1]),
        (vec![0], 0, vec![1], 1, vec![1]),
    ];

    println!("--- My Solution ---");
    for (nums1, m, nums2, n, expected) in &test_cases {
        let mut nums1 = nums1.clone();
        let mut nums2 = nums2.clone();
        my_solution::Solution::merge(&mut nums1, *m, &mut nums2, *n);
        println!("  nums1={nums1:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (nums1, m, nums2, n, expected) in &test_cases {
        let mut nums1 = nums1.clone();
        let mut nums2 = nums2.clone();
        ai_solution::Solution::merge(&mut nums1, *m, &mut nums2, *n);
        println!("  nums1={nums1:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_merge {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn run(mut nums1: Vec<i32>, m: i32, mut nums2: Vec<i32>, n: i32, expected: &[i32]) {
                    Solution::merge(&mut nums1, m, &mut nums2, n);
                    assert_eq!(nums1, expected);
                }

                #[test]
                fn test_example_1() {
                    run(
                        vec![1, 2, 3, 0, 0, 0],
                        3,
                        vec![2, 5, 6],
                        3,
                        &[1, 2, 2, 3, 5, 6],
                    );
                }

                #[test]
                fn test_nums2_empty() {
                    run(vec![1], 1, vec![], 0, &[1]);
                }

                #[test]
                fn test_nums1_empty() {
                    run(vec![0], 0, vec![1], 1, &[1]);
                }

                #[test]
                fn test_interleaved() {
                    run(
                        vec![1, 3, 5, 0, 0, 0],
                        3,
                        vec![2, 4, 6],
                        3,
                        &[1, 2, 3, 4, 5, 6],
                    );
                }

                #[test]
                fn test_nums2_all_smaller() {
                    run(
                        vec![4, 5, 6, 0, 0, 0],
                        3,
                        vec![1, 2, 3],
                        3,
                        &[1, 2, 3, 4, 5, 6],
                    );
                }

                #[test]
                fn test_nums2_all_larger() {
                    run(
                        vec![1, 2, 3, 0, 0, 0],
                        3,
                        vec![4, 5, 6],
                        3,
                        &[1, 2, 3, 4, 5, 6],
                    );
                }

                #[test]
                fn test_duplicates() {
                    run(vec![1, 2, 2, 0, 0], 3, vec![2, 3], 2, &[1, 2, 2, 2, 3]);
                }
            }
        };
    }

    test_merge!(my_tests, crate::my_solution::Solution);
    test_merge!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<i32>, i32, Vec<i32>, i32)> = vec![
            (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3),
            (vec![1, 3, 5, 0, 0, 0], 3, vec![2, 4, 6], 3),
            (vec![4, 5, 6, 0, 0, 0], 3, vec![1, 2, 3], 3),
            (vec![0], 0, vec![1], 1),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums1, m, nums2, n) in &inputs {
                let mut nums1 = nums1.clone();
                let mut nums2 = nums2.clone();
                std::hint::black_box(crate::my_solution::Solution::merge(
                    std::hint::black_box(&mut nums1),
                    std::hint::black_box(*m),
                    std::hint::black_box(&mut nums2),
                    std::hint::black_box(*n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (nums1, m, nums2, n) in &inputs {
                let mut nums1 = nums1.clone();
                let mut nums2 = nums2.clone();
                std::hint::black_box(crate::ai_solution::Solution::merge(
                    std::hint::black_box(&mut nums1),
                    std::hint::black_box(*m),
                    std::hint::black_box(&mut nums2),
                    std::hint::black_box(*n),
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
