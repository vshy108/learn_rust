// LeetCode 31: Next Permutation [Medium]
// https://leetcode.com/problems/next-permutation/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 174ns
//   AI solution avg: 248ns
//   Ratio: My solution is 1.42x faster
//
// Given an array of integers nums, find the next lexicographically greater
// permutation. If no such permutation exists (array is in descending order),
// rearrange to the lowest possible order (ascending).
//
// The replacement must be in-place with only constant extra memory.
//
// Example 1: nums = [1,2,3] -> [1,3,2]
// Example 2: nums = [3,2,1] -> [1,2,3]
// Example 3: nums = [1,1,5] -> [1,5,1]
//
// Constraints:
//   1 <= nums.length <= 100
//   0 <= nums[i] <= 100
//
// Run:  cargo run --example lc_0031_next_permutation
// Test: cargo test --example lc_0031_next_permutation
// Bench: cargo test --example lc_0031_next_permutation bench -- --nocapture

/// My solution — standard textbook algorithm.
///
/// 1. Find the largest index `i` such that nums[i] < nums[i+1] (the "pivot").
///    If none exists, the array is fully descending — reverse it.
/// 2. Find the largest index `j > i` where nums[j] > nums[i].
/// 3. Swap nums[i] and nums[j].
/// 4. Reverse the suffix after index i.
///
/// Time: O(n) — at most two linear scans plus a reverse.
/// Space: O(1) — in-place.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Modify nums in-place to its next permutation.
        ///
        /// # Arguments
        /// * `nums` - mutable slice of integers
        pub fn next_permutation(nums: &mut Vec<i32>) {
            let n = nums.len();
            if n <= 1 {
                return;
            }

            // Step 1: find pivot — rightmost element smaller than its successor.
            let mut i = n - 1;
            while i > 0 && nums[i - 1] >= nums[i] {
                i -= 1;
            }

            if i == 0 {
                // Entire array is non-increasing — wrap around to smallest.
                nums.reverse();
                return;
            }

            // `i - 1` is the pivot index.
            let pivot = i - 1;

            // Step 2: find the rightmost element greater than pivot value.
            let mut j = n - 1;
            while nums[j] <= nums[pivot] {
                j -= 1;
            }

            // Step 3: swap pivot with that element.
            nums.swap(pivot, j);

            // Step 4: reverse the suffix after pivot.
            nums[pivot + 1..].reverse();
        }
    }
}

/// AI-optimized solution — identical algorithm (it's already optimal).
///
/// The textbook next-permutation algorithm is already O(n) time / O(1) space.
/// This variant uses the same steps but with slightly different iteration style
/// using iterators for the scan steps.
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Modify nums in-place to its next permutation.
        ///
        /// # Arguments
        /// * `nums` - mutable slice of integers
        pub fn next_permutation(nums: &mut Vec<i32>) {
            let n = nums.len();
            if n <= 1 {
                return;
            }

            // Step 1: find pivot using rposition on windows(2).
            // `rposition` scans from the right for the first pair where a < b.
            let pivot = nums.windows(2).rposition(|w| w[0] < w[1]);

            match pivot {
                None => {
                    // No ascending pair — array is fully descending.
                    nums.reverse();
                }
                Some(p) => {
                    // Step 2: find rightmost element > nums[p].
                    let swap_idx = nums.iter().rposition(|&x| x > nums[p]).unwrap(); // guaranteed to exist since nums[p+1] > nums[p]

                    // Step 3: swap.
                    nums.swap(p, swap_idx);

                    // Step 4: reverse suffix after pivot.
                    nums[p + 1..].reverse();
                }
            }
        }
    }
}

fn main() {
    let cases: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1], vec![1, 1, 5], vec![1, 3, 2]];
    for nums in cases {
        let mut my = nums.clone();
        let mut ai = nums.clone();
        my_solution::Solution::next_permutation(&mut my);
        ai_solution::Solution::next_permutation(&mut ai);
        println!("{nums:?} -> my:{my:?}, ai:{ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_next_permutation {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_ascending() {
                    let mut nums = vec![1, 2, 3];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![1, 3, 2]);
                }

                #[test]
                fn test_descending_wraps() {
                    let mut nums = vec![3, 2, 1];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![1, 2, 3]);
                }

                #[test]
                fn test_duplicates() {
                    let mut nums = vec![1, 1, 5];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![1, 5, 1]);
                }

                #[test]
                fn test_mid_sequence() {
                    let mut nums = vec![1, 3, 2];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![2, 1, 3]);
                }

                #[test]
                fn test_single_element() {
                    let mut nums = vec![1];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![1]);
                }

                #[test]
                fn test_two_elements() {
                    let mut nums = vec![1, 2];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![2, 1]);
                }

                #[test]
                fn test_two_elements_desc() {
                    let mut nums = vec![2, 1];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![1, 2]);
                }

                #[test]
                fn test_longer() {
                    let mut nums = vec![1, 2, 3, 4];
                    Solution::next_permutation(&mut nums);
                    assert_eq!(nums, vec![1, 2, 4, 3]);
                }

                #[test]
                fn test_all_same() {
                    let mut nums = vec![2, 2, 2];
                    Solution::next_permutation(&mut nums);
                    // No larger permutation — wraps to sorted (same).
                    assert_eq!(nums, vec![2, 2, 2]);
                }
            }
        };
    }

    test_next_permutation!(my_tests, crate::my_solution::Solution);
    test_next_permutation!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let base: Vec<i32> = (0..100).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            crate::my_solution::Solution::next_permutation(std::hint::black_box(&mut nums));
            std::hint::black_box(&nums);
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            crate::ai_solution::Solution::next_permutation(std::hint::black_box(&mut nums));
            std::hint::black_box(&nums);
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
