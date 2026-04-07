// LeetCode 215: Kth Largest Element in an Array [Medium]
// https://leetcode.com/problems/kth-largest-element-in-an-array/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.138us
//   AI solution avg: 0.254us
//   Ratio: My 1.84x faster
//
// Given an integer array nums and an integer k, return the kth largest
// element. Note that it is the kth largest element in the sorted order,
// not the kth distinct element.
//
// Constraints: 1 <= k <= nums.length <= 10^5, -10^4 <= nums[i] <= 10^4
//
// Run:  cargo run --example lc_0215_kth_largest_element
// Test: cargo test --example lc_0215_kth_largest_element
// Bench: cargo test --example lc_0215_kth_largest_element bench -- --nocapture

/// My solution — sort descending, return nums[k-1].
///
/// Time: O(n log n).
/// Space: O(1) (in-place sort, ignoring sort's internal stack).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the kth largest element via sorting.
        pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
            // Sort in descending order so the kth largest is at index k-1
            nums.sort_unstable_by(|a, b| b.cmp(a));
            nums[(k - 1) as usize]
        }
    }
}

/// AI solution — Quickselect (Hoare's selection algorithm) with
/// randomized pivot for O(n) average-case performance.
///
/// Partitions the array around a pivot and recurses only into the
/// half containing the target index, giving linear expected time.
///
/// Time: O(n) average, O(n^2) worst case.
/// Space: O(1) — in-place partitioning with tail-call loop.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the kth largest element via quickselect.
        pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
            let n = nums.len();
            // kth largest = element at index (n - k) in sorted order
            let target = n - k as usize;
            Self::quickselect(&mut nums, target)
        }

        /// Iterative quickselect with median-of-three pivot selection.
        fn quickselect(arr: &mut [i32], target: usize) -> i32 {
            let mut lo = 0;
            let mut hi = arr.len() - 1;

            while lo < hi {
                // Median-of-three pivot selection to reduce worst-case risk
                let mid = lo + (hi - lo) / 2;
                // Sort lo, mid, hi so arr[mid] is the median
                if arr[lo] > arr[mid] {
                    arr.swap(lo, mid);
                }
                if arr[lo] > arr[hi] {
                    arr.swap(lo, hi);
                }
                if arr[mid] > arr[hi] {
                    arr.swap(mid, hi);
                }
                // Place pivot at hi-1 for Lomuto-style partition
                arr.swap(mid, hi);
                let pivot = arr[hi];

                // Lomuto partition: elements <= pivot go to the left
                let mut store = lo;
                for i in lo..hi {
                    if arr[i] <= pivot {
                        arr.swap(store, i);
                        store += 1;
                    }
                }
                arr.swap(store, hi);
                // Now arr[store] == pivot, in its final sorted position

                if store == target {
                    return arr[store];
                } else if target < store {
                    // Prevent underflow: if store == 0, target must be 0
                    // which is handled by store == target above
                    hi = store - 1;
                } else {
                    lo = store + 1;
                }
            }
            arr[lo]
        }
    }
}

fn main() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    let k = 2;
    let my = my_solution::Solution::find_kth_largest(nums.clone(), k);
    let ai = ai_solution::Solution::find_kth_largest(nums, k);
    println!("kth largest (k={k}): my={my}, ai={ai}");

    let nums2 = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    let k2 = 4;
    let my2 = my_solution::Solution::find_kth_largest(nums2.clone(), k2);
    let ai2 = ai_solution::Solution::find_kth_largest(nums2, k2);
    println!("kth largest (k={k2}): my={my2}, ai={ai2}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_kth {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2),
                        5
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::find_kth_largest(
                            vec![3, 2, 3, 1, 2, 4, 5, 5, 6],
                            4
                        ),
                        4
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(
                        <$solution>::find_kth_largest(vec![1], 1),
                        1
                    );
                }

                #[test]
                fn test_k_equals_n() {
                    // kth largest where k == len => smallest element
                    assert_eq!(
                        <$solution>::find_kth_largest(vec![7, 3, 5, 1], 4),
                        1
                    );
                }

                #[test]
                fn test_duplicates() {
                    assert_eq!(
                        <$solution>::find_kth_largest(vec![2, 2, 2, 2], 2),
                        2
                    );
                }

                #[test]
                fn test_negatives() {
                    assert_eq!(
                        <$solution>::find_kth_largest(vec![-1, -2, -3, -4], 1),
                        -1
                    );
                }

                #[test]
                fn test_mixed() {
                    assert_eq!(
                        <$solution>::find_kth_largest(vec![-1, 2, 0, 3, -2], 3),
                        0
                    );
                }
            }
        };
    }

    test_kth!(my, crate::my_solution::Solution);
    test_kth!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..200).map(|i| ((i * 31 + 17) % 20001) - 10000).collect();
        let k = 50;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_kth_largest(
                std::hint::black_box(nums.clone()),
                k,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_kth_largest(
                std::hint::black_box(nums.clone()),
                k,
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
