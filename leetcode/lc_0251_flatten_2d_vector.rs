// LeetCode 251: Flatten 2D Vector [Medium] (Premium)
// https://leetcode.com/problems/flatten-2d-vector/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.292us
//   AI solution avg: 0.433us
//   Ratio: My 1.48x faster
//
// Design an iterator to flatten a 2D vector.
//
// Implement Vector2D:
//   - Vector2D(vec) initializes with the 2D vector
//   - next() returns the next element, moves pointer forward
//   - has_next() returns true if elements remain
//
// Example:
//   Vector2D([[1,2],[3],[4]])
//   next() -> 1, next() -> 2, next() -> 3
//   has_next() -> true, next() -> 4, has_next() -> false
//
// Constraints:
//   - 0 <= vec.length <= 200
//   - 0 <= vec[i].length <= 500
//   - -500 <= vec[i][j] <= 500
//   - At most 10^5 calls to next and has_next
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0251.Flatten%202D%20Vector/README_EN.md
//
// Run:  cargo run --example lc_0251_flatten_2d_vector
// Test: cargo test --example lc_0251_flatten_2d_vector
// Bench: cargo test --example lc_0251_flatten_2d_vector bench -- --nocapture

/// My solution — two-index iterator with forward() (translated from Python in doocs).
///
/// Track row index `i` and column index `j`. The `forward()` helper
/// skips over empty inner vectors by advancing `i` until a non-empty
/// row is found (or we exhaust all rows).
///
/// Time: next O(1) amortized, has_next O(1) amortized
/// Space: O(1) extra (stores reference to original vec)
mod my_solution {
    pub struct Vector2D {
        i: usize,
        j: usize,
        vec: Vec<Vec<i32>>,
    }

    impl Vector2D {
        /// Initialize with the 2D vector.
        pub fn new(vec: Vec<Vec<i32>>) -> Self {
            Self { i: 0, j: 0, vec }
        }

        /// Return the next element.
        pub fn next(&mut self) -> i32 {
            self.forward();
            let val = self.vec[self.i][self.j];
            self.j += 1;
            val
        }

        /// Return true if there are more elements.
        pub fn has_next(&mut self) -> bool {
            self.forward();
            self.i < self.vec.len()
        }

        /// Skip empty rows to position at the next valid element.
        fn forward(&mut self) {
            while self.i < self.vec.len() && self.j >= self.vec[self.i].len() {
                self.i += 1;
                self.j = 0;
            }
        }
    }
}

/// AI solution — pre-flatten into a single Vec with an index.
///
/// Flatten the entire 2D vector into a 1D Vec at construction time.
/// next() and has_next() are trivial O(1) lookups.
/// Trades O(n) upfront space for simpler iteration.
///
/// Time: constructor O(n), next O(1), has_next O(1)
/// Space: O(n) — stores flattened copy
mod ai_solution {
    pub struct Vector2D {
        flat: Vec<i32>,
        idx: usize,
    }

    impl Vector2D {
        /// Initialize by flattening the 2D vector.
        pub fn new(vec: Vec<Vec<i32>>) -> Self {
            // Flatten all inner vectors into one contiguous Vec
            let flat: Vec<i32> = vec.into_iter().flatten().collect();
            Self { flat, idx: 0 }
        }

        /// Return the next element.
        pub fn next(&mut self) -> i32 {
            let val = self.flat[self.idx];
            self.idx += 1;
            val
        }

        /// Return true if there are more elements.
        pub fn has_next(&mut self) -> bool {
            self.idx < self.flat.len()
        }
    }
}

fn main() {
    let mut v = my_solution::Vector2D::new(vec![vec![1, 2], vec![3], vec![4]]);
    while v.has_next() {
        print!("{} ", v.next());
    }
    println!();

    let mut v = ai_solution::Vector2D::new(vec![vec![1, 2], vec![3], vec![4]]);
    while v.has_next() {
        print!("{} ", v.next());
    }
    println!();
}

#[cfg(test)]
mod tests {
    macro_rules! test_vector2d {
        ($mod_name:ident, $solution:ident) => {
            mod $mod_name {
                use crate::$solution::Vector2D;

                #[test]
                fn test_example() {
                    let mut v = Vector2D::new(vec![vec![1, 2], vec![3], vec![4]]);
                    assert_eq!(v.next(), 1);
                    assert_eq!(v.next(), 2);
                    assert_eq!(v.next(), 3);
                    assert!(v.has_next());
                    assert!(v.has_next());
                    assert_eq!(v.next(), 4);
                    assert!(!v.has_next());
                }

                #[test]
                fn test_empty_inner() {
                    let mut v = Vector2D::new(vec![vec![], vec![1], vec![], vec![2], vec![]]);
                    assert!(v.has_next());
                    assert_eq!(v.next(), 1);
                    assert_eq!(v.next(), 2);
                    assert!(!v.has_next());
                }

                #[test]
                fn test_all_empty() {
                    let mut v = Vector2D::new(vec![vec![], vec![], vec![]]);
                    assert!(!v.has_next());
                }

                #[test]
                fn test_empty_outer() {
                    let mut v = Vector2D::new(vec![]);
                    assert!(!v.has_next());
                }

                #[test]
                fn test_single_element() {
                    let mut v = Vector2D::new(vec![vec![42]]);
                    assert!(v.has_next());
                    assert_eq!(v.next(), 42);
                    assert!(!v.has_next());
                }

                #[test]
                fn test_multiple_has_next() {
                    let mut v = Vector2D::new(vec![vec![1]]);
                    assert!(v.has_next());
                    assert!(v.has_next());
                    assert!(v.has_next());
                    assert_eq!(v.next(), 1);
                    assert!(!v.has_next());
                }
            }
        };
    }

    test_vector2d!(my, my_solution);
    test_vector2d!(ai, ai_solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let data = vec![
            vec![1, 2, 3],
            vec![],
            vec![4, 5],
            vec![6],
            vec![],
            vec![7, 8, 9, 10],
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut v = crate::my_solution::Vector2D::new(data.clone());
            while v.has_next() {
                std::hint::black_box(v.next());
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut v = crate::ai_solution::Vector2D::new(data.clone());
            while v.has_next() {
                std::hint::black_box(v.next());
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
