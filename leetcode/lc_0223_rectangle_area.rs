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
//   ax1=-3, ay1=-3, ax2=3, ay2=3, bx1=-2, by1=-2, bx2=2, by2=2 -> 24
//   ax1=-2, ay1=-2, ax2=2, ay2=2, bx1=3, by1=3, bx2=4, by2=4   -> 17
//
// Constraints:
//   - -10^4 <= ax1 <= ax2 <= 10^4
//   - -10^4 <= ay1 <= ay2 <= 10^4
//   - -10^4 <= bx1 <= bx2 <= 10^4
//   - -10^4 <= by1 <= by2 <= 10^4
//
// Run:  cargo run --example lc_0223_rectangle_area
// Test: cargo test --example lc_0223_rectangle_area
// Bench: cargo test --example lc_0223_rectangle_area bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32, i32, i32, i32, i32, i32, i32, i32)> = vec![
        (-3, -3, 3, 3, -2, -2, 2, 2, 24),
        (-2, -2, 2, 2, 3, 3, 4, 4, 17),
    ];

    println!("--- My Solution ---");
    for &(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2, expected) in &test_cases {
        let result =
            my_solution::Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
        println!(
            "  ({ax1},{ay1},{ax2},{ay2}),({bx1},{by1},{bx2},{by2}) -> {result} (expected {expected})"
        );
    }

    println!("--- AI Solution ---");
    for &(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2, expected) in &test_cases {
        let result =
            ai_solution::Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2);
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
                    assert_eq!(
                        Solution::compute_area(-3, -3, 3, 3, -2, -2, 2, 2),
                        24
                    );
                }

                #[test]
                fn test_no_overlap() {
                    assert_eq!(
                        Solution::compute_area(-2, -2, 2, 2, 3, 3, 4, 4),
                        17
                    );
                }

                #[test]
                fn test_one_inside_other() {
                    // B is fully inside A
                    assert_eq!(
                        Solution::compute_area(-5, -5, 5, 5, -1, -1, 1, 1),
                        100
                    );
                }

                #[test]
                fn test_identical_rectangles() {
                    assert_eq!(
                        Solution::compute_area(0, 0, 2, 2, 0, 0, 2, 2),
                        4
                    );
                }

                #[test]
                fn test_touching_edge() {
                    // Rectangles share an edge but no area overlap
                    assert_eq!(
                        Solution::compute_area(0, 0, 2, 2, 2, 0, 4, 2),
                        8
                    );
                }

                #[test]
                fn test_partial_overlap_horizontal() {
                    assert_eq!(
                        Solution::compute_area(0, 0, 4, 4, 2, 0, 6, 4),
                        24
                    );
                }

                #[test]
                fn test_zero_area_rectangle() {
                    // Degenerate rectangle (line)
                    assert_eq!(
                        Solution::compute_area(0, 0, 0, 0, 1, 1, 3, 3),
                        4
                    );
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
