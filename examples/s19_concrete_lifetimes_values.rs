// Demonstrates concrete lifetimes for values.
//
// Every value in Rust has a **lifetime** — the period from creation
// to when it's dropped. For owned values, this is the scope they live in.
//
// Rust tracks lifetimes at compile time, so there's no GC overhead.
//
// Run: cargo run --example s19_concrete_lifetimes_values

fn main() {
    println!("=== Concrete Lifetimes for Values ===");

    // --- 1. Value with block scope ---
    {
        let x = 42; // x is born here
        println!("x = {x}");
    } // x is dropped here — its lifetime ends

    // println!("{x}"); // ERROR: x no longer exists

    // --- 2. Nested scopes ---
    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        println!("{outer}, {inner}"); // both alive
    } // inner dropped here
    println!("{outer}"); // outer still alive
    // println!("{inner}"); // ERROR: inner no longer exists

    // --- 3. Shadowing creates a new value ---
    let s = String::from("first");
    println!("s = {s}");
    let s = String::from("second"); // new value, new lifetime
    println!("s = {s}");

    // --- 4. Move transfers lifetime ownership ---
    let a = String::from("hello");
    let b = a; // a's value moves to b
    // println!("{a}"); // ERROR: a is no longer valid
    println!("b = {b}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scope_lifetime() {
        let s;
        {
            let inner = String::from("hello");
            s = inner.clone(); // clone so s owns its own copy
        }
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1; // new binding
        assert_eq!(x, 6);
    }
}
