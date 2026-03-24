// Demonstrates assert_ne! macro.
//
// `assert_ne!(left, right)` panics if left == right.
// Useful for checking that mutation occurred or values differ.
//
// Run: cargo test --example s22_assert_ne

/// Generate a username from a name (simplified).
fn generate_username(name: &str) -> String {
    let clean: String = name.chars().filter(|c| c.is_alphanumeric()).collect();
    clean.to_lowercase()
}

/// Rotate a value by adding an offset (wrapping).
fn rotate(value: u8, offset: u8) -> u8 {
    value.wrapping_add(offset)
}

fn main() {
    println!("username: {}", generate_username("John Doe"));
    println!("rotate(100, 50): {}", rotate(100, 50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_differs_from_input() {
        let input = "John Doe";
        let username = generate_username(input);
        assert_ne!(username, input); // should be transformed
    }

    #[test]
    fn test_username_no_spaces() {
        let username = generate_username("Jane Smith");
        assert_ne!(username.contains(' '), true);
    }

    #[test]
    fn test_rotate_changes_value() {
        let original: u8 = 100;
        let rotated = rotate(original, 50);
        assert_ne!(original, rotated);
    }

    #[test]
    fn test_different_names_different_usernames() {
        let u1 = generate_username("Alice");
        let u2 = generate_username("Bob");
        assert_ne!(u1, u2);
    }
}
