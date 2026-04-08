// LeetCode 459: Repeated Substring Pattern [Easy]
// https://leetcode.com/problems/repeated-substring-pattern/
//
// Given a string s, check if it can be constructed by taking a substring
// and appending multiple copies of it together.
//
// Examples:
//   "abab" -> true  ("ab" repeated twice)
//   "aba" -> false
//   "abcabcabcabc" -> true  ("abc" repeated four times)
//
// Constraints:
//   1 <= s.length <= 10^4
//   s consists of lowercase English letters
//
// Run:  cargo run --example lc_0459_repeated_substring_pattern
// Test: cargo test --example lc_0459_repeated_substring_pattern
// Bench: cargo test --example lc_0459_repeated_substring_pattern bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 59ns  (divisor check)
//   AI solution: avg 203ns (concatenation trick)
//   Ratio: 0.29x (My 3.4x faster — divisor check avoids string allocation)

/// My solution module — check divisor lengths.
///
/// Time:  O(n * sqrt(n)) or O(n^2) in worst case
/// Space: O(n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn repeated_substring_pattern(s: String) -> bool {
            // Try all divisors of length as possible pattern lengths
            let n = s.len();
            let bytes = s.as_bytes();
            for len in 1..=n / 2 {
                if n % len != 0 {
                    continue;
                }
                let pattern = &bytes[..len];
                if bytes.chunks(len).all(|chunk| chunk == pattern) {
                    return true;
                }
            }
            false
        }
    }
}

/// AI solution module — concatenation trick.
///
/// If s is made of repeated substrings, then (s + s) with first and last
/// char removed will still contain s. This works because the rotation
/// property of repeated strings.
///
/// Time:  O(n) with KMP-like find, or O(n^2) with naive find
/// Space: O(n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn repeated_substring_pattern(s: String) -> bool {
            let doubled = format!("{}{}", &s, &s);
            // Search in doubled[1..len*2-1] for s
            doubled[1..doubled.len() - 1].contains(&s)
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_abab() {
            assert!(Solution::repeated_substring_pattern("abab".into()));
        }
        #[test]
        fn test_aba() {
            assert!(!Solution::repeated_substring_pattern("aba".into()));
        }
        #[test]
        fn test_abcabc() {
            assert!(Solution::repeated_substring_pattern("abcabcabcabc".into()));
        }
        #[test]
        fn test_single_char() {
            assert!(!Solution::repeated_substring_pattern("a".into()));
        }
        #[test]
        fn test_aa() {
            assert!(Solution::repeated_substring_pattern("aa".into()));
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String) -> bool,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box("abcabcabcabc".to_string())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution(
        "My solution",
        my_solution::Solution::repeated_substring_pattern,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::repeated_substring_pattern,
    );
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::repeated_substring_pattern("abab".into())
    );
}
