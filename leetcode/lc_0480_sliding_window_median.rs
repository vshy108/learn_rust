// LeetCode 480: Sliding Window Median [Hard]
// https://leetcode.com/problems/sliding-window-median/
//
// Given an integer array nums and an integer k, return the median array
// for each sliding window of size k.
//
// The median is the middle value in sorted order. If even size, it's the
// average of the two middle values.
//
// Examples:
//   nums=[1,3,-1,-3,5,3,6,7], k=3 -> [1.0,-1.0,-1.0,3.0,5.0,6.0]
//   nums=[1,2,3,4,2,3,1,4,2], k=3 -> [2.0,3.0,3.0,3.0,2.0,3.0,2.0]
//
// Constraints:
//   1 <= k <= nums.length <= 10^5
//   -2^31 <= nums[i] <= 2^31 - 1
//
// Run:  cargo run --example lc_0480_sliding_window_median
// Test: cargo test --example lc_0480_sliding_window_median
// Bench: cargo test --example lc_0480_sliding_window_median bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 246ns (sorted Vec, partition_point insert/remove)
//   AI solution: avg 178ns (sorted Vec, binary_search remove)
//   Ratio: 1.38x (AI 1.4x faster)

/// My solution module — sorted window with binary search insert/remove.
///
/// Maintain a sorted Vec for the current window. Use binary search
/// for insertion and removal.
///
/// Time:  O(n * k) — each insert/remove is O(k) due to shifting
/// Space: O(k)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
            let k = k as usize;
            let mut window: Vec<i32> = Vec::with_capacity(k);
            let mut result = Vec::with_capacity(nums.len() - k + 1);

            for i in 0..nums.len() {
                // Insert nums[i] into sorted position
                let pos = window.partition_point(|&x| x < nums[i]);
                window.insert(pos, nums[i]);

                // Remove the element leaving the window
                if window.len() > k {
                    let old = nums[i - k];
                    let rm_pos = window.partition_point(|&x| x < old);
                    // Find exact match (partition_point gives first >= old)
                    window.remove(rm_pos);
                }

                // Record median when window is full
                if window.len() == k {
                    if k % 2 == 1 {
                        result.push(window[k / 2] as f64);
                    } else {
                        result.push((window[k / 2 - 1] as f64 + window[k / 2] as f64) / 2.0);
                    }
                }
            }
            result
        }
    }
}

/// AI solution module — sorted window with BTreeMap-like approach.
///
/// Same sorted-Vec approach but structured differently to emphasize
/// the binary search operations.
///
/// Time:  O(n * k)
/// Space: O(k)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
            let k = k as usize;
            // Build initial sorted window
            let mut sorted: Vec<i32> = nums[..k].to_vec();
            sorted.sort_unstable();

            let mut result = Vec::with_capacity(nums.len() - k + 1);
            result.push(Self::median(&sorted, k));

            for i in k..nums.len() {
                // Remove outgoing element
                let old = nums[i - k];
                let rm = sorted.binary_search(&old).unwrap();
                sorted.remove(rm);

                // Insert incoming element
                let ins = sorted.partition_point(|&x| x < nums[i]);
                sorted.insert(ins, nums[i]);

                result.push(Self::median(&sorted, k));
            }
            result
        }

        fn median(sorted: &[i32], k: usize) -> f64 {
            if k % 2 == 1 {
                sorted[k / 2] as f64
            } else {
                (sorted[k / 2 - 1] as f64 + sorted[k / 2] as f64) / 2.0
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        fn approx_eq(a: &[f64], b: &[f64]) -> bool {
            a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5)
        }

        #[test]
        fn test_example_1() {
            let result = Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
            assert!(approx_eq(&result, &[1.0, -1.0, -1.0, 3.0, 5.0, 6.0]));
        }
        #[test]
        fn test_example_2() {
            let result = Solution::median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3);
            assert!(approx_eq(&result, &[2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0]));
        }
        #[test]
        fn test_k_equals_1() {
            let result = Solution::median_sliding_window(vec![5, 2, 3], 1);
            assert!(approx_eq(&result, &[5.0, 2.0, 3.0]));
        }
        #[test]
        fn test_even_k() {
            let result = Solution::median_sliding_window(vec![1, 2, 3, 4], 2);
            assert!(approx_eq(&result, &[1.5, 2.5, 3.5]));
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>, i32) -> Vec<f64>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(vec![1, 3, -1, -3, 5, 3, 6, 7]),
            std::hint::black_box(3),
        ));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::median_sliding_window);
    bench_solution("AI solution", ai_solution::Solution::median_sliding_window);
}

fn main() {
    println!(
        "{:?}",
        ai_solution::Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
}
