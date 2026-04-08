// LeetCode 239: Sliding Window Maximum [Hard]
// https://leetcode.com/problems/sliding-window-maximum/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.446us
//   AI solution avg: 0.451us
//   Ratio: ~tied
//
// You are given an array of integers nums, and a sliding window of size k
// which moves from the very left to the very right. You can only see the
// k numbers in the window. Each time the window moves one position to the
// right. Return the max sliding window.
//
// Constraints: 1 <= nums.length <= 10^5, -10^4 <= nums[i] <= 10^4,
// 1 <= k <= nums.length
//
// Run:  cargo run --example lc_0239_sliding_window_maximum
// Test: cargo test --example lc_0239_sliding_window_maximum
// Bench: cargo test --example lc_0239_sliding_window_maximum bench -- --nocapture

/// My solution — brute force scan each window.
///
/// For each window position, scan all k elements to find the max.
/// Simple but suboptimal for large k.
///
/// Time: O(n * k).
/// Space: O(1) extra (output excluded).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the maximum value in each sliding window of size k.
        pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
            let k = k as usize;
            let n = nums.len();
            let mut result = Vec::with_capacity(n - k + 1);

            for i in 0..=n - k {
                // Find max in nums[i..i+k]
                let max_val = nums[i..i + k]
                    .iter()
                    .copied()
                    .max()
                    .expect("window is non-empty");
                result.push(max_val);
            }

            result
        }
    }
}

/// AI solution — monotonic deque.
///
/// Maintain a deque of indices in decreasing order of their values.
/// For each new element:
///   1. Remove indices that have fallen out of the window (front).
///   2. Remove indices from the back whose values are <= the new element
///      (they can never be a future window max).
///   3. Push the new index to the back.
/// The front of the deque is always the index of the current window max.
///
/// Time: O(n) — each index is pushed and popped at most once.
/// Space: O(k) — deque holds at most k indices.
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Return the maximum value in each sliding window of size k.
        pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
            let k = k as usize;
            let n = nums.len();
            let mut result = Vec::with_capacity(n - k + 1);
            // Deque stores indices; values at those indices are in
            // decreasing order (front = largest)
            let mut deque: VecDeque<usize> = VecDeque::new();

            for i in 0..n {
                // Remove indices that have slid out of the window
                while let Some(&front) = deque.front() {
                    if front + k <= i {
                        deque.pop_front();
                    } else {
                        break;
                    }
                }

                // Remove indices from the back that are smaller than
                // the current element — they can never be a window max
                while let Some(&back) = deque.back() {
                    if nums[back] <= nums[i] {
                        deque.pop_back();
                    } else {
                        break;
                    }
                }

                deque.push_back(i);

                // Once the first full window is formed, record the max
                if i >= k - 1 {
                    result.push(nums[*deque.front().expect("deque non-empty")]);
                }
            }

            result
        }
    }
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let my = my_solution::Solution::max_sliding_window(nums.clone(), k);
    let ai = ai_solution::Solution::max_sliding_window(nums, k);
    println!("My: {my:?}");
    println!("AI: {ai:?}");
    println!("Expected: [3, 3, 5, 5, 6, 7]");
}

#[cfg(test)]
mod tests {
    macro_rules! test_sliding_max {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3,),
                        vec![3, 3, 5, 5, 6, 7],
                    );
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(<$solution>::max_sliding_window(vec![1], 1), vec![1],);
                }

                #[test]
                fn test_k_equals_n() {
                    assert_eq!(<$solution>::max_sliding_window(vec![1, 3, 2], 3), vec![3],);
                }

                #[test]
                fn test_k_one() {
                    assert_eq!(
                        <$solution>::max_sliding_window(vec![4, 2, 5, 1], 1),
                        vec![4, 2, 5, 1],
                    );
                }

                #[test]
                fn test_decreasing() {
                    assert_eq!(
                        <$solution>::max_sliding_window(vec![5, 4, 3, 2, 1], 3),
                        vec![5, 4, 3],
                    );
                }

                #[test]
                fn test_increasing() {
                    assert_eq!(
                        <$solution>::max_sliding_window(vec![1, 2, 3, 4, 5], 3),
                        vec![3, 4, 5],
                    );
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        <$solution>::max_sliding_window(vec![7, 7, 7, 7], 2),
                        vec![7, 7, 7],
                    );
                }
            }
        };
    }

    test_sliding_max!(my, crate::my_solution::Solution);
    test_sliding_max!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..100).map(|i| (i * 37 + 13) % 97).collect();
        let k = 10;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::max_sliding_window(
                std::hint::black_box(nums.clone()),
                k,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::max_sliding_window(
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
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
