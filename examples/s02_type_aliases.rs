// Demonstrates type aliases in Rust.
//
// A type alias creates an alternative name for an existing type using `type`.
// It does NOT create a new type — the alias and original are interchangeable.
// This is similar to Python's `TypeAlias` or `type` statement (PEP 613/695).
//
// Run: cargo run --example s02_type_aliases

/// Alias for a common Result type used in I/O operations.
/// This pattern is widespread in the standard library (e.g., `std::io::Result`).
type IoResult<T> = Result<T, std::io::Error>;

/// Alias for a complex nested type to improve readability.
type UserScores = std::collections::HashMap<String, Vec<i32>>;

/// Alias for a simple numeric type.
type Meters = f64;
type Seconds = f64;

/// Calculate speed from distance and time.
///
/// # Arguments
///
/// * `distance` - Distance in meters
/// * `time` - Time in seconds
///
/// # Returns
///
/// Speed in meters per second
fn speed(distance: Meters, time: Seconds) -> f64 {
    distance / time
}

/// Simulates an I/O operation that can fail.
///
/// # Errors
///
/// Returns `std::io::Error` if `fail` is true
fn maybe_read(fail: bool) -> IoResult<String> {
    if fail {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ))
    } else {
        Ok("file contents".to_string())
    }
}

fn main() {
    // --- 1. Basic type alias ---
    // `Meters` and `f64` are the SAME type. No conversion needed.
    let distance: Meters = 100.0;
    let time: Seconds = 9.58;
    let s = speed(distance, time);
    println!("Speed: {s:.2} m/s");

    // --- 2. Alias for complex types ---
    // Without the alias, you'd write:
    //   let scores: HashMap<String, Vec<i32>> = HashMap::new();
    let mut scores: UserScores = UserScores::new();
    scores.insert("Alice".to_string(), vec![90, 85, 92]);
    scores.insert("Bob".to_string(), vec![78, 88, 95]);
    for (name, vals) in &scores {
        println!("{name}: {vals:?}");
    }

    // --- 3. Alias for Result types ---
    // The standard library uses this pattern extensively:
    //   type Result<T> = Result<T, io::Error>;
    // So you can write `io::Result<String>` instead of the full type.
    match maybe_read(false) {
        Ok(content) => println!("Read: {content}"),
        Err(e) => println!("Error: {e}"),
    }

    // --- 4. Aliases are NOT newtypes ---
    // The alias and the original type are fully interchangeable.
    // The compiler won't prevent mixing them up:
    let d: Meters = 100.0;
    let t: Seconds = 10.0;
    // Oops: passing time as distance — compiles fine since both are f64.
    let _wrong = speed(t, d);
    println!("Aliases don't prevent mixing: {_wrong:.2}");
    // To prevent this, use newtypes (struct wrapper) — covered later.

    // --- Gotcha 1: aliases don't create new traits/impls ---
    // You can't `impl MyAlias { ... }` separately from the original type.
    // `type Meters = f64;` means `Meters` IS `f64` — same impls, same traits.

    // --- Gotcha 2: type aliases and generics ---
    // Aliases can be generic. The standard library's `io::Result<T>` is:
    //   type Result<T> = Result<T, io::Error>;
    // The error type is "pre-filled", making the alias shorter to use.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_calculation() {
        let s = speed(100.0, 10.0);
        assert!((s - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_alias_interchangeable() {
        // Meters is just f64 — can use f64 methods directly.
        let d: Meters = 3.7;
        assert_eq!(d.floor(), 3.0);
    }

    #[test]
    fn test_io_result_ok() {
        let result = maybe_read(false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "file contents");
    }

    #[test]
    fn test_io_result_err() {
        let result = maybe_read(true);
        assert!(result.is_err());
    }
}
