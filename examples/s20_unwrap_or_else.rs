// Demonstrates the unwrap_or_else method.
//
// `unwrap_or_else` is like `unwrap_or`, but takes a closure
// for lazy default computation.
//
// Run: cargo run --example s20_unwrap_or_else

fn main() {
    println!("=== unwrap_or_else ===");

    // Clippy flags calling unwrap_or/unwrap_or_else on literal Some/None/Ok/Err values
    // because the result is statically known. We do it here intentionally to demonstrate
    // the API — in real code, these would come from function returns.

    // --- 1. Option::unwrap_or_else ---
    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;

    // unwrap_or: always evaluates the default
    let a = some_val.unwrap_or(0);
    let b = none_val.unwrap_or(0);
    println!("unwrap_or: a={a}, b={b}");

    // unwrap_or_else: only calls closure when None
    let a = some_val.unwrap_or_else(|| {
        println!("  (computing default for some_val)");
        expensive_default()
    });
    let b = none_val.unwrap_or_else(|| {
        println!("  (computing default for none_val)");
        expensive_default()
    });
    println!("unwrap_or_else: a={a}, b={b}");
    // Notice: "computing default" only prints for none_val

    // --- 2. Result::unwrap_or_else ---
    let ok: Result<i32, String> = Ok(100);
    let err: Result<i32, String> = Err("not found".into());

    let a = ok.unwrap_or_else(|e| {
        println!("  Error: {e}");
        -1
    });
    let b = err.unwrap_or_else(|e| {
        println!("  Error: {e}");
        -1
    });
    println!("Result: a={a}, b={b}");

    // --- 3. Practical: parse with fallback ---
    let input = "abc";
    let num = input.parse::<i32>().unwrap_or_else(|_| {
        println!("  '{input}' isn't a number, using 0");
        0
    });
    println!("parsed: {num}");
}

fn expensive_default() -> i32 {
    // Simulate expensive computation
    999
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_option_unwrap_or_else() {
        let x: Option<i32> = None;
        assert_eq!(x.unwrap_or_else(|| 42), 42);

        let y: Option<i32> = Some(10);
        assert_eq!(y.unwrap_or_else(|| 42), 10);
    }

    #[test]
    fn test_result_unwrap_or_else() {
        let ok: Result<i32, &str> = Ok(5);
        assert_eq!(ok.unwrap_or_else(|_| 0), 5);

        let err: Result<i32, &str> = Err("bad");
        assert_eq!(err.unwrap_or_else(|_| 0), 0);
    }
}
