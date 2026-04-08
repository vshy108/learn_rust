// LeetCode 292: Nim Game [Easy]
// https://leetcode.com/problems/nim-game/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 0.098us
//   AI solution avg: 0.109us
//   Ratio: My 1.11x faster
//
// You and your friend take turns removing 1-3 stones from a heap (you go first).
// The person who takes the last stone wins. Given n stones, return true if you
// can win assuming both play optimally.
//
// Example 1: n = 4 -> false
// Example 2: n = 1 -> true
// Example 3: n = 2 -> true
//
// Constraints: 1 <= n <= 2^31 - 1
//
// Key insight: If n is a multiple of 4, the first player always loses,
// because any move (1,2,3) leaves a non-multiple of 4 for the opponent
// who can then always reduce back to a multiple of 4.
//
// Run:  cargo run --example lc_0292_nim_game
// Test: cargo test --example lc_0292_nim_game
// Bench: cargo test --example lc_0292_nim_game bench -- --nocapture

struct Solution;

/// My solution — Math: n % 4 != 0 (doocs).
///
/// Time:  O(1)
/// Space: O(1)
mod my_solution {
    use super::Solution;

    impl Solution {
        pub fn can_win_nim_my(n: i32) -> bool {
            // If n is divisible by 4, first player loses
            n % 4 != 0
        }
    }
}

/// AI solution — Bitwise: check last two bits.
///
/// n % 4 == 0 iff the last two bits are 00. So n & 3 != 0.
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    use super::Solution;

    impl Solution {
        pub fn can_win_nim_ai(n: i32) -> bool {
            n & 3 != 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_nim {
        ($name:ident, $n:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::can_win_nim_my($n),
                    $expected,
                    "my_solution failed for n={}",
                    $n
                );
                assert_eq!(
                    Solution::can_win_nim_ai($n),
                    $expected,
                    "ai_solution failed for n={}",
                    $n
                );
            }
        };
    }

    test_nim!(test_example_1, 4, false);
    test_nim!(test_example_2, 1, true);
    test_nim!(test_example_3, 2, true);
    test_nim!(test_3, 3, true);
    test_nim!(test_5, 5, true);
    test_nim!(test_6, 6, true);
    test_nim!(test_7, 7, true);
    test_nim!(test_8, 8, false);
    test_nim!(test_12, 12, false);
    test_nim!(test_100, 100, false);
    test_nim!(test_101, 101, true);
    test_nim!(test_max, i32::MAX, true); // 2^31 - 1 is odd
}

fn main() {
    println!("LeetCode 292: Nim Game");
    for n in [1, 2, 3, 4, 5, 8] {
        println!(
            "can_win_nim({}) = my:{}, ai:{}",
            n,
            Solution::can_win_nim_my(n),
            Solution::can_win_nim_ai(n),
        );
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let iterations = 100_000;

        // Warm up
        for n in 1..=1_000 {
            std::hint::black_box(Solution::can_win_nim_my(n));
            std::hint::black_box(Solution::can_win_nim_ai(n));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for n in 1..=100 {
                std::hint::black_box(Solution::can_win_nim_my(n));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for n in 1..=100 {
                std::hint::black_box(Solution::can_win_nim_ai(n));
            }
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
