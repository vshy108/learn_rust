// LeetCode 461: Hamming Distance [Easy]
// https://leetcode.com/problems/hamming-distance/
//
// The Hamming distance between two integers is the number of positions at
// which the corresponding bits are different. Given two integers x and y,
// return the Hamming distance between them.
//
// Examples:
//   x = 1, y = 4 -> 2  (001 vs 100, bits differ at positions 0 and 2)
//   x = 3, y = 1 -> 1
//
// Constraints:
//   0 <= x, y <= 2^31 - 1
//
// Run:  cargo run --example lc_0461_hamming_distance
// Test: cargo test --example lc_0461_hamming_distance
// Bench: cargo test --example lc_0461_hamming_distance bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg ~0ns (Brian Kernighan)
//   AI solution: avg ~0ns (count_ones)
//   Ratio: ~1.0x (both too fast to measure)

/// My solution module — XOR + count_ones.
///
/// Time:  O(1) -- fixed 32-bit integers
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn hamming_distance(x: i32, y: i32) -> i32 {
            // XOR + Brian Kernighan: count bits by clearing lowest set bit
            let mut xor = x ^ y;
            let mut count = 0;
            while xor != 0 {
                xor &= xor - 1; // clear lowest set bit
                count += 1;
            }
            count
        }
    }
}

/// AI solution module — XOR + count_ones (Rust built-in).
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn hamming_distance(x: i32, y: i32) -> i32 {
            (x ^ y).count_ones() as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_1_4() {
            assert_eq!(Solution::hamming_distance(1, 4), 2);
        }
        #[test]
        fn test_3_1() {
            assert_eq!(Solution::hamming_distance(3, 1), 1);
        }
        #[test]
        fn test_same() {
            assert_eq!(Solution::hamming_distance(5, 5), 0);
        }
        #[test]
        fn test_zero() {
            assert_eq!(Solution::hamming_distance(0, 0), 0);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(i32, i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(1), std::hint::black_box(4)));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::hamming_distance);
    bench_solution("AI solution", ai_solution::Solution::hamming_distance);
}

fn main() {
    println!("{}", ai_solution::Solution::hamming_distance(1, 4));
}
