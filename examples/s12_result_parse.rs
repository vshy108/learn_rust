// Demonstrates Result with the parse method.
//
// `str::parse::<T>()` returns `Result<T, T::Err>`.
// It uses the `FromStr` trait under the hood.
//
// Run: cargo run --example s12_result_parse

fn main() {
    // --- 1. Successful parse ---
    let n: Result<i32, _> = "42".parse();
    println!("\"42\" → {n:?}");

    // --- 2. Failed parse ---
    let bad: Result<i32, _> = "hello".parse();
    println!("\"hello\" → {bad:?}");

    // --- 3. Turbofish vs type annotation ---
    let a = "3.14".parse::<f64>(); // turbofish
    let b: Result<f64, _> = "3.14".parse(); // type annotation
    println!("a = {a:?}, b = {b:?}");

    // --- 4. Handling parse results ---
    let inputs = ["10", "abc", "42", "", "99"];
    for input in inputs {
        match input.parse::<i32>() {
            Ok(n) => println!("  \"{input}\" → {n}"),
            Err(e) => println!("  \"{input}\" → error: {e}"),
        }
    }

    // --- 5. Chaining with map ---
    let doubled = "21".parse::<i32>().map(|n| n * 2);
    println!("doubled: {doubled:?}");

    // --- 6. The ? operator in a function ---
    fn add_strings(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
        let x: i32 = a.parse()?;
        let y: i32 = b.parse()?;
        Ok(x + y)
    }
    println!("add(\"10\", \"20\"): {:?}", add_strings("10", "20"));
    println!("add(\"10\", \"xx\"): {:?}", add_strings("10", "xx"));

    // --- 7. Parsing different types ---
    let _: Result<bool, _> = "true".parse();
    let _: Result<char, _> = "a".parse();
    let _: Result<std::net::IpAddr, _> = "127.0.0.1".parse();
    println!("bool: {:?}", "true".parse::<bool>());
    println!("char: {:?}", "z".parse::<char>());
    println!("ip:   {:?}", "127.0.0.1".parse::<std::net::IpAddr>());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_int_ok() {
        assert_eq!("42".parse::<i32>(), Ok(42));
    }

    #[test]
    fn test_parse_int_err() {
        assert!("abc".parse::<i32>().is_err());
    }

    #[test]
    fn test_parse_float() {
        let f = "3.14".parse::<f64>().unwrap();
        assert!((f - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!("true".parse::<bool>(), Ok(true));
        assert_eq!("false".parse::<bool>(), Ok(false));
        assert!("yes".parse::<bool>().is_err());
    }
}
