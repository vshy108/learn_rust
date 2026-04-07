// LeetCode 72: Edit Distance [Medium]
// https://leetcode.com/problems/edit-distance/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 4.692us
//   AI solution avg: 2.907us
//   Ratio: AI 1.61x faster
//
// Given two strings word1 and word2, return the minimum number of operations
// (insert, delete, replace) to convert word1 into word2.
//
// Constraints: 0 <= word1.length, word2.length <= 500
//
// Run:  cargo run --example lc_0072_edit_distance
// Test: cargo test --example lc_0072_edit_distance
// Bench: cargo test --example lc_0072_edit_distance bench -- --nocapture

/// My solution — 2D DP table.
///
/// dp[i][j] = min operations to convert word1[..i] to word2[..j].
///
/// Time: O(m * n) where m, n = lengths of word1, word2.
/// Space: O(m * n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute minimum edit distance between two strings.
        ///
        /// # Arguments
        /// * `word1` - source string
        /// * `word2` - target string
        ///
        /// # Returns
        /// Minimum number of insert/delete/replace operations
        pub fn min_distance(word1: String, word2: String) -> i32 {
            let w1 = word1.as_bytes();
            let w2 = word2.as_bytes();
            let m = w1.len();
            let n = w2.len();

            // dp[i][j] = edit distance for word1[..i] -> word2[..j]
            let mut dp = vec![vec![0i32; n + 1]; m + 1];

            // Base cases: converting to/from empty string
            for i in 0..=m {
                dp[i][0] = i as i32;
            }
            for j in 0..=n {
                dp[0][j] = j as i32;
            }

            for i in 1..=m {
                for j in 1..=n {
                    if w1[i - 1] == w2[j - 1] {
                        dp[i][j] = dp[i - 1][j - 1]; // chars match, no op
                    } else {
                        dp[i][j] = 1 + dp[i - 1][j - 1] // replace
                            .min(dp[i - 1][j])           // delete
                            .min(dp[i][j - 1]);           // insert
                    }
                }
            }

            dp[m][n]
        }
    }
}

/// AI solution — space-optimized DP using single row.
///
/// Only need previous row to compute current row.
///
/// Time: O(m * n).
/// Space: O(min(m, n)).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compute minimum edit distance between two strings.
        ///
        /// # Arguments
        /// * `word1` - source string
        /// * `word2` - target string
        ///
        /// # Returns
        /// Minimum number of insert/delete/replace operations
        pub fn min_distance(word1: String, word2: String) -> i32 {
            let (w1, w2) = if word1.len() < word2.len() {
                (word2.as_bytes(), word1.as_bytes())
            } else {
                (word1.as_bytes(), word2.as_bytes())
            };
            let m = w1.len();
            let n = w2.len();

            // Use shorter string as the DP row dimension
            let mut prev = Vec::with_capacity(n + 1);
            for j in 0..=n {
                prev.push(j as i32);
            }

            for i in 1..=m {
                let mut curr = vec![0i32; n + 1];
                curr[0] = i as i32;
                for j in 1..=n {
                    if w1[i - 1] == w2[j - 1] {
                        curr[j] = prev[j - 1];
                    } else {
                        curr[j] = 1 + prev[j - 1].min(prev[j]).min(curr[j - 1]);
                    }
                }
                prev = curr;
            }

            prev[n]
        }
    }
}

fn main() {
    let cases = [("horse", "ros"), ("intention", "execution")];
    for (w1, w2) in cases {
        let my = my_solution::Solution::min_distance(w1.to_string(), w2.to_string());
        let ai = ai_solution::Solution::min_distance(w1.to_string(), w2.to_string());
        println!("{w1} -> {w2}: my={my}, ai={ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_min_distance {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_horse_ros() {
                    assert_eq!(
                        <$solution>::min_distance(
                            "horse".to_string(),
                            "ros".to_string(),
                        ),
                        3
                    );
                }

                #[test]
                fn test_intention_execution() {
                    assert_eq!(
                        <$solution>::min_distance(
                            "intention".to_string(),
                            "execution".to_string(),
                        ),
                        5
                    );
                }

                #[test]
                fn test_empty_to_abc() {
                    assert_eq!(
                        <$solution>::min_distance(
                            "".to_string(),
                            "abc".to_string(),
                        ),
                        3
                    );
                }

                #[test]
                fn test_same() {
                    assert_eq!(
                        <$solution>::min_distance(
                            "abc".to_string(),
                            "abc".to_string(),
                        ),
                        0
                    );
                }

                #[test]
                fn test_both_empty() {
                    assert_eq!(
                        <$solution>::min_distance(
                            "".to_string(),
                            "".to_string(),
                        ),
                        0
                    );
                }
            }
        };
    }

    test_min_distance!(my, crate::my_solution::Solution);
    test_min_distance!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let w1 = "algorithm".to_string();
        let w2 = "altruistic".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::min_distance(
                std::hint::black_box(w1.clone()),
                std::hint::black_box(w2.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::min_distance(
                std::hint::black_box(w1.clone()),
                std::hint::black_box(w2.clone()),
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
