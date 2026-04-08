// LeetCode 40: Combination Sum II [Medium]
// https://leetcode.com/problems/combination-sum-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 5.300µs
//   AI solution avg: 5.339µs
//   Ratio: ~tied
//
// Given a collection of candidate numbers (may contain duplicates) and a
// target number, find all unique combinations where the candidates sum to
// target. Each number may only be used once.
//
// Example 1: candidates = [10,1,2,7,6,1,5], target = 8 -> [[1,1,6],[1,2,5],[1,7],[2,6]]
// Example 2: candidates = [2,5,2,1,2], target = 5 -> [[1,2,2],[5]]
//
// Constraints:
//   1 <= candidates.length <= 100
//   1 <= candidates[i] <= 50
//   1 <= target <= 30
//
// Run:  cargo run --example lc_0040_combination_sum_ii
// Test: cargo test --example lc_0040_combination_sum_ii
// Bench: cargo test --example lc_0040_combination_sum_ii bench -- --nocapture

/// My solution — backtracking with duplicate skip.
///
/// Sort candidates. At each recursion level, skip duplicate candidates
/// at the same position to avoid generating duplicate combinations.
///
/// Time: O(2^n) in the worst case.
/// Space: O(n) recursion depth.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find all unique combinations summing to target (each element used once).
        ///
        /// # Arguments
        /// * `candidates` - may contain duplicates
        /// * `target` - desired sum
        ///
        /// # Returns
        /// All unique combinations.
        pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
                // Skip duplicates at the same recursion level:
                // if candidates[i] == candidates[i-1] and i > start, we've
                // already explored branches starting with this value.
                if i > start && candidates[i] == candidates[i - 1] {
                    continue;
                }
                current.push(c);
                // Pass i+1: each element can only be used once.
                Self::backtrack(candidates, remaining - c, i + 1, current, result);
                current.pop();
            }
        }
    }
}

/// AI-optimized solution — same approach (standard optimal for this problem).
///
/// Backtracking with sorting + duplicate-skip is the canonical solution.
/// No meaningful optimization beyond this for the given constraints.
///
/// Time: O(2^n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find all unique combinations summing to target (each element used once).
        pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
                if i > start && candidates[i] == candidates[i - 1] {
                    continue;
                }
                current.push(c);
                Self::backtrack(candidates, remaining - c, i + 1, current, result);
                current.pop();
            }
        }
    }
}

fn main() {
    let my = my_solution::Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
    let ai = ai_solution::Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
    println!("my:{my:?}");
    println!("ai:{ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_combination_sum2 {
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
                    let result = sorted(Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8));
                    let expected =
                        sorted(vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_example_2() {
                    let result = sorted(Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5));
                    let expected = sorted(vec![vec![1, 2, 2], vec![5]]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_no_solution() {
                    let result = Solution::combination_sum2(vec![3, 5], 1);
                    assert_eq!(result, Vec::<Vec<i32>>::new());
                }

                #[test]
                fn test_single_element() {
                    let result = Solution::combination_sum2(vec![5], 5);
                    assert_eq!(result, vec![vec![5]]);
                }

                #[test]
                fn test_all_ones() {
                    let result = sorted(Solution::combination_sum2(vec![1, 1, 1, 1], 2));
                    // Only one unique combination: [1, 1].
                    assert_eq!(result, vec![vec![1, 1]]);
                }
            }
        };
    }

    test_combination_sum2!(my_tests, crate::my_solution::Solution);
    test_combination_sum2!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let candidates = vec![1, 1, 2, 2, 3, 3, 5, 5, 7, 7, 10, 10];
        let target = 15;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::combination_sum2(
                std::hint::black_box(candidates.clone()),
                std::hint::black_box(target),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::combination_sum2(
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
