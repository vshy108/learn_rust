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

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
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
            todo!()
        }
    }
}

/// Helper to sort points for comparison (order-independent equality).
fn sorted_points(mut pts: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    pts.sort();
    pts
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
    use super::sorted_points;

    macro_rules! test_outer_trees {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::sorted_points;
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
