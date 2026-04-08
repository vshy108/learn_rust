// LeetCode 519: Random Flip Matrix [Medium]
// https://leetcode.com/problems/random-flip-matrix/
//
// There is an m x n binary grid matrix with all values set to 0 initially.
// Design an algorithm to randomly pick an index (i, j) where matrix[i][j] == 0
// and flip it to 1. All indices where matrix[i][j] == 0 should be equally
// likely to be returned.
//
// Implement the Solution class:
//   Solution(int m, int n) — initialize with matrix dimensions
//   int[] flip()           — return random [i, j] and flip it to 1
//   void reset()           — reset all values to 0
//
// Examples:
//   Solution(3, 1) -> flip()=[1,0], flip()=[2,0], flip()=[0,0],
//                     reset(), flip()=[2,0]
//
// Constraints:
//   1 <= m, n <= 10^4
//   At least one free cell for each call to flip
//   At most 1000 calls to flip and reset
//
// Run:  cargo run --example lc_0519_random_flip_matrix
// Test: cargo test --example lc_0519_random_flip_matrix
// Bench: cargo test --example lc_0519_random_flip_matrix bench -- --nocapture
//
// First benchmark (10K iterations x 50 flips, 2026-04-08):
//   My solution: avg 3.31us
//   AI solution: avg 2.81us
//   AI is 1.18x faster
//   Reason: With only 50/10000 cells flipped, rejection sampling rarely
//   retries, avoiding Fisher-Yates HashMap overhead per flip.

use rand::Rng;
use std::collections::HashMap;

/// Fisher-Yates shuffle approach — map indices to avoid duplicates.
///
/// Adapted from doocs/leetcode Java solution. Maintain a virtual array of
/// remaining indices [0..total). On each flip, pick a random index in
/// [0..total), swap it with the last element using a HashMap, and shrink
/// total. Convert the 1D index to 2D coordinates.
///
/// Time:  O(1) per flip, O(1) per reset (amortized)
/// Space: O(k) where k is the number of flips
mod my_solution {
    use super::*;

    pub struct Solution {
        m: i32,
        n: i32,
        total: i32,
        map: HashMap<i32, i32>,
    }

    impl Solution {
        /// Create a new Solution with an m x n grid.
        ///
        /// # Arguments
        /// * `m` - number of rows
        /// * `n` - number of columns
        pub fn new(m: i32, n: i32) -> Self {
            Solution {
                m,
                n,
                total: m * n,
                map: HashMap::new(),
            }
        }

        /// Pick a random unflipped cell and flip it.
        ///
        /// # Returns
        /// [row, col] of the flipped cell
        pub fn flip(&mut self) -> Vec<i32> {
            self.total -= 1;
            let mut rng = rand::rng();
            let x = rng.random_range(0..=self.total);
            // Get the actual index (may have been swapped previously)
            let idx = *self.map.get(&x).unwrap_or(&x);
            // Swap position x with position total (the last available slot)
            let last = *self.map.get(&self.total).unwrap_or(&self.total);
            self.map.insert(x, last);
            vec![idx / self.n, idx % self.n]
        }

        /// Reset all cells back to 0.
        pub fn reset(&mut self) {
            self.total = self.m * self.n;
            self.map.clear();
        }
    }
}

/// HashSet-based rejection sampling approach.
///
/// Keep a HashSet of already-flipped 1D indices. On each flip, generate
/// random indices until we find one not in the set. Simple but slightly
/// slower when many cells are flipped.
///
/// Time:  O(1) expected per flip (when few cells flipped)
/// Space: O(k) where k is the number of flips
mod ai_solution {
    use super::*;
    use std::collections::HashSet;

    pub struct Solution {
        m: i32,
        n: i32,
        flipped: HashSet<i32>,
    }

    impl Solution {
        /// Create a new Solution with an m x n grid.
        ///
        /// # Arguments
        /// * `m` - number of rows
        /// * `n` - number of columns
        pub fn new(m: i32, n: i32) -> Self {
            Solution {
                m,
                n,
                flipped: HashSet::new(),
            }
        }

        /// Pick a random unflipped cell and flip it.
        ///
        /// # Returns
        /// [row, col] of the flipped cell
        pub fn flip(&mut self) -> Vec<i32> {
            let total = self.m * self.n;
            let mut rng = rand::rng();
            loop {
                let idx = rng.random_range(0..total);
                // If not already flipped, use this index
                if self.flipped.insert(idx) {
                    return vec![idx / self.n, idx % self.n];
                }
            }
        }

        /// Reset all cells back to 0.
        pub fn reset(&mut self) {
            self.flipped.clear();
        }
    }
}

fn main() {
    // Demonstrate my_solution
    let mut sol = my_solution::Solution::new(3, 1);
    println!("my  flip: {:?}", sol.flip());
    println!("my  flip: {:?}", sol.flip());
    println!("my  flip: {:?}", sol.flip());
    sol.reset();
    println!("my  reset, flip: {:?}", sol.flip());

    // Demonstrate ai_solution
    let mut sol2 = ai_solution::Solution::new(3, 1);
    println!("ai  flip: {:?}", sol2.flip());
    println!("ai  flip: {:?}", sol2.flip());
    println!("ai  flip: {:?}", sol2.flip());
    sol2.reset();
    println!("ai  reset, flip: {:?}", sol2.flip());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Validate that all flips produce unique, valid coordinates.
    fn validate_flips(flips: &[Vec<i32>], m: i32, n: i32) {
        let mut seen = HashSet::new();
        for f in flips {
            assert_eq!(f.len(), 2, "flip must return [row, col]");
            assert!(f[0] >= 0 && f[0] < m, "row out of bounds");
            assert!(f[1] >= 0 && f[1] < n, "col out of bounds");
            let key = f[0] * n + f[1];
            assert!(seen.insert(key), "duplicate flip at {:?}", f);
        }
    }

    #[test]
    fn test_my_solution_basic() {
        let mut sol = my_solution::Solution::new(3, 1);
        let mut flips = Vec::new();
        for _ in 0..3 {
            flips.push(sol.flip());
        }
        validate_flips(&flips, 3, 1);
    }

    #[test]
    fn test_ai_solution_basic() {
        let mut sol = ai_solution::Solution::new(3, 1);
        let mut flips = Vec::new();
        for _ in 0..3 {
            flips.push(sol.flip());
        }
        validate_flips(&flips, 3, 1);
    }

    #[test]
    fn test_my_solution_reset() {
        let mut sol = my_solution::Solution::new(2, 2);
        for _ in 0..4 {
            sol.flip();
        }
        sol.reset();
        // After reset, should be able to flip all 4 cells again
        let mut flips = Vec::new();
        for _ in 0..4 {
            flips.push(sol.flip());
        }
        validate_flips(&flips, 2, 2);
    }

    #[test]
    fn test_ai_solution_reset() {
        let mut sol = ai_solution::Solution::new(2, 2);
        for _ in 0..4 {
            sol.flip();
        }
        sol.reset();
        let mut flips = Vec::new();
        for _ in 0..4 {
            flips.push(sol.flip());
        }
        validate_flips(&flips, 2, 2);
    }

    #[test]
    fn test_my_solution_large() {
        let mut sol = my_solution::Solution::new(10, 10);
        let mut flips = Vec::new();
        for _ in 0..50 {
            flips.push(sol.flip());
        }
        validate_flips(&flips, 10, 10);
    }

    #[test]
    fn test_ai_solution_large() {
        let mut sol = ai_solution::Solution::new(10, 10);
        let mut flips = Vec::new();
        for _ in 0..50 {
            flips.push(sol.flip());
        }
        validate_flips(&flips, 10, 10);
    }

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;
        let flips_per_iter = 50;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut sol = my_solution::Solution::new(100, 100);
            for _ in 0..flips_per_iter {
                std::hint::black_box(sol.flip());
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut sol = ai_solution::Solution::new(100, 100);
            for _ in 0..flips_per_iter {
                std::hint::black_box(sol.flip());
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur / iterations;
        let ai_avg = ai_dur / iterations;
        println!("\n--- Benchmark ({iterations} iterations, {flips_per_iter} flips each) ---");
        println!("  My solution (Fisher-Yates): {my_avg:?} avg");
        println!("  AI solution (rejection):    {ai_avg:?} avg");
        if ai_avg < my_avg {
            println!(
                "  AI is {:.2}x faster",
                my_avg.as_nanos() as f64 / ai_avg.as_nanos() as f64
            );
        } else {
            println!(
                "  My is {:.2}x faster",
                ai_avg.as_nanos() as f64 / my_avg.as_nanos() as f64
            );
        }
    }
}
