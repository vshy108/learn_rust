// LeetCode 131: Palindrome Partitioning [Medium]
// https://leetcode.com/problems/palindrome-partitioning/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 10.539us
//   AI solution avg: 10.736us
//   Ratio: ~tied
//
// Given a string s, partition s such that every substring of the partition is
// a palindrome. Return all possible palindrome partitionings of s.
//
// Constraints: 1 <= s.length <= 16, s contains only lowercase English letters
//
// Run:  cargo run --example lc_0131_palindrome_partitioning
// Test: cargo test --example lc_0131_palindrome_partitioning
// Bench: cargo test --example lc_0131_palindrome_partitioning bench -- --nocapture

/// My solution — backtracking, check palindrome inline at each step.
///
/// Time: O(n * 2^n) in the worst case.
/// Space: O(n) recursion depth + output storage.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all palindrome partitions.
        pub fn partition(s: String) -> Vec<Vec<String>> {
            let s = s.as_bytes();
            let mut result = Vec::new();
            let mut path = Vec::new();
            Self::backtrack(s, 0, &mut path, &mut result);
            result
        }

        fn backtrack(
            s: &[u8],
            start: usize,
            path: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if start == s.len() {
                result.push(path.clone());
                return;
            }
            for end in start + 1..=s.len() {
                if Self::is_palindrome(&s[start..end]) {
                    // Convert valid palindrome substring and recurse
                    path.push(
                        std::str::from_utf8(&s[start..end])
                            .expect("input is ASCII")
                            .to_owned(),
                    );
                    Self::backtrack(s, end, path, result);
                    path.pop();
                }
            }
        }

        fn is_palindrome(s: &[u8]) -> bool {
            let (mut l, mut r) = (0, s.len().saturating_sub(1));
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }
    }
}

/// AI solution — precompute palindrome table via DP, then backtrack.
///
/// Time: O(n * 2^n) worst case but palindrome checks are O(1).
/// Space: O(n^2) for the DP table.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all palindrome partitions using precomputed DP table.
        pub fn partition(s: String) -> Vec<Vec<String>> {
            let n = s.len();
            let bytes = s.as_bytes();
            // is_pal[i][j] = true if s[i..=j] is a palindrome
            let mut is_pal = vec![vec![false; n]; n];
            for i in (0..n).rev() {
                for j in i..n {
                    is_pal[i][j] = bytes[i] == bytes[j] && (j - i < 2 || is_pal[i + 1][j - 1]);
                }
            }

            let mut result = Vec::new();
            let mut path = Vec::new();
            Self::backtrack(bytes, 0, &is_pal, &mut path, &mut result);
            result
        }

        fn backtrack(
            s: &[u8],
            start: usize,
            is_pal: &[Vec<bool>],
            path: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if start == s.len() {
                result.push(path.clone());
                return;
            }
            for end in start..s.len() {
                if is_pal[start][end] {
                    path.push(
                        std::str::from_utf8(&s[start..=end])
                            .expect("input is ASCII")
                            .to_owned(),
                    );
                    Self::backtrack(s, end + 1, is_pal, path, result);
                    path.pop();
                }
            }
        }
    }
}

fn main() {
    let my = my_solution::Solution::partition("aab".into());
    let ai = ai_solution::Solution::partition("aab".into());
    println!("palindrome partitions: my={my:?}");
    println!("palindrome partitions: ai={ai:?}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_partition {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                fn sorted(mut v: Vec<Vec<String>>) -> Vec<Vec<String>> {
                    v.sort();
                    v
                }

                #[test]
                fn test_example1() {
                    let result = sorted(<$solution>::partition("aab".into()));
                    let expected = sorted(vec![
                        vec!["a".into(), "a".into(), "b".into()],
                        vec!["aa".into(), "b".into()],
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_single() {
                    let result = <$solution>::partition("a".into());
                    assert_eq!(result, vec![vec!["a".to_string()]]);
                }

                #[test]
                fn test_all_same() {
                    let result = <$solution>::partition("aaa".into());
                    assert_eq!(result.len(), 4); // a|a|a, a|aa, aa|a, aaa
                }

                #[test]
                fn test_no_multi_char_palindrome() {
                    let result = <$solution>::partition("abc".into());
                    assert_eq!(
                        result,
                        vec![vec!["a".to_string(), "b".to_string(), "c".to_string(),]]
                    );
                }
            }
        };
    }

    test_partition!(my, crate::my_solution::Solution);
    test_partition!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "aababaabc".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::partition(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::partition(
                std::hint::black_box(s.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
