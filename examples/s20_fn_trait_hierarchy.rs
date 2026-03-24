// Demonstrates the Fn trait hierarchy.
//
// Closures implement one or more of these traits:
//
//   FnOnce — can be called once (may consume captured values)
//        ^
//        |
//      FnMut — can be called multiple times (may mutate captures)
//        ^
//        |
//        Fn — can be called multiple times (only reads captures)
//
// Every Fn is also FnMut is also FnOnce.
// The compiler picks the most restrictive trait that works.
//
// Run: cargo run --example s20_fn_trait_hierarchy

/// Accepts Fn: can call multiple times, closure only reads captures.
fn call_twice(f: impl Fn(i32) -> i32) -> (i32, i32) {
    (f(1), f(2))
}

/// Accepts FnMut: can call multiple times, closure may mutate captures.
fn call_counter(mut f: impl FnMut() -> i32) -> Vec<i32> {
    vec![f(), f(), f()]
}

/// Accepts FnOnce: can only call once (closure may consume captures).
fn call_once(f: impl FnOnce() -> String) -> String {
    f()
}

fn main() {
    // --- 1. Fn closure (reads only) ---
    let multiplier = 3;
    let result = call_twice(|x| x * multiplier);
    println!("Fn: {result:?}");

    // --- 2. FnMut closure (mutates captures) ---
    let mut count = 0;
    let results = call_counter(|| {
        count += 1;
        count
    });
    println!("FnMut: {results:?}");

    // --- 3. FnOnce closure (consumes captures) ---
    let name = String::from("Rust");
    let result = call_once(|| {
        // `name` is moved into the closure:
        format!("Hello, {name}!")
        // name is consumed here
    });
    println!("FnOnce: {result}");
    // println!("{name}"); // ERROR: name was moved

    // --- 4. Hierarchy: Fn closures work where FnOnce is expected ---
    let simple = || String::from("always works");
    let _r = call_once(simple); // Fn implements FnOnce

    println!();
    println!("Hierarchy: Fn <: FnMut <: FnOnce");
    println!("Use the LEAST restrictive bound your function needs.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn_closure() {
        let offset = 10;
        assert_eq!(call_twice(|x| x + offset), (11, 12));
    }

    #[test]
    fn test_fn_mut_closure() {
        let mut sum = 0;
        let results = call_counter(|| {
            sum += 1;
            sum
        });
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_fn_once_closure() {
        let data = vec![1, 2, 3];
        let result = call_once(|| format!("{data:?}"));
        assert!(result.contains("[1, 2, 3]"));
    }
}
