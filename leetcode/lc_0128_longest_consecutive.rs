// LeetCode 128: Longest Consecutive Sequence [Medium]
// https://leetcode.com/problems/longest-consecutive-sequence/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 7.974us
//   AI solution avg: 3.018us
//   Ratio: AI 2.64x faster
//
// Given an unsorted array of integers nums, return the length of the longest
// consecutive elements sequence. Must run in O(n) time.
//
// Constraints: 0 <= nums.length <= 10^5, -10^9 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0128_longest_consecutive
// Test: cargo test --example lc_0128_longest_consecutive
// Bench: cargo test --example lc_0128_longest_consecutive bench -- --nocapture

use std::collections::HashSet;

/// My solution — HashSet, for each sequence start, count consecutive.
///
/// Time: O(n) — each element visited at most twice.
/// Space: O(n) for the set.
mod my_solution {
    use super::*;

    pub struct Solution;

    impl Solution {
        /// Return the length of the longest consecutive sequence.
        pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
            let set: HashSet<i32> = nums.into_iter().collect();
            let mut longest = 0;

            for &num in &set {
                // Only start counting from sequence beginnings (no num-1 in set)
                if !set.contains(&(num - 1)) {
                    let mut current = num;
                    let mut length = 1;
                    while set.contains(&(current + 1)) {
                        current += 1;
                        length += 1;
                    }
                    longest = longest.max(length);
                }
            }
            longest
        }
    }
}

/// AI solution — sort-based approach (O(n log n) but very cache-friendly).
///
/// Time: O(n log n).
/// Space: O(1) extra (sort in-place after dedup handled).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return longest consecutive using sort + linear scan.
        pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
            if nums.is_empty() {
                return 0;
            }
            nums.sort_unstable();
            nums.dedup();

            let mut longest = 1;
            let mut current_len = 1;

            for i in 1..nums.len() {
                if nums[i] == nums[i - 1] + 1 {
                    current_len += 1;
                    longest = longest.max(current_len);
                } else {
                    current_len = 1;
                }
            }
            longest
        }
    }
}

fn main() {
    let my = my_solution::Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
    let ai = ai_solution::Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
    println!("longest consecutive: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_consecutive {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::longest_consecutive(vec![100, 4, 200, 1, 3, 2]),
                        4,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1],),
                        9,
                    );
                }

                #[test]
                fn test_empty() {
                    assert_eq!(<$solution>::longest_consecutive(vec![]), 0);
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::longest_consecutive(vec![1]), 1);
                }

                #[test]
                fn test_duplicates() {
                    assert_eq!(<$solution>::longest_consecutive(vec![1, 2, 0, 1]), 3,);
                }

                #[test]
                fn test_negatives() {
                    assert_eq!(<$solution>::longest_consecutive(vec![-1, -2, 0, 1]), 4,);
                }
            }
        };
    }

    test_consecutive!(my, crate::my_solution::Solution);
    test_consecutive!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = (0..500).map(|i| (i * 37 + 13) % 300).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::longest_consecutive(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::longest_consecutive(
                std::hint::black_box(nums.clone()),
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
