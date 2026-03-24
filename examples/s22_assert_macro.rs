// Demonstrates the assert! macro.
//
// `assert!(expr)` panics if `expr` is false.
// Use it for boolean conditions (unlike assert_eq! for equality).
//
// Run: cargo test --example s22_assert_macro

/// Check if a number is prime.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Check if a string is a palindrome.
fn is_palindrome(s: &str) -> bool {
    let clean: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower = clean.to_lowercase();
    lower == lower.chars().rev().collect::<String>()
}

fn main() {
    println!("is_prime(7) = {}", is_prime(7));
    println!("is_palindrome(\"racecar\") = {}", is_palindrome("racecar"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(97));
    }

    #[test]
    fn test_not_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome(""));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("rust"));
    }
}
