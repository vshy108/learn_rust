// LeetCode 293: Flip Game [Easy] (Premium)
// https://leetcode.com/problems/flip-game/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 14.270us
//   AI solution avg: 12.389us
//   Ratio: AI 1.15x faster
//
// Given a string with '+' and '-', return all possible states after flipping
// two consecutive "++" into "--". Return empty list if no valid move.
//
// Example 1: "++++" -> ["--++", "+--+", "++--"]
// Example 2: "+" -> []
//
// Constraints:
//   - 1 <= currentState.length <= 500
//   - currentState[i] is either '+' or '-'
//
// Run:  cargo run --example lc_0293_flip_game
// Test: cargo test --example lc_0293_flip_game
// Bench: cargo test --example lc_0293_flip_game bench -- --nocapture

struct Solution;

/// My solution — In-place mutation with swap-back (doocs approach).
///
/// Iterate through the string bytes, flip "++" to "--", record result,
/// then flip back. Avoids allocating a new string per iteration.
///
/// Time:  O(n^2) — n iterations, each producing an O(n) string
/// Space: O(n) per result string
mod my_solution {
    use super::Solution;

    impl Solution {
        pub fn generate_possible_next_moves_my(
            current_state: &str,
        ) -> Vec<String> {
            let mut s: Vec<u8> = current_state.bytes().collect();
            let mut ans = Vec::new();
            for i in 0..s.len().saturating_sub(1) {
                if s[i] == b'+' && s[i + 1] == b'+' {
                    s[i] = b'-';
                    s[i + 1] = b'-';
                    // SAFETY: only ASCII '+' and '-' characters
                    ans.push(String::from_utf8(s.clone()).unwrap());
                    s[i] = b'+';
                    s[i + 1] = b'+';
                }
            }
            ans
        }
    }
}

/// AI solution — Window-based with String::new construction.
///
/// For each "++" pair, build the result by concatenating slices:
/// prefix + "--" + suffix, avoiding mutation altogether.
///
/// Time:  O(n^2)
/// Space: O(n) per result string
mod ai_solution {
    use super::Solution;

    impl Solution {
        pub fn generate_possible_next_moves_ai(
            current_state: &str,
        ) -> Vec<String> {
            let s = current_state.as_bytes();
            let mut ans = Vec::new();
            for i in 0..s.len().saturating_sub(1) {
                if s[i] == b'+' && s[i + 1] == b'+' {
                    let mut result = String::with_capacity(s.len());
                    result.push_str(&current_state[..i]);
                    result.push_str("--");
                    result.push_str(&current_state[i + 2..]);
                    ans.push(result);
                }
            }
            ans
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
                let mut expected: Vec<String> =
                    $expected.iter().map(|s| s.to_string()).collect();
                expected.sort();

                let mut my_res =
                    Solution::generate_possible_next_moves_my($input);
                my_res.sort();
                assert_eq!(my_res, expected, "my_solution failed");

                let mut ai_res =
                    Solution::generate_possible_next_moves_ai($input);
                ai_res.sort();
                assert_eq!(ai_res, expected, "ai_solution failed");
            }
        };
    }

    test_flip!(test_example_1, "++++", vec!["--++", "+--+", "++--"]);
    test_flip!(test_example_2, "+", Vec::<&str>::new());

    // No consecutive "++"
    test_flip!(test_no_move, "+-+-", Vec::<&str>::new());

    // All plusses
    test_flip!(
        test_all_plus_3,
        "+++",
        vec!["--+", "+--"]
    );

    // Single pair
    test_flip!(test_single_pair, "++", vec!["--"]);

    // All minuses
    test_flip!(test_all_minus, "---", Vec::<&str>::new());

    // Mixed
    test_flip!(test_mixed, "-++--", vec!["-----"]);

    // Longer string
    test_flip!(
        test_longer,
        "++--++",
        vec!["----++", "++----"]
    );

    // Two characters, no move
    test_flip!(test_two_minus, "--", Vec::<&str>::new());

    // Empty-like (single char)
    test_flip!(test_single_minus, "-", Vec::<&str>::new());
}

fn main() {
    println!("LeetCode 293: Flip Game");
    let state = "++++";
    println!(
        "my_solution: {:?}",
        Solution::generate_possible_next_moves_my(state)
    );
    println!(
        "ai_solution: {:?}",
        Solution::generate_possible_next_moves_ai(state)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let state = "+".repeat(200);
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(
                Solution::generate_possible_next_moves_my(&state),
            );
            std::hint::black_box(
                Solution::generate_possible_next_moves_ai(&state),
            );
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                Solution::generate_possible_next_moves_my(&state),
            );
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                Solution::generate_possible_next_moves_ai(&state),
            );
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
