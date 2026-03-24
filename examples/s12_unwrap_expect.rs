// Demonstrates unwrap() and expect() on Option.
//
// `unwrap()` — extracts the value from Some, panics on None.
// `expect(msg)` — same as unwrap, but panics with your custom message.
//
// **Rule of thumb**:
// - **Never** use unwrap/expect in library code.
// - In application code, `expect("why this should be Some")` is OK
//   for cases that represent genuine programming errors.
// - For user-facing errors, use proper error handling (match, ?, etc.).
//
// Run: cargo run --example s12_unwrap_expect

fn main() {
    // --- 1. unwrap on Some ---
    let val: Option<i32> = Some(42);
    let n = val.unwrap(); // 42
    println!("unwrap Some: {n}");

    // --- 2. unwrap on None → PANIC ---
    // let boom: Option<i32> = None;
    // boom.unwrap(); // panic: called `Option::unwrap()` on a `None` value

    // --- 3. expect — unwrap with a custom message ---
    let config: Option<&str> = Some("production");
    let env = config.expect("CONFIG must be set"); // descriptive message
    println!("expect Some: {env}");

    // --- 4. expect on None → PANIC with your message ---
    // let missing: Option<&str> = None;
    // missing.expect("DATABASE_URL must be set");
    // panic: DATABASE_URL must be set

    // --- 5. When is unwrap acceptable? ---
    // a) In tests:
    //    assert_eq!("42".parse::<i32>().unwrap(), 42);
    //
    // b) When you've already checked:
    let v = vec![1, 2, 3];
    if !v.is_empty() {
        let first = v.first().unwrap(); // safe: we checked
        println!("first: {first}");
    }
    //
    // c) With `expect()` for invariants that should never fail:
    let nums = vec![10, 20, 30];
    let max = nums.iter().max().expect("vec should not be empty");
    println!("max: {max}");

    // --- 6. Safer alternatives ---
    let opt: Option<i32> = None;
    println!("unwrap_or:      {}", opt.unwrap_or(0));
    println!("unwrap_or_else: {}", opt.unwrap_or_else(|| 99));
    // unwrap_or_default works for types implementing Default:
    println!("unwrap_or_default: {}", opt.unwrap_or_default());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unwrap_some() {
        assert_eq!(Some(42).unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_unwrap_none_panics() {
        let opt: Option<i32> = None;
        opt.unwrap();
    }

    #[test]
    fn test_expect_some() {
        assert_eq!(Some("hello").expect("should have value"), "hello");
    }

    #[test]
    #[should_panic(expected = "critical value missing")]
    fn test_expect_none_panics() {
        let opt: Option<i32> = None;
        opt.expect("critical value missing");
    }

    #[test]
    fn test_unwrap_or() {
        assert_eq!(None::<i32>.unwrap_or(10), 10);
        assert_eq!(Some(5).unwrap_or(10), 5);
    }
}
