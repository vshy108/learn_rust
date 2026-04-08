// LeetCode 477: Total Hamming Distance [Medium]
// https://leetcode.com/problems/total-hamming-distance/
//
// The Hamming distance between two integers is the number of positions at
// which the corresponding bits are different. Given an integer array nums,
// return the sum of Hamming distances between all pairs of integers.
//
// Examples:
//   [4, 14, 2] -> 6  (HD(4,14)=2, HD(4,2)=2, HD(14,2)=2)
//   [4, 14, 4] -> 4
//
// Constraints:
//   1 <= nums.len() <= 10^4
//   0 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0477_total_hamming_distance
// Test: cargo test --example lc_0477_total_hamming_distance
// Bench: cargo test --example lc_0477_total_hamming_distance bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 97ns
//   AI solution: avg 96ns
//   Ratio: 1.01x (same algorithm)

/// My solution module — bit-position counting.
///
/// Time:  O(n * 32) = O(n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
            let n = nums.len() as i32;
            let mut total = 0;
            for bit in 0..30 {
                let ones = nums.iter().filter(|&&x| (x >> bit) & 1 == 1).count() as i32;
                total += ones * (n - ones);
            }
            total
        }
    }
}

/// AI solution module — for each bit position, count how many numbers have
/// that bit set (ones) vs unset (zeros). The contribution of that bit to
/// the total is ones * zeros.
///
/// Time:  O(32 * n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
            let n = nums.len() as i32;
            let mut total = 0i32;
            for bit in 0..30 {
                let ones = nums.iter().filter(|&&x| (x >> bit) & 1 == 1).count() as i32;
                let zeros = n - ones;
                // Each pair with different bits contributes 1 to the Hamming distance
                total += ones * zeros;
            }
            total
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            assert_eq!(Solution::total_hamming_distance(vec![4, 14, 2]), 6);
        }
        #[test]
        fn test_example2() {
            assert_eq!(Solution::total_hamming_distance(vec![4, 14, 4]), 4);
        }
        #[test]
        fn test_single() {
            assert_eq!(Solution::total_hamming_distance(vec![5]), 0);
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
    F: Fn(Vec<i32>) -> i32,
{
    let iterations = 500_000;
    let input = vec![4, 14, 2, 8, 16, 32, 64, 128, 255, 1023];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(input.clone())));
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
    bench_solution("My solution", my_solution::Solution::total_hamming_distance);
    bench_solution("AI solution", ai_solution::Solution::total_hamming_distance);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::total_hamming_distance(vec![4, 14, 2])
    );
}
