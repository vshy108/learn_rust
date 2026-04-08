// LeetCode 410: Split Array Largest Sum [Hard]
// https://leetcode.com/problems/split-array-largest-sum/
// Benchmark: 10000 iterations
//   My solution (binary search + greedy): avg 730ns
//   AI solution (DP O(n^2*k)):            avg 34.84us
//   -> My solution is 47.72x faster
//
// Given an integer array `nums` and an integer `k`, split `nums` into
// `k` non-empty subarrays such that the largest sum of any subarray
// is minimized. Return the minimized largest sum.
//
// Examples:
//   nums = [7,2,5,10,8], k = 2  -> 18  (split: [7,2,5] and [10,8])
//   nums = [1,2,3,4,5], k = 2   -> 9   (split: [1,2,3] and [4,5])
//
// Constraints:
//   1 <= nums.length <= 1000
//   0 <= nums[i] <= 10^6
//   1 <= k <= min(50, nums.length)
//
// Run:  cargo run --example lc_0410_split_array_largest_sum
// Test: cargo test --example lc_0410_split_array_largest_sum
// Bench: cargo test --example lc_0410_split_array_largest_sum bench -- --nocapture

/// My solution module — binary search on the answer.
///
/// The answer lies in [max(nums), sum(nums)]. Binary search for the
/// smallest `mid` such that we can split the array into <= k subarrays
/// each with sum <= mid. The greedy check scans left to right, starting
/// a new subarray whenever the running sum would exceed mid.
///
/// Time:  O(n * log(sum - max))
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimized largest sum of the split.
        ///
        /// # Arguments
        ///
        /// * `nums` - Non-negative integer array
        /// * `k` - Number of subarrays to split into
        ///
        /// # Returns
        ///
        /// The minimized largest subarray sum
        pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
            // Binary search bounds
            let mut lo: i64 = *nums.iter().max().expect("non-empty") as i64;
            let mut hi: i64 = nums.iter().map(|&x| x as i64).sum();

            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if Self::can_split(&nums, k, mid) {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }

            lo as i32
        }

        /// Check if we can split nums into at most k subarrays
        /// with each subarray sum <= max_sum.
        fn can_split(nums: &[i32], k: i32, max_sum: i64) -> bool {
            let mut parts = 1;
            let mut current_sum: i64 = 0;

            for &num in nums {
                current_sum += num as i64;
                if current_sum > max_sum {
                    // Start a new subarray with this element
                    parts += 1;
                    current_sum = num as i64;
                    if parts > k {
                        return false;
                    }
                }
            }

            true
        }
    }
}

/// AI solution module — DP approach.
///
/// dp[i][j] = minimum largest sum when splitting nums[0..i] into j parts.
/// Use prefix sums for O(1) range sum queries.
/// dp[i][j] = min over all valid split points p of max(dp[p][j-1], sum(p..i)).
///
/// Time:  O(n^2 * k)
/// Space: O(n * k)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimized largest sum of the split.
        ///
        /// # Arguments
        ///
        /// * `nums` - Non-negative integer array
        /// * `k` - Number of subarrays to split into
        ///
        /// # Returns
        ///
        /// The minimized largest subarray sum
        pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
            let n = nums.len();
            let k = k as usize;

            // Prefix sums for O(1) range sum
            let mut prefix = vec![0_i64; n + 1];
            for i in 0..n {
                prefix[i + 1] = prefix[i] + nums[i] as i64;
            }
            // Helper to get sum of nums[l..r] (0-indexed, exclusive end)
            let range_sum = |l: usize, r: usize| -> i64 {
                prefix[r] - prefix[l]
            };

            // dp[i][j] = min largest-sum splitting first i elements into j parts
            // i in 0..=n, j in 0..=k
            let mut dp = vec![vec![i64::MAX; k + 1]; n + 1];
            dp[0][0] = 0;

            for i in 1..=n {
                for j in 1..=k.min(i) {
                    // Try splitting: last subarray is nums[p..i]
                    for p in (j - 1)..i {
                        if dp[p][j - 1] == i64::MAX {
                            continue;
                        }
                        let candidate = dp[p][j - 1].max(range_sum(p, i));
                        if candidate < dp[i][j] {
                            dp[i][j] = candidate;
                        }
                    }
                }
            }

            dp[n][k] as i32
        }
    }
}

fn main() {
    let cases = [
        (vec![7, 2, 5, 10, 8], 2, 18),
        (vec![1, 2, 3, 4, 5], 2, 9),
    ];

    println!("--- My Solution ---");
    for (nums, k, expected) in &cases {
        let result = my_solution::Solution::split_array(nums.clone(), *k);
        println!("  nums={nums:?}, k={k} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, k, expected) in &cases {
        let result = ai_solution::Solution::split_array(nums.clone(), *k);
        println!("  nums={nums:?}, k={k} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_split {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::split_array(vec![7, 2, 5, 10, 8], 2),
                        18,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::split_array(vec![1, 2, 3, 4, 5], 2),
                        9,
                    );
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(
                        Solution::split_array(vec![10], 1),
                        10,
                    );
                }

                #[test]
                fn test_k_equals_n() {
                    // Each element is its own subarray -> answer is max element
                    assert_eq!(
                        Solution::split_array(vec![1, 4, 2], 3),
                        4,
                    );
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        Solution::split_array(vec![5, 5, 5, 5], 2),
                        10,
                    );
                }

                #[test]
                fn test_large_single_split() {
                    // k=1 -> answer is total sum
                    assert_eq!(
                        Solution::split_array(vec![1, 2, 3, 4, 5], 1),
                        15,
                    );
                }
            }
        };
    }

    test_split!(my_tests, crate::my_solution::Solution);
    test_split!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;

        // nums of length 100, k=5
        let nums: Vec<i32> = (1..=100).collect();
        let k = 5;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::split_array(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(k),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::split_array(
                    std::hint::black_box(nums.clone()),
                    std::hint::black_box(k),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64 / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64 / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
