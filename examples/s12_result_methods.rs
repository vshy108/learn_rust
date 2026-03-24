// Demonstrates Result methods (map, and_then, unwrap_or, etc.).
//
// Result has many of the same combinators as Option:
//
// - `map(f)` — transforms Ok value, passes Err through.
// - `map_err(f)` — transforms Err value, passes Ok through.
// - `and_then(f)` — chains fallible operations (flatmap).
// - `unwrap_or(default)` — returns Ok value or default.
// - `ok()` — converts to Option<T> (discards error).
// - `err()` — converts to Option<E> (discards success).
//
// Run: cargo run --example s12_result_methods

fn main() {
    // --- 1. map: transform Ok value ---
    let result: Result<i32, String> = Ok(5);
    let doubled = result.map(|n| n * 2);
    println!("map Ok: {doubled:?}"); // Ok(10)

    let err: Result<i32, String> = Err("oops".into());
    let doubled_err = err.map(|n| n * 2);
    println!("map Err: {doubled_err:?}"); // Err("oops")

    // --- 2. map_err: transform Error ---
    let err: Result<i32, &str> = Err("parse error");
    let mapped = err.map_err(|e| format!("FATAL: {e}"));
    println!("map_err: {mapped:?}");

    // --- 3. and_then: chain fallible operations ---
    fn parse_positive(s: &str) -> Result<u32, String> {
        let n: u32 = s.parse().map_err(|e| format!("parse: {e}"))?;
        if n > 0 {
            Ok(n)
        } else {
            Err("must be positive".into())
        }
    }

    let chained = Ok("42".to_string()).and_then(|s| parse_positive(&s));
    println!("and_then Ok: {chained:?}");

    let chained_err = Ok("abc".to_string()).and_then(|s| parse_positive(&s));
    println!("and_then Err: {chained_err:?}");

    // --- 4. unwrap_or / unwrap_or_else ---
    #[allow(clippy::unnecessary_literal_unwrap)] // intentional: demonstrating the method on known values
    let ok: Result<i32, &str> = Ok(42);
    #[allow(clippy::unnecessary_literal_unwrap)]
    let err_val: Result<i32, &str> = Err("fail");
    println!("unwrap_or Ok: {}", ok.unwrap_or(0));
    println!("unwrap_or Err: {}", err_val.unwrap_or(0));

    // --- 5. ok() / err() — convert to Option ---
    let r: Result<i32, &str> = Ok(10);
    println!("ok(): {:?}", r.ok()); // Some(10)
    println!("err(): {:?}", r.err()); // None

    let e: Result<i32, &str> = Err("bad");
    println!("ok(): {:?}", e.ok()); // None
    println!("err(): {:?}", e.err()); // Some("bad")

    // --- 6. or_else: provide fallback Result ---
    #[allow(clippy::unnecessary_lazy_evaluations)] // intentional: demonstrating or_else
    let fallback: Result<i32, &str> = Err("first").or_else(|_| Ok(99));
    println!("or_else: {fallback:?}"); // Ok(99)

    // --- 7. is_ok_and / is_err_and (Rust 1.70+) ---
    let r: Result<i32, &str> = Ok(42);
    println!("is_ok_and > 10: {}", r.is_ok_and(|n| n > 10));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_map() {
        assert_eq!(Ok::<_, &str>(5).map(|n| n * 2), Ok(10));
        assert_eq!(Err::<i32, _>("e").map(|n| n * 2), Err("e"));
    }

    #[test]
    fn test_map_err() {
        assert_eq!(
            Err::<i32, _>("oops").map_err(|e| e.to_uppercase()),
            Err("OOPS".to_string())
        );
    }

    #[test]
    fn test_and_then() {
        let r = Ok(5).and_then(|n: i32| if n > 0 { Ok(n.to_string()) } else { Err("neg") });
        assert_eq!(r, Ok("5".to_string()));
    }

    #[test]
    fn test_ok_err() {
        assert_eq!(Ok::<_, &str>(1).ok(), Some(1));
        assert_eq!(Ok::<_, &str>(1).err(), None);
        assert_eq!(Err::<i32, _>("e").ok(), None);
        assert_eq!(Err::<i32, _>("e").err(), Some("e"));
    }
}
