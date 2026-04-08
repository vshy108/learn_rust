// LeetCode 174: Dungeon Game [Hard]
// https://leetcode.com/problems/dungeon-game/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 3.432us
//   AI solution avg: 2.436us
//   Ratio: AI 1.41x faster
//
// The knight starts at top-left and must reach the princess at bottom-right.
// Each cell adds/subtracts health. The knight dies if health drops to 0
// or below at any point. Return the minimum initial health needed to
// reach the princess (must be at least 1 at every step).
//
// Constraints: m,n in [1,200], dungeon[i][j] in [-1000, 1000]
//
// Run:  cargo run --example lc_0174_dungeon_game
// Test: cargo test --example lc_0174_dungeon_game
// Bench: cargo test --example lc_0174_dungeon_game bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the minimum initial health to reach bottom-right.
        pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
            // 2D DP table from bottom-right to top-left
            let m = dungeon.len();
            let n = dungeon[0].len();
            let mut dp = vec![vec![0i32; n]; m];
            for i in (0..m).rev() {
                for j in (0..n).rev() {
                    let min_next = if i == m - 1 && j == n - 1 {
                        1
                    } else if i == m - 1 {
                        dp[i][j + 1]
                    } else if j == n - 1 {
                        dp[i + 1][j]
                    } else {
                        dp[i + 1][j].min(dp[i][j + 1])
                    };
                    dp[i][j] = (min_next - dungeon[i][j]).max(1);
                }
            }
            dp[0][0]
        }
    }
}

/// AI solution — DP from bottom-right to top-left.
///
/// dp[i][j] = minimum health needed when entering cell (i,j).
/// At every cell, need at least 1 health after applying dungeon[i][j].
/// Transition: dp[i][j] = max(1, min(dp[i+1][j], dp[i][j+1]) - dungeon[i][j]).
///
/// Time: O(m*n).
/// Space: O(n) with 1D rolling array.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Bottom-right to top-left DP with 1D array.
        pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
            let m = dungeon.len();
            let n = dungeon[0].len();

            // dp[j] = min health needed to enter column j of the current row
            let mut dp = vec![i32::MAX; n + 1];
            dp[n - 1] = 1; // need at least 1 health when leaving bottom-right

            for i in (0..m).rev() {
                for j in (0..n).rev() {
                    // Minimum health needed from the cell to the right or below
                    let min_next = dp[j].min(dp[j + 1]);
                    // Health needed at this cell: must be >= 1 after adding dungeon[i][j]
                    dp[j] = (min_next - dungeon[i][j]).max(1);
                }
                // Sentinel: no path to the right of last column from next row
                dp[n] = i32::MAX;
            }
            dp[0]
        }
    }
}

fn main() {
    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    let my = my_solution::Solution::calculate_minimum_hp(dungeon.clone());
    let ai = ai_solution::Solution::calculate_minimum_hp(dungeon);
    println!("min hp: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_hp {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example() {
                    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
                    assert_eq!(<$solution>::calculate_minimum_hp(dungeon), 7);
                }

                #[test]
                fn test_single_positive() {
                    assert_eq!(<$solution>::calculate_minimum_hp(vec![vec![100]]), 1);
                }

                #[test]
                fn test_single_negative() {
                    assert_eq!(<$solution>::calculate_minimum_hp(vec![vec![-5]]), 6);
                }

                #[test]
                fn test_row() {
                    assert_eq!(
                        <$solution>::calculate_minimum_hp(vec![vec![1, -3, 3, -1]]),
                        3, // path: 1+1=2, 2-3=-1 needs 3 to survive
                    );
                }
            }
        };
    }

    test_hp!(my, crate::my_solution::Solution);
    test_hp!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let dungeon: Vec<Vec<i32>> = (0..20)
            .map(|i| {
                (0..20)
                    .map(|j| ((i * 7 + j * 3) % 21) as i32 - 10)
                    .collect()
            })
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::calculate_minimum_hp(
                std::hint::black_box(dungeon.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::calculate_minimum_hp(
                std::hint::black_box(dungeon.clone()),
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
