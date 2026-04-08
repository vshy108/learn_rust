// LeetCode 587: Erect the Fence [Hard]
// https://leetcode.com/problems/erect-the-fence/
//
// You are given an array `trees` where trees[i] = [xi, yi] represents the
// location of a tree in the garden.
//
// Fence the entire garden using the minimum length of rope, as it is
// expensive. The garden is well-fenced only if all the trees are enclosed.
//
// Return the coordinates of trees that are exactly located on the fence
// perimeter. You may return the answer in any order.
//
// Examples:
//   trees = [[1,1],[2,2],[2,0],[2,4],[3,3],[4,2]]
//     -> [[1,1],[2,0],[4,2],[3,3],[2,4]]
//   trees = [[1,2],[2,2],[4,2]]
//     -> [[1,2],[2,2],[4,2]]
//
// Constraints:
//   - 1 <= trees.length <= 3000
//   - trees[i].length == 2
//   - 0 <= xi, yi <= 100
//   - All the given positions are unique
//
// Run:  cargo run --example lc_0587_erect_the_fence
// Test: cargo test --example lc_0587_erect_the_fence
// Bench: cargo test --example lc_0587_erect_the_fence bench -- --nocapture
//
// --- Rust concepts used ---
// - Closure capturing slices: `let cross = |o: &[i32], a: &[i32], b: &[i32]|`
//   defines a local closure for the cross product computation
// - `sort_unstable_by` with chained `cmp().then()`: multi-key sort
//   without allocating a stable temp buffer
// - `HashSet<usize>` for deduplication: collect hull indices then map
//   back to coordinates
//   (Previously covered: closure -> lc_0005, sort_unstable -> lc_0274,
//    HashSet -> lc_0003, i64 widening for overflow safety -> lc_0374)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  920.40ms (avg 9.20us)
// AI solution:  470.34ms (avg 4.70us)
// -> AI solution is 1.96x faster
// Reason: Andrew's monotone chain is O(n log n) vs Jarvis march
//   O(n * h); sort+dedup avoids HashSet overhead for deduplication.

/// My solution module — Jarvis march (gift wrapping) with collinear points.
///
/// Time:  O(n * h) where h = number of hull points
/// Space: O(h)
mod my_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Return the coordinates of trees on the convex hull fence.
        ///
        /// # Arguments
        ///
        /// * `trees` - Array of [x, y] tree coordinates
        ///
        /// # Returns
        ///
        /// Coordinates of trees on the fence perimeter
        pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let n = trees.len();
            if n <= 3 {
                return trees;
            }
            // cross product of vectors OA and OB
            let cross = |o: &[i32], a: &[i32], b: &[i32]| -> i64 {
                (a[0] as i64 - o[0] as i64) * (b[1] as i64 - o[1] as i64)
                    - (a[1] as i64 - o[1] as i64) * (b[0] as i64 - o[0] as i64)
            };
            let dist2 = |a: &[i32], b: &[i32]| -> i64 {
                let dx = a[0] as i64 - b[0] as i64;
                let dy = a[1] as i64 - b[1] as i64;
                dx * dx + dy * dy
            };
            // Find leftmost point
            let mut start = 0;
            for i in 1..n {
                if trees[i][0] < trees[start][0]
                    || (trees[i][0] == trees[start][0] && trees[i][1] < trees[start][1])
                {
                    start = i;
                }
            }
            let mut on_hull: HashSet<usize> = HashSet::new();
            let mut hull_indices: Vec<usize> = Vec::new();
            let mut current = start;
            loop {
                hull_indices.push(current);
                on_hull.insert(current);
                let mut next = 0;
                for candidate in 0..n {
                    if candidate == current {
                        continue;
                    }
                    let cp = cross(&trees[current], &trees[next], &trees[candidate]);
                    if next == current
                        || cp < 0
                        || (cp == 0
                            && dist2(&trees[current], &trees[candidate])
                                > dist2(&trees[current], &trees[next]))
                    {
                        next = candidate;
                    }
                }
                // Also include collinear points between current and next
                for candidate in 0..n {
                    if candidate != current && candidate != next && !on_hull.contains(&candidate) {
                        if cross(&trees[current], &trees[next], &trees[candidate]) == 0 {
                            hull_indices.push(candidate);
                            on_hull.insert(candidate);
                        }
                    }
                }
                current = next;
                if current == start {
                    break;
                }
            }
            hull_indices.into_iter().map(|i| trees[i].clone()).collect()
        }
    }
}

/// AI solution module — Andrew's monotone chain (sort + upper/lower hull).
///
/// Sort points by x (then y), build lower hull and upper hull separately.
/// Uses `<=` cross product to include collinear boundary points.
///
/// Time:  O(n log n) due to sorting
/// Space: O(n)
/// AI solution module — Andrew's monotone chain with sort+dedup.
///
/// Builds lower and upper hulls directly with cloned points, then
/// deduplicates via sort+dedup (avoids HashSet hashing overhead).
/// Uses a named `fn` instead of a closure for the cross product.
///
/// Time:  O(n log n) due to sorting
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the coordinates of trees on the convex hull fence.
        ///
        /// # Arguments
        ///
        /// * `trees` - Array of [x, y] tree coordinates
        ///
        /// # Returns
        ///
        /// Coordinates of trees on the fence perimeter
        pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let mut pts = trees;
            pts.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
            let n = pts.len();
            if n <= 1 {
                return pts;
            }
            fn cross(o: &[i32], a: &[i32], b: &[i32]) -> i64 {
                (a[0] - o[0]) as i64 * (b[1] - o[1]) as i64
                    - (a[1] - o[1]) as i64 * (b[0] - o[0]) as i64
            }
            let mut hull: Vec<Vec<i32>> = Vec::new();
            for p in &pts {
                while hull.len() >= 2 && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], p) < 0
                {
                    hull.pop();
                }
                hull.push(p.clone());
            }
            let lower_len = hull.len();
            for p in pts.iter().rev() {
                while hull.len() > lower_len
                    && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], p) < 0
                {
                    hull.pop();
                }
                hull.push(p.clone());
            }
            hull.pop();
            hull.sort_unstable();
            hull.dedup();
            hull
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)> = vec![
        (
            vec![
                vec![1, 1],
                vec![2, 2],
                vec![2, 0],
                vec![2, 4],
                vec![3, 3],
                vec![4, 2],
            ],
            vec![vec![1, 1], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]],
        ),
        (
            vec![vec![1, 2], vec![2, 2], vec![4, 2]],
            vec![vec![1, 2], vec![2, 2], vec![4, 2]],
        ),
    ];

    println!("--- My Solution ---");
    for (trees, expected) in &test_cases {
        let result = my_solution::Solution::outer_trees(trees.clone());
        println!("  {trees:?} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (trees, expected) in &test_cases {
        let result = ai_solution::Solution::outer_trees(trees.clone());
        println!("  {trees:?} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    /// Helper to sort points for comparison (order-independent equality).
    fn sorted_points(mut pts: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        pts.sort();
        pts
    }

    macro_rules! test_outer_trees {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted_points;
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    let trees = vec![
                        vec![1, 1],
                        vec![2, 2],
                        vec![2, 0],
                        vec![2, 4],
                        vec![3, 3],
                        vec![4, 2],
                    ];
                    let expected = vec![vec![1, 1], vec![2, 0], vec![2, 4], vec![3, 3], vec![4, 2]];
                    assert_eq!(
                        sorted_points(Solution::outer_trees(trees)),
                        sorted_points(expected)
                    );
                }

                #[test]
                fn test_collinear() {
                    let trees = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
                    let expected = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
                    assert_eq!(
                        sorted_points(Solution::outer_trees(trees)),
                        sorted_points(expected)
                    );
                }

                #[test]
                fn test_single_point() {
                    let trees = vec![vec![0, 0]];
                    assert_eq!(
                        sorted_points(Solution::outer_trees(trees)),
                        vec![vec![0, 0]]
                    );
                }

                #[test]
                fn test_two_points() {
                    let trees = vec![vec![0, 0], vec![1, 1]];
                    assert_eq!(
                        sorted_points(Solution::outer_trees(trees)),
                        sorted_points(vec![vec![0, 0], vec![1, 1]])
                    );
                }

                #[test]
                fn test_square() {
                    let trees = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
                    assert_eq!(
                        sorted_points(Solution::outer_trees(trees)),
                        sorted_points(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1],])
                    );
                }

                #[test]
                fn test_triangle_with_interior() {
                    let trees = vec![vec![0, 0], vec![4, 0], vec![2, 4], vec![2, 1]];
                    let expected = vec![vec![0, 0], vec![4, 0], vec![2, 4]];
                    assert_eq!(
                        sorted_points(Solution::outer_trees(trees)),
                        sorted_points(expected)
                    );
                }

                #[test]
                fn test_all_same_x() {
                    let trees = vec![vec![1, 0], vec![1, 1], vec![1, 2]];
                    assert_eq!(
                        sorted_points(Solution::outer_trees(trees)),
                        sorted_points(vec![vec![1, 0], vec![1, 1], vec![1, 2]])
                    );
                }
            }
        };
    }

    test_outer_trees!(my_tests, crate::my_solution::Solution);
    test_outer_trees!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<Vec<i32>>> = vec![
            vec![
                vec![1, 1],
                vec![2, 2],
                vec![2, 0],
                vec![2, 4],
                vec![3, 3],
                vec![4, 2],
            ],
            vec![vec![1, 2], vec![2, 2], vec![4, 2]],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for trees in &inputs {
                std::hint::black_box(crate::my_solution::Solution::outer_trees(
                    std::hint::black_box(trees.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for trees in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::outer_trees(
                    std::hint::black_box(trees.clone()),
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
