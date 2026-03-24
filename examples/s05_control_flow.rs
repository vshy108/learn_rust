// Demonstrates Rust's control flow constructs: if/else, loop, while, for, match.
//
// Key difference from Python:
//   - `if`, `match`, `loop`, etc. are *expressions* in Rust — they return values.
//   - No parentheses required around conditions (but braces `{}` are mandatory).
//   - No "truthy/falsy" — conditions must be `bool`.
//
// Run: cargo run --example s05_control_flow

fn main() {
    // --- 1. if / else if / else ---
    // Condition must be `bool`. No implicit conversion from int/string.
    let x = 42;
    if x > 100 {
        println!("big");
    } else if x > 10 {
        println!("medium"); // this branch executes
    } else {
        println!("small");
    }

    // `if` as an expression (like Python's ternary `a if cond else b`):
    let label = if x > 50 { "large" } else { "not large" };
    println!("x is {label}");

    // Gotcha: both branches must return the same type.
    // `if x > 0 { "yes" } else { 42 }` won't compile.

    // --- 2. loop (infinite loop) ---
    // `loop` runs forever unless you `break`. It can return a value via `break`.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10; // returns 50
        }
    };
    println!("loop result = {result}");

    // --- 3. while ---
    let mut n = 3;
    while n > 0 {
        println!("while: {n}");
        n -= 1;
    }

    // --- 4. for loop ---
    // Iterates over anything implementing `IntoIterator`.
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // Iterating over a collection:
    let colors = ["red", "green", "blue"];
    for color in &colors {
        print!("{color} ");
    }
    println!();

    // --- 5. match (pattern matching) ---
    // Like Python's `match/case` (3.10+), but exhaustive and more powerful.
    let day = 3;
    let name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 | 7 => "Weekend", // `|` matches multiple patterns
        _ => "Invalid",     // `_` is the catch-all (wildcard)
    };
    println!("day {day} = {name}");

    // Match is exhaustive: every possible value must be covered.
    // Removing `_` when matching an i32 would be a compile error.

    // Match with ranges:
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("score {score} = grade {grade}");

    // --- 6. Loop labels ---
    // You can label loops with `'label:` and break/continue specific loops.
    // Useful for nested loops.
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                println!("breaking outer at i={i}, j={j}");
                break 'outer;
            }
        }
    }

    // --- 7. continue ---
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // skip even numbers
        }
        print!("{i} ");
    }
    println!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_if_expression() {
        let x = 10;
        let label = if x > 5 { "big" } else { "small" };
        assert_eq!(label, "big");
    }

    #[test]
    fn test_loop_with_break_value() {
        let mut i = 0;
        let result = loop {
            i += 1;
            if i == 3 {
                break i * 100;
            }
        };
        assert_eq!(result, 300);
    }

    #[test]
    fn test_match_exhaustive() {
        let val = 2;
        let s = match val {
            1 => "one",
            2 => "two",
            _ => "other",
        };
        assert_eq!(s, "two");
    }

    #[test]
    fn test_match_or_pattern() {
        let day = 6;
        let is_weekend = matches!(day, 6 | 7);
        assert!(is_weekend);
    }

    #[test]
    fn test_for_range_collect() {
        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }
}
