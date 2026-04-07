// @lc app=leetcode id=330 lang=rust
// #[Hard] - Patching Array
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 0.005us
// ai_solution  | avg: 0.005us
// Performance: My 1.00x faster

/// # 330. Patching Array
///
/// Given a sorted integer array `nums` and an integer `n`, add/patch
/// elements so that any number in `[1, n]` can be formed by the sum of
/// some elements. Return the minimum number of patches required.
///
/// Constraints:
/// - 1 <= nums.length <= 1000
/// - 1 <= nums[i] <= 10^4
/// - nums is sorted in ascending order
/// - 1 <= n <= 2^31 - 1

/// ## My Solution — Greedy with i64 reach
///
/// Track `reach`: the smallest number we cannot yet form (starts at 1).
/// If `nums[i] <= reach`, extend reach to `reach + nums[i]`.
/// Otherwise, patch by adding `reach` itself, doubling our coverage.
///
/// - Time: O(m + log n) where m = nums.len()
/// - Space: O(1)
mod my_solution {
    pub fn min_patches(nums: &[i32], n: i32) -> i32 {
        // `reach` = smallest number we can't form yet; cover [1, reach)
        let mut reach: i64 = 1;
        let target = n as i64;
        let mut patches = 0;
        let mut i = 0;
        while reach <= target {
            if i < nums.len() && (nums[i] as i64) <= reach {
                // nums[i] extends our reach without patching
                reach += nums[i] as i64;
                i += 1;
            } else {
                // Patch: add `reach` itself to double coverage
                patches += 1;
                reach <<= 1; // reach *= 2
            }
        }
        patches
    }
}

/// ## AI Solution — Same greedy, iterative with explicit loop
///
/// Identical algorithm expressed with a for-loop over nums and a post
/// loop for remaining patches.
///
/// - Time: O(m + log n)
/// - Space: O(1)
mod ai_solution {
    pub fn min_patches(nums: &[i32], n: i32) -> i32 {
        let target = n as i64;
        let mut reach: i64 = 1;
        let mut patches = 0i32;
        let mut idx = 0;
        // Process until we can represent all of [1..=n]
        while reach <= target {
            if idx < nums.len() && (nums[idx] as i64) <= reach {
                reach += nums[idx] as i64;
                idx += 1;
            } else {
                // Must patch: add `reach` to our set
                patches += 1;
                reach += reach; // equivalent to reach <<= 1
            }
        }
        patches
    }
}

fn run_tests(f: fn(&[i32], i32) -> i32) {
    // Example 1: patch 2 => covers [1,6]
    assert_eq!(f(&[1, 3], 6), 1);
    // Example 2: patch [2,4] => covers [1,20]
    assert_eq!(f(&[1, 5, 10], 20), 2);
    // Example 3: already covers [1,5]
    assert_eq!(f(&[1, 2, 2], 5), 0);
    // Edge: empty-like scenario — need to cover [1,1]
    assert_eq!(f(&[2], 1), 1);
    // Large n with small nums
    assert_eq!(f(&[1, 2, 31, 33], 2_147_483_647), 28);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::min_patches);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::min_patches);
    }
}

fn main() {
    let nums = vec![1, 5, 10];
    let n = 20;
    let iterations = 100_000;

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(my_solution::min_patches(
            std::hint::black_box(&nums),
            std::hint::black_box(n),
        ));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::min_patches(
            std::hint::black_box(&nums),
            std::hint::black_box(n),
        ));
    }
    let my_dur = start.elapsed();

    // Warm up
    for _ in 0..1000 {
        std::hint::black_box(ai_solution::min_patches(
            std::hint::black_box(&nums),
            std::hint::black_box(n),
        ));
    }

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::min_patches(
            std::hint::black_box(&nums),
            std::hint::black_box(n),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 330 - Patching Array");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
