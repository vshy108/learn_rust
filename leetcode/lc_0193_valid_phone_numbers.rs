// LeetCode 193: Valid Phone Numbers [Easy]
// https://leetcode.com/problems/valid-phone-numbers/
//
// This is a Shell problem. Below is the Rust equivalent using the `regex`
// crate for pattern matching to demonstrate the same operations.
//
// In production Rust, you would use:
//   - `regex` crate for pattern matching (used here)
//   - `std::io::BufRead` for line-by-line file reading
//
// Bash Solution:
//   awk '/^([0-9]{3}-|\([0-9]{3}\) )[0-9]{3}-[0-9]{4}$/' file.txt
//
// Run:  cargo run --example lc_0193_valid_phone_numbers
// Test: cargo test --example lc_0193_valid_phone_numbers

use regex::Regex;

/// Filter lines that are valid US phone numbers.
///
/// # Arguments
/// * `text` - Multi-line text with one phone number per line
///
/// # Returns
/// A `Vec<String>` containing only lines matching either format:
///   - `xxx-xxx-xxxx`        (e.g. "987-123-4567")
///   - `(xxx) xxx-xxxx`      (e.g. "(123) 456-7890")
///
/// # Approach
/// Compile a single `Regex` matching both valid formats, anchored to
/// start (`^`) and end (`$`). Filter each line against it. This mirrors
/// the awk pattern match in the bash solution.
fn valid_phone_numbers(text: &str) -> Vec<String> {
    // Pattern breakdown:
    //   ^                      - start of line
    //   (                      - group for area code format:
    //     [0-9]{3}-            -   bare:   "123-"
    //     |                    -   or
    //     \([0-9]{3}\)\s       -   parens: "(123) "
    //   )
    //   [0-9]{3}-[0-9]{4}     - local number: "456-7890"
    //   $                      - end of line
    let re =
        Regex::new(r"^([0-9]{3}-|\([0-9]{3}\) )[0-9]{3}-[0-9]{4}$").expect("valid regex pattern");

    text.lines()
        .filter(|line| re.is_match(line))
        .map(String::from)
        .collect()
}

fn main() {
    println!("LeetCode 193: Valid Phone Numbers");

    let text = "987-123-4567\n123 456 7890\n(123) 456-7890";
    let valid = valid_phone_numbers(text);
    for number in &valid {
        println!("  {number}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_main() {
        let text = "987-123-4567\n123 456 7890\n(123) 456-7890";
        assert_eq!(
            valid_phone_numbers(text),
            vec!["987-123-4567", "(123) 456-7890"]
        );
    }

    #[test]
    fn dash_format_only() {
        let text = "111-222-3333\n444-555-6666";
        assert_eq!(
            valid_phone_numbers(text),
            vec!["111-222-3333", "444-555-6666"]
        );
    }

    #[test]
    fn parens_format_only() {
        let text = "(111) 222-3333\n(444) 555-6666";
        assert_eq!(
            valid_phone_numbers(text),
            vec!["(111) 222-3333", "(444) 555-6666"]
        );
    }

    #[test]
    fn no_valid_numbers() {
        let text = "123 456 7890\n1234567890\n12-345-6789";
        assert_eq!(valid_phone_numbers(text), Vec::<String>::new());
    }

    #[test]
    fn reject_missing_space_after_parens() {
        // "(123)456-7890" is invalid -- must have space after closing paren
        let text = "(123)456-7890";
        assert_eq!(valid_phone_numbers(text), Vec::<String>::new());
    }

    #[test]
    fn reject_extra_digits() {
        let text = "1987-123-4567\n987-123-45678";
        assert_eq!(valid_phone_numbers(text), Vec::<String>::new());
    }

    #[test]
    fn reject_letters() {
        let text = "abc-def-ghij\n(abc) def-ghij";
        assert_eq!(valid_phone_numbers(text), Vec::<String>::new());
    }

    #[test]
    fn mixed_valid_and_invalid() {
        let text = "not a number\n(800) 555-0199\nbad\n321-654-0987\n  spaces  ";
        assert_eq!(
            valid_phone_numbers(text),
            vec!["(800) 555-0199", "321-654-0987"]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(valid_phone_numbers(""), Vec::<String>::new());
    }

    #[test]
    fn reject_dot_separator() {
        let text = "987.123.4567";
        assert_eq!(valid_phone_numbers(text), Vec::<String>::new());
    }
}
