// Demonstrates closures capturing immutable references.
//
// By default, closures capture variables by the least restrictive
// method needed. If the closure only reads a variable, it captures
// by immutable reference (&T).
//
// Run: cargo run --example s20_capture_immut_refs

fn main() {
    println!("=== Closures Capturing Immutable References ===");

    // --- 1. Captures `name` by &String ---
    let name = String::from("Alice");
    let greet = || println!("Hello, {name}!");
    greet();
    greet(); // can call multiple times
    println!("name still accessible: {name}"); // original untouched

    // --- 2. Captures multiple values by reference ---
    let base = 100;
    let multiplier = 3;
    let compute = |x: i32| base + x * multiplier;
    println!("compute(5) = {}", compute(5));
    println!("base = {base}, multiplier = {multiplier}"); // still accessible

    // --- 3. In iterators — filter captures by reference ---
    let threshold = 3;
    let nums = vec![1, 2, 3, 4, 5];
    let above: Vec<_> = nums.iter().filter(|&&n| n > threshold).collect();
    println!("above {threshold}: {above:?}");

    // --- 4. Closure inside a function ---
    fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
        // `base` is moved into the closure (it's Copy)
        move |x| base + x
    }
    let add_ten = make_adder(10);
    println!("add_ten(5) = {}", add_ten(5));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_immut_capture() {
        let val = 42;
        let read_val = || val;
        assert_eq!(read_val(), 42);
        assert_eq!(val, 42); // still accessible
    }

    #[test]
    fn test_filter_capture() {
        let min = 2;
        let v: Vec<i32> = vec![1, 2, 3, 4].into_iter().filter(|&n| n >= min).collect();
        assert_eq!(v, vec![2, 3, 4]);
    }
}
