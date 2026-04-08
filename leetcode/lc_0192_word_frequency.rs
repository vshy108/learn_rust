// LeetCode 192: Word Frequency [Medium]
// https://leetcode.com/problems/word-frequency/
//
// This is a Shell problem. Below is the Rust equivalent using standard library
// I/O and text processing to demonstrate the same operations.
//
// In production Rust, you would use:
//   - `std::collections::HashMap` for counting/aggregation
//   - `std::io::BufRead` for line-by-line file reading
//   - `polars` for DataFrame-style word frequency analysis
//
// Bash Solution:
//   cat words.txt | tr -s ' ' '\n' | sort | uniq -c | sort -nr | awk '{print $2, $1}'
//
// Run:  cargo run --example lc_0192_word_frequency
// Test: cargo test --example lc_0192_word_frequency

use std::collections::HashMap;

/// A frequency entry: (word, count), sorted descending by count.
#[derive(Debug, Clone, PartialEq, Eq)]
struct WordFreq {
    word: String,
    count: usize,
}

/// Count word frequencies in multi-line text, sorted by count descending.
///
/// # Arguments
/// * `text` - Multi-line text containing only lowercase letters and spaces
///
/// # Returns
/// A `Vec<WordFreq>` sorted by count descending, then alphabetically for ties
///
/// # Approach
/// Split text on whitespace, count occurrences with a `HashMap`,
/// then collect and sort. This mirrors the bash pipeline:
///   `tr -s ' ' '\n'`   -> `split_whitespace()`
///   `sort | uniq -c`    -> HashMap insert/increment
///   `sort -nr`          -> sort_by descending count
fn word_frequency(text: &str) -> Vec<WordFreq> {
    let mut counts: HashMap<&str, usize> = HashMap::new();

    // Split on any whitespace (spaces, newlines) -- equivalent to `tr -s ' ' '\n'`
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    // Collect into Vec and sort: descending by count, then alphabetical for ties
    let mut result: Vec<WordFreq> = counts
        .into_iter()
        .map(|(word, count)| WordFreq {
            word: word.to_string(),
            count,
        })
        .collect();

    // sort -nr equivalent: primary key = count descending, secondary = word ascending
    result.sort_by(|a, b| b.count.cmp(&a.count).then(a.word.cmp(&b.word)));
    result
}

/// Format output matching the bash `awk '{print $2, $1}'` style.
fn format_output(freqs: &[WordFreq]) -> String {
    freqs
        .iter()
        .map(|wf| format!("{} {}", wf.word, wf.count))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    println!("LeetCode 192: Word Frequency");

    let text = "the day is sunny the the\nthe sunny is is";
    let freqs = word_frequency(text);
    let output = format_output(&freqs);
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build expected results concisely.
    fn wf(word: &str, count: usize) -> WordFreq {
        WordFreq {
            word: word.to_string(),
            count,
        }
    }

    #[test]
    fn example_main() {
        let text = "the day is sunny the the\nthe sunny is is";
        assert_eq!(
            word_frequency(text),
            vec![wf("the", 4), wf("is", 3), wf("sunny", 2), wf("day", 1)]
        );
    }

    #[test]
    fn formatted_output() {
        let text = "the day is sunny the the\nthe sunny is is";
        let freqs = word_frequency(text);
        assert_eq!(format_output(&freqs), "the 4\nis 3\nsunny 2\nday 1");
    }

    #[test]
    fn single_word() {
        let text = "hello";
        assert_eq!(word_frequency(text), vec![wf("hello", 1)]);
    }

    #[test]
    fn all_same_word() {
        let text = "a a a a";
        assert_eq!(word_frequency(text), vec![wf("a", 4)]);
    }

    #[test]
    fn tied_counts_alphabetical() {
        // When counts are equal, words appear alphabetically
        let text = "banana apple cherry";
        assert_eq!(
            word_frequency(text),
            vec![wf("apple", 1), wf("banana", 1), wf("cherry", 1)]
        );
    }

    #[test]
    fn multiple_spaces_between_words() {
        // split_whitespace handles multiple spaces like `tr -s`
        let text = "a  b   a";
        assert_eq!(word_frequency(text), vec![wf("a", 2), wf("b", 1)]);
    }

    #[test]
    fn multiline_input() {
        let text = "cat dog\ncat bird\ndog dog";
        assert_eq!(
            word_frequency(text),
            vec![wf("dog", 3), wf("cat", 2), wf("bird", 1)]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(word_frequency(""), Vec::<WordFreq>::new());
    }

    #[test]
    fn whitespace_only() {
        assert_eq!(word_frequency("   \n  \n  "), Vec::<WordFreq>::new());
    }
}
