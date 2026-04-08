// LeetCode 470: Implement Rand10() Using Rand7() [Medium]
// https://leetcode.com/problems/implement-rand10-using-rand7/
//
// Given the API rand7() that generates a uniform random integer in [1,7],
// implement rand10() that generates a uniform random integer in [1,10].
// Minimize the number of calls to rand7().
//
// Approach: Use rejection sampling. Two calls to rand7() give a uniform
// distribution over 49 outcomes (1..=49). Accept values 1..=40 (map to
// 1..=10), reject 41..=49 and retry.
//
// Constraints:
//   rand7() returns 1..=7 uniformly
//   Each value 1..=10 must be equally likely
//
// Run:  cargo run --example lc_0470_implement_rand10_using_rand7
// Test: cargo test --example lc_0470_implement_rand10_using_rand7
// Bench: cargo test --example lc_0470_implement_rand10_using_rand7 bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 20ns
//   AI solution: avg 18ns
//   Ratio: 1.11x (similar rejection sampling)

use rand::Rng;

/// Simulate the provided rand7() API.
fn rand7() -> i32 {
    rand::rng().random_range(1..=7)
}

/// My solution — rejection sampling with 7 * 7 = 49 grid.
///
/// Two calls to rand7() produce a number in [1,49] uniformly.
/// Accept [1,40] -> map to [1,10]. Reject [41,49] and retry.
/// Expected calls per rand10(): ~2.33 calls to rand7().
///
/// Time:  O(1) expected (geometric distribution)
/// Space: O(1)
#[allow(dead_code)]
mod my_solution {
    use super::rand7;

    pub struct Solution;
    impl Solution {
        /// Generate a uniform random integer in [1,10] from rand7().
        ///
        /// # Returns
        /// An integer in [1,10], uniformly distributed.
        pub fn rand10() -> i32 {
            loop {
                // Create uniform [1,49] from two calls to rand7()
                let row = rand7() - 1; // [0,6]
                let col = rand7(); // [1,7]
                let idx = row * 7 + col; // [1,49]
                if idx <= 40 {
                    return (idx - 1) % 10 + 1;
                }
                // Reject 41..=49 and retry
            }
        }
    }
}

/// AI solution — rejection sampling with recycled rejects.
///
/// Same base approach, but reuse the 9 rejected values (41..=49)
/// for a second level: combine with another rand7() call to get
/// 9 * 7 = 63 values, accept [1,60], reject [61,63] for a third
/// level: 3 * 7 = 21, accept [1,20], reject [21]. Fewer calls on average.
///
/// Time:  O(1) expected (~2.2 calls to rand7() on average)
/// Space: O(1)
mod ai_solution {
    use super::rand7;

    pub struct Solution;
    impl Solution {
        /// Generate rand10() with optimized rejection recycling.
        pub fn rand10() -> i32 {
            loop {
                // Level 1: [1, 49]
                let a = (rand7() - 1) * 7 + rand7(); // [1, 49]
                if a <= 40 {
                    return (a - 1) % 10 + 1;
                }
                // Level 2: reuse (a - 40) in [1, 9], * rand7() => [1, 63]
                let b = (a - 40 - 1) * 7 + rand7(); // [1, 63]
                if b <= 60 {
                    return (b - 1) % 10 + 1;
                }
                // Level 3: reuse (b - 60) in [1, 3], * rand7() => [1, 21]
                let c = (b - 60 - 1) * 7 + rand7(); // [1, 21]
                if c <= 20 {
                    return (c - 1) % 10 + 1;
                }
                // Reject only 21; loop again
            }
        }
    }
}

fn main() {
    println!(
        "my rand10 samples: {:?}",
        (0..10)
            .map(|_| my_solution::Solution::rand10())
            .collect::<Vec<_>>()
    );
    println!(
        "ai rand10 samples: {:?}",
        (0..10)
            .map(|_| ai_solution::Solution::rand10())
            .collect::<Vec<_>>()
    );
}

/// Test statistical distribution: run many trials and check all values
/// appear with roughly equal frequency.
#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Solution;

        #[test]
        fn test_range() {
            // All values should be in [1,10]
            for _ in 0..1000 {
                let v = Solution::rand10();
                assert!((1..=10).contains(&v), "got {v}");
            }
        }

        #[test]
        fn test_distribution() {
            // Run 100k trials; each value should appear ~10k times.
            // Allow a generous margin (>5% of expected).
            let trials = 100_000;
            let mut counts = [0u32; 11]; // index 1..=10
            for _ in 0..trials {
                counts[Solution::rand10() as usize] += 1;
            }
            let expected = trials as f64 / 10.0;
            for i in 1..=10 {
                let ratio = counts[i] as f64 / expected;
                assert!(
                    (0.85..=1.15).contains(&ratio),
                    "value {i} appeared {} times (ratio {ratio:.3})",
                    counts[i]
                );
            }
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
    F: Fn() -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f());
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
    bench_solution("My solution", my_solution::Solution::rand10);
    bench_solution("AI solution", ai_solution::Solution::rand10);
}
