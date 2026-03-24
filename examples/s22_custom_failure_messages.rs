// Demonstrates custom failure messages in assertions.
//
// All assert macros accept an optional message:
//   assert!(cond, "message with {}", format_args)
//
// Run: cargo test --example s22_custom_failure_messages

/// Validate an age value.
fn validate_age(age: i32) -> Result<i32, String> {
    if age < 0 {
        Err(format!("Age cannot be negative: {age}"))
    } else if age > 150 {
        Err(format!("Age unreasonably high: {age}"))
    } else {
        Ok(age)
    }
}

/// Calculate a discount percentage.
fn discount(price: f64, percent: f64) -> f64 {
    price * (1.0 - percent / 100.0)
}

fn main() {
    println!("validate_age(25): {:?}", validate_age(25));
    println!("discount(100, 20): {}", discount(100.0, 20.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_age() {
        let result = validate_age(25);
        assert!(
            result.is_ok(),
            "Expected valid age but got error: {result:?}"
        );
    }

    #[test]
    fn test_negative_age() {
        let result = validate_age(-5);
        assert!(
            result.is_err(),
            "Expected error for negative age, got: {result:?}"
        );
    }

    #[test]
    fn test_discount_amount() {
        let price = 100.0;
        let percent = 20.0;
        let result = discount(price, percent);
        assert_eq!(
            result, 80.0,
            "Expected {price} with {percent}% off to be 80.0, got {result}"
        );
    }

    #[test]
    fn test_discount_range() {
        let result = discount(200.0, 50.0);
        assert!(
            result > 0.0,
            "Discounted price should be positive, got {result}"
        );
        assert!(
            result < 200.0,
            "Discounted price should be less than original, got {result}"
        );
    }
}
