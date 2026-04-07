// LeetCode 68: Text Justification [Hard]
// https://leetcode.com/problems/text-justification/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 2.969us
//   AI solution avg: 4.350us
//   Ratio: My 1.47x faster
//
// Given an array of words and a maxWidth, format text such that each line
// has exactly maxWidth characters and is fully (left and right) justified.
// Last line is left-justified with no extra space between words.
//
// Constraints: 1 <= words.length <= 300, 1 <= words[i].length <= 20,
//              1 <= maxWidth <= 100
//
// Run:  cargo run --example lc_0068_text_justification
// Test: cargo test --example lc_0068_text_justification
// Bench: cargo test --example lc_0068_text_justification bench -- --nocapture

/// My solution — greedy line packing + distribute spaces.
///
/// Pack as many words per line as possible, then distribute extra spaces
/// evenly (left-biased remainder). Last line is left-justified.
///
/// Time: O(n * maxWidth) where n = total characters across all words.
/// Space: O(n * maxWidth) for output.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Format text with full justification.
        ///
        /// # Arguments
        /// * `words` - array of words
        /// * `max_width` - target line width
        ///
        /// # Returns
        /// Fully justified lines
        pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
            let max_width = max_width as usize;
            let mut result = Vec::new();
            let mut i = 0;

            while i < words.len() {
                // Greedily pack words into current line
                let mut line_len = words[i].len();
                let mut j = i + 1;
                while j < words.len() && line_len + 1 + words[j].len() <= max_width {
                    line_len += 1 + words[j].len();
                    j += 1;
                }

                let num_words = j - i;
                let mut line = String::with_capacity(max_width);

                // Last line or single word: left-justify
                if j == words.len() || num_words == 1 {
                    for k in i..j {
                        if k > i {
                            line.push(' ');
                        }
                        line.push_str(&words[k]);
                    }
                    // Pad right with spaces
                    while line.len() < max_width {
                        line.push(' ');
                    }
                } else {
                    // Distribute spaces evenly
                    let total_chars: usize = (i..j).map(|k| words[k].len()).sum();
                    let total_spaces = max_width - total_chars;
                    let gaps = num_words - 1;
                    let space_per_gap = total_spaces / gaps;
                    let extra = total_spaces % gaps;

                    for k in i..j {
                        if k > i {
                            let spaces = space_per_gap + if k - i <= extra { 1 } else { 0 };
                            for _ in 0..spaces {
                                line.push(' ');
                            }
                        }
                        line.push_str(&words[k]);
                    }
                }

                result.push(line);
                i = j;
            }
            result
        }
    }
}

/// AI solution — same greedy approach (canonical for this problem).
///
/// Time: O(n * maxWidth).
/// Space: O(n * maxWidth).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Format text with full justification.
        ///
        /// # Arguments
        /// * `words` - array of words
        /// * `max_width` - target line width
        ///
        /// # Returns
        /// Fully justified lines
        pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
            let mw = max_width as usize;
            let mut result = Vec::new();
            let mut i = 0;

            while i < words.len() {
                let mut j = i + 1;
                let mut width = words[i].len();
                while j < words.len() && width + 1 + words[j].len() <= mw {
                    width += 1 + words[j].len();
                    j += 1;
                }

                let is_last = j == words.len();
                let gaps = j - i - 1;

                if gaps == 0 || is_last {
                    // Left-justify
                    let mut line = words[i..j].join(" ");
                    line.push_str(&" ".repeat(mw - line.len()));
                    result.push(line);
                } else {
                    let chars: usize = (i..j).map(|k| words[k].len()).sum();
                    let total_sp = mw - chars;
                    let base = total_sp / gaps;
                    let extra = total_sp % gaps;

                    let mut line = String::with_capacity(mw);
                    for k in i..j {
                        if k > i {
                            let sp = base + if k - i <= extra { 1 } else { 0 };
                            line.push_str(&" ".repeat(sp));
                        }
                        line.push_str(&words[k]);
                    }
                    result.push(line);
                }
                i = j;
            }
            result
        }
    }
}

fn main() {
    let words: Vec<String> = ["This", "is", "an", "example", "of", "text", "justification."]
        .iter().map(|s| s.to_string()).collect();
    let my = my_solution::Solution::full_justify(words.clone(), 16);
    let ai = ai_solution::Solution::full_justify(words, 16);
    for (m, a) in my.iter().zip(ai.iter()) {
        println!("my:\"{m}\"  ai:\"{a}\"");
    }
}

#[cfg(test)]
mod tests {
    fn to_strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    macro_rules! test_full_justify {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::to_strings;

                #[test]
                fn test_example1() {
                    let words = to_strings(&["This", "is", "an", "example", "of", "text", "justification."]);
                    let result = <$solution>::full_justify(words, 16);
                    assert_eq!(result, vec![
                        "This    is    an",
                        "example  of text",
                        "justification.  ",
                    ]);
                }

                #[test]
                fn test_example2() {
                    let words = to_strings(&["What", "must", "be", "acknowledgment", "shall", "be"]);
                    let result = <$solution>::full_justify(words, 16);
                    assert_eq!(result, vec![
                        "What   must   be",
                        "acknowledgment  ",
                        "shall be        ",
                    ]);
                }

                #[test]
                fn test_single_word() {
                    let words = to_strings(&["hello"]);
                    let result = <$solution>::full_justify(words, 10);
                    assert_eq!(result, vec!["hello     "]);
                }

                #[test]
                fn test_exact_fit() {
                    let words = to_strings(&["ab", "cd"]);
                    let result = <$solution>::full_justify(words, 5);
                    assert_eq!(result, vec!["ab cd"]);
                }
            }
        };
    }

    test_full_justify!(my, crate::my_solution::Solution);
    test_full_justify!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let words: Vec<String> = ["This", "is", "an", "example", "of", "text", "justification.",
            "More", "words", "here", "to", "test", "the", "algorithm"]
            .iter().map(|s| s.to_string()).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::full_justify(
                std::hint::black_box(words.clone()), 16,
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::full_justify(
                std::hint::black_box(words.clone()), 16,
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
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
