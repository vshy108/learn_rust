// LeetCode 186: Reverse Words in a String II [Medium] (Premium)
// https://leetcode.com/problems/reverse-words-in-a-string-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.175us
//   AI solution avg: 0.148us
//   Ratio: AI 1.18x faster
//
// Given a character array `s`, reverse the order of the words.
// A word is defined as a sequence of non-space characters.
// Words in `s` are separated by a single space.
// Must solve in-place without allocating extra space.
//
// Example 1:
//   Input:  s = ["t","h","e"," ","s","k","y"," ","i","s"," ","b","l","u","e"]
//   Output:     ["b","l","u","e"," ","i","s"," ","s","k","y"," ","t","h","e"]
//
// Example 2:
//   Input:  s = ["a"]
//   Output:     ["a"]
//
// Constraints:
//   - 1 <= s.length <= 10^5
//   - s[i] is an English letter, digit, or space ' '
//   - At least one word in s
//   - No leading or trailing spaces
//   - Words separated by a single space
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0186.Reverse%20Words%20in%20a%20String%20II/README_EN.md
//
// Run:  cargo run --example lc_0186_reverse_words_in_a_string_ii
// Test: cargo test --example lc_0186_reverse_words_in_a_string_ii
// Bench: cargo test --example lc_0186_reverse_words_in_a_string_ii bench -- --nocapture

/// My solution — reverse each word then reverse entire array (translated from doocs).
///
/// Two-pass approach: first scan through to find each word boundary
/// and reverse that word in-place, then reverse the entire array.
/// This swaps the word order while restoring each word's characters.
///
/// Time: O(n) — each character is swapped at most twice
/// Space: O(1) — all operations are in-place
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse words in `s` in-place.
        pub fn reverse_words(s: &mut Vec<char>) {
            let n = s.len();
            let mut i = 0;
            // First pass: reverse each individual word
            for j in 0..n {
                if s[j] == ' ' {
                    // Word boundary found — reverse s[i..j-1]
                    Self::reverse(s, i, j - 1);
                    i = j + 1;
                } else if j == n - 1 {
                    // Last word — reverse s[i..j]
                    Self::reverse(s, i, j);
                }
            }
            // Second pass: reverse the entire array
            Self::reverse(s, 0, n - 1);
        }

        /// Swap elements in s[i..=j] to reverse that range.
        fn reverse(s: &mut [char], mut i: usize, mut j: usize) {
            while i < j {
                s.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
    }
}

/// AI solution — reverse entire array first, then reverse each word.
///
/// Opposite order from my_solution: reverse the whole array using
/// Rust's built-in `reverse()`, then scan for word boundaries and
/// reverse each word using slice `reverse()`. More idiomatic Rust
/// by leveraging standard library methods.
///
/// Time: O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse words in `s` in-place.
        pub fn reverse_words(s: &mut Vec<char>) {
            // First: reverse the entire array
            s.reverse();
            let n = s.len();
            let mut start = 0;
            // Second: reverse each word to restore character order
            for i in 0..=n {
                // Word ends at a space or at the end of the array
                if i == n || s[i] == ' ' {
                    // Reverse the word in s[start..i]
                    s[start..i].reverse();
                    start = i + 1;
                }
            }
        }
    }
}

fn main() {
    let mut s: Vec<char> = "the sky is blue".chars().collect();
    my_solution::Solution::reverse_words(&mut s);
    let result: String = s.iter().collect();
    println!("my: {result}");

    let mut s: Vec<char> = "the sky is blue".chars().collect();
    ai_solution::Solution::reverse_words(&mut s);
    let result: String = s.iter().collect();
    println!("ai: {result}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_reverse_words {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                fn run(input: &str) -> String {
                    let mut s: Vec<char> = input.chars().collect();
                    Sol::reverse_words(&mut s);
                    s.iter().collect()
                }

                #[test]
                fn test_example_1() {
                    assert_eq!(run("the sky is blue"), "blue is sky the");
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(run("a"), "a");
                }

                #[test]
                fn test_two_words() {
                    assert_eq!(run("hello world"), "world hello");
                }

                #[test]
                fn test_three_words() {
                    assert_eq!(run("a b c"), "c b a");
                }

                #[test]
                fn test_single_chars() {
                    assert_eq!(run("x y z w"), "w z y x");
                }

                #[test]
                fn test_long_word() {
                    assert_eq!(run("abcdef"), "abcdef");
                }

                #[test]
                fn test_with_digits() {
                    assert_eq!(run("abc 123 def"), "def 123 abc");
                }

                #[test]
                fn test_equal_length_words() {
                    assert_eq!(run("ab cd ef"), "ef cd ab");
                }
            }
        };
    }

    test_reverse_words!(my, crate::my_solution::Solution);
    test_reverse_words!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        // Build a test string with multiple words
        let input: Vec<char> = "the quick brown fox jumps over the lazy dog near the river bank"
            .chars()
            .collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut s = input.clone();
            crate::my_solution::Solution::reverse_words(&mut s);
            std::hint::black_box(&s);
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut s = input.clone();
            crate::ai_solution::Solution::reverse_words(&mut s);
            std::hint::black_box(&s);
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
