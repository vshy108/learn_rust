// Demonstrates closure syntax shortcuts.
//
// Rust offers progressively shorter syntax for closures.
//
// Run: cargo run --example s20_closure_shortcuts

fn apply(f: impl Fn(i32) -> i32, val: i32) -> i32 {
    f(val)
}

fn main() {
    println!("=== Closure Shortcuts ===");

    // 1. Full syntax (like a function):
    let full = |x: i32| -> i32 { x * 2 };

    // 2. Inferred return type:
    let infer_return = |x: i32| x * 2;

    // 3. Inferred parameter types (when used with context):
    let infer_all = |x| x * 2;

    // 4. Single expression — no braces needed:
    let no_braces = |x: i32| x * 2;

    // All produce the same result:
    println!("full:         {}", apply(full, 5));
    println!("infer_return: {}", apply(infer_return, 5));
    println!("infer_all:    {}", apply(infer_all, 5));
    println!("no_braces:    {}", apply(no_braces, 5));

    // --- In practice with iterators ---
    let names = vec!["Alice", "Bob", "Charlie"];

    // Verbose:
    let _lengths: Vec<usize> = names
        .iter()
        .map(|name: &&str| -> usize { name.len() })
        .collect();

    // Idiomatic:
    let lengths: Vec<usize> = names.iter().map(|name| name.len()).collect();
    println!("lengths: {lengths:?}");

    // Even shorter with method reference (not quite a closure):
    // names.iter().map(str::len) — doesn't work for &&str directly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_closure() {
        assert_eq!(apply(|x| x + 1, 10), 11);
    }

    #[test]
    fn test_all_forms_equal() {
        let a = (|x: i32| -> i32 { x * 3 })(4);
        let b = (|x: i32| x * 3)(4);
        let c = (|x| -> i32 { x * 3 })(4);
        assert_eq!(a, b);
        assert_eq!(b, c);
    }
}
