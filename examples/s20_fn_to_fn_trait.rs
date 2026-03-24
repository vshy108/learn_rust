// Demonstrates passing a regular function where an Fn trait is expected.
//
// Named functions automatically implement Fn, FnMut, and FnOnce.
// You can pass them anywhere a closure is expected.
//
// Run: cargo run --example s20_fn_to_fn_trait

/// A regular named function.
fn double(x: i32) -> i32 {
    x * 2
}

/// Another regular function.
fn is_even(n: &i32) -> bool {
    n % 2 == 0
}

/// Accepts any Fn closure (or function).
fn apply(f: impl Fn(i32) -> i32, value: i32) -> i32 {
    f(value)
}

/// Accepts any Fn predicate.
fn count_matching(items: &[i32], predicate: impl Fn(&i32) -> bool) -> usize {
    items.iter().filter(|x| predicate(x)).count()
}

/// Higher-order: returns a function pointer.
fn get_operation(op: &str) -> fn(i32) -> i32 {
    match op {
        "double" => double,
        "negate" => |x| -x,
        _ => |x| x,
    }
}

fn main() {
    // --- 1. Pass named function as Fn parameter ---
    let result = apply(double, 5);
    println!("apply(double, 5) = {result}");

    // Compare with closure:
    let result2 = apply(|x| x * 2, 5);
    println!("apply(closure, 5) = {result2}");

    // --- 2. Named function as filter predicate ---
    let nums = vec![1, 2, 3, 4, 5, 6];
    let even_count = count_matching(&nums, is_even);
    println!("even count: {even_count}");

    // --- 3. With standard library methods ---
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|&x| double(x)).collect();
    println!("doubled: {doubled:?}");

    // Passing function directly:
    let strings = vec!["42", "abc", "7"];
    let parsed: Vec<Result<i32, _>> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("parsed: {parsed:?}");

    // --- 4. Function pointers ---
    let op = get_operation("double");
    println!("op(10) = {}", op(10));

    // Store in a Vec:
    let operations: Vec<fn(i32) -> i32> = vec![double, |x| x + 1, |x| x * x];
    for (i, op) in operations.iter().enumerate() {
        println!("  op[{i}](3) = {}", op(3));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_function() {
        assert_eq!(apply(double, 5), 10);
    }

    #[test]
    fn test_apply_closure() {
        assert_eq!(apply(|x| x + 1, 5), 6);
    }

    #[test]
    fn test_count_matching() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(count_matching(&v, is_even), 2);
    }

    #[test]
    fn test_get_operation() {
        let op = get_operation("double");
        assert_eq!(op(7), 14);
    }
}
