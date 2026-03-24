// Demonstrates closures with ownership (consuming captures).
//
// If a closure moves a captured value (e.g., returns it or drops it),
// it implements FnOnce — it can only be called once.
//
// Run: cargo run --example s20_closures_ownership

fn consume_closure(f: impl FnOnce() -> String) -> String {
    f()
}

fn main() {
    println!("=== Closures with Ownership ===");

    // --- 1. Closure that moves a String ---
    let name = String::from("Rust");
    let make_greeting = || {
        // `name` is moved INTO the closure's return value:
        format!("Hello from {name}!")
    };
    let greeting = make_greeting();
    println!("{greeting}");
    // make_greeting(); // ERROR: FnOnce, already called
    // println!("{name}"); // ERROR: name was moved

    // --- 2. Closure that drops a value ---
    let data = vec![1, 2, 3];
    let consume = || {
        drop(data); // data is consumed
        println!("data consumed!");
    };
    consume();
    // consume(); // ERROR: can't call again
    // println!("{data:?}"); // ERROR: data moved

    // --- 3. Passing to FnOnce parameter ---
    let items = vec!["a".to_string(), "b".to_string()];
    let result = consume_closure(|| {
        items.into_iter().collect::<String>() // consumes items
    });
    println!("joined: {result}");

    // --- 4. Copy types don't cause FnOnce ---
    let x = 42; // i32 is Copy
    let read_x = || x; // copies x, doesn't move
    println!("{}", read_x());
    println!("{}", read_x()); // can call multiple times
    println!("x still: {x}"); // x still valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_closure() {
        let s = String::from("test");
        let result = consume_closure(|| format!("got: {s}"));
        assert!(result.contains("test"));
    }

    #[test]
    fn test_copy_not_moved() {
        let n = 5;
        let f = || n * 2;
        assert_eq!(f(), 10);
        assert_eq!(f(), 10); // can call multiple times
    }
}
