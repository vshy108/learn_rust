// @lc app=leetcode id=378 lang=rust
// #[Medium] - Kth Smallest Element in a Sorted Matrix
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.064us
// ai_solution  | avg: 0.074us
// Performance: My 1.15x faster

/// # 378. Kth Smallest Element in a Sorted Matrix
///
/// Given an n x n matrix where rows and columns are sorted ascending,
/// return the kth smallest element.
///
/// Constraints:
/// - n == matrix.length == matrix[i].length
/// - 1 <= n <= 300
/// - -10^9 <= matrix[i][j] <= 10^9
/// - Rows and columns sorted non-decreasing
/// - 1 <= k <= n^2

/// ## My Solution — Binary search on value + staircase count O(n * log(max-min))
///
/// Binary search the answer value. For a given mid, count how many elements <= mid
/// using a staircase walk from bottom-left: O(n) per count.
///
/// - Time: O(n * log(max - min))
/// - Space: O(1)
mod my_solution {
    pub fn kth_smallest(matrix: &[Vec<i32>], k: i32) -> i32 {
        let n = matrix.len();
        let (mut lo, mut hi) = (matrix[0][0], matrix[n - 1][n - 1]);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if count_le(matrix, mid, n) >= k {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    /// Count elements <= mid using staircase walk from bottom-left.
    fn count_le(matrix: &[Vec<i32>], mid: i32, n: usize) -> i32 {
        let mut count = 0i32;
        let mut row = n as i32 - 1;
        let mut col = 0usize;
        while row >= 0 && col < n {
            if matrix[row as usize][col] <= mid {
                // Everything above (including this row) in this column is <= mid
                count += row + 1;
                col += 1;
            } else {
                row -= 1;
            }
        }
        count
    }
}

/// ## AI Solution — Min-heap merging first elements of each row O(k log n)
///
/// Push first column of each row into a min-heap. Pop k times,
/// pushing the next element in that row.
///
/// - Time: O(k * log n)
/// - Space: O(n)
mod ai_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    pub fn kth_smallest(matrix: &[Vec<i32>], k: i32) -> i32 {
        let n = matrix.len();
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::with_capacity(n);

        for i in 0..n {
            heap.push(Reverse((matrix[i][0], i, 0)));
        }

        let mut val = 0;
        for _ in 0..k {
            let Reverse((v, row, col)) = heap.pop().unwrap();
            val = v;
            if col + 1 < n {
                heap.push(Reverse((matrix[row][col + 1], row, col + 1)));
            }
        }
        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[Vec<i32>], i32) -> i32) {
        assert_eq!(
            f(&[vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
        assert_eq!(f(&[vec![-5]], 1), -5);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::kth_smallest);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::kth_smallest);
    }
}

fn main() {
    let matrix = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
    let k = 8;
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::kth_smallest(std::hint::black_box(&matrix), k));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::kth_smallest(std::hint::black_box(&matrix), k));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::kth_smallest(std::hint::black_box(&matrix), k));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::kth_smallest(std::hint::black_box(&matrix), k));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 378 - Kth Smallest Element in a Sorted Matrix");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
