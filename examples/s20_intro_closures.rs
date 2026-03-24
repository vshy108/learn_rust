// Introduces closures — anonymous functions that capture their environment.
//
// Closures are like Python's `lambda` but much more powerful:
// - Can capture surrounding variables
// - Can have multiple statements
// - Type inference for parameters and return
//
// Syntax: |params| expression
//
// Run: cargo run --example s20_intro_closures

fn main() {
    println!("=== Intro to Closures ===");

    // --- 1. Basic closure ---
    let greet = |name: &str| format!("Hello, {name}!");
    println!("{}", greet("Rust"));

    // --- 2. Type inference ---
    let double = |x| x * 2; // types inferred from usage
    println!("double(5) = {}", double(5));

    // --- 3. Capturing environment ---
    let offset = 10;
    let add_offset = |x| x + offset; // captures `offset`
    println!("add_offset(5) = {}", add_offset(5));

    // --- 4. Multi-line closure ---
    let classify = |n: i32| -> &str {
        if n > 0 {
            "positive"
        } else if n < 0 {
            "negative"
        } else {
            "zero"
        }
    };
    println!("classify(-3) = {}", classify(-3));

    // --- 5. Closure as function parameter ---
    let nums = vec![1, 2, 3, 4, 5];
    let evens: Vec<_> = nums.into_iter().filter(|n| n % 2 == 0).collect();
    println!("evens: {evens:?}");

    // --- 6. Immediately invoked closure ---
    let result = (|a, b| a + b)(3, 7);
    println!("immediately invoked: {result}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_closure_capture() {
        let base = 100;
        let add = |x| x + base;
        assert_eq!(add(5), 105);
    }

    #[test]
    fn test_closure_as_arg() {
        let v = vec![1, 2, 3, 4];
        let sum: i32 = v.iter().filter(|&&x| x > 2).sum();
        assert_eq!(sum, 7);
    }
}
