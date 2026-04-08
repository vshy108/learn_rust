// LeetCode 195: Tenth Line [Easy]
// https://leetcode.com/problems/tenth-line/
//
// This is a Shell problem. Below is the Rust equivalent using standard library
// I/O and text processing to demonstrate the same operations.
//
// In production Rust, you would use:
//   - `std::io::BufRead` for line-by-line file reading
//   - `lines().nth(9)` for zero-cost skipping to the 10th line
//
// Bash Solution:
//   sed -n 10p file.txt
//
// Run:  cargo run --example lc_0195_tenth_line
// Test: cargo test --example lc_0195_tenth_line

/// Print just the 10th line of a text input.
///
/// # Arguments
/// * `text` - Multi-line text
///
/// # Returns
/// `Some(String)` containing the 10th line if it exists, `None` otherwise.
///
/// # Approach
/// Use the iterator adapter `nth(9)` (0-indexed) to skip directly to the
/// 10th line without allocating intermediate collections.
/// This is the Rust equivalent of `sed -n 10p`.
fn tenth_line(text: &str) -> Option<String> {
    // nth(9) consumes the first 9 elements and returns the 10th (0-indexed)
    // This is O(n) in the number of lines skipped, but allocates nothing extra.
    text.lines().nth(9).map(String::from)
}

/// Generic version: return the Nth line (1-indexed) from text.
///
/// # Arguments
/// * `text` - Multi-line text
/// * `n` - 1-indexed line number to retrieve
///
/// # Returns
/// `Some(String)` if line `n` exists, `None` otherwise
#[cfg(test)]
fn nth_line(text: &str, n: usize) -> Option<String> {
    if n == 0 {
        return None; // No "0th" line in 1-indexed scheme
    }
    // Convert 1-indexed to 0-indexed for nth()
    text.lines().nth(n - 1).map(String::from)
}

fn main() {
    println!("LeetCode 195: Tenth Line");

    // Build sample text with 11 lines
    let text: String = (1..=11)
        .map(|i| format!("Line {i}"))
        .collect::<Vec<_>>()
        .join("\n");

    match tenth_line(&text) {
        Some(line) => println!("  10th line: \"{line}\""),
        None => println!("  File has fewer than 10 lines"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to build multi-line text with N lines.
    fn make_text(n: usize) -> String {
        (1..=n)
            .map(|i| format!("Line {i}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn example_exactly_ten_lines() {
        let text = make_text(10);
        assert_eq!(tenth_line(&text), Some("Line 10".to_string()));
    }

    #[test]
    fn example_more_than_ten_lines() {
        let text = make_text(15);
        assert_eq!(tenth_line(&text), Some("Line 10".to_string()));
    }

    #[test]
    fn fewer_than_ten_lines() {
        let text = make_text(5);
        assert_eq!(tenth_line(&text), None);
    }

    #[test]
    fn exactly_nine_lines() {
        let text = make_text(9);
        assert_eq!(tenth_line(&text), None);
    }

    #[test]
    fn empty_input() {
        assert_eq!(tenth_line(""), None);
    }

    #[test]
    fn single_line() {
        assert_eq!(tenth_line("only one line"), None);
    }

    #[test]
    fn tenth_line_is_empty() {
        // Lines 1-9 have content, line 10 is empty, line 11 has content.
        // A trailing "\n" alone won't produce an empty line for `lines()`,
        // so we need content after the empty line to anchor it.
        let mut parts: Vec<String> = (1..=9).map(|i| format!("Line {i}")).collect();
        parts.push(String::new()); // empty 10th line
        parts.push("Line 11".to_string()); // ensures line 10 is visible to lines()
        let text = parts.join("\n");
        assert_eq!(tenth_line(&text), Some(String::new()));
    }

    // Tests for the generic nth_line helper
    #[test]
    fn nth_line_first() {
        let text = make_text(5);
        assert_eq!(nth_line(&text, 1), Some("Line 1".to_string()));
    }

    #[test]
    fn nth_line_last() {
        let text = make_text(5);
        assert_eq!(nth_line(&text, 5), Some("Line 5".to_string()));
    }

    #[test]
    fn nth_line_beyond_end() {
        let text = make_text(5);
        assert_eq!(nth_line(&text, 6), None);
    }

    #[test]
    fn nth_line_zero_returns_none() {
        let text = make_text(5);
        assert_eq!(nth_line(&text, 0), None);
    }
}
