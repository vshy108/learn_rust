// LeetCode 458: Poor Pigs [Hard]
// https://leetcode.com/problems/poor-pigs/
//
// There are `buckets` buckets of liquid, exactly one is poisonous.
// You have `minutes_to_test` minutes and pigs die after `minutes_to_die` minutes.
// Return the minimum number of pigs needed to determine which bucket is poisonous.
//
// Key insight: each pig can test (minutes_to_test / minutes_to_die + 1) outcomes
// (it can die in round 1, 2, ..., or survive all rounds). With p pigs,
// you can distinguish base^p buckets where base = rounds + 1.
//
// Examples:
//   buckets=4, die=15, test=15 -> 2 (base=2, 2^2=4)
//   buckets=4, die=15, test=30 -> 2 (base=3, but still need 2)
//
// Constraints:
//   1 <= buckets <= 1000
//   1 <= minutesToDie <= minutesToTest <= 100
//
// Run:  cargo run --example lc_0458_poor_pigs
// Test: cargo test --example lc_0458_poor_pigs
// Bench: cargo test --example lc_0458_poor_pigs bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 2ns   (iterative multiplication)
//   AI solution: avg 6ns   (logarithm)
//   Ratio: 0.33x (My 3x faster -- no floating point)

/// My solution module — iterative multiplication.
///
/// Count how many pigs needed so that base^pigs >= buckets.
///
/// Time:  O(log_base(buckets))
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
            let base = minutes_to_test / minutes_to_die + 1;
            let mut pigs = 0;
            let mut capacity = 1; // base^pigs
            while capacity < buckets {
                capacity *= base;
                pigs += 1;
            }
            pigs
        }
    }
}

/// AI solution module — logarithm approach.
///
/// pigs = ceil(log(buckets) / log(base))
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
            let base = (minutes_to_test / minutes_to_die + 1) as f64;
            // ceil(log_base(buckets)) = ceil(ln(buckets) / ln(base))
            ((buckets as f64).ln() / base.ln()).ceil() as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_four_buckets_one_round() {
            assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
        }
        #[test]
        fn test_four_buckets_two_rounds() {
            assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
        }
        #[test]
        fn test_one_bucket() {
            assert_eq!(Solution::poor_pigs(1, 1, 1), 0);
        }
        #[test]
        fn test_1000_buckets() {
            // base = 100/15 + 1 = 7, need ceil(log7(1000)) = ceil(3.55) = 4
            assert_eq!(Solution::poor_pigs(1000, 15, 100), 4);
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
    F: Fn(i32, i32, i32) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(1000),
            std::hint::black_box(15),
            std::hint::black_box(100),
        ));
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
    bench_solution("My solution", my_solution::Solution::poor_pigs);
    bench_solution("AI solution", ai_solution::Solution::poor_pigs);
}

fn main() {
    println!("{}", ai_solution::Solution::poor_pigs(4, 15, 15));
}
