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
//
// --- Benchmark (100,000 iterations, 2026-04-04) ---
// My solution: avg 220ns
// AI solution: avg 104ns
// Ratio: AI solution is 2.10x faster
// Reason: stdlib rotate_right uses ptr::rotate (block-swap algorithm) which
//         is cache-friendly and avoids the overhead of three separate reverse
//         passes. The three-reverse approach touches every element 2x; the
//         block-swap moves elements in larger contiguous chunks.
//
// --- Rust concepts used ---
// - usize underflow: `0_usize - 1` panics (no negative unsigned values).
//   Ruby's `-1` wraps to a negative index; Rust requires an early return guard.
// - `mut` params: Rust function params are immutable by default, need `mut`
//   to reassign (unlike Ruby where all locals are reassignable)
// - `.swap(a, b)`: no parallel assignment (Ruby: `a, b = b, a`) in Rust.
//   .swap() uses internal temp to avoid overwrite-before-read.
// - Inner function: `fn reverse(...)` defined inside a method has no access
//   to outer variables (unlike closures). Takes `&mut Vec` explicitly.
// - Slice `.reverse()`: built-in in-place reversal, no manual loop needed.
// - `.rotate_right(k)`: stdlib single-call O(n) rotation using
//   ptr::rotate internally (block-swap algorithm, cache-friendly)

/// My solution module.
///
/// Three-reverse algorithm: reverse all, reverse first k, reverse rest.
/// Classic O(n) in-place rotation with O(1) extra space.
///
/// Time:  O(n) - three passes, each reversing a portion of the array
/// Space: O(1) - in-place swaps, no extra allocation
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
            let n = nums.len();
            let k = k as usize % n;
            // Early return: k == 0 means no rotation needed.
            // Also prevents `k - 1` underflow below since k is usize (unsigned).
            // Ruby: k - 1 when k == 0 gives -1 (negative index wraps).
            // Rust: usize can't be negative — 0_usize - 1 panics with overflow.
            if k == 0 {
                return;
            }
            // Slice .reverse() replaces the manual reverse loop.
            // The manual version (`end_idx -= 1`) risks usize underflow
            // if end_idx reaches 0 inside the loop. Slice .reverse()
            // handles all edge cases internally with safe pointer arithmetic.
            // `..=` is inclusive range: nums[0..=2] reverses indices 0,1,2
            nums[..].reverse(); // reverse all
            nums[..k].reverse(); // reverse first k elements
            nums[k..].reverse(); // reverse remaining elements
            // No return needed: function returns () and mutates nums in-place
        }
    }
}

/// AI solution module.
///
/// Uses slice `.rotate_right(k)` which calls `ptr::rotate` internally —
/// a cache-friendly block-swap algorithm. Single stdlib call, zero manual
/// logic, same O(n) time and O(1) space.
///
/// Time:  O(n) - single pass block-swap rotation
/// Space: O(1) - in-place (small stack buffer for block-swap)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Rotate an array to the right by k steps in-place.
        ///
        /// Delegates to the stdlib `rotate_right` which implements an
        /// optimized block-swap algorithm internally. More cache-friendly
        /// than the three-reverse approach on large arrays.
        ///
        /// # Arguments
        ///
        /// * `nums` - Mutable reference to the array to rotate
        /// * `k` - Number of steps to rotate right
        pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let n = nums.len();
            let k = k as usize % n;
            // stdlib rotate_right: O(n) block-swap, handles k == 0 internally
            nums.rotate_right(k);
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
