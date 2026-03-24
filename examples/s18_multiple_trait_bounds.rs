// Demonstrates multiple trait bounds.
//
// A parameter can require multiple traits with `+`:
//   fn foo(x: &(impl Display + Clone))
//   fn foo<T: Display + Clone>(x: &T)
//
// Run: cargo run --example s18_multiple_trait_bounds

use std::fmt::{Debug, Display};

/// Requires both Display and Debug.
fn inspect<T: Display + Debug>(item: &T) {
    println!("Display: {item}");
    println!("Debug:   {item:?}");
}

/// Using impl Trait syntax with +:
fn clone_and_print(item: &(impl Display + Clone)) {
    let cloned = item.clone();
    println!("original: {item}, clone: {cloned}");
}

/// Multiple bounds on multiple parameters:
fn compare_and_show<T: Display + PartialOrd>(a: &T, b: &T) {
    if a > b {
        println!("{a} > {b}");
    } else {
        println!("{a} <= {b}");
    }
}

fn main() {
    // i32 implements Display + Debug:
    inspect(&42);
    println!();

    // String implements Display + Clone:
    clone_and_print(&String::from("hello"));
    clone_and_print(&42);
    println!();

    // Compare and show:
    compare_and_show(&10, &20);
    compare_and_show(&"zebra", &"apple");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect() {
        // Just verify it compiles with a type that has both traits.
        inspect(&42);
        inspect(&"hello");
    }

    #[test]
    fn test_compare() {
        compare_and_show(&1, &2);
    }
}
