// LeetCode 39: Combination Sum [Medium]
// https://leetcode.com/problems/combination-sum/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 24.753µs
//   AI solution avg: 24.597µs
//   Ratio: ~tied
//
// Given an array of distinct integers candidates and a target integer,
// return all unique combinations where the chosen numbers sum to target.
// The same number may be chosen unlimited times. Two combinations are
// unique if the frequency of at least one chosen number differs.
//
// Example 1: candidates = [2,3,6,7], target = 7 -> [[2,2,3],[7]]
// Example 2: candidates = [2,3,5], target = 8 -> [[2,2,2,2],[2,3,3],[3,5]]
// Example 3: candidates = [2], target = 1 -> []
//
// Constraints:
//   1 <= candidates.length <= 30
//   2 <= candidates[i] <= 40
//   All elements of candidates are distinct
//   1 <= target <= 40
//
// Run:  cargo run --example lc_0039_combination_sum
// Test: cargo test --example lc_0039_combination_sum
// Bench: cargo test --example lc_0039_combination_sum bench -- --nocapture

/// My solution — backtracking.
///
/// Sort candidates first. For each candidate, either include it (possibly
/// multiple times) or skip to the next. Skip early if candidate > remaining.
///
/// Time: O(N^(T/M)) where N = len, T = target, M = min candidate.
/// Space: O(T/M) recursion depth.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find all combinations summing to target.
        ///
        /// # Arguments
        /// * `candidates` - distinct positive integers
        /// * `target` - desired sum
        ///
        /// # Returns
        /// All unique combinations (order-independent).
        pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            candidates.sort_unstable();
            let mut result = Vec::new();
            let mut current = Vec::new();
            Self::backtrack(&candidates, target, 0, &mut current, &mut result);
            result
        }

        fn backtrack(
            candidates: &[i32],
            remaining: i32,
            start: usize,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remaining == 0 {
                result.push(current.clone());
                return;
            }

            for i in start..candidates.len() {
                let c = candidates[i];
                if c > remaining {
                    break; // sorted — no point continuing
                }
                current.push(c);
                // Pass `i` (not i+1) because we can reuse the same element.
                Self::backtrack(candidates, remaining - c, i, current, result);
                current.pop();
            }
        }
    }
}

/// AI-optimized solution — same backtracking (already optimal for this problem).
///
/// The backtracking with sorting + early termination is the standard optimal
/// approach. This variant is identical — there's no meaningful optimization
/// beyond what the basic approach provides for constraint sizes <= 40.
///
/// Time: O(N^(T/M)).
/// Space: O(T/M).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find all combinations summing to target.
        ///
        /// # Arguments
        /// * `candidates` - distinct positive integers
        /// * `target` - desired sum
        ///
        /// # Returns
        /// All unique combinations (order-independent).
        pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
            candidates.sort_unstable();
            let mut result = Vec::new();
            let mut current = Vec::new();
            Self::backtrack(&candidates, target, 0, &mut current, &mut result);
            result
        }

        fn backtrack(
            candidates: &[i32],
            remaining: i32,
            start: usize,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remaining == 0 {
                result.push(current.clone());
                return;
            }

            for i in start..candidates.len() {
                let c = candidates[i];
                if c > remaining {
                    break;
                }
                current.push(c);
                Self::backtrack(candidates, remaining - c, i, current, result);
                current.pop();
            }
        }
    }
}

fn main() {
    let my = my_solution::Solution::combination_sum(vec![2, 3, 6, 7], 7);
    let ai = ai_solution::Solution::combination_sum(vec![2, 3, 6, 7], 7);
    println!("my:{my:?}");
    println!("ai:{ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_combination_sum {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn sorted(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
                    for inner in &mut v {
                        inner.sort();
                    }
                    v.sort();
                    v
                }

                #[test]
                fn test_example_1() {
                    let result = sorted(Solution::combination_sum(vec![2, 3, 6, 7], 7));
                    assert_eq!(result, sorted(vec![vec![2, 2, 3], vec![7]]));
                }

                #[test]
                fn test_example_2() {
                    let result = sorted(Solution::combination_sum(vec![2, 3, 5], 8));
                    assert_eq!(
                        result,
                        sorted(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]),
                    );
                }

                #[test]
                fn test_no_solution() {
                    let result = Solution::combination_sum(vec![2], 1);
                    assert_eq!(result, Vec::<Vec<i32>>::new());
                }

                #[test]
                fn test_single_candidate_exact() {
                    let result = Solution::combination_sum(vec![3], 9);
                    assert_eq!(result, vec![vec![3, 3, 3]]);
                }

                #[test]
                fn test_target_equals_candidate() {
                    let result = Solution::combination_sum(vec![2, 3, 5], 5);
                    let expected = sorted(vec![vec![2, 3], vec![5]]);
                    assert_eq!(sorted(result), expected);
                }
            }
        };
    }

    test_combination_sum!(my_tests, crate::my_solution::Solution);
    test_combination_sum!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let candidates = vec![2, 3, 5, 7, 11, 13];
        let target = 30;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::combination_sum(
                std::hint::black_box(candidates.clone()),
                std::hint::black_box(target),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::combination_sum(
                std::hint::black_box(candidates.clone()),
                std::hint::black_box(target),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
