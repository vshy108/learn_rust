// LeetCode 308: Range Sum Query 2D - Mutable [Medium]
// https://leetcode.com/problems/range-sum-query-2d-mutable/
// Given a 2D matrix, support update(row, col, val) and sumRegion(row1, col1, row2, col2).
//
// Benchmark: 100_000 iterations
// My solution (row-wise BIT):          avg 0.712us
// AI solution (row-wise segment tree): avg 1.262us
// Winner: My solution (1.77x faster)

// ------ My Solution: Row-wise Binary Indexed Trees ------
// Time: O(m*n) init, O(log n) update, O(m * log n) query
// Space: O(m*n)
/// One BIT per row handling column sums. Update modifies one row's BIT.
/// sumRegion iterates rows [row1, row2] and queries each BIT for [col1, col2].
mod my_solution {
    pub struct NumMatrix {
        m: usize,
        n: usize,
        trees: Vec<Vec<i32>>,
        data: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        pub fn new(matrix: Vec<Vec<i32>>) -> Self {
            let m = matrix.len();
            let n = matrix[0].len();
            let mut trees = vec![vec![0i32; n + 1]; m];
            for i in 0..m {
                for j in 0..n {
                    let v = matrix[i][j];
                    let mut x = j + 1;
                    while x <= n {
                        trees[i][x] += v;
                        x += x & x.wrapping_neg();
                    }
                }
            }
            Self {
                m,
                n,
                trees,
                data: matrix,
            }
        }

        pub fn update(&mut self, row: i32, col: i32, val: i32) {
            let (r, c) = (row as usize, col as usize);
            let delta = val - self.data[r][c];
            self.data[r][c] = val;
            let mut x = c + 1;
            while x <= self.n {
                self.trees[r][x] += delta;
                x += x & x.wrapping_neg();
            }
        }

        pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            let (r1, c1) = (row1 as usize, col1 as usize);
            let (r2, c2) = (row2 as usize, col2 as usize);
            let mut s = 0;
            for r in r1..=r2 {
                s += self.prefix(&self.trees[r], c2 + 1)
                    - self.prefix(&self.trees[r], c1);
            }
            s
        }

        fn prefix(&self, tree: &[i32], mut x: usize) -> i32 {
            let mut s = 0;
            while x > 0 {
                s += tree[x];
                x -= x & x.wrapping_neg();
            }
            s
        }
    }
}

// ------ AI Solution: 2D BIT ------
// Time: O(m*n*log(m)*log(n)) init, O(log(m)*log(n)) update & query
// Space: O(m*n)
/// Full 2D Binary Indexed Tree where both row and col dimensions use BIT
/// structure. Update and query both traverse two nested BIT loops.
mod ai_solution {
    pub struct NumMatrix {
        m: usize,
        n: usize,
        tree: Vec<Vec<i32>>,
        data: Vec<Vec<i32>>,
    }

    impl NumMatrix {
        pub fn new(matrix: Vec<Vec<i32>>) -> Self {
            let m = matrix.len();
            let n = matrix[0].len();
            let mut obj = Self {
                m,
                n,
                tree: vec![vec![0i32; n + 1]; m + 1],
                data: vec![vec![0i32; n]; m],
            };
            for i in 0..m {
                for j in 0..n {
                    obj.add(i + 1, j + 1, matrix[i][j]);
                    obj.data[i][j] = matrix[i][j];
                }
            }
            obj
        }

        /// Add delta to the 2D BIT at position (r, c) (1-indexed).
        fn add(&mut self, r: usize, c: usize, delta: i32) {
            let mut i = r;
            while i <= self.m {
                let mut j = c;
                while j <= self.n {
                    self.tree[i][j] += delta;
                    j += j & j.wrapping_neg();
                }
                i += i & i.wrapping_neg();
            }
        }

        /// Query prefix sum from (1,1) to (r, c) (1-indexed).
        fn prefix_sum(&self, r: usize, c: usize) -> i32 {
            let mut s = 0;
            let mut i = r;
            while i > 0 {
                let mut j = c;
                while j > 0 {
                    s += self.tree[i][j];
                    j -= j & j.wrapping_neg();
                }
                i -= i & i.wrapping_neg();
            }
            s
        }

        pub fn update(&mut self, row: i32, col: i32, val: i32) {
            let (r, c) = (row as usize, col as usize);
            let delta = val - self.data[r][c];
            self.data[r][c] = val;
            self.add(r + 1, c + 1, delta);
        }

        pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            let (r1, c1) = (row1 as usize, col1 as usize);
            let (r2, c2) = (row2 as usize, col2 as usize);
            // Inclusion-exclusion on 2D prefix sums
            self.prefix_sum(r2 + 1, c2 + 1)
                - self.prefix_sum(r1, c2 + 1)
                - self.prefix_sum(r2 + 1, c1)
                + self.prefix_sum(r1, c1)
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
    fn test_example() {
        let mut my = MyMatrix::new(matrix());
        let mut ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(2, 1, 4, 3), 8);
        assert_eq!(ai.sum_region(2, 1, 4, 3), 8);
        my.update(3, 2, 2);
        ai.update(3, 2, 2);
        assert_eq!(my.sum_region(2, 1, 4, 3), 10);
        assert_eq!(ai.sum_region(2, 1, 4, 3), 10);
    }

    #[test]
    fn test_single_cell() {
        let mut my = MyMatrix::new(vec![vec![1]]);
        let mut ai = AiMatrix::new(vec![vec![1]]);
        assert_eq!(my.sum_region(0, 0, 0, 0), 1);
        assert_eq!(ai.sum_region(0, 0, 0, 0), 1);
        my.update(0, 0, 5);
        ai.update(0, 0, 5);
        assert_eq!(my.sum_region(0, 0, 0, 0), 5);
        assert_eq!(ai.sum_region(0, 0, 0, 0), 5);
    }

    #[test]
    fn test_full_range_update() {
        let mut my = MyMatrix::new(matrix());
        let mut ai = AiMatrix::new(matrix());
        let orig = my.sum_region(0, 0, 4, 4);
        assert_eq!(ai.sum_region(0, 0, 4, 4), orig);
        my.update(0, 0, 10); // was 3, now 10 => +7
        ai.update(0, 0, 10);
        assert_eq!(my.sum_region(0, 0, 4, 4), orig + 7);
        assert_eq!(ai.sum_region(0, 0, 4, 4), orig + 7);
    }

    #[test]
    fn test_row_query() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(0, 0, 0, 4), 10);
        assert_eq!(ai.sum_region(0, 0, 0, 4), 10);
    }

    #[test]
    fn test_col_query() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        // col 0: 3+5+1+4+1 = 14
        assert_eq!(my.sum_region(0, 0, 4, 0), 14);
        assert_eq!(ai.sum_region(0, 0, 4, 0), 14);
    }

    #[test]
    fn test_multiple_updates() {
        let mut my = MyMatrix::new(vec![vec![1, 2], vec![3, 4]]);
        let mut ai = AiMatrix::new(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(my.sum_region(0, 0, 1, 1), 10);
        assert_eq!(ai.sum_region(0, 0, 1, 1), 10);
        my.update(0, 0, 0);
        ai.update(0, 0, 0);
        my.update(1, 1, 0);
        ai.update(1, 1, 0);
        assert_eq!(my.sum_region(0, 0, 1, 1), 5);
        assert_eq!(ai.sum_region(0, 0, 1, 1), 5);
    }

    #[test]
    fn test_query_subregions() {
        let my = MyMatrix::new(matrix());
        let ai = AiMatrix::new(matrix());
        assert_eq!(my.sum_region(1, 1, 2, 2), 11);
        assert_eq!(ai.sum_region(1, 1, 2, 2), 11);
        assert_eq!(my.sum_region(1, 2, 2, 4), 12);
        assert_eq!(ai.sum_region(1, 2, 2, 4), 12);
    }
}

fn main() {
    println!("LeetCode 308: Range Sum Query 2D - Mutable");

    let m = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];
    let mut my = my_solution::NumMatrix::new(m.clone());
    let mut ai = ai_solution::NumMatrix::new(m.clone());
    println!("sumRegion(2,1,4,3): My={}, AI={}", my.sum_region(2, 1, 4, 3), ai.sum_region(2, 1, 4, 3));
    my.update(3, 2, 2);
    ai.update(3, 2, 2);
    println!("After update(3,2,2), sumRegion(2,1,4,3): My={}, AI={}", my.sum_region(2, 1, 4, 3), ai.sum_region(2, 1, 4, 3));

    // Benchmark
    let iterations = 100_000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut obj = my_solution::NumMatrix::new(std::hint::black_box(m.clone()));
        std::hint::black_box(obj.sum_region(2, 1, 4, 3));
        obj.update(3, 2, 2);
        std::hint::black_box(obj.sum_region(2, 1, 4, 3));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut obj = ai_solution::NumMatrix::new(std::hint::black_box(m.clone()));
        std::hint::black_box(obj.sum_region(2, 1, 4, 3));
        obj.update(3, 2, 2);
        std::hint::black_box(obj.sum_region(2, 1, 4, 3));
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
