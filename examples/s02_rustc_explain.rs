// Demonstrates `rustc --explain` for understanding compiler error codes.
//
// When Rust shows an error like `error[E0384]`, you can run:
//   rustc --explain E0384
// to get a detailed explanation with examples.
//
// This file intentionally triggers common errors (commented out) and
// shows the error code you'd see, along with how `--explain` helps.
//
// Run: cargo run --example s02_rustc_explain

fn main() {
    // --- 1. E0384: cannot assign twice to immutable variable ---
    // Uncomment the lines below to see this error:
    //   let x = 5;
    //   x = 10;  // error[E0384]
    //
    // Fix: add `mut`
    let mut x = 5;
    println!("x was {x}");
    x = 10;
    println!("E0384 fix: x = {x}");
    // Run `rustc --explain E0384` to learn about immutability.

    // --- 2. E0308: mismatched types ---
    // Uncomment to see:
    //   let y: i32 = "hello";  // error[E0308]: expected `i32`, found `&str`
    //
    // Rust is statically typed — no implicit type coercion like Python.
    let y: i32 = 42;
    println!("E0308 fix: y = {y}");

    // --- 3. E0425: cannot find value in this scope ---
    // Uncomment to see:
    //   println!("{z}");  // error[E0425]: cannot find value `z` in this scope
    //
    // Unlike Python, Rust requires variables to be declared before use.
    let z = "declared first";
    println!("E0425 fix: z = {z}");

    // --- 4. E0382: use of moved value ---
    // Uncomment to see:
    //   let s1 = String::from("hello");
    //   let s2 = s1;          // s1 is moved to s2
    //   println!("{}", s1);   // error[E0382]: value used after move
    //
    // In Python, assignment copies a reference. In Rust, heap-allocated
    // types like String are *moved* — the original binding is invalidated.
    let s1 = String::from("hello");
    let _s2 = s1.clone(); // clone() to keep both valid
    println!("E0382 fix: s1 is cloned, not moved");

    // --- 5. How to use `rustc --explain` ---
    // In your terminal, run:
    //   rustc --explain E0384
    //   rustc --explain E0308
    //   rustc --explain E0425
    //   rustc --explain E0382
    //
    // Each returns a multi-page explanation with:
    //   - What the error means
    //   - Why Rust enforces this rule
    //   - Code examples showing the error and the fix
    //
    // Tip: You can also use `cargo` to trigger errors and then copy the
    // error code directly into `rustc --explain`.

    // --- 6. Finding error codes in IDE ---
    // In VS Code with rust-analyzer, error codes appear in the
    // "Problems" panel and are clickable links to the documentation.
    // This is faster than running `rustc --explain` manually.

    println!();
    println!("Try running these in your terminal:");
    println!("  rustc --explain E0384");
    println!("  rustc --explain E0308");
    println!("  rustc --explain E0425");
    println!("  rustc --explain E0382");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mutable_reassignment() {
        // Demonstrates the fix for E0384.
        let mut val = 1;
        val = 2;
        assert_eq!(val, 2);
    }

    #[test]
    fn test_clone_prevents_move() {
        // Demonstrates the fix for E0382.
        let s1 = String::from("hello");
        let s2 = s1.clone();
        // Both s1 and s2 are valid after clone.
        assert_eq!(s1, s2);
    }
}
