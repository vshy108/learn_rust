// LeetCode 495: Teemo Attacking [Easy]
// https://leetcode.com/problems/teemo-attacking/
//
// Teemo attacks an enemy at given time points. Each attack poisons the enemy
// for `duration` seconds. If attacked again while poisoned, the timer resets.
// Return the total poisoned time.
//
// Examples:
//   timeSeries = [1,4], duration = 2   -> 4
//   timeSeries = [1,2], duration = 2   -> 3 (overlap: 1-3, 2-4 -> total 3)
//
// Constraints:
//   1 <= timeSeries.len() <= 10^4
//   0 <= timeSeries[i], duration <= 10^7
//   timeSeries is sorted in non-decreasing order
//
// Run:  cargo run --example lc_0495_teemo_attacking
// Test: cargo test --example lc_0495_teemo_attacking
// Bench: cargo test --example lc_0495_teemo_attacking bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 23ns
//   AI solution: avg 23ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — greedy overlap counting.
///
/// Time:  O(n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
            if time_series.is_empty() {
                return 0;
            }
            let mut total = 0;
            for i in 1..time_series.len() {
                total += (time_series[i] - time_series[i - 1]).min(duration);
            }
            total + duration
        }
    }
}

/// AI solution module — for each consecutive pair, add min(gap, duration).
/// Plus one full duration for the last attack.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
            if time_series.is_empty() {
                return 0;
            }
            let mut total = 0i32;
            for i in 1..time_series.len() {
                let gap = time_series[i] - time_series[i - 1];
                total += gap.min(duration);
            }
            // Last attack always contributes full duration
            total + duration
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_no_overlap() {
            assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
        }
        #[test]
        fn test_overlap() {
            assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
        }
        #[test]
        fn test_single() {
            assert_eq!(Solution::find_poisoned_duration(vec![1], 5), 5);
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
    F: Fn(Vec<i32>, i32) -> i32,
{
    let iterations = 500_000;
    let input = vec![1, 2, 4, 7, 8, 10, 15, 16, 17, 20];
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(input.clone()),
            std::hint::black_box(3),
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
    bench_solution("My solution", my_solution::Solution::find_poisoned_duration);
    bench_solution("AI solution", ai_solution::Solution::find_poisoned_duration);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_poisoned_duration(vec![1, 4], 2)
    );
}
