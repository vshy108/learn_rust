// Demonstrates `if let` — a concise way to match one pattern.
//
// `if let` is syntactic sugar for a `match` that handles one pattern
// and ignores everything else:
//
//   // These are equivalent:
//   if let Some(val) = opt { ... }
//   match opt { Some(val) => { ... }, _ => {} }
//
// Use `if let` when you only care about ONE variant.
// Use `match` when you need to handle ALL variants.
//
// Run: cargo run --example s10_if_let_construct

#[derive(Debug)]
enum Config {
    Path(String),
    Port(u16),
    Debug,
}

fn main() {
    // --- 1. Basic if let ---
    let some_value: Option<i32> = Some(42);
    if let Some(x) = some_value {
        println!("Got value: {x}");
    }

    // --- 2. if let with else ---
    let maybe: Option<&str> = None;
    if let Some(s) = maybe {
        println!("Found: {s}");
    } else {
        println!("Nothing found");
    }

    // --- 3. if let with custom enum ---
    let configs = vec![
        Config::Path("/tmp".to_string()),
        Config::Port(8080),
        Config::Debug,
        Config::Path("/var/log".to_string()),
    ];

    for cfg in &configs {
        if let Config::Path(p) = cfg {
            println!("Path config: {p}");
        }
    }

    // --- 4. if let in combination with else if ---
    let input: Option<i32> = Some(-5);
    if let Some(n) = input {
        if n > 0 {
            println!("Positive: {n}");
        } else {
            println!("Non-positive: {n}");
        }
    } else {
        println!("No input");
    }

    // --- 5. Comparing: match vs if let ---
    let val: Option<i32> = Some(10);

    // match: must handle all variants
    match val {
        Some(x) => println!("match: {x}"),
        None => println!("match: none"),
    }

    // if let: concise, only handles one case
    if let Some(x) = val {
        println!("if let: {x}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_if_let_some() {
        let opt = Some(42);
        let mut result = 0;
        if let Some(x) = opt {
            result = x;
        }
        assert_eq!(result, 42);
    }

    #[test]
    fn test_if_let_none() {
        let opt: Option<i32> = None;
        let mut was_none = false;
        if let Some(_) = opt {
            // not reached
        } else {
            was_none = true;
        }
        assert!(was_none);
    }

    #[test]
    fn test_if_let_enum() {
        use super::Config;
        let cfg = Config::Port(3000);
        let mut found_port = false;
        if let Config::Port(p) = cfg {
            assert_eq!(p, 3000);
            found_port = true;
        }
        assert!(found_port);
    }
}
