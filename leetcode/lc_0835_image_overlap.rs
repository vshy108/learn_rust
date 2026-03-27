// LeetCode 835: Image Overlap [Medium]
// https://leetcode.com/problems/image-overlap/
//
// You are given two images, img1 and img2, represented as binary, square
// matrices of size n x n. A binary matrix has only 0s and 1s as values.
//
// We translate one image however we choose by sliding all the 1s left, right,
// up, and/or down any number of units. We then place it on top of the other
// image. We can then calculate the overlap by counting the number of positions
// that have a 1 in both images.
//
// Note also that a translation does NOT include any kind of rotation. Any 1s
// that are translated outside of the matrix borders are erased.
//
// Return the largest possible overlap.
//
// Examples:
//   img1 = [[1,1,0],[0,1,0],[0,1,0]]
//   img2 = [[0,0,0],[0,1,1],[0,0,1]] -> 3
//   img1 = [[1]], img2 = [[1]]       -> 1
//   img1 = [[0]], img2 = [[0]]       -> 0
//
// Constraints:
//   - n == img1.length == img1[i].length
//   - n == img2.length == img2[i].length
//   - 1 <= n <= 30
//   - img1[i][j] is either 0 or 1
//   - img2[i][j] is either 0 or 1
//
// Run:  cargo run --example lc_0835_image_overlap
// Test: cargo test --example lc_0835_image_overlap
// Bench: cargo test --example lc_0835_image_overlap bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the largest overlap between two binary matrices after translation.
        ///
        /// # Arguments
        ///
        /// * `img1` - First n x n binary matrix
        /// * `img2` - Second n x n binary matrix
        ///
        /// # Returns
        ///
        /// Maximum number of overlapping 1s after any translation
        pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
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
        /// Find the largest overlap between two binary matrices after translation.
        ///
        /// # Arguments
        ///
        /// * `img1` - First n x n binary matrix
        /// * `img2` - Second n x n binary matrix
        ///
        /// # Returns
        ///
        /// Maximum number of overlapping 1s after any translation
        pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>, i32)> = vec![
        (
            vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
            vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
            3,
        ),
        (vec![vec![1]], vec![vec![1]], 1),
        (vec![vec![0]], vec![vec![0]], 0),
    ];

    println!("--- My Solution ---");
    for (img1, img2, expected) in &test_cases {
        let result = my_solution::Solution::largest_overlap(img1.clone(), img2.clone());
        println!("  img1={img1:?}, img2={img2:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (img1, img2, expected) in &test_cases {
        let result = ai_solution::Solution::largest_overlap(img1.clone(), img2.clone());
        println!("  img1={img1:?}, img2={img2:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_largest_overlap {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::largest_overlap(
                            vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                            vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
                        ),
                        3
                    );
                }

                #[test]
                fn test_single_one() {
                    assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![1]]), 1);
                }

                #[test]
                fn test_single_zero() {
                    assert_eq!(Solution::largest_overlap(vec![vec![0]], vec![vec![0]]), 0);
                }

                #[test]
                fn test_no_overlap_possible() {
                    assert_eq!(
                        Solution::largest_overlap(
                            vec![vec![1, 0], vec![0, 0]],
                            vec![vec![0, 0], vec![0, 1]],
                        ),
                        1
                    );
                }

                #[test]
                fn test_identical_images() {
                    assert_eq!(
                        Solution::largest_overlap(
                            vec![vec![1, 0], vec![0, 1]],
                            vec![vec![1, 0], vec![0, 1]],
                        ),
                        2
                    );
                }

                #[test]
                fn test_all_ones() {
                    assert_eq!(
                        Solution::largest_overlap(
                            vec![vec![1, 1], vec![1, 1]],
                            vec![vec![1, 1], vec![1, 1]],
                        ),
                        4
                    );
                }

                #[test]
                fn test_all_zeros() {
                    assert_eq!(
                        Solution::largest_overlap(
                            vec![vec![0, 0], vec![0, 0]],
                            vec![vec![0, 0], vec![0, 0]],
                        ),
                        0
                    );
                }
            }
        };
    }

    test_largest_overlap!(my_tests, crate::my_solution::Solution);
    test_largest_overlap!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(Vec<Vec<i32>>, Vec<Vec<i32>>)> = vec![
            (
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
            ),
            (vec![vec![1]], vec![vec![1]]),
            (vec![vec![0]], vec![vec![0]]),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for (img1, img2) in &inputs {
                std::hint::black_box(crate::my_solution::Solution::largest_overlap(
                    std::hint::black_box(img1.clone()),
                    std::hint::black_box(img2.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for (img1, img2) in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::largest_overlap(
                    std::hint::black_box(img1.clone()),
                    std::hint::black_box(img2.clone()),
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
