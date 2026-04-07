// LeetCode 267: Palindrome Permutation II [Medium] (Premium)
// https://leetcode.com/problems/palindrome-permutation-ii/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 6.048us
//   AI solution avg: 2.823us
//   Ratio: AI 2.14x faster
//
// Given a string s, return all palindromic permutations (without duplicates).
// If s has no palindromic permutation, return an empty list.
//
// Example 1: s = "aabb" -> ["abba","baab"]
// Example 2: s = "abc"  -> []
//
// Constraints:
//   - 1 <= s.length <= 16
//   - s consists of only lowercase English letters
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0267.Palindrome%20Permutation%20II/README_EN.md
//
// Run:  cargo run --example lc_0267_palindrome_permutation_ii
// Test: cargo test --example lc_0267_palindrome_permutation_ii
// Bench: cargo test --example lc_0267_palindrome_permutation_ii bench -- --nocapture

/// My solution — DFS building outward from center (from doocs Python).
///
/// Count character frequencies. At most one odd-count character allowed
/// (becomes the center). Use DFS: pick a char with count >= 2, wrap
/// it around both ends, recurse until full length.
///
/// Time: O((n/2)!) in worst case
/// Space: O(n) for recursion + result
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all palindromic permutations of `s`.
        pub fn generate_palindromes(s: String) -> Vec<String> {
            let n = s.len();
            let mut cnt = [0u8; 26];
            for b in s.bytes() {
                cnt[(b - b'a') as usize] += 1;
            }
            // Find the middle character (if any odd-count char)
            let mut mid = String::new();
            for i in 0..26 {
                if cnt[i] & 1 == 1 {
                    if !mid.is_empty() {
                        // More than one odd-count char -> impossible
                        return Vec::new();
                    }
                    mid.push((b'a' + i as u8) as char);
                    cnt[i] -= 1;
                }
            }
            let mut ans: Vec<String> = Vec::new();
            Self::dfs(&mid, n, &mut cnt, &mut ans);
            ans
        }

        /// Build palindrome outward: `t` is the current partial palindrome.
        fn dfs(t: &str, n: usize, cnt: &mut [u8; 26], ans: &mut Vec<String>) {
            if t.len() == n {
                ans.push(t.to_string());
                return;
            }
            for i in 0..26 {
                if cnt[i] > 1 {
                    let c = (b'a' + i as u8) as char;
                    cnt[i] -= 2;
                    // Wrap c around both sides of t
                    let next = format!("{c}{t}{c}");
                    Self::dfs(&next, n, cnt, ans);
                    cnt[i] += 2;
                }
            }
        }
    }
}

/// AI solution — build half-palindrome, then mirror.
///
/// Collect half the characters (count/2 of each). Permute the half
/// (with dedup via sorted + skip-duplicate logic). For each half
/// permutation, mirror it and insert the middle char if needed.
///
/// Time: O((n/2)!)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all palindromic permutations of `s`.
        pub fn generate_palindromes(s: String) -> Vec<String> {
            let mut cnt = [0u8; 26];
            for b in s.bytes() {
                cnt[(b - b'a') as usize] += 1;
            }
            let mut mid: Option<u8> = None;
            let mut half: Vec<u8> = Vec::new();
            for i in 0..26 {
                if cnt[i] & 1 == 1 {
                    if mid.is_some() {
                        return Vec::new();
                    }
                    mid = Some(b'a' + i as u8);
                }
                // Add count/2 copies of this char to the half
                for _ in 0..cnt[i] / 2 {
                    half.push(b'a' + i as u8);
                }
            }
            // Generate unique permutations of `half`
            let mut ans: Vec<String> = Vec::new();
            let mut used = vec![false; half.len()];
            let mut perm: Vec<u8> = Vec::with_capacity(half.len());
            Self::permute(&half, &mut used, &mut perm, mid, &mut ans);
            ans
        }

        fn permute(
            half: &[u8],
            used: &mut [bool],
            perm: &mut Vec<u8>,
            mid: Option<u8>,
            ans: &mut Vec<String>,
        ) {
            if perm.len() == half.len() {
                // Build palindrome: perm + mid + perm.rev()
                let mut s: Vec<u8> = perm.clone();
                if let Some(m) = mid {
                    s.push(m);
                }
                s.extend(perm.iter().rev());
                ans.push(String::from_utf8(s).unwrap());
                return;
            }
            for i in 0..half.len() {
                if used[i] {
                    continue;
                }
                // Skip duplicates: if half[i] == half[i-1] and half[i-1] wasn't used
                if i > 0 && half[i] == half[i - 1] && !used[i - 1] {
                    continue;
                }
                used[i] = true;
                perm.push(half[i]);
                Self::permute(half, used, perm, mid, ans);
                perm.pop();
                used[i] = false;
            }
        }
    }
}

fn main() {
    println!("my 'aabb': {:?}", my_solution::Solution::generate_palindromes("aabb".into()));
    println!("ai 'abc': {:?}", ai_solution::Solution::generate_palindromes("abc".into()));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    fn to_set(v: Vec<String>) -> HashSet<String> {
        v.into_iter().collect()
    }

    macro_rules! test_palindrome_perm_ii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    let result = to_set(Sol::generate_palindromes("aabb".into()));
                    let expected: HashSet<String> =
                        ["abba", "baab"].iter().map(|s| s.to_string()).collect();
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_example_2_impossible() {
                    assert!(Sol::generate_palindromes("abc".into()).is_empty());
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Sol::generate_palindromes("a".into()), vec!["a"]);
                }

                #[test]
                fn test_two_same() {
                    assert_eq!(Sol::generate_palindromes("aa".into()), vec!["aa"]);
                }

                #[test]
                fn test_odd_center() {
                    let result = to_set(Sol::generate_palindromes("aab".into()));
                    let expected: HashSet<String> =
                        ["aba"].iter().map(|s| s.to_string()).collect();
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(
                        Sol::generate_palindromes("aaaa".into()),
                        vec!["aaaa"]
                    );
                }

                #[test]
                fn test_longer() {
                    let result = to_set(Sol::generate_palindromes("aabbcc".into()));
                    // Should have 6 palindromes: all permutations of "abc" mirrored
                    assert_eq!(result.len(), 6);
                    for s in &result {
                        let rev: String = s.chars().rev().collect();
                        assert_eq!(s, &rev, "{s} should be a palindrome");
                    }
                }
            }
        };
    }

    test_palindrome_perm_ii!(my, crate::my_solution::Solution);
    test_palindrome_perm_ii!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "aabbccdd".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::generate_palindromes(
                s.clone(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::generate_palindromes(
                s.clone(),
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
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
