// LeetCode 189: Rotate Array [Medium]
// https://leetcode.com/problems/rotate-array/
//
// Given an integer array nums, rotate the array to the right by k steps,
// where k is non-negative.
//
// Examples:
//   nums = [1,2,3,4,5,6,7], k = 3 -> [5,6,7,1,2,3,4]
//   nums = [-1,-100,3,99],  k = 2 -> [3,99,-1,-100]
//
// Constraints:
//   - 1 <= nums.length <= 10^5
//   - -2^31 <= nums[i] <= 2^31 - 1
//   - 0 <= k <= 10^5
//
// Follow-up: Try to come up with as many solutions as you can. Could you
// do it in-place with O(1) extra space?
//
// Run:  cargo run --example lc_0189_rotate_array
// Test: cargo test --example lc_0189_rotate_array
// Bench: cargo test --example lc_0189_rotate_array bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Rotate an array to the right by k steps in-place.
        ///
        /// # Arguments
        ///
        /// * `nums` - Mutable reference to the array to rotate
        /// * `k` - Number of steps to rotate right
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            // n = nums.length
            // k %= n
            // helper reverse(arr, start_idx, end_idx):
            //   while start_idx < end_idx
            //     swap arr[start_idx], arr[end_idx]
            //     start_idx += 1; end_idx -= 1
            //   end
            // reverse(nums, 0, n - 1)
            // reverse(nums, 0, k - 1)
            // reverse(nums, k, n - 1)
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Rotate an array to the right by k steps in-place.
        ///
        /// # Arguments
        ///
        /// * `nums` - Mutable reference to the array to rotate
        /// * `k` - Number of steps to rotate right
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]),
        (vec![-1, -100, 3, 99], 2, vec![3, 99, -1, -100]),
    ];

    println!("--- My Solution ---");
    for (nums, k, expected) in &test_cases {
        let mut result = nums.clone();
        my_solution::Solution::rotate(&mut result, *k);
        println!("  {nums:?}, k={k} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (nums, k, expected) in &test_cases {
        let mut result = nums.clone();
        ai_solution::Solution::rotate(&mut result, *k);
        println!("  {nums:?}, k={k} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_rotate {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic_rotate() {
                    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
                    Solution::rotate(&mut nums, 3);
                    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
                }

                #[test]
                fn test_negative_values() {
                    let mut nums = vec![-1, -100, 3, 99];
                    Solution::rotate(&mut nums, 2);
                    assert_eq!(nums, vec![3, 99, -1, -100]);
                }

                #[test]
                fn test_k_zero() {
                    let mut nums = vec![1, 2, 3];
                    Solution::rotate(&mut nums, 0);
                    assert_eq!(nums, vec![1, 2, 3]);
                }

                #[test]
                fn test_k_equals_length() {
                    let mut nums = vec![1, 2, 3];
                    Solution::rotate(&mut nums, 3);
                    assert_eq!(nums, vec![1, 2, 3]);
                }

                #[test]
                fn test_k_greater_than_length() {
                    let mut nums = vec![1, 2, 3];
                    Solution::rotate(&mut nums, 4);
                    assert_eq!(nums, vec![3, 1, 2]);
                }

                #[test]
                fn test_single_element() {
                    let mut nums = vec![1];
                    Solution::rotate(&mut nums, 5);
                    assert_eq!(nums, vec![1]);
                }

                #[test]
                fn test_two_elements() {
                    let mut nums = vec![1, 2];
                    Solution::rotate(&mut nums, 1);
                    assert_eq!(nums, vec![2, 1]);
                }
            }
        };
    }

    test_rotate!(my_tests, crate::my_solution::Solution);
    test_rotate!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let base = vec![1, 2, 3, 4, 5, 6, 7];

        let start = Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            crate::my_solution::Solution::rotate(
                std::hint::black_box(&mut nums),
                std::hint::black_box(3),
            );
            std::hint::black_box(&nums);
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut nums = base.clone();
            crate::ai_solution::Solution::rotate(
                std::hint::black_box(&mut nums),
                std::hint::black_box(3),
            );
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
