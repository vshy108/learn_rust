// Demonstrates the unwrap_or family of methods on Option.
//
// These methods provide a default value when the Option is None,
// without panicking:
//
// - `unwrap_or(default)` — returns `default` if None.
// - `unwrap_or_else(|| closure)` — lazily computes default if None.
// - `unwrap_or_default()` — uses the type's Default impl.
//
// Run: cargo run --example s12_unwrap_or

fn main() {
    // --- 1. unwrap_or: static fallback ---
    let some: Option<i32> = Some(42);
    let none: Option<i32> = None;
    println!("some.unwrap_or(0): {}", some.unwrap_or(0));
    println!("none.unwrap_or(0): {}", none.unwrap_or(0));

    // --- 2. unwrap_or_else: lazy fallback ---
    // The closure only runs if the Option is None.
    let val = none.unwrap_or_else(|| {
        println!("  Computing default...");
        99
    });
    println!("unwrap_or_else: {val}");

    // --- 3. unwrap_or_default: uses Default trait ---
    let s: Option<String> = None;
    let default_s = s.unwrap_or_default(); // String::default() → ""
    println!("unwrap_or_default String: \"{default_s}\"");

    let n: Option<i32> = None;
    let default_n = n.unwrap_or_default(); // i32::default() → 0
    println!("unwrap_or_default i32: {default_n}");

    let v: Option<Vec<i32>> = None;
    let default_v = v.unwrap_or_default(); // Vec::default() → []
    println!("unwrap_or_default Vec: {default_v:?}");

    // --- 4. Practical: config with defaults ---
    fn get_port() -> Option<u16> {
        None // simulate missing config
    }
    let port = get_port().unwrap_or(8080);
    println!("Port: {port}");

    // --- 5. unwrap_or vs unwrap_or_else ---
    // unwrap_or always evaluates the default (eager).
    // unwrap_or_else only evaluates if needed (lazy).
    fn expensive_default() -> i32 {
        println!("  expensive_default called!");
        42
    }
    let x: Option<i32> = Some(10);
    // This DOES call expensive_default even though x is Some:
    let _ = x.unwrap_or(expensive_default());
    // This does NOT call the closure because x is Some:
    let _ = x.unwrap_or_else(expensive_default);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unwrap_or_some() {
        assert_eq!(Some(5).unwrap_or(0), 5);
    }

    #[test]
    fn test_unwrap_or_none() {
        assert_eq!(None::<i32>.unwrap_or(42), 42);
    }

    #[test]
    fn test_unwrap_or_else() {
        let result = None::<i32>.unwrap_or_else(|| 99);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_unwrap_or_default_string() {
        let s: Option<String> = None;
        assert_eq!(s.unwrap_or_default(), "");
    }

    #[test]
    fn test_unwrap_or_default_int() {
        assert_eq!(None::<i32>.unwrap_or_default(), 0);
    }
}
