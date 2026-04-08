// LeetCode 482: License Key Formatting [Easy]
// https://leetcode.com/problems/license-key-formatting/
//
// You are given a license key represented as a string s that consists of only
// alphanumeric characters and dashes. Reformat the string such that each
// group contains exactly k characters, except for the first group which
// could be shorter. Also convert all lowercase letters to uppercase.
//
// Examples:
//   s = "5F3Z-2e-9-w", k = 4  -> "5F3Z-2E9W"
//   s = "2-5g-3-J", k = 2     -> "2-5G-3J"
//
// Constraints:
//   1 <= s.len() <= 10^5
//   s consists of English letters, digits, and '-'
//   1 <= k <= 10^4
//
// Run:  cargo run --example lc_0482_license_key_formatting
// Test: cargo test --example lc_0482_license_key_formatting
// Bench: cargo test --example lc_0482_license_key_formatting bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 131ns
//   AI solution: avg 129ns
//   Ratio: 1.02x (same algorithm)

/// My solution module — build from right to left.
///
/// Time:  O(n)
/// Space: O(n)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn license_key_formatting(s: String, k: i32) -> String {
            let k = k as usize;
            let chars: Vec<u8> = s
                .bytes()
                .filter(|&b| b != b'-')
                .map(|b| b.to_ascii_uppercase())
                .collect();
            if chars.is_empty() {
                return String::new();
            }
            let mut result = Vec::with_capacity(chars.len() + chars.len() / k);
            let mut count = 0;
            for &ch in chars.iter().rev() {
                if count == k {
                    result.push(b'-');
                    count = 0;
                }
                result.push(ch);
                count += 1;
            }
            result.reverse();
            String::from_utf8(result).expect("ASCII")
        }
    }
}

/// AI solution module — strip dashes, uppercase, then insert dashes from the end.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn license_key_formatting(s: String, k: i32) -> String {
            let k = k as usize;
            // Collect non-dash chars, uppercase
            let chars: Vec<u8> = s
                .bytes()
                .filter(|&b| b != b'-')
                .map(|b| b.to_ascii_uppercase())
                .collect();

            if chars.is_empty() {
                return String::new();
            }

            let mut result = Vec::with_capacity(chars.len() + chars.len() / k);
            let mut count = 0;

            // Process from the end so the first group gets the remainder
            for &ch in chars.iter().rev() {
                if count == k {
                    result.push(b'-');
                    count = 0;
                }
                result.push(ch);
                count += 1;
            }

            result.reverse();
            // Safety: all bytes are ASCII uppercase letters, digits, or '-'
            String::from_utf8(result).expect("all ASCII chars")
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            assert_eq!(
                Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4),
                "5F3Z-2E9W"
            );
        }
        #[test]
        fn test_example2() {
            assert_eq!(
                Solution::license_key_formatting("2-5g-3-J".to_string(), 2),
                "2-5G-3J"
            );
        }
        #[test]
        fn test_all_dashes() {
            assert_eq!(Solution::license_key_formatting("---".to_string(), 3), "");
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
    F: Fn(String, i32) -> String,
{
    let iterations = 500_000;
    let input = "5F3Z-2e-9-w".to_string();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(input.clone()),
            std::hint::black_box(4),
        ));
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
    bench_solution("My solution", my_solution::Solution::license_key_formatting);
    bench_solution("AI solution", ai_solution::Solution::license_key_formatting);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::license_key_formatting("5F3Z-2e-9-w".to_string(), 4)
    );
}
