// LeetCode 294: Flip Game II [Medium] (Premium)
// https://leetcode.com/problems/flip-game-ii/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 2.221us
//   AI solution avg: 0.501us
//   Ratio: AI 4.43x faster
//
// You and your friend take turns flipping two consecutive "++" into "--".
// The game ends when a player can no longer make a move (that player loses).
// Return true if the starting player can guarantee a win.
//
// Example 1: "++++" -> true (flip middle to "+--+", opponent can't win)
// Example 2: "+" -> false
//
// Constraints:
//   - 1 <= currentState.length <= 60
//   - currentState[i] is '+' or '-'
//   - No more than 20 consecutive '+'
//
// Run:  cargo run --example lc_0294_flip_game_ii
// Test: cargo test --example lc_0294_flip_game_ii
// Bench: cargo test --example lc_0294_flip_game_ii bench -- --nocapture

use std::collections::HashMap;

struct Solution;

/// My solution — Bitmask DFS with memoization (doocs Solution 1).
///
/// Represent the state as a bitmask (bit i = 1 if position i is '+').
/// For each pair of consecutive '+' bits, flip them and recurse.
/// Current player wins if any move leads to the opponent losing.
///
/// Time:  O(2^n) with memoization reducing revisits
/// Space: O(2^n) for memo table
mod my_solution {
    use super::*;

    impl Solution {
        pub fn can_win_my(current_state: &str) -> bool {
            let n = current_state.len();
            let mut mask: u64 = 0;
            for (i, c) in current_state.bytes().enumerate() {
                if c == b'+' {
                    mask |= 1u64 << i;
                }
            }
            let mut memo: HashMap<u64, bool> = HashMap::new();
            Self::dfs_my(mask, n, &mut memo)
        }

        fn dfs_my(
            mask: u64,
            n: usize,
            memo: &mut HashMap<u64, bool>,
        ) -> bool {
            if let Some(&result) = memo.get(&mask) {
                return result;
            }
            for i in 0..n - 1 {
                // Check if positions i and i+1 are both '+'
                if (mask & (1u64 << i)) == 0 || (mask & (1u64 << (i + 1))) == 0 {
                    continue;
                }
                // Flip both bits (make the move)
                let next = mask ^ (1u64 << i) ^ (1u64 << (i + 1));
                // If opponent cannot win from next state, we win
                if !Self::dfs_my(next, n, memo) {
                    memo.insert(mask, true);
                    return true;
                }
            }
            memo.insert(mask, false);
            false
        }
    }
}

/// AI solution — Sprague-Grundy theorem (doocs Solution 2).
///
/// Split the string by '-' sequences into independent '+' runs.
/// Compute the Grundy number (nimber) for each run using the recurrence:
/// sg(k) = mex{ sg(j) XOR sg(k-j-2) : 0 <= j < k-1 }.
/// XOR all nimbers; if non-zero, first player wins.
///
/// Time:  O(n^2) for Grundy computation
/// Space: O(n)
mod ai_solution {
    use super::*;

    impl Solution {
        pub fn can_win_ai(current_state: &str) -> bool {
            let n = current_state.len();
            let s = current_state.as_bytes();
            let mut sg = vec![-1i32; n + 1];

            // Compute nimber for a run of `k` consecutive '+' chars
            fn win(k: usize, sg: &mut [i32], n: usize) -> i32 {
                if sg[k] != -1 {
                    return sg[k];
                }
                // Base cases: runs of 0 or 1 can't be played
                if k <= 1 {
                    sg[k] = 0;
                    return 0;
                }
                // mex of all reachable states
                let mut vis = vec![false; n + 1];
                for j in 0..k.saturating_sub(1) {
                    let val = (win(j, sg, n) ^ win(k - j - 2, sg, n)) as usize;
                    if val < vis.len() {
                        vis[val] = true;
                    }
                }
                for j in 0..vis.len() {
                    if !vis[j] {
                        sg[k] = j as i32;
                        return sg[k];
                    }
                }
                0
            }

            // XOR the nimbers of all consecutive '+' runs
            let mut ans = 0;
            let mut i = 0;
            while i < n {
                let j_start = i;
                while i < n && s[i] == b'+' {
                    i += 1;
                }
                let run_len = i - j_start;
                if run_len > 0 {
                    ans ^= win(run_len, &mut sg, n);
                }
                i += 1; // skip the '-'
            }
            ans > 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_flip {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::can_win_my($input),
                    $expected,
                    "my_solution failed for \"{}\"",
                    $input
                );
                assert_eq!(
                    Solution::can_win_ai($input),
                    $expected,
                    "ai_solution failed for \"{}\"",
                    $input
                );
            }
        };
    }

    test_flip!(test_example_1, "++++", true);
    test_flip!(test_example_2, "+", false);

    // Can't move: all '-'
    test_flip!(test_no_moves, "---", false);

    // Single pair -> first player wins (takes it, opponent stuck)
    test_flip!(test_single_pair, "++", true);

    // Three consecutive: "+++" -> can flip to "--+" or "+--", both
    // leave opponent with no pairs -> first player wins
    test_flip!(test_three, "+++", true);

    // Five consecutive
    test_flip!(test_five, "+++++", false);

    // Two separate pairs (Grundy: 1^1=0, first player loses)
    test_flip!(test_two_pairs, "++-++", false);

    // No consecutive pair
    test_flip!(test_alternating, "+-+-+-", false);

    // Single plus surrounded by minuses
    test_flip!(test_single_plus, "-+-", false);

    // Two separate singles
    test_flip!(test_two_singles, "+-+", false);

    // Two runs of 2 separated by '--' (Grundy: 1^1=0)
    test_flip!(test_complex, "++--++", false);
}

fn main() {
    println!("LeetCode 294: Flip Game II");
    for state in ["++++", "+", "++", "+++", "+++++"] {
        println!(
            "can_win(\"{}\") = {}",
            state,
            Solution::can_win_my(state)
        );
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let state = "++++++++++"; // 10 consecutive '+'
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(Solution::can_win_my(state));
            std::hint::black_box(Solution::can_win_ai(state));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::can_win_my(state));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::can_win_ai(state));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
