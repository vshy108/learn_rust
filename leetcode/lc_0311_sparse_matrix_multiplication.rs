// LeetCode 311. Sparse Matrix Multiplication (Premium)
// https://leetcode.com/problems/sparse-matrix-multiplication/
//
// Benchmark results (100_000 iterations, release mode, 2025-01-XX):
//   my_solution  avg: 0.523us
//   ai_solution  avg: 0.853us
//   Ratio: my_solution 1.63x faster

/// Given two sparse matrices mat1 (m x k) and mat2 (k x n), return mat1 x mat2.
///
/// Example 1:
///   Input: mat1 = [[1,0,0],[-1,0,3]], mat2 = [[7,0,0],[0,0,0],[0,0,1]]
///   Output: [[7,0,0],[-7,0,3]]
///
/// Example 2:
///   Input: mat1 = [[0]], mat2 = [[0]]
///   Output: [[0]]
///
/// Constraints:
///   - m == mat1.length, k == mat1[i].length == mat2.length, n == mat2[i].length
///   - 1 <= m, n, k <= 100
///   - -100 <= mat1[i][j], mat2[i][j] <= 100

/// # my_solution: Direct with sparsity skip
/// Time: O(m * k * n) worst case, much better for sparse matrices
/// Space: O(m * n) for the result matrix
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Multiply two sparse matrices using direct approach with zero-skip.
        ///
        /// # Arguments
        /// * `mat1` - m x k matrix
        /// * `mat2` - k x n matrix
        ///
        /// # Returns
        /// The m x n product matrix
        pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let m = mat1.len();
            let k = mat1[0].len();
            let n = mat2[0].len();
            let mut ans = vec![vec![0i32; n]; m];

            // For each row i of mat1, skip zero entries in mat1[i][p]
            // before iterating over columns j of mat2
            for i in 0..m {
                for p in 0..k {
                    // Key optimization: skip if mat1 entry is zero
                    if mat1[i][p] == 0 {
                        continue;
                    }
                    for j in 0..n {
                        ans[i][j] += mat1[i][p] * mat2[p][j];
                    }
                }
            }

            ans
        }
    }
}

/// # ai_solution: Sparse representation preprocessing
/// Time: O(m * k * n) worst case, but only iterates non-zero entries
/// Space: O(m * n + nnz) where nnz = total non-zero elements in both matrices
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Multiply two sparse matrices by preprocessing non-zero elements.
        ///
        /// We store each row as a list of (column_index, value) for non-zero
        /// entries only, then only multiply non-zero pairs.
        ///
        /// # Arguments
        /// * `mat1` - m x k matrix
        /// * `mat2` - k x n matrix
        ///
        /// # Returns
        /// The m x n product matrix
        pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let m = mat1.len();
            let n = mat2[0].len();

            // Build sparse representation: g[i] = vec of (col, val) for non-zero entries
            let g1 = sparse_rows(&mat1);
            let g2 = sparse_rows(&mat2);

            let mut ans = vec![vec![0i32; n]; m];

            // For each row i of mat1, iterate only its non-zero entries (k, x),
            // then for each non-zero entry (j, y) in row k of mat2:
            //   ans[i][j] += x * y
            for i in 0..m {
                for &(k, x) in &g1[i] {
                    for &(j, y) in &g2[k] {
                        ans[i][j] += x * y;
                    }
                }
            }

            ans
        }
    }

    /// Convert a dense matrix into sparse row representation.
    ///
    /// # Arguments
    /// * `mat` - Dense matrix
    ///
    /// # Returns
    /// A vec where each element is a vec of (col_index, value) tuples
    /// for non-zero entries in that row
    fn sparse_rows(mat: &[Vec<i32>]) -> Vec<Vec<(usize, i32)>> {
        mat.iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &v)| v != 0)
                    .map(|(j, &v)| (j, v))
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn run_tests(multiply: fn(Vec<Vec<i32>>, Vec<Vec<i32>>) -> Vec<Vec<i32>>) {
        // Example 1: basic sparse multiply
        assert_eq!(
            multiply(
                vec![vec![1, 0, 0], vec![-1, 0, 3]],
                vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]],
            ),
            vec![vec![7, 0, 0], vec![-7, 0, 3]]
        );

        // Example 2: all zeros
        assert_eq!(multiply(vec![vec![0]], vec![vec![0]]), vec![vec![0]]);

        // Single element non-zero
        assert_eq!(multiply(vec![vec![3]], vec![vec![5]]), vec![vec![15]]);

        // Identity-like: diagonal mat2
        assert_eq!(
            multiply(vec![vec![1, 2], vec![3, 4]], vec![vec![1, 0], vec![0, 1]],),
            vec![vec![1, 2], vec![3, 4]]
        );

        // Fully dense small case
        assert_eq!(
            multiply(vec![vec![1, 2, 3]], vec![vec![4], vec![5], vec![6]],),
            vec![vec![32]] // 1*4 + 2*5 + 3*6 = 32
        );

        // All zeros in mat1
        assert_eq!(
            multiply(vec![vec![0, 0], vec![0, 0]], vec![vec![1, 2], vec![3, 4]],),
            vec![vec![0, 0], vec![0, 0]]
        );

        // Sparse with negatives
        assert_eq!(
            multiply(vec![vec![0, -1], vec![2, 0]], vec![vec![0, 3], vec![-4, 0]],),
            vec![vec![4, 0], vec![0, 6]]
        );
    }

    #[test]
    fn test_my_solution() {
        run_tests(MySolution::multiply);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(AiSolution::multiply);
    }
}

fn main() {
    use std::hint::black_box;
    use std::time::Instant;

    let mat1 = vec![vec![1, 0, 0], vec![-1, 0, 3]];
    let mat2 = vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];

    println!("Input: mat1 = {:?}", mat1);
    println!("       mat2 = {:?}", mat2);
    println!(
        "Output (my):  {:?}",
        my_solution::Solution::multiply(mat1.clone(), mat2.clone())
    );
    println!(
        "Output (ai):  {:?}",
        ai_solution::Solution::multiply(mat1.clone(), mat2.clone())
    );

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(my_solution::Solution::multiply(
            black_box(mat1.clone()),
            black_box(mat2.clone()),
        ));
    }
    let my_dur = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        black_box(ai_solution::Solution::multiply(
            black_box(mat1.clone()),
            black_box(mat2.clone()),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    println!("\nBenchmark ({iterations} iterations, release mode):");
    println!("  my_solution  avg: {my_avg:.3}us");
    println!("  ai_solution  avg: {ai_avg:.3}us");
    if my_avg < ai_avg {
        println!("  my_solution is {:.2}x faster", ai_avg / my_avg);
    } else {
        println!("  ai_solution is {:.2}x faster", my_avg / ai_avg);
    }
}
