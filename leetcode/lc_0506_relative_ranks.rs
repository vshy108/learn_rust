// LeetCode 506: Relative Ranks [Easy]
// https://leetcode.com/problems/relative-ranks/
//
// Given an integer array score of size n, where score[i] is the score of the
// i-th athlete. All scores are unique. Return an array of ranks:
//   1st place -> "Gold Medal"
//   2nd place -> "Silver Medal"
//   3rd place -> "Bronze Medal"
//   x-th place -> "x"
//
// Examples:
//   [5,4,3,2,1] -> ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
//   [10,3,8,9,4] -> ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
//
// Constraints:
//   n == score.length
//   1 <= n <= 10^4
//   0 <= score[i] <= 10^6
//   All values in score are unique
//
// Run:  cargo run --example lc_0506_relative_ranks
// Test: cargo test --example lc_0506_relative_ranks
// Bench: cargo test --example lc_0506_relative_ranks bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 6.15us
//   AI solution: avg 14.27us
//   My is 2.32x faster
//   Reason: index-sort avoids cloning the entire score array and skips HashMap
//   hashing overhead; direct index assignment is cache-friendlier.

/// My solution — sort indices by score descending, assign ranks.
///
/// Translated from Python/Java solution in doocs/leetcode.
///
/// Time:  O(n log n)
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Assign ranks to athletes based on their scores.
        ///
        /// # Arguments
        /// * `score` - unique scores for each athlete
        ///
        /// # Returns
        /// Vec of rank strings
        pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
            let n = score.len();
            let mut idx: Vec<usize> = (0..n).collect();
            // Sort indices by score descending
            idx.sort_unstable_by(|&a, &b| score[b].cmp(&score[a]));

            let top3 = ["Gold Medal", "Silver Medal", "Bronze Medal"];
            let mut ans = vec![String::new(); n];
            for (i, &j) in idx.iter().enumerate() {
                ans[j] = if i < 3 {
                    top3[i].to_string()
                } else {
                    (i + 1).to_string()
                };
            }
            ans
        }
    }
}

/// AI solution — HashMap approach: map score -> rank from sorted order.
///
/// Sort a copy of scores descending, then build a HashMap from score to its
/// rank string. Finally map original scores through the HashMap.
///
/// Time:  O(n log n)
/// Space: O(n)
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Assign ranks using a HashMap lookup.
        ///
        /// # Arguments
        /// * `score` - unique scores for each athlete
        ///
        /// # Returns
        /// Vec of rank strings
        pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
            let mut sorted = score.clone();
            sorted.sort_unstable_by(|a, b| b.cmp(a));

            let top3 = ["Gold Medal", "Silver Medal", "Bronze Medal"];
            let rank_map: HashMap<i32, String> = sorted
                .iter()
                .enumerate()
                .map(|(i, &s)| {
                    let rank = if i < 3 {
                        top3[i].to_string()
                    } else {
                        (i + 1).to_string()
                    };
                    (s, rank)
                })
                .collect();

            score.iter().map(|s| rank_map[s].clone()).collect()
        }
    }
}

fn main() {
    let score = vec![5, 4, 3, 2, 1];
    let my = my_solution::Solution::find_relative_ranks(score.clone());
    let ai = ai_solution::Solution::find_relative_ranks(score);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_ranks {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::find_relative_ranks(vec![5, 4, 3, 2, 1]),
                        vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::find_relative_ranks(vec![10, 3, 8, 9, 4]),
                        vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
                    );
                }

                #[test]
                fn test_single() {
                    assert_eq!(
                        <$solution>::find_relative_ranks(vec![1]),
                        vec!["Gold Medal"]
                    );
                }

                #[test]
                fn test_two() {
                    assert_eq!(
                        <$solution>::find_relative_ranks(vec![2, 1]),
                        vec!["Gold Medal", "Silver Medal"]
                    );
                }
            }
        };
    }

    test_ranks!(my, crate::my_solution::Solution);
    test_ranks!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 500_000;
        let score: Vec<i32> = (0..100).rev().collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_relative_ranks(
                std::hint::black_box(score.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_relative_ranks(
                std::hint::black_box(score.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
