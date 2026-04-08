// LeetCode 471: Encode String with Shortest Length [Hard]
// https://leetcode.com/problems/encode-string-with-shortest-length/
//
// Given a non-empty string s, encode it such that its encoded length is
// the shortest. The encoding rule is: k[encoded_string], where the
// encoded_string inside the square brackets is repeated exactly k times.
// If encoding does not make the string shorter, do not encode it. If
// there are multiple solutions, return any of them.
//
// Examples:
//   "aaa"        -> "aaa"           (encoding "3[a]" is same length, no gain)
//   "aaaaa"      -> "5[a]"          (1 char shorter)
//   "aaaaaaaaaa" -> "10[a]"         (5 chars shorter)
//   "aabcaabcd"  -> "2[aabc]d"
//   "abbbabbbcabbbabbbc" -> "2[2[abbb]c]"
//
// Constraints:
//   1 <= s.length <= 150
//   s consists of only lowercase English letters
//
// Run:  cargo run --example lc_0471_encode_string_with_shortest_length
// Test: cargo test --example lc_0471_encode_string_with_shortest_length
// Bench: cargo test --example lc_0471_encode_string_with_shortest_length bench -- --nocapture
//
// First benchmark (1K iterations, 2026-04-08):
//   My solution: avg 5.351ms
//   AI solution: avg 2.839ms
//   Ratio: 1.88x (AI divisor-check faster)

/// Time: O(n^3), Space: O(n^2)
/// Interval DP with the classic `(t+t).find(t[1..])` trick to detect the
/// shortest repeating period of a substring. For each interval [i, j],
/// compute the shortest encoding by considering:
///   1) The raw substring itself
///   2) A compressed form `k[encode(one_period)]` if a repeat pattern exists
///   3) All possible split points `f[i][k] + f[k+1][j]`
#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Encode `s` to the shortest representation using `k[...]` notation.
        ///
        /// # Arguments
        /// * `s` - A non-empty lowercase English string (len <= 150)
        ///
        /// # Returns
        /// The shortest encoded string
        pub fn encode(s: String) -> String {
            let n = s.len();
            let bytes = s.as_bytes();
            // dp[i][j] = shortest encoding of s[i..=j]
            let mut dp: Vec<Vec<String>> = vec![vec![String::new(); n]; n];

            // Fill intervals of increasing length
            for len in 1..=n {
                for i in 0..=n - len {
                    let j = i + len - 1;
                    let sub = &s[i..=j];

                    // Start with the raw substring
                    dp[i][j] = sub.to_string();

                    if len < 5 {
                        // Encoding "k[...]" needs at least "2[x]" = 4 chars,
                        // so no gain possible when len < 5; skip compression
                        // but still try splits below
                        if len >= 2 {
                            for k in i..j {
                                let candidate_len = dp[i][k].len() + dp[k + 1][j].len();
                                if candidate_len < dp[i][j].len() {
                                    dp[i][j] = format!("{}{}", dp[i][k], dp[k + 1][j]);
                                }
                            }
                        }
                        continue;
                    }

                    // Detect repeating pattern using the (t+t).find(t,1) trick:
                    // Concatenate sub with itself, then search for sub starting
                    // at index 1. The position found gives the period length.
                    let doubled = [bytes[i..=j].to_vec(), bytes[i..=j].to_vec()].concat();
                    // Find sub in doubled starting at position 1
                    let period = Self::find_substr(&doubled[1..], bytes, i, j);

                    if let Some(p) = period {
                        // p is the offset within doubled[1..] where we found sub,
                        // so actual period = p + 1
                        let period_len = p + 1;
                        if period_len < len {
                            let repeat_count = len / period_len;
                            let encoded_period = &dp[i][i + period_len - 1];
                            let compressed = format!("{}[{}]", repeat_count, encoded_period);
                            if compressed.len() < dp[i][j].len() {
                                dp[i][j] = compressed;
                            }
                        }
                    }

                    // Try all split points
                    for k in i..j {
                        let candidate_len = dp[i][k].len() + dp[k + 1][j].len();
                        if candidate_len < dp[i][j].len() {
                            dp[i][j] = format!("{}{}", dp[i][k], dp[k + 1][j]);
                        }
                    }
                }
            }

            dp[0][n - 1].clone()
        }

        /// Find `needle = bytes[ni..=nj]` in `haystack`, returning the
        /// offset of the first match, or `None`.
        fn find_substr(haystack: &[u8], bytes: &[u8], ni: usize, nj: usize) -> Option<usize> {
            let needle_len = nj - ni + 1;
            if haystack.len() < needle_len {
                return None;
            }
            for start in 0..=haystack.len() - needle_len {
                if haystack[start..start + needle_len] == bytes[ni..=nj] {
                    return Some(start);
                }
            }
            None
        }
    }
}

/// Time: O(n^3), Space: O(n^2)
/// Same interval DP but detects repeating patterns by explicitly checking
/// all divisors of the substring length. For each divisor d, verify that
/// s[i..=j] is composed of repetitions of s[i..i+d].
#[allow(dead_code)]
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Encode `s` to the shortest representation using `k[...]` notation.
        ///
        /// # Arguments
        /// * `s` - A non-empty lowercase English string (len <= 150)
        ///
        /// # Returns
        /// The shortest encoded string
        pub fn encode(s: String) -> String {
            let n = s.len();
            let bytes = s.as_bytes();
            // dp[i][j] = shortest encoding of s[i..=j]
            let mut dp: Vec<Vec<String>> = vec![vec![String::new(); n]; n];

            for len in 1..=n {
                for i in 0..=n - len {
                    let j = i + len - 1;
                    let sub = &s[i..=j];

                    dp[i][j] = sub.to_string();

                    if len >= 5 {
                        // Try all divisors of len as potential period lengths
                        // d divides len, and d < len
                        for d in 1..len {
                            if len % d != 0 {
                                continue;
                            }
                            // Check if s[i..=j] is made of (len/d) copies
                            // of s[i..i+d]
                            let pattern = &bytes[i..i + d];
                            let is_repeat = (1..len / d).all(|rep| {
                                let start = i + rep * d;
                                bytes[start..start + d] == *pattern
                            });
                            if is_repeat {
                                let k = len / d;
                                let encoded_period = &dp[i][i + d - 1];
                                let compressed = format!("{}[{}]", k, encoded_period);
                                if compressed.len() < dp[i][j].len() {
                                    dp[i][j] = compressed;
                                }
                            }
                        }
                    }

                    // Try all split points
                    for k in i..j {
                        let candidate_len = dp[i][k].len() + dp[k + 1][j].len();
                        if candidate_len < dp[i][j].len() {
                            dp[i][j] = format!("{}{}", dp[i][k], dp[k + 1][j]);
                        }
                    }
                }
            }

            dp[0][n - 1].clone()
        }
    }
}

fn main() {
    println!("LeetCode 471: Encode String with Shortest Length");
    let cases = vec![
        ("aaa", "aaa"),
        ("aaaaa", "5[a]"),
        ("aaaaaaaaaa", "10[a]"),
        ("aabcaabcd", "2[aabc]d"),
        ("abbbabbbcabbbabbbc", "2[2[abbb]c]"),
    ];
    for (input, expected) in &cases {
        let r1 = my_solution::Solution::encode(input.to_string());
        let r2 = ai_solution::Solution::encode(input.to_string());
        println!("  \"{input}\" => my=\"{r1}\", ai=\"{r2}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Multiple valid encodings exist; we check that the encoded length
    // matches the optimal and that decoding produces the original string.
    macro_rules! test_both {
        ($name:ident, $input:expr, $max_len:expr) => {
            mod $name {
                use super::*;

                /// Decode a k[...] encoded string back to its original form.
                fn decode(encoded: &str) -> String {
                    let bytes = encoded.as_bytes();
                    let mut stack: Vec<(String, usize)> = vec![];
                    let mut current = String::new();
                    let mut num = 0usize;
                    for &b in bytes {
                        match b {
                            b'0'..=b'9' => {
                                num = num * 10 + (b - b'0') as usize;
                            }
                            b'[' => {
                                stack.push((current.clone(), num));
                                current.clear();
                                num = 0;
                            }
                            b']' => {
                                let (prev, k) = stack.pop().expect("balanced brackets");
                                let repeated = current.repeat(k);
                                current = prev + &repeated;
                            }
                            _ => {
                                current.push(b as char);
                                num = 0;
                            }
                        }
                    }
                    current
                }

                #[test]
                fn my() {
                    let input: &str = $input;
                    let result = my_solution::Solution::encode(input.to_string());
                    assert!(
                        result.len() <= $max_len,
                        "my_solution encoding \"{}\" has len {} > max {}",
                        result,
                        result.len(),
                        $max_len
                    );
                    assert_eq!(
                        decode(&result),
                        input,
                        "my_solution decoding mismatch: encoded=\"{}\"",
                        result
                    );
                }

                #[test]
                fn ai() {
                    let input: &str = $input;
                    let result = ai_solution::Solution::encode(input.to_string());
                    assert!(
                        result.len() <= $max_len,
                        "ai_solution encoding \"{}\" has len {} > max {}",
                        result,
                        result.len(),
                        $max_len
                    );
                    assert_eq!(
                        decode(&result),
                        input,
                        "ai_solution decoding mismatch: encoded=\"{}\"",
                        result
                    );
                }
            }
        };
    }

    // max_len = expected optimal encoded length
    test_both!(no_gain_aaa, "aaa", 3);
    test_both!(simple_repeat, "aaaaa", 4);
    test_both!(ten_a, "aaaaaaaaaa", 5);
    test_both!(two_aabc_d, "aabcaabcd", 9);
    test_both!(nested, "abbbabbbcabbbabbbc", 14);
    test_both!(single_char, "a", 1);
    test_both!(two_chars, "ab", 2);
    test_both!(four_a, "aaaa", 4);
    test_both!(abcabc, "abcabc", 6);
    test_both!(long_repeat, "abababababab", 7);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String) -> String,
{
    let iterations = 1_000;
    // Build a string with mixed repeating patterns (length ~150)
    let mut s = String::new();
    for _ in 0..15 {
        s.push_str("abcabcabc!");
    }
    let s = s[..150].to_string();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(s.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::encode);
    bench_solution("AI solution", ai_solution::Solution::encode);
}
