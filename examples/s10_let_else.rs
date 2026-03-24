// Demonstrates the `let else` construct (Rust 1.65+).
//
// `let else` lets you bind a pattern or diverge (return, break, continue, panic).
//
// Syntax:
//   let Some(x) = expression else {
//       // must diverge: return, break, continue, or panic!
//   };
//   // x is available here
//
// It's the inverse of `if let`: instead of "do something if it matches",
// it's "bind or bail out".
//
// Run: cargo run --example s10_let_else

/// Parses a "key=value" string and returns the value part.
///
/// Uses `let else` to early-return on failure.
///
/// # Arguments
///
/// * `input` - A string in "key=value" format
///
/// # Returns
///
/// The value part, or None if format is invalid
fn parse_value(input: &str) -> Option<&str> {
    // let else: if split doesn't produce exactly key=value, return None.
    let Some((_key, value)) = input.split_once('=') else {
        return None;
    };
    // value is now bound and usable
    if value.is_empty() {
        return None;
    }
    Some(value)
}

/// Processes a config line, printing the result.
fn process_config(line: &str) {
    let Some(value) = parse_value(line) else {
        println!("  Skipping invalid line: \"{line}\"");
        return;
    };
    println!("  Value: {value}");
}

fn main() {
    // --- 1. Basic let else with Option ---
    let numbers = vec![10, 20, 30];
    let Some(first) = numbers.first() else {
        println!("Empty vector!");
        return;
    };
    println!("First element: {first}");

    // --- 2. let else in a function ---
    let configs = [
        "host=localhost",
        "port=8080",
        "invalid",
        "debug=",
        "name=rust",
    ];
    for line in configs {
        process_config(line);
    }

    // --- 3. let else with pattern matching ---
    let value: Result<i32, &str> = Ok(42);
    let Ok(num) = value else {
        println!("Error!");
        return;
    };
    println!("Parsed number: {num}");

    // --- 4. Comparison: if let vs let else ---
    let opt: Option<i32> = Some(5);

    // if let: do something IF it matches
    if let Some(x) = opt {
        println!("if let: {x}");
    }

    // let else: bind OR bail out
    let Some(x) = opt else {
        println!("was None");
        return;
    };
    println!("let else: {x}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_valid() {
        assert_eq!(parse_value("key=value"), Some("value"));
        assert_eq!(parse_value("host=localhost"), Some("localhost"));
    }

    #[test]
    fn test_parse_value_no_equals() {
        assert_eq!(parse_value("invalid"), None);
    }

    #[test]
    fn test_parse_value_empty() {
        assert_eq!(parse_value("key="), None);
    }

    #[test]
    fn test_let_else_in_loop() {
        let items: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
        let mut sum: i32 = 0;
        for item in items {
            let Some(val) = item else {
                continue; // diverge with continue
            };
            sum += val;
        }
        assert_eq!(sum, 4);
    }
}
