// LeetCode 229: Majority Element II [Medium]
// https://leetcode.com/problems/majority-element-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.531us
//   AI solution avg: 0.177us
//   Ratio: AI 8.66x faster
//
// Given an integer array of size n, find all elements that appear more
// than n/3 times.
//
// Follow up: Could you solve the problem in linear time and O(1) space?
//
// Constraints: 1 <= nums.length <= 5 * 10^4, -10^9 <= nums[i] <= 10^9
//
// Run:  cargo run --example lc_0229_majority_element_ii
// Test: cargo test --example lc_0229_majority_element_ii
// Bench: cargo test --example lc_0229_majority_element_ii bench -- --nocapture

/// My solution — HashMap counting.
///
/// Count occurrences with a HashMap, then filter entries > n/3.
/// Simple and readable but uses O(n) extra space.
///
/// Time: O(n).
/// Space: O(n) for the HashMap.
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return all elements appearing more than n/3 times.
        pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
            let threshold = nums.len() / 3;
            let mut counts: HashMap<i32, usize> = HashMap::new();

            for &num in &nums {
                *counts.entry(num).or_insert(0) += 1;
            }

            // Collect entries exceeding n/3 (at most 2 such elements)
            counts
                .into_iter()
                .filter(|&(_, count)| count > threshold)
                .map(|(val, _)| val)
                .collect()
        }
    }
}

/// AI solution — Boyer-Moore Voting (extended to 2 candidates).
///
/// There can be at most 2 elements appearing more than n/3 times.
/// Maintain two candidates with counters. In a second pass, verify
/// each candidate actually exceeds the threshold.
///
/// Time: O(n) — two passes.
/// Space: O(1) — only a few variables.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all elements appearing more than n/3 times.
        pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
            // Phase 1: find up to two candidates
            let mut cand1 = 0i32;
            let mut cand2 = 0i32;
            let mut count1 = 0u32;
            let mut count2 = 0u32;

            for &num in &nums {
                if count1 > 0 && num == cand1 {
                    count1 += 1;
                } else if count2 > 0 && num == cand2 {
                    count2 += 1;
                } else if count1 == 0 {
                    cand1 = num;
                    count1 = 1;
                } else if count2 == 0 {
                    cand2 = num;
                    count2 = 1;
                } else {
                    // All three slots disagree — cancel one from each
                    count1 -= 1;
                    count2 -= 1;
                }
            }

            // Phase 2: verify the candidates actually exceed n/3
            let threshold = nums.len() / 3;
            let mut result = Vec::new();

            let real1 = nums.iter().filter(|&&x| x == cand1).count();
            if real1 > threshold {
                result.push(cand1);
            }

            // Avoid double-counting if both candidates are the same
            if cand2 != cand1 || count1 == 0 {
                let real2 = nums.iter().filter(|&&x| x == cand2).count();
                if real2 > threshold {
                    result.push(cand2);
                }
            }

            result
        }
    }
}

fn main() {
    let cases: Vec<(Vec<i32>, &str)> = vec![
        (vec![3, 2, 3], "[3]"),
        (vec![1], "[1]"),
        (vec![1, 2], "[1, 2]"),
    ];

    for (nums, expected) in &cases {
        let mut my = my_solution::Solution::majority_element(nums.clone());
        let mut ai = ai_solution::Solution::majority_element(nums.clone());
        my.sort();
        ai.sort();
        println!("{nums:?} => my={my:?}, ai={ai:?} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    /// Helper to compare results regardless of order.
    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort();
        v
    }

    macro_rules! test_majority {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        sorted(<$solution>::majority_element(vec![3, 2, 3])),
                        vec![3],
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        sorted(<$solution>::majority_element(vec![1])),
                        vec![1],
                    );
                }

                #[test]
                fn test_two_majorities() {
                    assert_eq!(
                        sorted(<$solution>::majority_element(vec![1, 2])),
                        vec![1, 2],
                    );
                }

                #[test]
                fn test_no_majority() {
                    // [1, 2, 3] — each appears once, threshold = 1,
                    // none exceed 1
                    let result = sorted(<$solution>::majority_element(vec![1, 2, 3]));
                    assert!(result.is_empty());
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        sorted(<$solution>::majority_element(vec![5, 5, 5, 5])),
                        vec![5],
                    );
                }

                #[test]
                fn test_two_candidates() {
                    // [1,1,1,2,2,2,3] n=7, threshold=2
                    // 1 appears 3 times (>2), 2 appears 3 times (>2)
                    assert_eq!(
                        sorted(<$solution>::majority_element(vec![1, 1, 1, 2, 2, 2, 3])),
                        vec![1, 2],
                    );
                }
            }
        };
    }

    test_majority!(my, crate::my_solution::Solution);
    test_majority!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a deterministic array with two majority elements
        let mut nums = Vec::with_capacity(90);
        for _ in 0..35 {
            nums.push(7);
        }
        for _ in 0..35 {
            nums.push(11);
        }
        for i in 0..20 {
            nums.push(i);
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::majority_element(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::majority_element(
                std::hint::black_box(nums.clone()),
            ));
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
