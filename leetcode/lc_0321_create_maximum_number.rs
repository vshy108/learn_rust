// LeetCode 321. Create Maximum Number
// https://leetcode.com/problems/create-maximum-number/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.877us
//   ai_solution  avg: 0.900us
//   Ratio: my_solution 1.03x faster

/// Given nums1 and nums2 of lengths m and n, and integer k, create the maximum
/// number of length k from digits of both arrays, preserving relative order.
///
/// Example 1: nums1=[3,4,6,5], nums2=[9,1,2,5,8,3], k=5 -> [9,8,6,5,3]
/// Example 2: nums1=[6,7], nums2=[6,0,4], k=5 -> [6,7,6,0,4]
/// Example 3: nums1=[3,9], nums2=[8,9], k=3 -> [9,8,9]
///
/// Constraints: 1<=m,n<=500, 0<=nums[i]<=9, 1<=k<=m+n

/// # my_solution: Monotonic stack + merge (iterative compare)
/// Time: O(k * (m+n)) for each split, O(k) splits => O(k^2 * (m+n))
/// Space: O(k)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Create maximum number of length k from two arrays.
        ///
        /// Try all ways to split k into x from nums1 and k-x from nums2.
        /// For each split, get max subsequence of length x from nums1
        /// and k-x from nums2 using a monotonic stack, then merge them.
        pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
            let (m, n, k) = (nums1.len(), nums2.len(), k as usize);
            let lo = k.saturating_sub(n);
            let hi = k.min(m);
            let mut best = vec![0i32; k];

            for x in lo..=hi {
                let a = max_subseq(&nums1, x);
                let b = max_subseq(&nums2, k - x);
                let merged = merge(&a, &b);
                if merged > best {
                    best = merged;
                }
            }
            best
        }
    }

    /// Get the maximum subsequence of length k from nums using a monotonic stack.
    fn max_subseq(nums: &[i32], k: usize) -> Vec<i32> {
        let n = nums.len();
        let mut stk = Vec::with_capacity(k);
        let mut remain = n - k; // how many we can still drop
        for &x in nums {
            while let Some(&top) = stk.last() {
                if top < x && remain > 0 {
                    stk.pop();
                    remain -= 1;
                } else {
                    break;
                }
            }
            if stk.len() < k {
                stk.push(x);
            } else {
                remain -= 1;
            }
        }
        stk
    }

    /// Merge two sorted-desc subsequences into the lexicographically largest result.
    fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut i = 0;
        let mut j = 0;
        let mut result = Vec::with_capacity(a.len() + b.len());
        while i < a.len() && j < b.len() {
            // Compare remaining suffixes lexicographically
            // clippy::needless_borrow: &a[i..] already yields &[i32], so
            // writing &a[i..] >= &b[j..] adds an unnecessary & on both sides.
            // Slice comparison works directly via PartialOrd for [T].
            if a[i..] >= b[j..] {
                result.push(a[i]);
                i += 1;
            } else {
                result.push(b[j]);
                j += 1;
            }
        }
        result.extend_from_slice(&a[i..]);
        result.extend_from_slice(&b[j..]);
        result
    }
}

/// # ai_solution: Same approach, recursive compare
/// Time: O(k^2 * (m+n)) same asymptotic
/// Space: O(k)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Same split-merge approach but uses recursive comparison for merge.
        pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
            let (m, n, k) = (nums1.len(), nums2.len(), k as usize);
            // clippy::manual_saturating_arithmetic (same as my_solution)
            let lo = k.saturating_sub(n);
            let hi = k.min(m);
            let mut best = vec![0i32; k];

            for x in lo..=hi {
                let a = max_subseq(&nums1, x);
                let b = max_subseq(&nums2, k - x);
                let merged = merge_recursive(&a, &b);
                if compare(&merged, 0, &best, 0) {
                    best = merged;
                }
            }
            best
        }
    }

    fn max_subseq(nums: &[i32], k: usize) -> Vec<i32> {
        let n = nums.len();
        let mut stk = vec![0i32; k];
        let mut top: i32 = -1;
        let mut remain = n - k;
        for &x in nums {
            while top >= 0 && stk[top as usize] < x && remain > 0 {
                top -= 1;
                remain -= 1;
            }
            if (top + 1) < k as i32 {
                top += 1;
                stk[top as usize] = x;
            } else {
                remain -= 1;
            }
        }
        stk
    }

    /// Recursively compare two sequences lexicographically.
    fn compare(a: &[i32], i: usize, b: &[i32], j: usize) -> bool {
        if i >= a.len() {
            return false;
        }
        if j >= b.len() {
            return true;
        }
        if a[i] != b[j] {
            return a[i] > b[j];
        }
        compare(a, i + 1, b, j + 1)
    }

    fn merge_recursive(a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut i = 0;
        let mut j = 0;
        let mut result = Vec::with_capacity(a.len() + b.len());
        while i < a.len() && j < b.len() {
            if compare(a, i, b, j) {
                result.push(a[i]);
                i += 1;
            } else {
                result.push(b[j]);
                j += 1;
            }
        }
        result.extend_from_slice(&a[i..]);
        result.extend_from_slice(&b[j..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(solve: fn(Vec<i32>, Vec<i32>, i32) -> Vec<i32>) {
        assert_eq!(
            solve(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
        assert_eq!(solve(vec![6, 7], vec![6, 0, 4], 5), vec![6, 7, 6, 0, 4]);
        assert_eq!(solve(vec![3, 9], vec![8, 9], 3), vec![9, 8, 9]);

        // All from one array
        assert_eq!(solve(vec![9, 5], vec![], 2), vec![9, 5]);
        assert_eq!(solve(vec![], vec![4, 3], 2), vec![4, 3]);

        // k = 1
        assert_eq!(solve(vec![3], vec![7], 1), vec![7]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::max_number);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::max_number);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let nums1 = vec![3, 4, 6, 5];
    let nums2 = vec![9, 1, 2, 5, 8, 3];
    let k = 5;
    println!("Input: nums1={nums1:?}, nums2={nums2:?}, k={k}");
    println!(
        "Output (my): {:?}",
        my_solution::Solution::max_number(nums1.clone(), nums2.clone(), k)
    );
    println!(
        "Output (ai): {:?}",
        ai_solution::Solution::max_number(nums1.clone(), nums2.clone(), k)
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::max_number(
            black_box(nums1.clone()),
            black_box(nums2.clone()),
            black_box(k),
        ));
    }
    let my_dur = start.elapsed();
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::max_number(
            black_box(nums1.clone()),
            black_box(nums2.clone()),
            black_box(k),
        ));
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
