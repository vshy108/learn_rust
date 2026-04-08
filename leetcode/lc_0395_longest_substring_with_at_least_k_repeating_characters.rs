// LeetCode 395: Longest Substring with At Least K Repeating Characters [Medium]
// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/
//
// Benchmark: 100,000 iterations (2026-04-08)
//   My solution (divide & conquer):   avg 13.55us
//   AI solution (sliding window x26): avg 378.93us
//   -> My solution is 27.96x faster
//
// Given a string `s` and an integer `k`, return the length of the longest
// substring of `s` such that the frequency of each character in this
// substring is >= k. If no such substring exists, return 0.
//
// Examples:
//   s = "aaabb", k = 3 -> 3  ("aaa")
//   s = "ababbc", k = 2 -> 5 ("ababb")
//
// Constraints:
//   - 1 <= s.length <= 10^4
//   - s consists of only lowercase English letters
//   - 1 <= k <= 10^5
//
// Run:  cargo run --example lc_0395_longest_substring_with_at_least_k_repeating_characters
// Test: cargo test --example lc_0395_longest_substring_with_at_least_k_repeating_characters
// Bench: cargo test --example lc_0395_longest_substring_with_at_least_k_repeating_characters bench -- --nocapture

/// My solution module — divide and conquer (recursive split).
///
/// Count character frequencies. Find a character that appears < k times;
/// it cannot be part of any valid substring, so split on it and recurse
/// on the resulting segments. If all characters meet the threshold, the
/// entire range is valid.
///
/// Time:  O(n * 26) worst case — at most 26 levels of recursion, each O(n)
/// Space: O(n) recursion stack
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the longest substring where every char appears >= k times.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of lowercase letters
        /// * `k` - Minimum frequency threshold
        ///
        /// # Returns
        ///
        /// Length of the longest valid substring
        pub fn longest_substring(s: String, k: i32) -> i32 {
            let bytes = s.as_bytes();
            Self::dfs(bytes, k as usize)
        }

        /// Recursive helper operating on a byte slice.
        fn dfs(bytes: &[u8], k: usize) -> i32 {
            if bytes.is_empty() {
                return 0;
            }

            // Count frequencies
            let mut cnt = [0u32; 26];
            for &b in bytes {
                cnt[(b - b'a') as usize] += 1;
            }

            // Find a character that violates the threshold
            let split_char = cnt.iter().enumerate().find_map(|(i, &c)| {
                if c > 0 && (c as usize) < k {
                    Some(b'a' + i as u8)
                } else {
                    None
                }
            });

            match split_char {
                None => bytes.len() as i32, // all chars meet threshold
                Some(ch) => {
                    // Split on `ch` and recurse on each segment
                    bytes
                        .split(|&b| b == ch)
                        .map(|seg| Self::dfs(seg, k))
                        .max()
                        .unwrap_or(0)
                }
            }
        }
    }
}

/// AI solution module — sliding window with unique-count enumeration.
///
/// For each target number of unique characters `t` in [1, 26], use a
/// sliding window to find the longest substring with exactly `t` unique
/// chars where all of them appear >= k times.
///
/// This avoids recursion and achieves a predictable O(26 * n) runtime.
///
/// Time:  O(26 * n)
/// Space: O(1) — fixed 26-element array
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the longest substring where every char appears >= k times.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string of lowercase letters
        /// * `k` - Minimum frequency threshold
        ///
        /// # Returns
        ///
        /// Length of the longest valid substring
        pub fn longest_substring(s: String, k: i32) -> i32 {
            let bytes = s.as_bytes();
            let k = k as u32;
            let mut ans = 0i32;

            // Try each possible number of unique characters in the window
            for target_unique in 1..=26u32 {
                let mut cnt = [0u32; 26];
                let mut unique = 0u32; // distinct chars in window
                let mut at_least_k = 0u32; // chars with count >= k
                let mut left = 0usize;

                for right in 0..bytes.len() {
                    let ri = (bytes[right] - b'a') as usize;
                    cnt[ri] += 1;
                    if cnt[ri] == 1 {
                        unique += 1;
                    }
                    if cnt[ri] == k {
                        at_least_k += 1;
                    }

                    // Shrink window when we have too many unique chars
                    while unique > target_unique {
                        let li = (bytes[left] - b'a') as usize;
                        if cnt[li] == k {
                            at_least_k -= 1;
                        }
                        cnt[li] -= 1;
                        if cnt[li] == 0 {
                            unique -= 1;
                        }
                        left += 1;
                    }

                    // Valid window: exactly target_unique chars, all >= k
                    if unique == target_unique && at_least_k == target_unique {
                        ans = ans.max((right - left + 1) as i32);
                    }
                }
            }
            ans
        }
    }
}

fn main() {
    let cases = [("aaabb", 3, 3), ("ababbc", 2, 5)];

    println!("--- My Solution ---");
    for (s, k, expected) in &cases {
        let result = my_solution::Solution::longest_substring(s.to_string(), *k);
        println!("  s={s:?}, k={k} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, k, expected) in &cases {
        let result = ai_solution::Solution::longest_substring(s.to_string(), *k);
        println!("  s={s:?}, k={k} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_longest {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
                }

                #[test]
                fn test_all_same() {
                    assert_eq!(Solution::longest_substring("aaaa".to_string(), 2), 4);
                }

                #[test]
                fn test_no_valid() {
                    assert_eq!(Solution::longest_substring("abcde".to_string(), 2), 0);
                }

                #[test]
                fn test_k_equals_1() {
                    assert_eq!(Solution::longest_substring("abcde".to_string(), 1), 5);
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::longest_substring("a".to_string(), 1), 1);
                }
            }
        };
    }

    test_longest!(my_tests, crate::my_solution::Solution);
    test_longest!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let s: String = "abcabc".repeat(1600); // 9600 chars
        let k = 3;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::longest_substring(
                std::hint::black_box(s.clone()),
                std::hint::black_box(k),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::longest_substring(
                std::hint::black_box(s.clone()),
                std::hint::black_box(k),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
