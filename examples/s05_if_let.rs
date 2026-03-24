// Demonstrates `if let` — a concise way to match a single pattern.
//
// `match` is powerful but verbose when you only care about one variant.
// `if let` is syntactic sugar for a `match` with one arm + a wildcard `_`.
//
// Python equivalent: roughly `if (m := re.match(...)): use(m)`
// but Rust's `if let` works with *any* pattern, not just Option/Result.
//
// Run: cargo run --example s05_if_let

fn main() {
    // --- 1. Basic if let with Option ---
    let some_value: Option<i32> = Some(42);

    // Verbose match version:
    match some_value {
        Some(v) => println!("match: got {v}"),
        None => {}
    }

    // Equivalent `if let` — much cleaner when you don't care about `None`:
    if let Some(v) = some_value {
        println!("if let: got {v}");
    }

    // --- 2. if let with else ---
    let no_value: Option<i32> = None;

    if let Some(v) = no_value {
        println!("got {v}");
    } else {
        println!("if let else: got nothing");
    }

    // --- 3. if let with enums ---
    #[derive(Debug)]
    enum Coin {
        Penny,
        Quarter(String), // quarters can have state names
    }

    let coin = Coin::Quarter("Alaska".to_string());

    // Only care about quarters:
    if let Coin::Quarter(state) = &coin {
        println!("Quarter from {state}");
    }
    // If `coin` were `Penny`, this block would be skipped entirely.

    // --- 4. if let with references ---
    let names = vec!["Alice", "Bob", "Charlie"];
    // `.first()` returns `Option<&&str>`
    if let Some(first) = names.first() {
        println!("first name: {first}");
    }

    // --- 5. Chaining if let (Rust 2024 edition) ---
    // You can chain `if let` with `&&`:
    let a: Option<i32> = Some(10);
    let b: Option<i32> = Some(20);
    if let Some(x) = a
        && let Some(y) = b
    {
        println!("both present: {x} + {y} = {}", x + y);
    }

    // --- 6. if let vs match: when to use which ---
    // Use `if let` when you care about ONE pattern and want to ignore the rest.
    // Use `match` when you need exhaustive handling of all variants.
    // Gotcha: `if let` does NOT check exhaustiveness — easy to miss cases.

    // --- 7. if let with Result ---
    let parsed: Result<i32, _> = "123".parse();
    if let Ok(n) = parsed {
        println!("parsed: {n}");
    }

    // Compare with unwrap (which panics on Err):
    // let n = "123".parse::<i32>().unwrap();

    println!("coin = {coin:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_if_let_some() {
        let opt = Some(42);
        let mut result = 0;
        if let Some(v) = opt {
            result = v;
        }
        assert_eq!(result, 42);
    }

    #[test]
    fn test_if_let_none_skips_body() {
        let opt: Option<i32> = None;
        let mut entered = false;
        if let Some(_v) = opt {
            entered = true;
        }
        assert!(!entered);
    }

    #[test]
    fn test_if_let_else() {
        let opt: Option<i32> = None;
        let result = if let Some(v) = opt { v } else { -1 };
        assert_eq!(result, -1);
    }

    #[test]
    fn test_if_let_result() {
        let parsed: Result<i32, _> = "42".parse();
        let mut value = 0;
        if let Ok(n) = parsed {
            value = n;
        }
        assert_eq!(value, 42);
    }
}
