// LeetCode 42: Trapping Rain Water [Hard]
// https://leetcode.com/problems/trapping-rain-water/
//
// Given n non-negative integers representing an elevation map where the
// width of each bar is 1, compute how much water it can trap after raining.
//
// Examples:
//   height = [0,1,0,2,1,0,1,3,2,1,2,1] -> 6
//   height = [4,2,0,3,2,5]               -> 9
//
// Constraints:
//   - n == height.length
//   - 1 <= n <= 2 * 10^4
//   - 0 <= height[i] <= 10^5
//
// Run:  cargo run --example lc_0042_trapping_rain_water
// Test: cargo test --example lc_0042_trapping_rain_water
// Bench: cargo test --example lc_0042_trapping_rain_water bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-03-27) ---
//   My solution: 68.88ms  (avg 688ns)
//   AI solution: 249.25ms (avg 2.492us)
//   -> My solution is 3.62x faster
//   Reason: Two-pointer does a single pass with O(1) space and no allocations,
//           while prefix/suffix max allocates two Vec<i32> of length n (3 passes
//           + allocation overhead dominates on small inputs).
//
// --- Rust concepts used ---
//   - Two-pointer convergence: left/right pointers meeting in the middle
//   - Prefix/suffix max arrays: precompute running max from both directions
//   - Vec::with_capacity(n): pre-allocate when size is known
//   - (0..n).rev(): reverse range iterator for right-to-left traversal
//   - Iterator chain: (0..n).map(...).sum() for functional accumulation
//   - i32::max() / i32::min(): Ord trait methods for comparisons

/// My solution module.
///
/// Time:  O(n) -- single pass with two pointers converging from both ends
/// Space: O(1) -- only a few scalar variables
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute how much water can be trapped after raining.
        ///
        /// # Arguments
        ///
        /// * `height` - A vector of non-negative integers representing
        ///   the elevation map
        ///
        /// # Returns
        ///
        /// The total amount of trapped water
        pub fn trap(height: Vec<i32>) -> i32 {
            if height.is_empty() {
                return 0;
            };

            let mut left = 0;
            let mut right = height.len() - 1;
            let mut left_max = 0;
            let mut right_max = 0;
            let mut water = 0;

            while left < right {
                if height[left] < height[right] {
                    if height[left] >= left_max {
                        left_max = height[left]
                    } else {
                        water += left_max - height[left]
                    }
                    left += 1
                } else {
                    if height[right] >= right_max {
                        right_max = height[right];
                    } else {
                        water += right_max - height[right];
                    }
                    right -= 1
                }
            }

            water
        }
    }
}

/// AI solution module.
///
/// Uses prefix/suffix max arrays: precompute the max height to the left
/// and right of each position, then water at each index is
/// `min(left_max[i], right_max[i]) - height[i]`. Three passes total.
///
/// Time:  O(n) -- three linear passes (left max, right max, accumulate)
/// Space: O(n) -- two auxiliary vectors of length n
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compute how much water can be trapped after raining.
        ///
        /// # Arguments
        ///
        /// * `height` - A vector of non-negative integers representing
        ///   the elevation map
        ///
        /// # Returns
        ///
        /// The total amount of trapped water
        pub fn trap(height: Vec<i32>) -> i32 {
            let n = height.len();
            if n == 0 {
                return 0;
            }

            // Build prefix max: left_max[i] = max height in height[0..=i]
            let mut left_max = Vec::with_capacity(n);
            let mut max_so_far = 0;
            for &h in &height {
                max_so_far = max_so_far.max(h);
                left_max.push(max_so_far);
            }

            // Build suffix max: right_max[i] = max height in height[i..n]
            let mut right_max = vec![0; n];
            max_so_far = 0;
            for i in (0..n).rev() {
                max_so_far = max_so_far.max(height[i]);
                right_max[i] = max_so_far;
            }

            // Water at each position = min(left_max, right_max) - height
            (0..n)
                .map(|i| left_max[i].min(right_max[i]) - height[i])
                .sum()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
        (vec![4, 2, 0, 3, 2, 5], 9),
    ];

    println!("--- My Solution ---");
    for (height, expected) in &test_cases {
        let result = my_solution::Solution::trap(height.clone());
        println!("  height={height:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (height, expected) in &test_cases {
        let result = ai_solution::Solution::trap(height.clone());
        println!("  height={height:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_trap {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
                }

                #[test]
                fn test_no_trap_ascending() {
                    assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5]), 0);
                }

                #[test]
                fn test_no_trap_descending() {
                    assert_eq!(Solution::trap(vec![5, 4, 3, 2, 1]), 0);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(Solution::trap(vec![5]), 0);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::trap(vec![1, 2]), 0);
                }

                #[test]
                fn test_flat() {
                    assert_eq!(Solution::trap(vec![3, 3, 3, 3]), 0);
                }
            }
        };
    }

    test_trap!(my_tests, crate::my_solution::Solution);
    test_trap!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
            vec![4, 2, 0, 3, 2, 5],
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 1, 2, 1, 4, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for height in &inputs {
                std::hint::black_box(crate::my_solution::Solution::trap(std::hint::black_box(
                    height.clone(),
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for height in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::trap(std::hint::black_box(
                    height.clone(),
                )));
            }
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
