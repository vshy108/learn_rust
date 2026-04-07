// LeetCode 3437: Permutations III [Medium] (Premium)
// https://leetcode.com/problems/permutations-iii/
//
// Given an integer n, an alternating permutation is a permutation of the
// first n positive integers such that no two adjacent elements are both
// odd or both even.
//
// Return all such alternating permutations sorted in lexicographical order.
//
// Examples:
//   n = 4 -> [[1,2,3,4],[1,4,3,2],[2,1,4,3],[2,3,4,1],
//             [3,2,1,4],[3,4,1,2],[4,1,2,3],[4,3,2,1]]
//   n = 2 -> [[1,2],[2,1]]
//   n = 3 -> [[1,2,3],[3,2,1]]
//
// Constraints:
//   - 1 <= n <= 10
//
// Run:  cargo run --example lc_3437_permutations_iii
// Test: cargo test --example lc_3437_permutations_iii
// Bench: cargo test --example lc_3437_permutations_iii bench -- --nocapture
//
// --- Rust concepts used ---
// - Backtracking with `Vec<i32>` current + `Vec<bool>` used: classic DFS
//   pattern; push/pop + set/unset for O(n) space reuse
// - `current.last()` parity check: `last % 2 == i % 2` prunes branches
//   where adjacent elements share parity
// - `map_or(true, |&last| ...)`: provides default for empty current vec,
//   allowing any starting parity
// - `1..=n` inclusive range: iterates candidate values [1, n]
//   (Previously covered: .clone() for snapshot -> lc_0046,
//    backtracking DFS -> lc_0046, Vec<bool> visited -> lc_0587)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  1047.30ms (avg 10.473us)
// AI solution:  1113.56ms (avg 11.135us)
// -> My solution is 1.06x faster
// Reason: both use identical backtracking; the AI's need_odd/need_even
//   booleans add a slight per-candidate overhead vs direct modulo check.

/// My solution module — backtracking with parity check.
///
/// Time:  O(n!) in the worst case (with pruning)
/// Space: O(n) for recursion depth + used array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all alternating permutations of [1..=n] in lexicographic order.
        ///
        /// # Arguments
        ///
        /// * `n` - The upper bound of the permutation range [1, n]
        ///
        /// # Returns
        ///
        /// All valid permutations where no two adjacent elements share parity
        pub fn permute(n: i32) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let mut current = Vec::with_capacity(n as usize);
            let mut used = vec![false; n as usize + 1];
            Self::dfs(n, &mut used, &mut current, &mut result);
            result
        }

        fn dfs(
            n: i32,
            used: &mut Vec<bool>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == n as usize {
                result.push(current.clone());
                return;
            }
            for i in 1..=n {
                if used[i as usize] {
                    continue;
                }
                // Check parity alternates: adjacent elements must differ in parity
                if let Some(&last) = current.last() {
                    if last % 2 == i % 2 {
                        continue;
                    }
                }
                used[i as usize] = true;
                current.push(i);
                Self::dfs(n, used, current, result);
                current.pop();
                used[i as usize] = false;
            }
        }
    }
}

/// AI solution module — backtracking with separate odd/even pools.
///
/// Pre-separates odd and even numbers, then alternately picks from
/// each pool to build permutations.
///
/// Time:  O(n!) with early pruning
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all alternating permutations of [1..=n] in lexicographic order.
        ///
        /// # Arguments
        ///
        /// * `n` - The upper bound of the permutation range [1, n]
        ///
        /// # Returns
        ///
        /// All valid permutations where no two adjacent elements share parity
        pub fn permute(n: i32) -> Vec<Vec<i32>> {
            let mut result = Vec::new();
            let mut current = Vec::with_capacity(n as usize);
            let mut used = vec![false; n as usize + 1];
            // Try starting with odd, then even
            Self::backtrack(n, &mut used, &mut current, &mut result);
            result
        }

        fn backtrack(
            n: i32,
            used: &mut Vec<bool>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == n as usize {
                result.push(current.clone());
                return;
            }
            // Determine required parity: must differ from last element
            let need_odd = current.last().is_none_or(|&last| last % 2 == 0);
            let need_even = current.last().is_none_or(|&last| last % 2 != 0);
            for i in 1..=n {
                if used[i as usize] {
                    continue;
                }
                let is_odd = i % 2 != 0;
                if (is_odd && !need_odd) || (!is_odd && !need_even) {
                    continue;
                }
                used[i as usize] = true;
                current.push(i);
                Self::backtrack(n, used, current, result);
                current.pop();
                used[i as usize] = false;
            }
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, Vec<Vec<i32>>)> = vec![
        (2, vec![vec![1, 2], vec![2, 1]]),
        (3, vec![vec![1, 2, 3], vec![3, 2, 1]]),
    ];

    println!("--- My Solution ---");
    for (n, expected) in &test_cases {
        let result = my_solution::Solution::permute(*n);
        println!("  n={n} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (n, expected) in &test_cases {
        let result = ai_solution::Solution::permute(*n);
        println!("  n={n} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_permute {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_n2() {
                    assert_eq!(Solution::permute(2), vec![vec![1, 2], vec![2, 1]],);
                }

                #[test]
                fn test_n3() {
                    assert_eq!(Solution::permute(3), vec![vec![1, 2, 3], vec![3, 2, 1]],);
                }

                #[test]
                fn test_n4() {
                    assert_eq!(
                        Solution::permute(4),
                        vec![
                            vec![1, 2, 3, 4],
                            vec![1, 4, 3, 2],
                            vec![2, 1, 4, 3],
                            vec![2, 3, 4, 1],
                            vec![3, 2, 1, 4],
                            vec![3, 4, 1, 2],
                            vec![4, 1, 2, 3],
                            vec![4, 3, 2, 1],
                        ],
                    );
                }

                #[test]
                fn test_n1() {
                    assert_eq!(Solution::permute(1), vec![vec![1]]);
                }

                #[test]
                fn test_n5_count() {
                    // n=5: 3 odd + 2 even. Alternating must start with odd.
                    // Verify we get some results (exact count: 12).
                    let result = Solution::permute(5);
                    assert!(!result.is_empty());
                    // All permutations should alternate parity.
                    for perm in &result {
                        for w in perm.windows(2) {
                            assert_ne!(w[0] % 2, w[1] % 2, "adjacent same parity in {perm:?}");
                        }
                    }
                }
            }
        };
    }

    test_permute!(my_tests, crate::my_solution::Solution);
    test_permute!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![2, 3, 4, 5];

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::my_solution::Solution::permute(std::hint::black_box(
                    n,
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::permute(std::hint::black_box(
                    n,
                )));
            }
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
