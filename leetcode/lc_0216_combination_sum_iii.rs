// LeetCode 216: Combination Sum III [Medium]
// https://leetcode.com/problems/combination-sum-iii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.215us
//   AI solution avg: 3.656us
//   Ratio: My 3.01x faster
//
// Find all valid combinations of k numbers that sum up to n such that:
// - Only numbers 1 through 9 are used.
// - Each number is used at most once.
// Return a list of all possible valid combinations.
//
// Constraints: 2 <= k <= 9, 1 <= n <= 60
//
// Run:  cargo run --example lc_0216_combination_sum_iii
// Test: cargo test --example lc_0216_combination_sum_iii
// Bench: cargo test --example lc_0216_combination_sum_iii bench -- --nocapture

/// My solution — standard backtracking with explicit loop and Vec state.
///
/// Iterate digits 1..=9, pruning when the remaining sum is negative or
/// when there are not enough digits left. Build combinations in a mutable
/// Vec, push/pop to backtrack.
///
/// Time: O(C(9, k)) — at most 2^9 = 512 subsets explored.
/// Space: O(k) for the recursion stack and current combination.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all valid k-digit combinations that sum to n.
        pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let mut current = Vec::with_capacity(k as usize);
            Self::backtrack(k as usize, n, 1, &mut current, &mut result);
            result
        }

        /// Recursive backtracking helper.
        ///
        /// * `remaining_k` — how many more digits we need
        /// * `remaining_sum` — target sum still to achieve
        /// * `start` — smallest digit we may pick (avoids duplicates)
        /// * `current` — digits chosen so far
        /// * `result` — accumulator for valid combinations
        fn backtrack(
            remaining_k: usize,
            remaining_sum: i32,
            start: i32,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remaining_k == 0 {
                if remaining_sum == 0 {
                    result.push(current.clone());
                }
                return;
            }
            // Prune: need `remaining_k` digits starting from `start`;
            // the last digit we can pick is 9 - remaining_k + 1
            for digit in start..=(9 - remaining_k as i32 + 1) {
                if digit > remaining_sum {
                    // All subsequent digits are larger, so prune
                    break;
                }
                current.push(digit);
                Self::backtrack(
                    remaining_k - 1,
                    remaining_sum - digit,
                    digit + 1,
                    current,
                    result,
                );
                current.pop();
            }
        }
    }
}

/// AI solution — bit-mask enumeration over all 2^9 subsets of {1..9}.
///
/// For each subset, check whether it has exactly k bits set and whether
/// the sum of the corresponding digits equals n. No recursion needed.
///
/// Time: O(2^9 * 9) = O(4608) — constant.
/// Space: O(1) extra beyond the output.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all valid k-digit combinations that sum to n.
        pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
            let k = k as u32;
            let mut result = Vec::new();

            // Enumerate all subsets of {1, 2, ..., 9} via a 9-bit mask.
            // Bit i (0-indexed) represents the digit (i + 1).
            for mask in 0u16..(1 << 9) {
                // Fast popcount check — skip subsets with wrong size
                if mask.count_ones() != k {
                    continue;
                }

                let mut sum = 0i32;
                let mut combo = Vec::with_capacity(k as usize);
                for bit in 0..9 {
                    if mask & (1 << bit) != 0 {
                        let digit = bit + 1;
                        sum += digit;
                        combo.push(digit);
                    }
                }

                if sum == n {
                    result.push(combo);
                }
            }
            result
        }
    }
}

#[cfg(test)]
fn sorted_nested(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for inner in &mut v {
        inner.sort_unstable();
    }
    v.sort();
    v
}

fn main() {
    let test_cases = [(3, 7), (3, 9), (4, 1), (2, 18)];
    for (k, n) in test_cases {
        let my = my_solution::Solution::combination_sum3(k, n);
        let ai = ai_solution::Solution::combination_sum3(k, n);
        println!("k={k}, n={n}: my={my:?}");
        println!("          ai={ai:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_combo3 {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::sorted_nested;

                #[test]
                fn test_example1() {
                    // k=3, n=7 => [[1,2,4]]
                    assert_eq!(
                        sorted_nested(<$solution>::combination_sum3(3, 7)),
                        vec![vec![1, 2, 4]]
                    );
                }

                #[test]
                fn test_example2() {
                    // k=3, n=9 => [[1,2,6],[1,3,5],[2,3,4]]
                    assert_eq!(
                        sorted_nested(<$solution>::combination_sum3(3, 9)),
                        vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
                    );
                }

                #[test]
                fn test_impossible_too_small() {
                    // k=4, n=1 => impossible (min sum of 4 digits is 1+2+3+4=10)
                    assert_eq!(<$solution>::combination_sum3(4, 1), Vec::<Vec<i32>>::new());
                }

                #[test]
                fn test_max_pair() {
                    // k=2, n=18 => only possibility is [9, 9] but can't reuse => empty
                    // Actually max pair is [8, 9] = 17, so n=18 is empty
                    assert_eq!(<$solution>::combination_sum3(2, 18), Vec::<Vec<i32>>::new());
                }

                #[test]
                fn test_pair() {
                    // k=2, n=17 => [[8,9]]
                    assert_eq!(
                        sorted_nested(<$solution>::combination_sum3(2, 17)),
                        vec![vec![8, 9]]
                    );
                }

                #[test]
                fn test_single_combo() {
                    // k=2, n=3 => [[1,2]]
                    assert_eq!(
                        sorted_nested(<$solution>::combination_sum3(2, 3)),
                        vec![vec![1, 2]]
                    );
                }

                #[test]
                fn test_all_nine() {
                    // k=9, n=45 => [[1,2,3,4,5,6,7,8,9]] (sum of 1..=9 is 45)
                    assert_eq!(
                        sorted_nested(<$solution>::combination_sum3(9, 45)),
                        vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]]
                    );
                }
            }
        };
    }

    test_combo3!(my, crate::my_solution::Solution);
    test_combo3!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let k = 4;
        let n = 24;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::combination_sum3(
                std::hint::black_box(k),
                std::hint::black_box(n),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::combination_sum3(
                std::hint::black_box(k),
                std::hint::black_box(n),
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
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
