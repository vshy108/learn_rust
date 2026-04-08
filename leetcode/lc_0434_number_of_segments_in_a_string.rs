// LeetCode 434: Number of Segments in a String [Easy]
// https://leetcode.com/problems/number-of-segments-in-a-string/
//
// Given a string s, return the number of segments in the string.
// A segment is a contiguous sequence of non-space characters.
//
// Examples:
//   "Hello, my name is John" -> 5
//   "Hello" -> 1
//   "" -> 0
//
// Constraints:
//   0 <= s.length <= 300
//   s consists of lowercase/uppercase English letters, digits, or "!@#$%^&*()_+-=',.:"
//   The only space character in s is ' '
//
// Run:  cargo run --example lc_0434_number_of_segments_in_a_string
// Test: cargo test --example lc_0434_number_of_segments_in_a_string
// Bench: cargo test --example lc_0434_number_of_segments_in_a_string bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 47ns
//   AI solution: avg 33ns
//   Ratio: 1.42x (AI faster — byte scan vs split_whitespace)

/// My solution module — split_whitespace approach.
///
/// Use Rust's built-in split_whitespace() which handles multiple
/// consecutive spaces and leading/trailing spaces correctly.
///
/// Time:  O(n) -- single pass over the string
/// Space: O(1) -- iterator, no extra allocation

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of segments in a string.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string
        ///
        /// # Returns
        ///
        /// Number of contiguous non-space segments
        pub fn count_segments(s: String) -> i32 {
            // Use split_whitespace which handles multiple consecutive spaces
            s.split_whitespace().count() as i32
        }
    }
}

/// AI solution module — manual scan for segment starts.
///
/// A segment starts when the current character is not a space and
/// either it's the first character or the previous character is a space.
///
/// Time:  O(n) -- single pass over bytes
/// Space: O(1) -- only a counter variable
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count the number of segments in a string.
        ///
        /// # Arguments
        ///
        /// * `s` - Input string
        ///
        /// # Returns
        ///
        /// Number of contiguous non-space segments
        pub fn count_segments(s: String) -> i32 {
            let bytes = s.as_bytes();
            let mut ans = 0;
            for i in 0..bytes.len() {
                // Current char is non-space AND (first char or prev is space)
                if bytes[i] != b' ' && (i == 0 || bytes[i - 1] == b' ') {
                    ans += 1;
                }
            }
            ans
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_five_segments() {
            assert_eq!(
                Solution::count_segments("Hello, my name is John".to_string()),
                5
            );
        }

        #[test]
        fn test_single_word() {
            assert_eq!(Solution::count_segments("Hello".to_string()), 1);
        }

        #[test]
        fn test_empty_string() {
            assert_eq!(Solution::count_segments("".to_string()), 0);
        }

        #[test]
        fn test_only_spaces() {
            assert_eq!(Solution::count_segments("   ".to_string()), 0);
        }

        #[test]
        fn test_leading_trailing_spaces() {
            assert_eq!(Solution::count_segments("  hello world  ".to_string()), 2);
        }

        #[test]
        fn test_multiple_spaces_between() {
            assert_eq!(Solution::count_segments("a   b   c".to_string()), 3);
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
    F: Fn(String) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(
            "Hello, my name is John".to_string(),
        )));
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
    bench_solution("My solution", |s| my_solution::Solution::count_segments(s));
    bench_solution("AI solution", |s| ai_solution::Solution::count_segments(s));
}

fn main() {
    let result = ai_solution::Solution::count_segments("Hello, my name is John".to_string());
    println!("Number of segments: {}", result);
}
