// LeetCode 299: Bulls and Cows [Medium]
// https://leetcode.com/problems/bulls-and-cows/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 0.083us
//   AI solution avg: 0.099us
//   Ratio: My 1.18x faster
//
// Given secret and guess strings (digits only), return a hint "xAyB" where:
//   x = bulls (correct digit in correct position)
//   y = cows (correct digit in wrong position)
//
// Example 1: secret = "1807", guess = "7810" -> "1A3B"
// Example 2: secret = "1123", guess = "0111" -> "1A1B"
//
// Constraints:
//   - 1 <= secret.length, guess.length <= 1000
//   - secret.length == guess.length
//   - Both consist of digits only
//
// Run:  cargo run --example lc_0299_bulls_and_cows
// Test: cargo test --example lc_0299_bulls_and_cows
// Bench: cargo test --example lc_0299_bulls_and_cows bench -- --nocapture

struct Solution;

/// My solution — Two frequency arrays (doocs approach).
///
/// First pass: count bulls (matching positions). For non-bulls, build
/// digit frequency arrays for secret and guess. Second: cows = sum of
/// min(cnt_s[d], cnt_g[d]) for each digit d.
///
/// Time:  O(n)
/// Space: O(1) — fixed 10-element arrays
mod my_solution {
    use super::Solution;

    impl Solution {
        pub fn get_hint_my(secret: &str, guess: &str) -> String {
            let mut bulls = 0;
            let mut cnt_s = [0i32; 10];
            let mut cnt_g = [0i32; 10];

            for (s, g) in secret.bytes().zip(guess.bytes()) {
                if s == g {
                    bulls += 1;
                } else {
                    cnt_s[(s - b'0') as usize] += 1;
                    cnt_g[(g - b'0') as usize] += 1;
                }
            }

            let cows: i32 = (0..10).map(|i| cnt_s[i].min(cnt_g[i])).sum();

            format!("{bulls}A{cows}B")
        }
    }
}

/// AI solution — Single-pass with one counter array.
///
/// Use a single counter array. For each position:
/// - If digits match, increment bulls.
/// - Otherwise: if cnt[s] < 0, a previous guess used this digit -> cow++.
///   if cnt[g] > 0, a previous secret had this digit -> cow++.
///   Then increment cnt[s] and decrement cnt[g].
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    use super::Solution;

    impl Solution {
        pub fn get_hint_ai(secret: &str, guess: &str) -> String {
            let mut bulls = 0;
            let mut cows = 0;
            let mut cnt = [0i32; 10];

            for (s, g) in secret.bytes().zip(guess.bytes()) {
                if s == g {
                    bulls += 1;
                } else {
                    let si = (s - b'0') as usize;
                    let gi = (g - b'0') as usize;
                    // If secret digit was previously "needed" by guess
                    if cnt[si] < 0 {
                        cows += 1;
                    }
                    // If guess digit was previously "needed" by secret
                    if cnt[gi] > 0 {
                        cows += 1;
                    }
                    cnt[si] += 1;
                    cnt[gi] -= 1;
                }
            }

            format!("{bulls}A{cows}B")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_hint {
        ($name:ident, $secret:expr, $guess:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::get_hint_my($secret, $guess),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    Solution::get_hint_ai($secret, $guess),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    test_hint!(test_example_1, "1807", "7810", "1A3B");
    test_hint!(test_example_2, "1123", "0111", "1A1B");

    // All bulls
    test_hint!(test_all_bulls, "1234", "1234", "4A0B");

    // No bulls, no cows
    test_hint!(test_no_match, "1234", "5678", "0A0B");

    // All cows
    test_hint!(test_all_cows, "1234", "4321", "0A4B");

    // Single digit
    test_hint!(test_single_match, "1", "1", "1A0B");
    test_hint!(test_single_no_match, "1", "2", "0A0B");

    // Duplicate digits
    test_hint!(test_duplicates, "1122", "2211", "0A4B");
    test_hint!(test_dup_partial, "1122", "1222", "3A0B");

    // Cow count limited by min frequency
    test_hint!(test_freq_limit, "1111", "1000", "1A0B");

    // Longer string
    test_hint!(test_longer, "11234567", "01234568", "6A0B");
}

fn main() {
    println!("LeetCode 299: Bulls and Cows");
    println!(
        "my_solution: getHint(\"1807\", \"7810\") = {}",
        Solution::get_hint_my("1807", "7810")
    );
    println!(
        "ai_solution: getHint(\"1807\", \"7810\") = {}",
        Solution::get_hint_ai("1807", "7810")
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let secret = "1807180718071807180718071807";
        let guess = "7810781078107810781078107810";
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(Solution::get_hint_my(secret, guess));
            std::hint::black_box(Solution::get_hint_ai(secret, guess));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::get_hint_my(secret, guess));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(Solution::get_hint_ai(secret, guess));
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
