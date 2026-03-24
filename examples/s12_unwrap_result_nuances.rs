// Demonstrates nuances of unwrap on Result.
//
// `unwrap()` on Result:
// - `Ok(v).unwrap()` → returns v
// - `Err(e).unwrap()` → panics, printing e's Debug representation
//
// Gotcha: unwrap on Result<T, E> requires E: Debug.
//
// Run: cargo run --example s12_unwrap_result_nuances

fn main() {
    // --- 1. unwrap on Ok ---
    let ok: Result<i32, String> = Ok(42);
    println!("unwrap Ok: {}", ok.unwrap());

    // --- 2. unwrap on Err panics ---
    // let err: Result<i32, String> = Err("something went wrong".into());
    // err.unwrap(); // panic: called `Result::unwrap()` on an `Err` value: "something went wrong"

    // --- 3. expect with custom message ---
    let ok: Result<i32, String> = Ok(42);
    let n = ok.expect("should have a value");
    println!("expect: {n}");

    // --- 4. unwrap_err: the inverse ---
    // Useful in tests to verify an error was returned.
    let err: Result<i32, String> = Err("expected error".into());
    let e = err.unwrap_err();
    println!("unwrap_err: {e}");

    // --- 5. unwrap_or / unwrap_or_else / unwrap_or_default ---
    let err: Result<i32, &str> = Err("fail");
    println!("unwrap_or: {}", err.unwrap_or(0));
    println!("unwrap_or_else: {}", err.unwrap_or_else(|_| 99));

    let err: Result<String, &str> = Err("fail");
    println!("unwrap_or_default: \"{}\"", err.unwrap_or_default());

    // --- 6. Pattern: use ? instead of unwrap in real code ---
    // In functions returning Result, prefer `?` over `unwrap()`.
    fn process(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = s.parse()?; // propagate error instead of panicking
        Ok(n * 2)
    }
    println!("process(\"21\"): {:?}", process("21"));
    println!("process(\"abc\"): {:?}", process("abc"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unwrap_ok() {
        assert_eq!(Ok::<i32, &str>(42).unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn test_unwrap_err_panics() {
        Err::<i32, &str>("oops").unwrap();
    }

    #[test]
    fn test_unwrap_err() {
        assert_eq!(Err::<i32, &str>("fail").unwrap_err(), "fail");
    }

    #[test]
    #[should_panic(expected = "must succeed")]
    fn test_expect_err_panics_with_message() {
        Err::<i32, &str>("x").expect("must succeed");
    }

    #[test]
    fn test_unwrap_or_default() {
        assert_eq!(Err::<i32, &str>("e").unwrap_or_default(), 0);
        assert_eq!(Err::<String, &str>("e").unwrap_or_default(), "");
    }
}
