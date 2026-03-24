// Demonstrates the panic! macro.
//
// `panic!` immediately terminates the current thread with an error.
// It's for unrecoverable errors — situations that should never happen.
//
// By default, panic unwinds the stack (runs destructors).
// You can set it to abort in Cargo.toml: `panic = "abort"`.
//
// Run: cargo run --example s17_panic

fn main() {
    println!("=== The panic! macro ===");
    println!();

    // --- 1. Basic panic (commented out to let the example run) ---
    // panic!("something terrible happened!");

    // --- 2. Conditions that implicitly panic ---
    // let v = vec![1, 2, 3];
    // v[10]; // index out of bounds → panic!

    // let x: i32 = 0;
    // let _y = 1 / x; // division by zero → panic! (in debug mode)

    // --- 3. When to use panic! ---
    println!("Use panic! when:");
    println!("  - A bug is detected (impossible state reached)");
    println!("  - An invariant is violated");
    println!("  - In examples/prototypes for quick failure");
    println!();
    println!("Use Result<T, E> when:");
    println!("  - Failure is expected (file not found, invalid input)");
    println!("  - The caller should decide how to handle it");

    // --- 4. unwrap/expect = conditional panic ---
    let v = vec![1, 2, 3];
    let _first = v.first().expect("vec should not be empty");
    // .expect("msg") is like: match v.first() { Some(x) => x, None => panic!("msg") }

    // --- 5. RUST_BACKTRACE ---
    println!();
    println!("Set RUST_BACKTRACE=1 for detailed panic traces:");
    println!("  RUST_BACKTRACE=1 cargo run --example s17_panic");
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "oh no")]
    fn test_panic() {
        panic!("oh no");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_panic() {
        let v = vec![1, 2, 3];
        let _x = v[10];
    }
}
