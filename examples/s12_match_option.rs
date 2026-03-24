// Demonstrates using `match` with Option<T>.
//
// `match` forces exhaustive handling of both Some and None,
// making it impossible to forget the "nothing" case.
//
// Run: cargo run --example s12_match_option

/// Divides two numbers, returning None for division by zero.
///
/// # Arguments
///
/// * `a` - Numerator
/// * `b` - Denominator
///
/// # Returns
///
/// Some(result) or None if b is zero
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

/// Looks up a user by ID.
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

fn main() {
    // --- 1. Match on Option ---
    let result = safe_divide(10.0, 3.0);
    match result {
        Some(val) => println!("10 / 3 = {val:.4}"),
        None => println!("Division by zero!"),
    }

    match safe_divide(5.0, 0.0) {
        Some(val) => println!("5 / 0 = {val}"),
        None => println!("Cannot divide by zero!"),
    }

    // --- 2. Match with binding and computation ---
    let user = find_user(1);
    match user {
        Some(name) => println!("Found user: {name}"),
        None => println!("User not found"),
    }

    // --- 3. Match returning a value ---
    let greeting = match find_user(2) {
        Some(name) => format!("Hello, {name}!"),
        None => "Hello, stranger!".to_string(),
    };
    println!("{greeting}");

    // --- 4. Nested match ---
    let a = safe_divide(100.0, 4.0);
    let b = safe_divide(50.0, 0.0);
    let sum = match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    };
    println!("sum of divisions: {sum:?}");

    // --- 5. Match with guard ---
    let opt = Some(42);
    match opt {
        Some(n) if n > 100 => println!("Big: {n}"),
        Some(n) if n > 0 => println!("Positive: {n}"),
        Some(n) => println!("Non-positive: {n}"),
        None => println!("Nothing"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide_ok() {
        match safe_divide(10.0, 2.0) {
            Some(v) => assert!((v - 5.0).abs() < f64::EPSILON),
            None => panic!("expected Some"),
        }
    }

    #[test]
    fn test_safe_divide_zero() {
        assert_eq!(safe_divide(1.0, 0.0), None);
    }

    #[test]
    fn test_find_user_found() {
        assert_eq!(find_user(1), Some("Alice".to_string()));
    }

    #[test]
    fn test_find_user_not_found() {
        assert_eq!(find_user(999), None);
    }
}
