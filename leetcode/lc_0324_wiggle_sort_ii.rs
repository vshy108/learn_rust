// LeetCode 324. Wiggle Sort II
// https://leetcode.com/problems/wiggle-sort-ii/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.084us
//   ai_solution  avg: 2.624us
//   Ratio: my_solution 31.30x faster

/// Reorder nums so that nums[0] < nums[1] > nums[2] < nums[3]...
///
/// Example 1: nums=[1,5,1,1,6,4] -> [1,6,1,5,1,4] or [1,4,1,5,1,6]
/// Example 2: nums=[1,3,2,2,3,1] -> [2,3,1,3,1,2]
///
/// Constraints: 1<=nums.length<=5*10^4, 0<=nums[i]<=5000

/// # my_solution: Sort + reverse interleave
/// Time: O(n log n) for sorting
/// Space: O(n) for sorted copy
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Wiggle sort by sorting and interleaving from midpoint.
        ///
        /// Sort a copy. Place smaller half (reversed) at even indices
        /// and larger half (reversed) at odd indices. Reversing prevents
        /// equal values from being adjacent.
        pub fn wiggle_sort(nums: &mut Vec<i32>) {
            let mut arr = nums.clone();
            arr.sort_unstable();
            let n = arr.len();
            // i starts at the middle, j at the end — both go backwards
            let mut i = (n - 1) / 2;  // last index of lower half
            let mut j = n - 1;        // last index of upper half

            for k in 0..n {
                if k % 2 == 0 {
                    nums[k] = arr[i];
                    // wrapping_sub to handle underflow on last iteration
                    i = i.wrapping_sub(1);
                } else {
                    nums[k] = arr[j];
                    j = j.wrapping_sub(1);
                }
            }
        }
    }
}

/// # ai_solution: Bucket sort (counting sort for values 0..5000)
/// Time: O(n + V) where V = 5001 (value range)
/// Space: O(V)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Wiggle sort using bucket/counting sort.
        ///
        /// Since values are in [0, 5000], use a counting array.
        /// Fill odd indices first with largest values, then even indices,
        /// ensuring wiggle property.
        pub fn wiggle_sort(nums: &mut Vec<i32>) {
            let n = nums.len();
            let mut bucket = [0u32; 5001];
            for &v in nums.iter() {
                bucket[v as usize] += 1;
            }

            let mut j = 5000usize;

            // Fill odd positions first with large values
            let mut k = 1;
            while k < n {
                while bucket[j] == 0 {
                    j -= 1;
                }
                nums[k] = j as i32;
                bucket[j] -= 1;
                k += 2;
            }

            // Fill even positions with remaining (smaller) values
            k = 0;
            while k < n {
                while bucket[j] == 0 {
                    if j == 0 {
                        break;
                    }
                    j -= 1;
                }
                nums[k] = j as i32;
                bucket[j] -= 1;
                k += 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    /// Verify the wiggle property: nums[0]<nums[1]>nums[2]<nums[3]...
    fn is_wiggle(nums: &[i32]) -> bool {
        for i in 1..nums.len() {
            if i % 2 == 1 {
                // odd index: should be > neighbors
                if nums[i] <= nums[i - 1] {
                    return false;
                }
            } else {
                // even index: should be < previous
                if nums[i] >= nums[i - 1] {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_my_solution() {
        let mut v = vec![1, 5, 1, 1, 6, 4];
        MySolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v), "failed: {v:?}");

        let mut v = vec![1, 3, 2, 2, 3, 1];
        MySolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v), "failed: {v:?}");

        let mut v = vec![1];
        MySolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v));

        let mut v = vec![1, 2];
        MySolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v));

        let mut v = vec![4, 5, 5, 6];
        MySolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v), "failed: {v:?}");
    }

    #[test]
    fn test_ai_solution() {
        let mut v = vec![1, 5, 1, 1, 6, 4];
        AiSolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v), "failed: {v:?}");

        let mut v = vec![1, 3, 2, 2, 3, 1];
        AiSolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v), "failed: {v:?}");

        let mut v = vec![1];
        AiSolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v));

        let mut v = vec![1, 2];
        AiSolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v));

        let mut v = vec![4, 5, 5, 6];
        AiSolution::wiggle_sort(&mut v);
        assert!(is_wiggle(&v), "failed: {v:?}");
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let nums = vec![1, 5, 1, 1, 6, 4];
    println!("Input: nums = {:?}", nums);

    let mut my_nums = nums.clone();
    my_solution::Solution::wiggle_sort(&mut my_nums);
    println!("Output (my): {:?}", my_nums);

    let mut ai_nums = nums.clone();
    ai_solution::Solution::wiggle_sort(&mut ai_nums);
    println!("Output (ai): {:?}", ai_nums);

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let mut v = black_box(nums.clone());
        my_solution::Solution::wiggle_sort(&mut v);
        black_box(&v);
    }
    let my_dur = start.elapsed();
    let start = Instant::now();
    for _ in 0..iterations {
        let mut v = black_box(nums.clone());
        ai_solution::Solution::wiggle_sort(&mut v);
        black_box(&v);
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
