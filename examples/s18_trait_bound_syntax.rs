// Demonstrates trait bound syntax.
//
// `impl Trait` is sugar for the longer trait bound syntax:
//   fn foo(x: &impl Trait)       // sugar
//   fn foo<T: Trait>(x: &T)      // full syntax
//
// The full syntax is needed when you need the same type for multiple params.
//
// Run: cargo run --example s18_trait_bound_syntax

use std::fmt::Display;

/// impl Trait syntax (sugar).
fn print_impl(item: &impl Display) {
    println!("impl Trait: {item}");
}

/// Trait bound syntax (equivalent).
fn print_bound<T: Display>(item: &T) {
    println!("trait bound: {item}");
}

/// When you need SAME type for both parameters:
fn largest<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> &'a T {
    if a >= b { a } else { b }
}

/// impl Trait allows DIFFERENT types:
fn print_pair(a: &impl Display, b: &impl Display) {
    println!("{a} and {b}");
}

/// Trait bound forces SAME type:
fn print_same<T: Display>(a: &T, b: &T) {
    println!("{a} and {b}");
}

fn main() {
    // Both work the same:
    print_impl(&42);
    print_bound(&42);

    // Largest needs same type:
    println!("largest: {}", largest(&3, &7));
    println!("largest: {}", largest(&"apple", &"banana"));

    // Different types allowed with impl Trait:
    print_pair(&42, &"hello");

    // Same type required with generic bound:
    print_same(&1, &2);
    print_same(&"a", &"b");
    // print_same(&1, &"hello"); // ERROR: mismatched types
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest() {
        assert_eq!(*largest(&3, &7), 7);
        assert_eq!(*largest(&10, &5), 10);
    }
}
