// LeetCode 87: Scramble String [Hard]
// https://leetcode.com/problems/scramble-string/
//
// First benchmark: 10_000 iterations, 2025-07-08
//   My solution avg: 8.450us
//   AI solution avg: 18.285us
//   Ratio: My 2.16x faster
//
// We can scramble a string by splitting it at a random index, then swapping
// (or not) the two halves, and recursively doing the same for each half.
// Given s1 and s2, return true if s2 is a scrambled string of s1.
//
// Constraints: s1.length == s2.length, 1 <= s1.length <= 30, lowercase English
//
// Run:  cargo run --example lc_0087_scramble_string
// Test: cargo test --example lc_0087_scramble_string
// Bench: cargo test --example lc_0087_scramble_string bench -- --nocapture

/// My solution — recursive with memoization using HashMap.
///
/// Try every split point: check if substrings match directly or swapped.
///
/// Time: O(n^4) with memoization.
/// Space: O(n^3) for memo table.
mod my_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Check if s2 is a scrambled string of s1.
        ///
        /// # Arguments
        /// * `s1` - original string
        /// * `s2` - potentially scrambled string
        ///
        /// # Returns
        /// true if s2 is a scramble of s1
        pub fn is_scramble(s1: String, s2: String) -> bool {
            let mut memo = HashMap::new();
            Self::check(s1.as_bytes(), s2.as_bytes(), &mut memo)
        }

        fn check(
            s1: &[u8],
            s2: &[u8],
            memo: &mut HashMap<(Vec<u8>, Vec<u8>), bool>,
        ) -> bool {
            if s1 == s2 {
                return true;
            }
            if s1.len() != s2.len() {
                return false;
            }
            // Quick character frequency check
            let mut freq = [0i32; 26];
            for (&a, &b) in s1.iter().zip(s2.iter()) {
                freq[(a - b'a') as usize] += 1;
                freq[(b - b'a') as usize] -= 1;
            }
            if freq.iter().any(|&f| f != 0) {
                return false;
            }

            let key = (s1.to_vec(), s2.to_vec());
            if let Some(&cached) = memo.get(&key) {
                return cached;
            }

            let n = s1.len();
            let mut result = false;
            for i in 1..n {
                // No swap: s1[..i] matches s2[..i] and s1[i..] matches s2[i..]
                if Self::check(&s1[..i], &s2[..i], memo)
                    && Self::check(&s1[i..], &s2[i..], memo)
                {
                    result = true;
                    break;
                }
                // Swap: s1[..i] matches s2[n-i..] and s1[i..] matches s2[..n-i]
                if Self::check(&s1[..i], &s2[n - i..], memo)
                    && Self::check(&s1[i..], &s2[..n - i], memo)
                {
                    result = true;
                    break;
                }
            }

            memo.insert(key, result);
            result
        }
    }
}

/// AI solution — 3D DP.
///
/// dp[len][i][j] = true if s1[i..i+len] is a scramble of s2[j..j+len].
///
/// Time: O(n^4).
/// Space: O(n^3).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check if s2 is a scrambled string of s1.
        ///
        /// # Arguments
        /// * `s1` - original string
        /// * `s2` - potentially scrambled string
        ///
        /// # Returns
        /// true if s2 is a scramble of s1
        pub fn is_scramble(s1: String, s2: String) -> bool {
            let b1 = s1.as_bytes();
            let b2 = s2.as_bytes();
            let n = b1.len();
            if n != b2.len() {
                return false;
            }

            // dp[len][i][j]: can s1[i..i+len] scramble to s2[j..j+len]?
            let mut dp = vec![vec![vec![false; n]; n]; n + 1];

            // Base case: single characters
            for i in 0..n {
                for j in 0..n {
                    dp[1][i][j] = b1[i] == b2[j];
                }
            }

            // Fill for lengths 2..=n
            for len in 2..=n {
                for i in 0..=n - len {
                    for j in 0..=n - len {
                        for k in 1..len {
                            // No swap
                            if dp[k][i][j] && dp[len - k][i + k][j + k] {
                                dp[len][i][j] = true;
                                break;
                            }
                            // Swap
                            if dp[k][i][j + len - k] && dp[len - k][i + k][j] {
                                dp[len][i][j] = true;
                                break;
                            }
                        }
                    }
                }
            }

            dp[n][0][0]
        }
    }
}

fn main() {
    let cases = [("great", "rgeat"), ("abcde", "caebd"), ("a", "a")];
    for (s1, s2) in cases {
        let my = my_solution::Solution::is_scramble(s1.to_string(), s2.to_string());
        let ai = ai_solution::Solution::is_scramble(s1.to_string(), s2.to_string());
        println!("{s1} vs {s2}: my={my}, ai={ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_scramble {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_true1() {
                    assert!(<$solution>::is_scramble(
                        "great".to_string(),
                        "rgeat".to_string(),
                    ));
                }

                #[test]
                fn test_false1() {
                    assert!(!<$solution>::is_scramble(
                        "abcde".to_string(),
                        "caebd".to_string(),
                    ));
                }

                #[test]
                fn test_single_char() {
                    assert!(<$solution>::is_scramble(
                        "a".to_string(),
                        "a".to_string(),
                    ));
                }

                #[test]
                fn test_same() {
                    assert!(<$solution>::is_scramble(
                        "abc".to_string(),
                        "abc".to_string(),
                    ));
                }
            }
        };
    }

    test_scramble!(my, crate::my_solution::Solution);
    test_scramble!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 10_000;
        let s1 = "abcdefgh".to_string();
        let s2 = "hgfedcba".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_scramble(
                std::hint::black_box(s1.clone()),
                std::hint::black_box(s2.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_scramble(
                std::hint::black_box(s1.clone()),
                std::hint::black_box(s2.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
