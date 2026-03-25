// Demonstrates variable shadowing in Rust.
//
// Shadowing lets you re-declare a variable with the same name using `let`.
// The new binding *shadows* (hides) the previous one. This is NOT the same
// as mutation (`mut`) — it creates an entirely new variable.
//
// Rust's shadowing can change the *type* of the binding.
//
// Run: cargo run --example s02_variable_shadowing

fn main() {
    // --- 1. Basic shadowing ---
    // Each `let x` creates a new binding that shadows the previous one.
    let x = 5;
    println!("x = {x}"); // 5
    let x = x + 1;
    println!("x = {x}"); // 6
    let x = x * 2;
    println!("x = {x}"); // 12

    // --- 2. Shadowing can change the type ---
    // Unlike `mut`, shadowing allows a completely different type.
    // This is the key advantage over mutation.
    let spaces = "   "; // &str
    println!("spaces (str): '{spaces}'");
    let spaces = spaces.len(); // usize — different type, same name
    println!("spaces (len): {spaces}");

    // With `mut`, changing type would fail:
    //   let mut spaces = "   ";
    //   spaces = spaces.len();  // error[E0308]: mismatched types

    // --- 3. Shadowing in inner scopes ---
    // A shadow inside a block only lasts for that block.
    let y = 10;
    {
        let y = y + 20; // shadows outer y inside this block
        println!("inner y = {y}"); // 30
    }
    println!("outer y = {y}"); // 10 — the outer y is unchanged

    // --- 4. Shadowing vs mut ---
    // Shadowing: creates a NEW immutable binding (old value is dropped).
    // `mut`: keeps the SAME binding, allows changing its value.
    //
    // Use shadowing when:
    //   - You want to transform a value and keep the same name
    //   - You want to change the type
    //   - You want the result to be immutable
    //
    // Use `mut` when:
    //   - You need to update the value in a loop (e.g., counter)
    //   - You need to pass a mutable reference to a function

    // --- 5. Shadowing with pattern matching ---
    // Shadowing is common with `if let` and `match` (covered later).
    let value: Option<i32> = Some(42);
    // `value` here shadows with an unwrapped i32 inside the block.
    if let Some(value) = value {
        println!("unwrapped value = {value}"); // 42
    }

    // --- Gotcha 1: shadowing does NOT modify the original ---
    // Each `let` allocates a new slot. The old value still exists until
    // the old binding goes out of scope (and is dropped).
    let a = String::from("hello");
    let _ref_to_a = &a; // borrow a
    // `let a = ...` below would NOT invalidate _ref_to_a if we don't
    // use _ref_to_a after the shadow. But if we do, we get a borrow error.
    // Uncomment to see E0506:
    //   let a = String::from("world");
    //   println!("{}", _ref_to_a);
    println!("a = {a}");

    // --- Gotcha 2: shadowing in the same scope is sequential ---
    // Each `let` sees the value from the PREVIOUS binding.
    let n = 1;
    let n = n + 1; // n is now 2, the old `1` is shadowed
    let n = n * n; // n is now 4, the old `2` is shadowed
    println!("final n = {n}"); // 4
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        assert_eq!(x, 12);
    }

    #[test]
    fn test_shadowing_changes_type() {
        let val = "hello"; // &str
        let val = val.len(); // usize
        assert_eq!(val, 5);
    }

    #[test]
    fn test_scope_shadowing() {
        let x = 10;
        {
            let x = x + 20;
            assert_eq!(x, 30);
        }
        // Outer x is untouched.
        assert_eq!(x, 10);
    }

    #[test]
    fn test_sequential_shadowing() {
        let n = 1;
        let n = n + 1;
        let n = n * n;
        assert_eq!(n, 4);
    }
}
