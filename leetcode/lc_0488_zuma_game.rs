// LeetCode 488: Zuma Game [Hard]
// https://leetcode.com/problems/zuma-game/
//
// Given a board of colored balls and balls in your hand, find the minimum
// number of hand balls to insert to clear the board. When 3+ consecutive
// same-colored balls form, they are removed. Return -1 if impossible.
//
// Colors: R (red), Y (yellow), B (blue), G (green), W (white)
//
// Examples:
//   board="WRRBBW", hand="RB" -> -1
//   board="WWRRBBWW", hand="WRBRW" -> 2
//   board="G", hand="GGGGG" -> 2
//
// Constraints:
//   1 <= board.length <= 16
//   1 <= hand.length <= 5
//   board and hand consist of 'R', 'Y', 'B', 'G', 'W'
//
// Run:  cargo run --example lc_0488_zuma_game
// Test: cargo test --example lc_0488_zuma_game
// Bench: cargo test --example lc_0488_zuma_game bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 1.192us (BFS with string state)
//   AI solution: avg 1.234us (BFS with Vec<u8> state)
//   Ratio: ~1.0x

/// My solution module — BFS with string state.
///
/// BFS over (board, hand) states. For each state, try inserting each
/// unique hand ball at each position, remove 3+ consecutive groups,
/// and check if board is empty.
///
/// Time:  O(exponential, but bounded by small constraints)
/// Space: O(states visited)

#[allow(dead_code)]
mod my_solution {
    use std::collections::{HashSet, VecDeque};

    pub struct Solution;
    impl Solution {
        pub fn find_min_step(board: String, hand: String) -> i32 {
            let hand_bytes: Vec<u8> = {
                let mut h: Vec<u8> = hand.bytes().collect();
                h.sort_unstable();
                h
            };

            let mut visited: HashSet<(String, Vec<u8>)> = HashSet::new();
            let mut queue: VecDeque<(String, Vec<u8>)> = VecDeque::new();

            visited.insert((board.clone(), hand_bytes.clone()));
            queue.push_back((board, hand_bytes));
            let mut steps = 0;

            while !queue.is_empty() {
                steps += 1;
                for _ in 0..queue.len() {
                    let (state, balls) = queue.pop_front().unwrap();

                    // Try each unique ball from hand
                    for bi in 0..balls.len() {
                        // Skip duplicate balls in hand
                        if bi > 0 && balls[bi] == balls[bi - 1] {
                            continue;
                        }
                        let ball = balls[bi];

                        // Remove this ball from hand
                        let mut remaining = balls.clone();
                        remaining.remove(bi);

                        // Try inserting at each position
                        let state_bytes = state.as_bytes();
                        for pos in 0..=state_bytes.len() {
                            // Pruning: only insert next to same color
                            if pos > 0
                                && state_bytes[pos - 1] != ball
                                && (pos >= state_bytes.len() || state_bytes[pos] != ball)
                            {
                                continue;
                            }

                            let mut new_board = String::with_capacity(state.len() + 1);
                            new_board.push_str(&state[..pos]);
                            new_board.push(ball as char);
                            new_board.push_str(&state[pos..]);

                            let cleaned = Self::remove_consecutive(&new_board);

                            if cleaned.is_empty() {
                                return steps;
                            }

                            if !visited.contains(&(cleaned.clone(), remaining.clone())) {
                                visited.insert((cleaned.clone(), remaining.clone()));
                                queue.push_back((cleaned, remaining.clone()));
                            }
                        }
                    }
                }
            }
            -1
        }

        /// Remove all groups of 3+ consecutive same-colored balls, repeatedly.
        fn remove_consecutive(s: &str) -> String {
            let mut result = s.to_string();
            loop {
                let mut new_result = String::new();
                let bytes = result.as_bytes();
                let mut i = 0;
                let mut changed = false;
                while i < bytes.len() {
                    let mut j = i;
                    while j < bytes.len() && bytes[j] == bytes[i] {
                        j += 1;
                    }
                    if j - i >= 3 {
                        changed = true;
                    } else {
                        new_result.push_str(&result[i..j]);
                    }
                    i = j;
                }
                if !changed {
                    return result;
                }
                result = new_result;
            }
        }
    }
}

/// AI solution module — BFS with same pruning strategy.
///
/// Same BFS approach but with additional pruning: only insert ball
/// adjacent to matching color on the board.
///
/// Time:  O(exponential, bounded)
/// Space: O(states visited)
mod ai_solution {
    use std::collections::{HashSet, VecDeque};

    pub struct Solution;
    impl Solution {
        pub fn find_min_step(board: String, hand: String) -> i32 {
            // Sort hand for consistent state representation
            let mut hand_sorted: Vec<u8> = hand.bytes().collect();
            hand_sorted.sort_unstable();

            let mut visited: HashSet<(Vec<u8>, Vec<u8>)> = HashSet::new();
            let board_bytes: Vec<u8> = board.bytes().collect();
            visited.insert((board_bytes.clone(), hand_sorted.clone()));

            let mut queue: VecDeque<(Vec<u8>, Vec<u8>)> = VecDeque::new();
            queue.push_back((board_bytes, hand_sorted));
            let mut level = 0;

            while !queue.is_empty() {
                level += 1;
                let size = queue.len();
                for _ in 0..size {
                    let (brd, hnd) = queue.pop_front().unwrap();

                    for hi in 0..hnd.len() {
                        if hi > 0 && hnd[hi] == hnd[hi - 1] {
                            continue;
                        }
                        let ball = hnd[hi];
                        let mut remaining = hnd.clone();
                        remaining.remove(hi);

                        // Insert ball at each valid position
                        for pos in 0..=brd.len() {
                            // Only insert if adjacent to same color
                            let left_match = pos > 0 && brd[pos - 1] == ball;
                            let right_match = pos < brd.len() && brd[pos] == ball;
                            if !left_match && !right_match {
                                continue;
                            }

                            let mut new_brd = Vec::with_capacity(brd.len() + 1);
                            new_brd.extend_from_slice(&brd[..pos]);
                            new_brd.push(ball);
                            new_brd.extend_from_slice(&brd[pos..]);

                            let cleaned = Self::clean(&new_brd);

                            if cleaned.is_empty() {
                                return level;
                            }

                            if !remaining.is_empty()
                                && !visited.contains(&(cleaned.clone(), remaining.clone()))
                            {
                                visited.insert((cleaned.clone(), remaining.clone()));
                                queue.push_back((cleaned, remaining.clone()));
                            }
                        }
                    }
                }
            }
            -1
        }

        fn clean(board: &[u8]) -> Vec<u8> {
            let mut result: Vec<u8> = board.to_vec();
            loop {
                let mut new_result: Vec<u8> = Vec::new();
                let mut i = 0;
                let mut changed = false;
                while i < result.len() {
                    let mut j = i;
                    while j < result.len() && result[j] == result[i] {
                        j += 1;
                    }
                    if j - i >= 3 {
                        changed = true;
                    } else {
                        new_result.extend_from_slice(&result[i..j]);
                    }
                    i = j;
                }
                if !changed {
                    return result;
                }
                result = new_result;
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_impossible() {
            assert_eq!(Solution::find_min_step("WRRBBW".into(), "RB".into()), -1);
        }
        #[test]
        fn test_two_moves() {
            assert_eq!(
                Solution::find_min_step("WWRRBBWW".into(), "WRBRW".into()),
                2
            );
        }
        #[test]
        fn test_g() {
            assert_eq!(Solution::find_min_step("G".into(), "GGGGG".into()), 2);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String, String) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box("G".into()),
            std::hint::black_box("GGGGG".into()),
        ));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::find_min_step);
    bench_solution("AI solution", ai_solution::Solution::find_min_step);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_min_step("WWRRBBWW".into(), "WRBRW".into())
    );
}
