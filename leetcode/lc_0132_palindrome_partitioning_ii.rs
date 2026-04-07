// LeetCode 132: Palindrome Partitioning II [Hard]
// https://leetcode.com/problems/palindrome-partitioning-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 1.266us
//   AI solution avg: 0.121us
//   Ratio: AI 10.45x faster
//
// Given a string s, return the minimum number of cuts needed so that every
// substring of the partition is a palindrome.
//
// Constraints: 1 <= s.length <= 2000, s consists of lowercase English letters
//
// Run:  cargo run --example lc_0132_palindrome_partitioning_ii
// Test: cargo test --example lc_0132_palindrome_partitioning_ii
// Bench: cargo test --example lc_0132_palindrome_partitioning_ii bench -- --nocapture

/// My solution — DP with precomputed palindrome table.
///
/// Time: O(n^2).
/// Space: O(n^2) for palindrome table + O(n) for cuts array.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum cuts for palindrome partitioning.
        pub fn min_cut(s: String) -> i32 {
            let n = s.len();
            let bytes = s.as_bytes();
            // is_pal[i][j] = true if s[i..=j] is palindrome
            let mut is_pal = vec![vec![false; n]; n];
            for i in (0..n).rev() {
                for j in i..n {
                    is_pal[i][j] = bytes[i] == bytes[j]
                        && (j - i < 2 || is_pal[i + 1][j - 1]);
                }
            }

            // cuts[i] = min cuts for s[0..=i]
            let mut cuts = vec![0i32; n];
            for i in 0..n {
                if is_pal[0][i] {
                    cuts[i] = 0;
                    continue;
                }
                cuts[i] = i as i32; // worst case: cut every char
                for j in 1..=i {
                    if is_pal[j][i] {
                        cuts[i] = cuts[i].min(cuts[j - 1] + 1);
                    }
                }
            }
            cuts[n - 1]
        }
    }
}

/// AI solution — expand-around-center palindrome detection + 1D DP.
///
/// Time: O(n^2).
/// Space: O(n) — no 2D palindrome table needed.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum cuts using expand-around-center.
        pub fn min_cut(s: String) -> i32 {
            let n = s.len();
            let bytes = s.as_bytes();
            // cuts[i] = min cuts for s[0..i] (prefix of length i)
            let mut cuts: Vec<i32> = (0..=n as i32).map(|i| i - 1).collect();

            for center in 0..n {
                // Odd-length palindromes
                Self::expand(bytes, &mut cuts, center, center);
                // Even-length palindromes
                if center + 1 < n {
                    Self::expand(bytes, &mut cuts, center, center + 1);
                }
            }
            cuts[n]
        }

        fn expand(bytes: &[u8], cuts: &mut [i32], mut left: usize, mut right: usize) {
            while right < bytes.len() && bytes[left] == bytes[right] {
                // s[left..=right] is a palindrome
                cuts[right + 1] = cuts[right + 1].min(cuts[left] + 1);
                if left == 0 { break; }
                left -= 1;
                right += 1;
            }
        }
    }
}

fn main() {
    let my = my_solution::Solution::min_cut("aab".into());
    let ai = ai_solution::Solution::min_cut("aab".into());
    println!("min cuts: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_min_cut {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::min_cut("aab".into()), 1);
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::min_cut("a".into()), 0);
                }

                #[test]
                fn test_already_palindrome() {
                    assert_eq!(<$solution>::min_cut("aba".into()), 0);
                }

                #[test]
                fn test_all_different() {
                    assert_eq!(<$solution>::min_cut("abcd".into()), 3);
                }

                #[test]
                fn test_repeated() {
                    assert_eq!(<$solution>::min_cut("aaaa".into()), 0);
                }

                #[test]
                fn test_complex() {
                    assert_eq!(<$solution>::min_cut("abacbc".into()), 1);
                }
            }
        };
    }

    test_min_cut!(my, crate::my_solution::Solution);
    test_min_cut!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "abacbcababacbc".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::min_cut(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::min_cut(
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
