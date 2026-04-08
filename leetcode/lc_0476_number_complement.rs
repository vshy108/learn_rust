// LeetCode 476: Number Complement [Easy]
// https://leetcode.com/problems/number-complement/
//
// The complement of an integer is the integer you get when you flip all the 0's
// to 1's and all the 1's to 0's in its binary representation. Given an integer
// num, return its complement. (Ignore leading zeros.)
//
// Examples:
//   5  (101) -> 2 (010)
//   1  (1)   -> 0 (0)
//
// Constraints:
//   1 <= num < 2^31
//
// Run:  cargo run --example lc_0476_number_complement
// Test: cargo test --example lc_0476_number_complement
// Bench: cargo test --example lc_0476_number_complement bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 1ns   (iterative bit flip)
//   AI solution: avg ~0ns  (XOR mask)
//   Ratio: XOR mask slightly faster

/// My solution module — bit flip with mask.
///
/// Time:  O(1)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_complement(num: i32) -> i32 {
            // Iteratively build complement bit by bit
            let mut result = 0;
            let mut bit = 0;
            let mut n = num;
            while n > 0 {
                // Flip the lowest bit of n and place it at bit position
                if n & 1 == 0 {
                    result |= 1 << bit;
                }
                n >>= 1;
                bit += 1;
            }
            result
        }
    }
}

/// AI solution module — create a mask of all 1's with same bit-length, then XOR.
///
/// For num=5 (101), bit_length=3, mask=111=7, 5^7=2 (010).
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_complement(num: i32) -> i32 {
            // Number of bits needed to represent num
            let bits = 32 - (num as u32).leading_zeros();
            // Mask with 'bits' ones: (1 << bits) - 1
            let mask = (1u32 << bits) - 1;
            (num as u32 ^ mask) as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_5() {
            assert_eq!(Solution::find_complement(5), 2);
        }
        #[test]
        fn test_1() {
            assert_eq!(Solution::find_complement(1), 0);
        }
        #[test]
        fn test_7() {
            assert_eq!(Solution::find_complement(7), 0);
        }
        #[test]
        fn test_2() {
            assert_eq!(Solution::find_complement(2), 1);
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
    F: Fn(i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(5)));
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
    bench_solution("My solution", my_solution::Solution::find_complement);
    bench_solution("AI solution", ai_solution::Solution::find_complement);
}

fn main() {
    println!("{}", ai_solution::Solution::find_complement(5));
}
