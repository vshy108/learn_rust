// Additional module file — demonstrates utils as a module.

/// Repeats a string n times.
pub fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
}

/// Returns the larger of two values.
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_str() {
        assert_eq!(repeat_str("ab", 3), "ababab");
    }

    #[test]
    fn test_max() {
        assert_eq!(max(3, 7), 7);
        assert_eq!(max(10, 2), 10);
    }
}
