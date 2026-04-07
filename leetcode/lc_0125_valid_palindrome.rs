// LeetCode 125: Valid Palindrome [Easy]
// https://leetcode.com/problems/valid-palindrome/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.856us
//   AI solution avg: 0.307us
//   Ratio: AI 2.79x faster
//
// A phrase is a palindrome if, after converting all uppercase letters to lowercase
// and removing all non-alphanumeric characters, it reads the same forward and backward.
//
// Constraints: 1 <= s.length <= 2 * 10^5, s consists only of printable ASCII
//
// Run:  cargo run --example lc_0125_valid_palindrome
// Test: cargo test --example lc_0125_valid_palindrome
// Bench: cargo test --example lc_0125_valid_palindrome bench -- --nocapture

/// My solution — filter, collect, then compare with reverse.
///
/// Time: O(n).
/// Space: O(n) for the filtered string.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check if a string is a valid palindrome (alphanumeric only, case-insensitive).
        pub fn is_palindrome(s: String) -> bool {
            let filtered: Vec<u8> = s.bytes()
                .filter(|b| b.is_ascii_alphanumeric())
                .map(|b| b.to_ascii_lowercase())
                .collect();
            filtered.iter().eq(filtered.iter().rev())
        }
    }
}

/// AI solution — two-pointer, O(1) space (no allocation).
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check palindrome using two pointers on the original bytes.
        pub fn is_palindrome(s: String) -> bool {
            let bytes = s.as_bytes();
            let mut left = 0usize;
            let mut right = bytes.len();

            while left < right {
                if !bytes[left].is_ascii_alphanumeric() {
                    left += 1;
                    continue;
                }
                // right is 1-indexed to avoid underflow on empty/all-non-alnum
                right -= 1;
                if !bytes[right].is_ascii_alphanumeric() {
                    continue;
                }
                if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
                    return false;
                }
                left += 1;
            }
            true
        }
    }
}

fn main() {
    let my = my_solution::Solution::is_palindrome(
        "A man, a plan, a canal: Panama".into(),
    );
    let ai = ai_solution::Solution::is_palindrome(
        "A man, a plan, a canal: Panama".into(),
    );
    println!("palindrome: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_palindrome {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert!(<$solution>::is_palindrome(
                        "A man, a plan, a canal: Panama".into(),
                    ));
                }

                #[test]
                fn test_not_palindrome() {
                    assert!(!<$solution>::is_palindrome("race a car".into()));
                }

                #[test]
                fn test_empty_after_filter() {
                    assert!(<$solution>::is_palindrome(" ".into()));
                }

                #[test]
                fn test_single_char() {
                    assert!(<$solution>::is_palindrome("a".into()));
                }

                #[test]
                fn test_numeric() {
                    assert!(<$solution>::is_palindrome("121".into()));
                }

                #[test]
                fn test_mixed() {
                    assert!(!<$solution>::is_palindrome("0P".into()));
                }
            }
        };
    }

    test_palindrome!(my, crate::my_solution::Solution);
    test_palindrome!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "A man, a plan, a canal: Panama".repeat(10);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_palindrome(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_palindrome(
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
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
