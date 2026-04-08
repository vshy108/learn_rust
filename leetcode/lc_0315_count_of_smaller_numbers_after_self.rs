// LeetCode 315. Count of Smaller Numbers After Self
// https://leetcode.com/problems/count-of-smaller-numbers-after-self/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.186us
//   ai_solution  avg: 0.211us
//   Ratio: my_solution 1.14x faster

/// Given integer array nums, return counts[i] = number of smaller elements
/// to the right of nums[i].
///
/// Example 1:
///   Input: nums = [5,2,6,1]
///   Output: [2,1,1,0]
///
/// Example 2:
///   Input: nums = [-1]
///   Output: [0]
///
/// Example 3:
///   Input: nums = [-1,-1]
///   Output: [0,0]
///
/// Constraints:
///   - 1 <= nums.length <= 10^5
///   - -10^4 <= nums[i] <= 10^4

/// # my_solution: Binary Indexed Tree with coordinate compression
/// Time: O(n log n) — BIT operations are O(log n) each
/// Space: O(n) for BIT and coordinate mapping
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count smaller elements to the right using a BIT.
        ///
        /// We coordinate-compress the values, then iterate from right to left.
        /// For each value, query BIT for prefix sum [1, rank-1] (count of
        /// smaller elements already inserted), then update BIT at rank.
        ///
        /// # Arguments
        /// * `nums` - Input array
        ///
        /// # Returns
        /// Vec where result[i] = count of smaller elements to the right of nums[i]
        pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
            // Coordinate compress: deduplicate + sort, then map value -> rank (1-indexed)
            let mut sorted = nums.clone();
            sorted.sort_unstable();
            sorted.dedup();

            // Binary search to find rank of each value
            let rank = |v: i32| -> usize { sorted.partition_point(|&x| x < v) + 1 };

            let m = sorted.len();
            let mut bit = vec![0i32; m + 1];

            // BIT helper: add delta at position x
            let update = |bit: &mut Vec<i32>, mut x: usize, delta: i32| {
                while x <= m {
                    bit[x] += delta;
                    // lowbit: isolate lowest set bit
                    x += x & x.wrapping_neg();
                }
            };

            // BIT helper: prefix sum [1, x]
            let query = |bit: &[i32], mut x: usize| -> i32 {
                let mut s = 0;
                while x > 0 {
                    s += bit[x];
                    x -= x & x.wrapping_neg();
                }
                s
            };

            let n = nums.len();
            let mut ans = vec![0i32; n];

            // Iterate right to left: query how many smaller, then insert current
            for i in (0..n).rev() {
                let r = rank(nums[i]);
                ans[i] = if r > 1 { query(&bit, r - 1) } else { 0 };
                update(&mut bit, r, 1);
            }

            ans
        }
    }
}

/// # ai_solution: Merge sort counting inversions
/// Time: O(n log n) — merge sort
/// Space: O(n) for temporary array and index tracking
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count smaller elements to the right using modified merge sort.
        ///
        /// Pair each element with its original index. During merge sort,
        /// when an element from the right half is placed before one from
        /// the left half, it means it's smaller and comes after — increment count.
        ///
        /// # Arguments
        /// * `nums` - Input array
        ///
        /// # Returns
        /// Vec where result[i] = count of smaller elements to the right of nums[i]
        pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
            let n = nums.len();
            let mut ans = vec![0i32; n];
            // (value, original_index)
            let mut pairs: Vec<(i32, usize)> =
                nums.iter().enumerate().map(|(i, &v)| (v, i)).collect();
            let mut tmp = vec![(0i32, 0usize); n];
            merge_sort(&mut pairs, &mut tmp, &mut ans, 0, n);
            ans
        }
    }

    /// Merge sort that counts, for each element, how many smaller elements
    /// originally appeared to its right.
    fn merge_sort(
        arr: &mut [(i32, usize)],
        tmp: &mut [(i32, usize)],
        ans: &mut [i32],
        lo: usize,
        hi: usize,
    ) {
        if hi - lo <= 1 {
            return;
        }
        let mid = lo + (hi - lo) / 2;
        merge_sort(arr, tmp, ans, lo, mid);
        merge_sort(arr, tmp, ans, mid, hi);

        // Merge [lo, mid) and [mid, hi)
        let (mut i, mut j, mut k) = (lo, mid, lo);
        while i < mid && j < hi {
            if arr[i].0 <= arr[j].0 {
                // arr[i] is not greater than arr[j], so elements [mid, j)
                // that have already been placed are smaller than arr[i]
                ans[arr[i].1] += (j - mid) as i32;
                tmp[k] = arr[i];
                i += 1;
            } else {
                tmp[k] = arr[j];
                j += 1;
            }
            k += 1;
        }
        // Remaining left half elements: all of [mid, hi) were smaller
        while i < mid {
            ans[arr[i].1] += (j - mid) as i32;
            tmp[k] = arr[i];
            i += 1;
            k += 1;
        }
        while j < hi {
            tmp[k] = arr[j];
            j += 1;
            k += 1;
        }
        arr[lo..hi].copy_from_slice(&tmp[lo..hi]);
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(Vec<i32>) -> Vec<i32>) {
        // Example 1
        assert_eq!(solve(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);

        // Example 2
        assert_eq!(solve(vec![-1]), vec![0]);

        // Example 3
        assert_eq!(solve(vec![-1, -1]), vec![0, 0]);

        // Already sorted ascending
        assert_eq!(solve(vec![1, 2, 3, 4]), vec![0, 0, 0, 0]);

        // Sorted descending
        assert_eq!(solve(vec![4, 3, 2, 1]), vec![3, 2, 1, 0]);

        // All same values
        assert_eq!(solve(vec![5, 5, 5]), vec![0, 0, 0]);

        // Two elements
        assert_eq!(solve(vec![2, 1]), vec![1, 0]);

        // Negative values
        assert_eq!(solve(vec![-3, -1, -2]), vec![0, 1, 0]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::count_smaller);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::count_smaller);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let nums = vec![5, 2, 6, 1];
    println!("Input: nums = {:?}", nums);
    println!(
        "Output (my): {:?}",
        my_solution::Solution::count_smaller(nums.clone())
    );
    println!(
        "Output (ai): {:?}",
        ai_solution::Solution::count_smaller(nums.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::count_smaller(black_box(
            nums.clone(),
        )));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::count_smaller(black_box(
            nums.clone(),
        )));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
