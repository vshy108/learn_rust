// LeetCode 304: Range Sum Query 2D - Immutable [Medium]
// https://leetcode.com/problems/range-sum-query-2d-immutable/
// Given a 2D matrix, handle multiple queries for the sum of elements inside a rectangle
// defined by upper-left (row1, col1) and lower-right (row2, col2).
//
// Benchmark: 100_000 iterations
// My solution (2D prefix sum (m+1)x(n+1)): avg 0.789us
// AI solution (row-wise prefix sums):       avg 0.710us
// Winner: AI solution (1.11x faster)

// ------ My Solution: 2D Prefix Sum ------
// Time: O(m*n) init, O(1) query
// Space: O(m*n) — prefix sum matrix
/// Builds 2D prefix sum `s[i+1][j+1] = s[i+1][j] + s[i][j+1] - s[i][j] + matrix[i][j]`.
/// Query: inclusion-exclusion on the four corners.
mod my_solution {
    pub struct NumMatrix {
        s: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        pub fn new(matrix: Vec<Vec<i32>>) -> Self {
            let m = matrix.len();
            let n = matrix[0].len();
            let mut s = vec![vec![0i32; n + 1]; m + 1];
            for i in 0..m {
                for j in 0..n {
                    s[i + 1][j + 1] =
                        s[i + 1][j] + s[i][j + 1] - s[i][j] + matrix[i][j];
                }
            }
            Self { s }
        }

        pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            let (r1, c1) = (row1 as usize, col1 as usize);
            let (r2, c2) = (row2 as usize, col2 as usize);
            // Inclusion-exclusion on the 2D prefix sum
            self.s[r2 + 1][c2 + 1] - self.s[r2 + 1][c1] - self.s[r1][c2 + 1] + self.s[r1][c1]
        }
    }
}

// ------ AI Solution: Row-wise Prefix Sums ------
// Time: O(m*n) init, O(m) query — sum across rows in the column range
// Space: O(m*n) — row prefix sums
/// Stores prefix sums for each row individually. To query a rectangle,
/// iterate over the rows in [row1, row2] and sum up each row's range.
mod ai_solution {
    pub struct NumMatrix {
        row_prefix: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        pub fn new(matrix: Vec<Vec<i32>>) -> Self {
            let m = matrix.len();
            let n = matrix[0].len();
            let mut row_prefix = vec![vec![0i32; n + 1]; m];
            for i in 0..m {
                for j in 0..n {
                    row_prefix[i][j + 1] = row_prefix[i][j] + matrix[i][j];
                }
            }
            Self { row_prefix }
        }

        pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            let (r1, c1) = (row1 as usize, col1 as usize);
            let (r2, c2) = (row2 as usize, col2 as usize);
            // Sum each row's contribution in O(1) per row, O(m) total
            (r1..=r2)
                .map(|r| self.row_prefix[r][c2 + 1] - self.row_prefix[r][c1])
                .sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::NumMatrix as AiMatrix;
    use super::my_solution::NumMatrix as MyMatrix;

    fn matrix() -> Vec<Vec<i32>> {
        vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]
    }

    #[test]
    fn test_example_query1() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(2, 1, 4, 3), 8);
        assert_eq!(ai.sum_region(2, 1, 4, 3), 8);
    }

    #[test]
    fn test_example_query2() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(1, 1, 2, 2), 11);
        assert_eq!(ai.sum_region(1, 1, 2, 2), 11);
    }

    #[test]
    fn test_example_query3() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(1, 2, 2, 4), 12);
        assert_eq!(ai.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn test_single_cell() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(0, 0, 0, 0), 3);
        assert_eq!(ai.sum_region(0, 0, 0, 0), 3);
    }

    #[test]
    fn test_full_matrix() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        // Sum of all: 3+0+1+4+2+5+6+3+2+1+1+2+0+1+5+4+1+0+1+7+1+0+3+0+5 = 58
        assert_eq!(my.sum_region(0, 0, 4, 4), 58);
        assert_eq!(ai.sum_region(0, 0, 4, 4), 58);
    }

    #[test]
    fn test_single_row() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(0, 0, 0, 4), 10);
        assert_eq!(ai.sum_region(0, 0, 0, 4), 10);
    }

    #[test]
    fn test_single_col() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        // col 0: 3+5+1+4+1 = 14
        assert_eq!(my.sum_region(0, 0, 4, 0), 14);
        assert_eq!(ai.sum_region(0, 0, 4, 0), 14);
    }

    #[test]
    fn test_1x1_matrix() {
        let my = MyMatrix::new(vec![vec![42]]);
        let ai = AiMatrix::new(vec![vec![42]]);
        assert_eq!(my.sum_region(0, 0, 0, 0), 42);
        assert_eq!(ai.sum_region(0, 0, 0, 0), 42);
    }

    #[test]
    fn test_negative_values() {
        let m = vec![vec![-1, -2], vec![-3, -4]];
        let my = MyMatrix::new(m.clone());
        let ai = AiMatrix::new(m);
        assert_eq!(my.sum_region(0, 0, 1, 1), -10);
        assert_eq!(ai.sum_region(0, 0, 1, 1), -10);
    }
}

fn main() {
    println!("LeetCode 304: Range Sum Query 2D - Immutable");

    let m = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];
    let my = my_solution::NumMatrix::new(m.clone());
    let ai = ai_solution::NumMatrix::new(m.clone());
    println!("sumRegion(2,1,4,3): My={}, AI={}", my.sum_region(2, 1, 4, 3), ai.sum_region(2, 1, 4, 3));

    // Benchmark
    let iterations = 100_000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let obj = my_solution::NumMatrix::new(std::hint::black_box(m.clone()));
        std::hint::black_box(obj.sum_region(2, 1, 4, 3));
        std::hint::black_box(obj.sum_region(1, 1, 2, 2));
        std::hint::black_box(obj.sum_region(1, 2, 2, 4));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let obj = ai_solution::NumMatrix::new(std::hint::black_box(m.clone()));
        std::hint::black_box(obj.sum_region(2, 1, 4, 3));
        std::hint::black_box(obj.sum_region(1, 1, 2, 2));
        std::hint::black_box(obj.sum_region(1, 2, 2, 4));
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
