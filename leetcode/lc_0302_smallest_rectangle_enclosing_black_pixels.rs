// LeetCode 302: Smallest Rectangle Enclosing Black Pixels [Hard]
// https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels/
// Given an m x n binary matrix (0=white, 1=black), with one connected black region,
// and a black pixel at (x, y), return the area of the smallest axis-aligned rectangle
// enclosing all black pixels. Must be better than O(mn).
//
// Benchmark: 100_000 iterations
// My solution (4x binary search): avg 0.191us
// AI solution (linear scan):      avg 0.476us
// Winner: My solution (2.49x faster)

// ------ My Solution: Binary Search on rows/columns ------
// Time: O(m log n + n log m) — 4 binary searches, each scanning a row/col
// Space: O(1) — only index variables
/// Uses binary search to find the top, bottom, left, right boundaries.
/// For each boundary direction, binary search on rows or columns checking
/// if the row/col contains any '1'.
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
            let m = image.len();
            let n = image[0].len();
            let (x, y) = (x as usize, y as usize);

            // Find top boundary: first row with a '1' in [0, x]
            let mut lo = 0usize;
            let mut hi = x;
            while lo < hi {
                let mid = (lo + hi) / 2;
                // Check if row `mid` has any '1'
                if image[mid].iter().any(|&c| c == '1') {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            let top = lo;

            // Find bottom boundary: last row with a '1' in [x, m-1]
            lo = x;
            hi = m - 1;
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if image[mid].iter().any(|&c| c == '1') {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            let bottom = lo;

            // Find left boundary: first col with a '1' in [0, y]
            lo = 0;
            hi = y;
            while lo < hi {
                let mid = (lo + hi) / 2;
                if (0..m).any(|r| image[r][mid] == '1') {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }
            let left = lo;

            // Find right boundary: last col with a '1' in [y, n-1]
            lo = y;
            hi = n - 1;
            while lo < hi {
                let mid = (lo + hi + 1) / 2;
                if (0..m).any(|r| image[r][mid] == '1') {
                    lo = mid;
                } else {
                    hi = mid - 1;
                }
            }
            let right = lo;

            ((bottom - top + 1) * (right - left + 1)) as i32
        }
    }
}

// ------ AI Solution: DFS to find min/max row/col ------
// Time: O(k) where k = number of black pixels (bounded by m*n)
// Space: O(k) — visited set and recursion stack
/// DFS from the given black pixel, tracking min/max row and column
/// visited. The bounding rectangle area = (max_r - min_r + 1) * (max_c - min_c + 1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn min_area(image: Vec<Vec<char>>, x: i32, y: i32) -> i32 {
            let m = image.len();
            let n = image[0].len();
            let mut visited = vec![vec![false; n]; m];
            let mut min_r = x as usize;
            let mut max_r = x as usize;
            let mut min_c = y as usize;
            let mut max_c = y as usize;

            // Iterative DFS using a stack to avoid stack overflow
            let mut stack = vec![(x as usize, y as usize)];
            visited[x as usize][y as usize] = true;

            while let Some((r, c)) = stack.pop() {
                min_r = min_r.min(r);
                max_r = max_r.max(r);
                min_c = min_c.min(c);
                max_c = max_c.max(c);

                // Explore 4 neighbors
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !visited[nr][nc] && image[nr][nc] == '1' {
                            visited[nr][nc] = true;
                            stack.push((nr, nc));
                        }
                    }
                }
            }

            ((max_r - min_r + 1) * (max_c - min_c + 1)) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn make_grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn test_example1() {
        let image = make_grid(&["0010", "0110", "0100"]);
        assert_eq!(MySolution::min_area(image.clone(), 0, 2), 6);
        assert_eq!(AiSolution::min_area(image, 0, 2), 6);
    }

    #[test]
    fn test_single_pixel() {
        let image = make_grid(&["1"]);
        assert_eq!(MySolution::min_area(image.clone(), 0, 0), 1);
        assert_eq!(AiSolution::min_area(image, 0, 0), 1);
    }

    #[test]
    fn test_full_row() {
        let image = make_grid(&["000", "111", "000"]);
        assert_eq!(MySolution::min_area(image.clone(), 1, 1), 3);
        assert_eq!(AiSolution::min_area(image, 1, 1), 3);
    }

    #[test]
    fn test_full_col() {
        let image = make_grid(&["010", "010", "010"]);
        assert_eq!(MySolution::min_area(image.clone(), 1, 1), 3);
        assert_eq!(AiSolution::min_area(image, 1, 1), 3);
    }

    #[test]
    fn test_corner() {
        let image = make_grid(&["10", "00"]);
        assert_eq!(MySolution::min_area(image.clone(), 0, 0), 1);
        assert_eq!(AiSolution::min_area(image, 0, 0), 1);
    }

    #[test]
    fn test_l_shape() {
        let image = make_grid(&["100", "100", "110"]);
        assert_eq!(MySolution::min_area(image.clone(), 0, 0), 6);
        assert_eq!(AiSolution::min_area(image, 0, 0), 6);
    }

    #[test]
    fn test_diagonal_connected() {
        // Diagonal pixels: connected via shared neighbors
        let image = make_grid(&["110", "011"]);
        // Bounding box: rows 0-1, cols 0-2 = 2*3=6
        assert_eq!(MySolution::min_area(image.clone(), 0, 0), 6);
        assert_eq!(AiSolution::min_area(image, 0, 0), 6);
    }

    #[test]
    fn test_large_block() {
        let image = make_grid(&["0000", "0110", "0110", "0000"]);
        assert_eq!(MySolution::min_area(image.clone(), 1, 1), 4);
        assert_eq!(AiSolution::min_area(image, 1, 1), 4);
    }

    #[test]
    fn test_single_row_multiple_pixels() {
        let image = make_grid(&["01110"]);
        assert_eq!(MySolution::min_area(image.clone(), 0, 2), 3);
        assert_eq!(AiSolution::min_area(image, 0, 2), 3);
    }

    #[test]
    fn test_entire_grid_black() {
        let image = make_grid(&["111", "111", "111"]);
        assert_eq!(MySolution::min_area(image.clone(), 1, 1), 9);
        assert_eq!(AiSolution::min_area(image, 1, 1), 9);
    }
}

fn main() {
    println!("LeetCode 302: Smallest Rectangle Enclosing Black Pixels");

    let image: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0'],
        vec!['0', '1', '1', '0'],
        vec!['0', '1', '0', '0'],
    ];
    println!(
        "My: {}, AI: {}",
        my_solution::Solution::min_area(image.clone(), 0, 2),
        ai_solution::Solution::min_area(image, 0, 2)
    );

    // Benchmark
    let iterations = 100_000;
    let test_image: Vec<Vec<char>> = vec![
        vec!['0', '0', '1', '0'],
        vec!['0', '1', '1', '0'],
        vec!['0', '1', '0', '0'],
    ];

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::Solution::min_area(
            std::hint::black_box(test_image.clone()),
            0,
            2,
        ));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::Solution::min_area(
            std::hint::black_box(test_image.clone()),
            0,
            2,
        ));
    }
    let ai_time = start.elapsed();

    let my_avg = my_time.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_time.as_nanos() as f64 / iterations as f64 / 1000.0;

    println!("\nBenchmark ({iterations} iterations):");
    println!("  My solution:  {my_avg:.3}us avg");
    println!("  AI solution:  {ai_avg:.3}us avg");
    if my_avg < ai_avg {
        println!("  Winner: My solution ({:.2}x faster)", ai_avg / my_avg);
    } else {
        println!("  Winner: AI solution ({:.2}x faster)", my_avg / ai_avg);
    }
}
