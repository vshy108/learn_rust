// LeetCode 391: Perfect Rectangle [Hard]
// https://leetcode.com/problems/perfect-rectangle/
//
// Benchmark: 50,000 iterations (2025-04-08)
//   My solution (HashSet toggle): avg 1.06ms
//   AI solution (HashMap count):  avg 1.15ms
//   -> My solution is 1.09x faster
//
// Given an array `rectangles` where `rectangles[i] = [xi, yi, ai, bi]`
// represents an axis-aligned rectangle with bottom-left (xi, yi) and
// top-right (ai, bi), return true if all rectangles together form an
// exact cover of a rectangular region (no gaps, no overlaps).
//
// Examples:
//   [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]] -> true
//   [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]           -> false
//   [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]           -> false
//
// Constraints:
//   - 1 <= rectangles.length <= 2 * 10^4
//   - rectangles[i].length == 4
//   - -10^5 <= xi, yi, ai, bi <= 10^5
//
// Run:  cargo run --example lc_0391_perfect_rectangle
// Test: cargo test --example lc_0391_perfect_rectangle
// Bench: cargo test --example lc_0391_perfect_rectangle bench -- --nocapture

/// My solution module — corner counting + area check.
///
/// Key insight: A perfect rectangle has exactly these properties:
/// 1. The sum of all sub-rectangle areas equals the bounding box area.
/// 2. Every corner point appears an odd number of times only at the
///    four corners of the bounding box. Interior corners always appear
///    an even number of times (2 or 4), canceling out.
///
/// We use a HashSet to toggle corners: insert if absent, remove if present.
/// After processing all rectangles, exactly 4 corners should remain — the
/// bounding box corners. Combined with the area check, this guarantees
/// no gaps or overlaps.
///
/// Time:  O(n) where n = number of rectangles
/// Space: O(n) for the corner set
mod my_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Check if rectangles form a perfect rectangle.
        ///
        /// # Arguments
        ///
        /// * `rectangles` - Vec of [x1, y1, x2, y2] sub-rectangles
        ///
        /// # Returns
        ///
        /// true if they form an exact rectangular cover
        pub fn is_perfect_rect(rectangles: Vec<Vec<i32>>) -> bool {
            let mut area_sum: i64 = 0;
            let mut min_x = i32::MAX;
            let mut min_y = i32::MAX;
            let mut max_x = i32::MIN;
            let mut max_y = i32::MIN;
            let mut corners: HashSet<(i32, i32)> = HashSet::new();

            for r in &rectangles {
                let (x1, y1, x2, y2) = (r[0], r[1], r[2], r[3]);

                // Update bounding box
                min_x = min_x.min(x1);
                min_y = min_y.min(y1);
                max_x = max_x.max(x2);
                max_y = max_y.max(y2);

                // Accumulate area (use i64 to avoid overflow)
                area_sum += (x2 - x1) as i64 * (y2 - y1) as i64;

                // Toggle each of the 4 corners in the set
                for point in [(x1, y1), (x1, y2), (x2, y1), (x2, y2)] {
                    if !corners.remove(&point) {
                        corners.insert(point);
                    }
                }
            }

            // Check 1: total area must match bounding box area
            let bounding_area = (max_x - min_x) as i64 * (max_y - min_y) as i64;
            if area_sum != bounding_area {
                return false;
            }

            // Check 2: exactly 4 corners remain — the bounding box corners
            corners.len() == 4
                && corners.contains(&(min_x, min_y))
                && corners.contains(&(min_x, max_y))
                && corners.contains(&(max_x, min_y))
                && corners.contains(&(max_x, max_y))
        }
    }
}

/// AI solution module — corner counting with explicit parity map.
///
/// Same algorithm but uses a HashMap<(i32,i32), u32> to count exact
/// occurrences rather than toggling. At the end, only the 4 bounding
/// box corners should have an odd count.
///
/// This makes the parity logic more explicit and easier to debug.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Check if rectangles form a perfect rectangle.
        ///
        /// # Arguments
        ///
        /// * `rectangles` - Vec of [x1, y1, x2, y2] sub-rectangles
        ///
        /// # Returns
        ///
        /// true if they form an exact rectangular cover
        pub fn is_perfect_rect(rectangles: Vec<Vec<i32>>) -> bool {
            let mut area_sum: i64 = 0;
            let mut min_x = i32::MAX;
            let mut min_y = i32::MAX;
            let mut max_x = i32::MIN;
            let mut max_y = i32::MIN;
            let mut count: HashMap<(i32, i32), u32> = HashMap::new();

            for r in &rectangles {
                let (x1, y1, x2, y2) = (r[0], r[1], r[2], r[3]);

                min_x = min_x.min(x1);
                min_y = min_y.min(y1);
                max_x = max_x.max(x2);
                max_y = max_y.max(y2);

                area_sum += (x2 - x1) as i64 * (y2 - y1) as i64;

                for point in [(x1, y1), (x1, y2), (x2, y1), (x2, y2)] {
                    *count.entry(point).or_insert(0) += 1;
                }
            }

            let bounding_area = (max_x - min_x) as i64 * (max_y - min_y) as i64;
            if area_sum != bounding_area {
                return false;
            }

            // Collect all corners with odd count
            let odd_corners: Vec<_> = count
                .iter()
                .filter(|(_, v)| *v % 2 == 1)
                .map(|(&k, _)| k)
                .collect();

            odd_corners.len() == 4
                && odd_corners.contains(&(min_x, min_y))
                && odd_corners.contains(&(min_x, max_y))
                && odd_corners.contains(&(max_x, min_y))
                && odd_corners.contains(&(max_x, max_y))
        }
    }
}

fn main() {
    let cases: Vec<(Vec<Vec<i32>>, bool)> = vec![
        (
            vec![
                vec![1, 1, 3, 3],
                vec![3, 1, 4, 2],
                vec![3, 2, 4, 4],
                vec![1, 3, 2, 4],
                vec![2, 3, 3, 4],
            ],
            true,
        ),
        (
            vec![
                vec![1, 1, 2, 3],
                vec![1, 3, 2, 4],
                vec![3, 1, 4, 2],
                vec![3, 2, 4, 4],
            ],
            false,
        ),
        (
            vec![
                vec![1, 1, 3, 3],
                vec![3, 1, 4, 2],
                vec![1, 3, 2, 4],
                vec![2, 2, 4, 4],
            ],
            false,
        ),
    ];

    println!("--- My Solution ---");
    for (rects, expected) in &cases {
        let result = my_solution::Solution::is_perfect_rect(rects.clone());
        println!("  -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (rects, expected) in &cases {
        let result = ai_solution::Solution::is_perfect_rect(rects.clone());
        println!("  -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_perfect_rect {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1_true() {
                    assert!(Solution::is_perfect_rect(vec![
                        vec![1, 1, 3, 3],
                        vec![3, 1, 4, 2],
                        vec![3, 2, 4, 4],
                        vec![1, 3, 2, 4],
                        vec![2, 3, 3, 4],
                    ]));
                }

                #[test]
                fn test_example2_gap() {
                    assert!(!Solution::is_perfect_rect(vec![
                        vec![1, 1, 2, 3],
                        vec![1, 3, 2, 4],
                        vec![3, 1, 4, 2],
                        vec![3, 2, 4, 4],
                    ]));
                }

                #[test]
                fn test_example3_overlap() {
                    assert!(!Solution::is_perfect_rect(vec![
                        vec![1, 1, 3, 3],
                        vec![3, 1, 4, 2],
                        vec![1, 3, 2, 4],
                        vec![2, 2, 4, 4],
                    ]));
                }

                #[test]
                fn test_single_rect() {
                    assert!(Solution::is_perfect_rect(vec![vec![0, 0, 1, 1],]));
                }

                #[test]
                fn test_two_side_by_side() {
                    assert!(Solution::is_perfect_rect(vec![
                        vec![0, 0, 1, 1],
                        vec![1, 0, 2, 1],
                    ]));
                }

                #[test]
                fn test_duplicate_overlap() {
                    // Two identical rectangles overlap
                    assert!(!Solution::is_perfect_rect(vec![
                        vec![0, 0, 1, 1],
                        vec![0, 0, 1, 1],
                    ]));
                }
            }
        };
    }

    test_perfect_rect!(my_tests, crate::my_solution::Solution);
    test_perfect_rect!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 50_000_u32;
        // Build a 100x100 grid of unit rectangles (10,000 rects)
        let mut rects = Vec::with_capacity(10_000);
        for x in 0..100 {
            for y in 0..100 {
                rects.push(vec![x, y, x + 1, y + 1]);
            }
        }

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_perfect_rect(
                std::hint::black_box(rects.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_perfect_rect(
                std::hint::black_box(rects.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
