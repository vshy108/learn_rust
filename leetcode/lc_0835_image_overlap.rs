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
//
// --- Rust concepts used ---
// - Nested `for` with i32 ranges including negative: `-(n-1)..n`
//   iterates signed ranges for translation offsets
// - HashMap<(i32, i32), i32> counting: tuple key for (dx, dy) offset
//   with increment via `entry().or_insert(0) += 1`
// - `.values().copied().max().unwrap_or(0)`: chain to extract max
//   from HashMap values with a default for empty maps
//   (Previously covered: HashMap entry -> lc_0380, nested loops -> lc_0042,
//    flat_map -> lc_0005)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  548.70ms (avg 5.48us)
// AI solution:  1069.13ms (avg 10.69us)
// -> My solution is 1.95x faster
// Reason: brute-force translation is cache-friendly with sequential
//   matrix access; the HashMap approach has hashing overhead and
//   random access patterns that hurt on small matrices.

/// My solution module — brute-force all translations.
///
/// Time:  O(n^4)
/// Space: O(1)
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
            let n = img1.len() as i32;
            let mut max_overlap = 0;
            for dx in -(n - 1)..n {
                for dy in -(n - 1)..n {
                    let mut overlap = 0;
                    for i in 0..n {
                        for j in 0..n {
                            let ni = i + dx;
                            let nj = j + dy;
                            if ni >= 0
                                && ni < n
                                && nj >= 0
                                && nj < n
                                && img1[i as usize][j as usize] == 1
                                && img2[ni as usize][nj as usize] == 1
                            {
                                overlap += 1;
                            }
                        }
                    }
                    max_overlap = max_overlap.max(overlap);
                }
            }
            max_overlap
        }
    }
}

/// AI solution module — HashMap counting of translation vectors.
///
/// Collects 1-positions from both images, counts each (dx, dy)
/// offset that aligns a pair of 1s, and returns the max count.
///
/// Time:  O(n^2 * m^2) where m = number of 1s (but often much less than n^4)
/// Space: O(m^2) for the counter map
mod ai_solution {
    use std::collections::HashMap;

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
            let n = img1.len();
            // Collect positions of 1s in each image
            let mut ones1 = Vec::new();
            let mut ones2 = Vec::new();
            for i in 0..n {
                for j in 0..n {
                    if img1[i][j] == 1 {
                        ones1.push((i as i32, j as i32));
                    }
                    if img2[i][j] == 1 {
                        ones2.push((i as i32, j as i32));
                    }
                }
            }
            let mut count: HashMap<(i32, i32), i32> = HashMap::new();
            for &(r1, c1) in &ones1 {
                for &(r2, c2) in &ones2 {
                    *count.entry((r1 - r2, c1 - c2)).or_insert(0) += 1;
                }
            }
            count.values().copied().max().unwrap_or(0)
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
