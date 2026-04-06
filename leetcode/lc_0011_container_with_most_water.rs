// LeetCode 11: Container With Most Water [Medium]
// https://leetcode.com/problems/container-with-most-water/
//
// Benchmark (100,000 iterations, 2026-03-27):
//   My solution:  avg 638ns
//   AI solution:  avg 506ns
//   AI is 1.26x faster (skip optimization avoids redundant comparisons)
//
// You are given an integer array `height` of length n. There are n vertical
// lines drawn such that the two endpoints of the i-th line are (i, 0) and
// (i, height[i]).
//
// Find two lines that together with the x-axis form a container, such that
// the container contains the most water. Return the maximum amount of water
// a container can store. You may not slant the container.
//
// Examples:
//   height = [1,8,6,2,5,4,8,3,7] -> 49
//   height = [1,1]                -> 1
//
// Constraints:
//   - n == height.length
//   - 2 <= n <= 10^5
//   - 0 <= height[i] <= 10^4
//
// Run:  cargo run --example lc_0011_container_with_most_water
// Test: cargo test --example lc_0011_container_with_most_water
// Bench: cargo test --example lc_0011_container_with_most_water bench -- --nocapture
//
// --- Rust concepts used ---
//   (Previously covered: two-pointer convergence -> lc_0003,
//    i32::min/max method syntax -> lc_0003, `as i32` cast -> lc_0004)
//
// --- Reference: faster cached-locals approach (LeetCode top submission) ---
//   Caches height[start] and height[end] into locals `s_h`, `e_h`:
//
//   let s_h = height[start];
//   let e_h = height[end];
//   let area = (end - start) as i32 * s_h.min(e_h);
//
//   Why faster than skip optimization (AI solution):
//   - Skip optimization adds inner while-loops with extra branches per
//     iteration. On small/medium inputs the skip rarely fires, so the
//     branch overhead costs more than it saves.
//   - Simple loop body = tighter machine code, better CPU pipelining,
//     and more predictable branch patterns.
//   - Caching height[left]/height[right] into locals avoids 2 redundant
//     bounds-checked array accesses per iteration (4 lookups -> 2).
//     LLVM may optimize this away, but explicit caching guarantees it.
//   - Takeaway: for two-pointer problems with small constant-factor
//     differences, a minimal loop body often beats clever skip tricks.

/// My solution module — classic two-pointer approach.
///
/// Start with the widest container (left=0, right=n-1) and shrink
/// inward by always moving the pointer with the shorter height.
/// Moving the taller side can never increase area: the bottleneck
/// is the shorter side, and the width only decreases.
///
/// Time:  O(n) — each pointer moves at most n steps total
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find two lines forming a container with the most water.
        ///
        /// # Arguments
        ///
        /// * `height` - Array of line heights
        ///
        /// # Returns
        ///
        /// Maximum area of water the container can hold
        pub fn max_area(height: Vec<i32>) -> i32 {
            let mut left = 0;
            let mut right = height.len() - 1;
            let mut max_area = 0;

            while left < right {
                // width is `right - left` (usize), cast to i32 for the multiplication
                let width = right - left;
                // area = min(height[left], height[right]) * width
                let current_area = height[left].min(height[right]) * width as i32;
                // .max() returns the larger of the two — equivalent to Ruby's [a, b].max
                max_area = max_area.max(current_area);

                // Always move the shorter side inward:
                // moving the taller side cannot increase the min-height,
                // and the width shrinks, so area can only decrease.
                if height[left] < height[right] {
                    left += 1;
                } else {
                    right -= 1;
                }
            }

            max_area
        }
    }
}

/// AI solution module — two-pointer with skip optimization.
///
/// Same two-pointer idea, but after moving a pointer, skip any heights
/// that are <= the height we just moved away from, since they can never
/// produce a larger area (the width is shrinking AND the min-height is
/// not increasing).
///
/// Time:  O(n) — each pointer moves at most n steps total
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find two lines forming a container with the most water.
        ///
        /// # Arguments
        ///
        /// * `height` - Array of line heights
        ///
        /// # Returns
        ///
        /// Maximum area of water the container can hold
        pub fn max_area(height: Vec<i32>) -> i32 {
            let mut left = 0usize;
            let mut right = height.len() - 1;
            let mut best = 0i32;

            while left < right {
                // Compute area with the shorter side as the limiting height
                let h = height[left].min(height[right]);
                best = best.max(h * (right - left) as i32);

                // Move the shorter side inward, skipping heights that
                // are <= h because they cannot improve the result:
                // the width is shrinking AND the bottleneck height is
                // not increasing, so area can only decrease.
                if height[left] < height[right] {
                    while left < right && height[left] <= h {
                        left += 1;
                    }
                } else {
                    while left < right && height[right] <= h {
                        right -= 1;
                    }
                }
            }

            best
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> =
        vec![(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49), (vec![1, 1], 1)];

    println!("--- My Solution ---");
    for (height, expected) in &test_cases {
        let result = my_solution::Solution::max_area(height.clone());
        println!("  {height:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (height, expected) in &test_cases {
        let result = ai_solution::Solution::max_area(height.clone());
        println!("  {height:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_max_area {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Solution::max_area(vec![1, 1]), 1);
                }

                #[test]
                fn test_decreasing() {
                    assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::max_area(vec![5, 5, 5, 5]), 15);
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(Solution::max_area(vec![3, 7]), 3);
                }
            }
        };
    }

    test_max_area!(my_tests, crate::my_solution::Solution);
    test_max_area!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
            vec![1, 1],
            vec![4, 3, 2, 1, 4],
            vec![5, 5, 5, 5],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for h in &inputs {
                std::hint::black_box(crate::my_solution::Solution::max_area(
                    std::hint::black_box(h.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for h in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::max_area(
                    std::hint::black_box(h.clone()),
                ));
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
