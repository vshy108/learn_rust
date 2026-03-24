// Demonstrates using ? with Option.
//
// The ? operator works with Option<T> similarly to Result:
// - If `None`, returns `None` from the enclosing function.
// - If `Some(v)`, unwraps to `v`.
//
// The function must return `Option<T>` (not `Result`).
//
// Run: cargo run --example s17_question_mark_option

/// Gets the middle element of a slice.
fn middle<T>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        return None;
    }
    items.get(items.len() / 2)
}

/// Extracts domain from an email address.
fn email_domain(email: &str) -> Option<&str> {
    let at_pos = email.find('@')?; // returns None if no '@'
    let domain = email.get(at_pos + 1..)?; // returns None if out of bounds
    if domain.is_empty() {
        None
    } else {
        Some(domain)
    }
}

/// Chains multiple Option operations.
fn nested_get(data: &[Vec<i32>], outer: usize, inner: usize) -> Option<i32> {
    let row = data.get(outer)?; // ? on Option
    let val = row.get(inner)?; // ? on Option
    Some(*val)
}

/// Finds the length of the longest word.
fn longest_word_len(text: &str) -> Option<usize> {
    let longest = text.split_whitespace().max_by_key(|w| w.len())?;
    Some(longest.len())
}

fn main() {
    // --- 1. middle ---
    let nums = vec![10, 20, 30, 40, 50];
    println!("middle: {:?}", middle(&nums));
    println!("middle empty: {:?}", middle::<i32>(&[]));

    // --- 2. email_domain ---
    println!("domain: {:?}", email_domain("user@example.com"));
    println!("no @: {:?}", email_domain("invalid"));
    println!("empty domain: {:?}", email_domain("user@"));

    // --- 3. nested_get ---
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("nested_get(0,1): {:?}", nested_get(&matrix, 0, 1));
    println!("nested_get(5,0): {:?}", nested_get(&matrix, 5, 0));

    // --- 4. longest_word_len ---
    println!(
        "longest word: {:?}",
        longest_word_len("the quick brown fox")
    );
    println!("empty: {:?}", longest_word_len(""));

    // --- 5. Mixing ? with Option and Result ---
    // You CANNOT mix them directly:
    // fn foo() -> Option<i32> {
    //     let n: i32 = "42".parse()?; // ERROR: can't use ? on Result in Option fn
    //     Some(n)
    // }
    // Use .ok() to convert Result to Option:
    fn parse_option(s: &str) -> Option<i32> {
        s.parse().ok() // Result → Option (discards error info)
    }
    println!("parse_option(\"42\"): {:?}", parse_option("42"));
    println!("parse_option(\"abc\"): {:?}", parse_option("abc"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle() {
        assert_eq!(middle(&[1, 2, 3, 4, 5]), Some(&3));
        assert_eq!(middle::<i32>(&[]), None);
    }

    #[test]
    fn test_email_domain() {
        assert_eq!(email_domain("a@b.com"), Some("b.com"));
        assert_eq!(email_domain("nope"), None);
        assert_eq!(email_domain("a@"), None);
    }

    #[test]
    fn test_nested_get() {
        let data = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(nested_get(&data, 0, 1), Some(2));
        assert_eq!(nested_get(&data, 5, 0), None);
    }

    #[test]
    fn test_longest_word_len() {
        assert_eq!(longest_word_len("hi there everyone"), Some(8));
        assert_eq!(longest_word_len(""), None);
    }
}
