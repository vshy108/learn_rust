// Demonstrates using Result<T, E> in tests.
//
// Instead of panicking, tests can return `Result<(), E>`.
// Returning `Err` fails the test with the error message.
// This enables using the `?` operator inside tests.
//
// Run: cargo test --example s22_result_in_tests

/// Parse a config value from a string.
fn parse_config(input: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = input.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("invalid config: {input}"));
    }
    Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
}

fn main() {
    match parse_config("name = rust") {
        Ok((key, val)) => println!("{key} = {val}"),
        Err(e) => println!("error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Using Result allows `?` operator in tests
    #[test]
    fn test_parse_config() -> Result<(), String> {
        let (key, val) = parse_config("name = rust")?;
        assert_eq!(key, "name");
        assert_eq!(val, "rust");
        Ok(())
    }

    #[test]
    fn test_parse_config_no_spaces() -> Result<(), String> {
        let (key, val) = parse_config("debug=true")?;
        assert_eq!(key, "debug");
        assert_eq!(val, "true");
        Ok(())
    }

    #[test]
    fn test_parse_invalid() {
        let result = parse_config("no_equals_sign");
        assert!(result.is_err());
    }

    // Using Box<dyn Error> for mixed error types
    #[test]
    fn test_parse_number() -> Result<(), Box<dyn std::error::Error>> {
        let value: i32 = "42".parse()?;
        assert_eq!(value, 42);
        Ok(())
    }
}
