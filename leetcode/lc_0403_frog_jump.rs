// LeetCode 403: Frog Jump [Hard]
// https://leetcode.com/problems/frog-jump/
// Benchmark: 10000 iterations
//   My solution (bottom-up DP):       avg 1.93ms
//   AI solution (DFS + memoization): avg 255.54us
//   -> AI solution is 7.54x faster
//
// A frog crosses a river by jumping on stones. Given sorted stone positions,
// determine if the frog can reach the last stone. First jump must be 1 unit.
// If last jump was k, next must be k-1, k, or k+1 (forward only).
//
// Examples:
//   [0,1,3,5,6,8,12,17] -> true  (jumps: 1,2,2,1,3,4,5)
//   [0,1,2,3,4,8,9,11]  -> false
//
// Constraints:
//   - 2 <= stones.length <= 2000
//   - 0 <= stones[i] <= 2^31 - 1
//   - stones[0] == 0, sorted strictly increasing
//
// Run:  cargo run --example lc_0403_frog_jump
// Test: cargo test --example lc_0403_frog_jump
// Bench: cargo test --example lc_0403_frog_jump bench -- --nocapture

/// My solution module — DP with HashSet for positions.
///
/// dp[i][k] = true if we can reach stone i with a jump of size k.
/// Use a HashMap<i32, usize> to look up stone positions by value.
/// For each stone i, iterate backwards through previous stones j,
/// compute k = stones[i] - stones[j], and check dp[j][k-1..=k+1].
///
/// Optimization: k can't exceed j (since from stone 0 we jump 1,
/// max jump at stone j is j+1). So if k-1 > j, break.
///
/// Time:  O(n^2)
/// Space: O(n^2)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Determine if the frog can cross by landing on the last stone.
        ///
        /// # Arguments
        ///
        /// * `stones` - Sorted stone positions
        ///
        /// # Returns
        ///
        /// true if the frog can reach the last stone
        pub fn can_cross(stones: Vec<i32>) -> bool {
            let n = stones.len();
            // dp[i][k] = can reach stone i with jump of size k
            let mut dp = vec![vec![false; n]; n];
            dp[0][0] = true;

            for i in 1..n {
                for j in (0..i).rev() {
                    let k = (stones[i] - stones[j]) as usize;
                    // Max possible jump at stone j is j+1 (since max jump
                    // grows by 1 per stone from start)
                    if k > j + 1 {
                        break;
                    }
                    // Check if we could have arrived at j with jump k-1, k, or k+1
                    dp[i][k] = dp[j][k.saturating_sub(1)]
                        || dp[j][k]
                        || (k + 1 < n && dp[j][k + 1]);
                    if i == n - 1 && dp[i][k] {
                        return true;
                    }
                }
            }
            false
        }
    }
}

/// AI solution module — DFS with memoization + HashMap position lookup.
///
/// Recursive DFS from stone 0 with jump 0. At each stone, try jumps
/// k-1, k, k+1. Use a memo table to avoid recomputation.
///
/// Time:  O(n^2) — each (i, k) pair computed at most once
/// Space: O(n^2) for memo + recursion stack
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Determine if the frog can cross by landing on the last stone.
        ///
        /// # Arguments
        ///
        /// * `stones` - Sorted stone positions
        ///
        /// # Returns
        ///
        /// true if the frog can reach the last stone
        pub fn can_cross(stones: Vec<i32>) -> bool {
            let n = stones.len();
            // Map stone position -> index for O(1) lookup
            let pos: HashMap<i32, usize> = stones
                .iter()
                .enumerate()
                .map(|(i, &s)| (s, i))
                .collect();

            let mut memo = vec![vec![-1i8; n]; n]; // -1=unvisited, 0=false, 1=true
            Self::dfs(0, 0, n, &pos, &stones, &mut memo)
        }

        /// Recursive helper: can we reach the last stone from stone i,
        /// having arrived with a jump of size k?
        fn dfs(
            i: usize,
            k: usize,
            n: usize,
            pos: &HashMap<i32, usize>,
            stones: &[i32],
            memo: &mut [Vec<i8>],
        ) -> bool {
            if i == n - 1 {
                return true;
            }
            if memo[i][k] != -1 {
                return memo[i][k] == 1;
            }

            let start = if k == 0 { 1 } else { k - 1 };
            for jump in start..=k + 1 {
                if jump > 0 {
                    let target = stones[i] + jump as i32;
                    if let Some(&next_idx) = pos.get(&target) {
                        if Self::dfs(next_idx, jump, n, pos, stones, memo) {
                            memo[i][k] = 1;
                            return true;
                        }
                    }
                }
            }
            memo[i][k] = 0;
            false
        }
    }
}

fn main() {
    let cases = [
        (vec![0, 1, 3, 5, 6, 8, 12, 17], true),
        (vec![0, 1, 2, 3, 4, 8, 9, 11], false),
    ];

    println!("--- My Solution ---");
    for (stones, expected) in &cases {
        let result = my_solution::Solution::can_cross(stones.clone());
        println!("  {stones:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (stones, expected) in &cases {
        let result = ai_solution::Solution::can_cross(stones.clone());
        println!("  {stones:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_frog {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert!(Solution::can_cross(
                        vec![0, 1, 3, 5, 6, 8, 12, 17]
                    ));
                }

                #[test]
                fn test_example2() {
                    assert!(!Solution::can_cross(
                        vec![0, 1, 2, 3, 4, 8, 9, 11]
                    ));
                }

                #[test]
                fn test_minimal_true() {
                    assert!(Solution::can_cross(vec![0, 1]));
                }

                #[test]
                fn test_minimal_false() {
                    assert!(!Solution::can_cross(vec![0, 2]));
                }

                #[test]
                fn test_three_stones() {
                    // 0 -> 1 (jump 1) -> 3 (jump 2) = true
                    assert!(Solution::can_cross(vec![0, 1, 3]));
                }

                #[test]
                fn test_gap_too_large() {
                    // 0 -> 1 (jump 1), max next jump is 2, but 4 is 3 away
                    assert!(!Solution::can_cross(vec![0, 1, 4]));
                }
            }
        };
    }

    test_frog!(my_tests, crate::my_solution::Solution);
    test_frog!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;
        // Build stones that the frog can traverse (each gap is 1)
        let stones: Vec<i32> = (0..2000).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::can_cross(
                    std::hint::black_box(stones.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::can_cross(
                    std::hint::black_box(stones.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
