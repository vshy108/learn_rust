// LeetCode 223: Rectangle Area [Medium]
// https://leetcode.com/problems/rectangle-area/
//
// Given the coordinates of two rectilinear rectangles in a 2D plane,
// return the total area covered by the two rectangles.
//
// The first rectangle is defined by its bottom-left corner (ax1, ay1)
// and its top-right corner (ax2, ay2).
//
// The second rectangle is defined by its bottom-left corner (bx1, by1)
// and its top-right corner (bx2, by2).
//
// Examples:
//   ax1=-3, ay1=0, ax2=3, ay2=4, bx1=0, by1=-1, bx2=9, by2=2   -> 45
//   ax1=-2, ay1=-2, ax2=2, ay2=2, bx1=-2, by1=-2, bx2=2, by2=2 -> 16
//
// Constraints:
//   - -10^4 <= ax1 <= ax2 <= 10^4
//   - -10^4 <= ay1 <= ay2 <= 10^4
//   - -10^4 <= bx1 <= bx2 <= 10^4
//   - -10^4 <= by1 <= by2 <= 10^4
//
// --- Benchmark (100_000 iterations, 2026-04-05) ---
//   My solution: 15.3ms (avg 153ns)
//   AI solution: 12.0ms (avg 120ns)
//   -> AI solution is 1.28x faster
//   Reason: Early exit on non-overlapping rectangles skips the min/max
//     chain entirely for 1 of 4 benchmark inputs, saving branch work.
//
// --- Rust concepts used ---
//   - .min() / .max() on i32: Ord trait methods, zero allocation.
//     Ruby equivalent: [a, b].min creates an array then calls min.
//   - .max(0) for clamping: replaces Ruby's [[val, 0].max] pattern.
//   - #[allow(clippy::too_many_arguments)]: suppresses clippy warning
//     for 8-param function (LeetCode's signature, can't change it).
//
// Run:  cargo run --example lc_0223_rectangle_area
// Test: cargo test --example lc_0223_rectangle_area
// Bench: cargo test --example lc_0223_rectangle_area bench -- --nocapture

/// My solution module.
///
/// Inclusion-exclusion: area_A + area_B - overlap.
/// Overlap computed via min/max clamped to 0.
///
/// Time:  O(1) — fixed arithmetic operations
/// Space: O(1) — no extra allocations
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute the total area covered by two axis-aligned rectangles.
        ///
        /// # Arguments
        ///
        /// * `ax1`, `ay1` - Bottom-left corner of rectangle A
        /// * `ax2`, `ay2` - Top-right corner of rectangle A
        /// * `bx1`, `by1` - Bottom-left corner of rectangle B
        /// * `bx2`, `by2` - Top-right corner of rectangle B
        ///
        /// # Returns
        ///
        /// Total area covered (area_A + area_B - overlap)
        #[allow(clippy::too_many_arguments)]
        pub fn compute_area(
            ax1: i32,
            ay1: i32,
            ax2: i32,
            ay2: i32,
            bx1: i32,
            by1: i32,
            bx2: i32,
            by2: i32,
        ) -> i32 {
            let area_a = (ax2 - ax1) * (ay2 - ay1);
            let area_b = (bx2 - bx1) * (by2 - by1);
            // Ruby: [[ax2, bx2].min - [ax1, bx1].max, 0].max
            // Rust: no [a, b].min on arrays — use .min() / .max()
            // methods on i32 (from Ord trait). Zero allocation.
            let overlap_width = (ax2.min(bx2) - ax1.max(bx1)).max(0);
            let overlap_height = (ay2.min(by2) - ay1.max(by1)).max(0);
            let overlap_area = overlap_width * overlap_height;

            area_a + area_b - overlap_area
        }
    }
}

/// AI solution module.
///
/// Same inclusion-exclusion formula, but uses an explicit
/// non-overlap check before computing overlap, avoiding
/// the min/max chain entirely when rectangles don't intersect.
///
/// Time:  O(1) — fixed arithmetic operations
/// Space: O(1) — no extra allocations
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compute the total area covered by two axis-aligned rectangles.
        ///
        /// # Arguments
        ///
        /// * `ax1`, `ay1` - Bottom-left corner of rectangle A
        /// * `ax2`, `ay2` - Top-right corner of rectangle A
        /// * `bx1`, `by1` - Bottom-left corner of rectangle B
        /// * `bx2`, `by2` - Top-right corner of rectangle B
        ///
        /// # Returns
        ///
        /// Total area covered (area_A + area_B - overlap)
        #[allow(clippy::too_many_arguments)]
        pub fn compute_area(
            ax1: i32,
            ay1: i32,
            ax2: i32,
            ay2: i32,
            bx1: i32,
            by1: i32,
            bx2: i32,
            by2: i32,
        ) -> i32 {
            let area_a = (ax2 - ax1) * (ay2 - ay1);
            let area_b = (bx2 - bx1) * (by2 - by1);

            // Early exit: if rectangles don't overlap at all, skip overlap math
            // Separated conditions are clearer than the clamped min/max approach
            if ax1 >= bx2 || bx1 >= ax2 || ay1 >= by2 || by1 >= ay2 {
                return area_a + area_b;
            }

            // Rectangles overlap — compute intersection dimensions directly
            let overlap = (ax2.min(bx2) - ax1.max(bx1)) * (ay2.min(by2) - ay1.max(by1));
            area_a + area_b - overlap
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32)> = vec![
        (-3, 0, 3, 4, 0, -1, 9, 2, 45),
        (-2, -2, 2, 2, -2, -2, 2, 2, 16),
    ];

    println!("--- My Solution ---");
    for &(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2, expected) in &test_cases {
        let result = my_solution::Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
        println!(
            "  ({ax1},{ay1},{ax2},{ay2}),({bx1},{by1},{bx2},{by2}) -> {result} (expected {expected})"
        );
    }

    println!("--- AI Solution ---");
    for &(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2, expected) in &test_cases {
        let result = ai_solution::Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
        println!(
            "  ({ax1},{ay1},{ax2},{ay2}),({bx1},{by1},{bx2},{by2}) -> {result} (expected {expected})"
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_rectangle_area {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_overlapping_rectangles() {
                    assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
                }

                #[test]
                fn test_identical_rectangles_example2() {
                    // LeetCode Example 2: identical rectangles
                    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
                }

                #[test]
                fn test_one_inside_other() {
                    // B is fully inside A
                    assert_eq!(Solution::compute_area(-5, -5, 5, 5, -1, -1, 1, 1), 100);
                }

                #[test]
                fn test_no_overlap() {
                    // Rectangles are completely separate
                    assert_eq!(Solution::compute_area(-2, -2, 2, 2, 3, 3, 4, 4), 17);
                }

                #[test]
                fn test_identical_rectangles() {
                    assert_eq!(Solution::compute_area(0, 0, 2, 2, 0, 0, 2, 2), 4);
                }

                #[test]
                fn test_touching_edge() {
                    // Rectangles share an edge but no area overlap
                    assert_eq!(Solution::compute_area(0, 0, 2, 2, 2, 0, 4, 2), 8);
                }

                #[test]
                fn test_partial_overlap_horizontal() {
                    assert_eq!(Solution::compute_area(0, 0, 4, 4, 2, 0, 6, 4), 24);
                }

                #[test]
                fn test_zero_area_rectangle() {
                    // Degenerate rectangle (line)
                    assert_eq!(Solution::compute_area(0, 0, 0, 0, 1, 1, 3, 3), 4);
                }
            }
        };
    }

    test_rectangle_area!(my_tests, crate::my_solution::Solution);
    test_rectangle_area!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(i32, i32, i32, i32, i32, i32, i32, i32)> = vec![
            (-3, -3, 3, 3, -2, -2, 2, 2),
            (-2, -2, 2, 2, 3, 3, 4, 4),
            (0, 0, 4, 4, 2, 0, 6, 4),
            (-5, -5, 5, 5, -1, -1, 1, 1),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::compute_area(
                    std::hint::black_box(ax1),
                    std::hint::black_box(ay1),
                    std::hint::black_box(ax2),
                    std::hint::black_box(ay2),
                    std::hint::black_box(bx1),
                    std::hint::black_box(by1),
                    std::hint::black_box(bx2),
                    std::hint::black_box(by2),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::compute_area(
                    std::hint::black_box(ax1),
                    std::hint::black_box(ay1),
                    std::hint::black_box(ax2),
                    std::hint::black_box(ay2),
                    std::hint::black_box(bx1),
                    std::hint::black_box(by1),
                    std::hint::black_box(bx2),
                    std::hint::black_box(by2),
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
