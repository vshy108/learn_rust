// LeetCode 58: Length of Last Word [Easy]
// https://leetcode.com/problems/length-of-last-word/
//
// Given a string s consisting of words and spaces, return the length of
// the last word in the string.
//
// A word is a maximal substring consisting of non-space characters only.
//
// Examples:
//   s = "Hello World"                    -> 5
//   s = "   fly me   to   the moon  "   -> 4
//   s = "luffy is still joyboy"          -> 6
//
// Constraints:
//   - 1 <= s.length <= 10^4
//   - s consists of only English letters and spaces ' '
//   - There will be at least one word in s
//
// Run:  cargo run --example lc_0058_length_of_last_word
// Test: cargo test --example lc_0058_length_of_last_word
// Bench: cargo test --example lc_0058_length_of_last_word bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-03-27) ---
//   My solution: 39.43ms  (avg 394ns)
//   AI solution: 121.84ms (avg 1.218us)
//   -> My solution is 3.09x faster
//   Reason: Byte-level backward scan with as_bytes() avoids UTF-8 validation
//           overhead. trim_end + rsplit_once operate on str (UTF-8 aware) and
//           involve pattern matching machinery that's slower for simple ASCII.
//
// --- Rust concepts used ---
//   - str::rsplit_once(' '): split at last occurrence, returns Option<(&str, &str)>
//   (Previously covered: as_bytes lc_0005, byte literals lc_0008,
//    usize underflow avoidance lc_0005, str::trim_end lc_0008,
//    match on Option lc_0002)

/// My solution module.
///
/// Time:  O(n) -- at most one full backward pass over the string
/// Space: O(1) -- only two usize variables, operates on borrowed bytes
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the last word in the string.
        ///
        /// # Arguments
        ///
        /// * `s` - A string consisting of words and spaces
        ///
        /// # Returns
        ///
        /// The length of the last word
        pub fn length_of_last_word(s: String) -> i32 {
            // Ruby: s[index] indexes directly into a string.
            // Rust: String can't be indexed by usize -- it's UTF-8 and a byte
            // index might split a multi-byte character. Use as_bytes() to get
            // a &[u8] slice (safe here since the problem guarantees ASCII only).
            let bytes = s.as_bytes();

            // Ruby: index = s.length - 1, then index >= 0 check.
            // Rust: usize can't go negative, so use i > 0 and access bytes[i - 1]
            // to avoid underflow when the pointer reaches the start of the string.
            let mut i = bytes.len();

            // Skip trailing spaces -- compare with b' ' (byte literal) not ' ' (char)
            // because bytes[i] is u8, not char.
            while i > 0 && bytes[i - 1] == b' ' {
                i -= 1;
            }

            let end = i;

            // Count last word characters
            while i > 0 && bytes[i - 1] != b' ' {
                i -= 1;
            }

            (end - i) as i32
        }
    }
}

/// AI solution module.
///
/// Uses `str::trim_end()` to strip trailing spaces, then `rsplit_once(' ')`
/// to split at the last space. If there's no space, the entire trimmed
/// string is the last word.
///
/// Time:  O(n) -- trim_end scans trailing spaces, rsplit_once scans backward
/// Space: O(1) -- trim_end returns a &str slice (no allocation)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the last word in the string.
        ///
        /// # Arguments
        ///
        /// * `s` - A string consisting of words and spaces
        ///
        /// # Returns
        ///
        /// The length of the last word
        pub fn length_of_last_word(s: String) -> i32 {
            // trim_end() returns &str (a slice into s), no allocation.
            // rsplit_once(' ') splits at the last space, returning
            // Some((before, after)) or None if no space exists.
            let trimmed = s.trim_end();
            match trimmed.rsplit_once(' ') {
                Some((_, last_word)) => last_word.len() as i32,
                None => trimmed.len() as i32,
            }
        }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32)> = vec![
        ("Hello World", 5),
        ("   fly me   to   the moon  ", 4),
        ("luffy is still joyboy", 6),
    ];

    println!("--- My Solution ---");
    for (s, expected) in &test_cases {
        let result = my_solution::Solution::length_of_last_word(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &test_cases {
        let result = ai_solution::Solution::length_of_last_word(s.to_string());
        println!("  \"{s}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_last_word {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_two_words() {
                    assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
                }

                #[test]
                fn test_trailing_spaces() {
                    assert_eq!(
                        Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
                        4
                    );
                }

                #[test]
                fn test_no_trailing_spaces() {
                    assert_eq!(
                        Solution::length_of_last_word("luffy is still joyboy".to_string()),
                        6
                    );
                }

                #[test]
                fn test_single_word() {
                    assert_eq!(Solution::length_of_last_word("hello".to_string()), 5);
                }

                #[test]
                fn test_single_char() {
                    assert_eq!(Solution::length_of_last_word("a".to_string()), 1);
                }

                #[test]
                fn test_leading_spaces_single_word() {
                    assert_eq!(Solution::length_of_last_word("   word".to_string()), 4);
                }

                #[test]
                fn test_single_char_with_spaces() {
                    assert_eq!(Solution::length_of_last_word("  a  ".to_string()), 1);
                }
            }
        };
    }

    test_last_word!(my_tests, crate::my_solution::Solution);
    test_last_word!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<&str> = vec![
            "Hello World",
            "   fly me   to   the moon  ",
            "luffy is still joyboy",
            "a",
            "   word   ",
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::my_solution::Solution::length_of_last_word(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &s in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::length_of_last_word(
                    std::hint::black_box(s.to_string()),
                ));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
