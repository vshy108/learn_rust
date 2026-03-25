#![allow(dead_code, unused_imports, unused_parens)]
// Introduces generics at a high level — just enough to read Rust code.
//
// Rust functions are monomorphic by default. Generics let you write
// code that works across multiple types while keeping compile-time safety.
//
// This is a *preview*. Full generics coverage is in Section 11.
//
// Run: cargo run --example s03_generics_intro

/// Return whichever value is larger.
///
/// `T: PartialOrd` is a **trait bound**: it says "T can be any type,
/// as long as it supports comparison with `<`, `>`, etc."
/// Without the bound, the compiler wouldn't know that `>` is valid for `T`.
fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

/// A point that works with any numeric-like type.
///
/// `#[derive(Debug)]` auto-generates the Debug trait so we can print it.
/// The `<T>` after the struct name makes it generic over type `T`.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // --- 1. Generic functions ---
    // The compiler generates separate code for each concrete type used.
    // This is called **monomorphization** — zero runtime cost.
    println!("larger(3, 7)       = {}", larger(3, 7)); // i32
    println!("larger(3.1, 2.9)   = {}", larger(3.1, 2.9)); // f64
    println!("larger('a', 'z')   = {}", larger('a', 'z')); // char

    // --- 2. Generic structs ---
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!("int_point   = {int_point:?}");
    println!("float_point = {float_point:?}");

    // Gotcha: both fields must be the *same* type T.
    // `Point { x: 5, y: 2.5 }` won't compile because 5 is i32 and 2.5 is f64.
    // To mix types, you'd need `Point<T, U>` with two type parameters.

    // --- 3. Type inference with generics ---
    // Rust infers the concrete type from usage. You rarely need turbofish.
    let result = larger(10, 20); // compiler infers T = i32
    println!("result = {result}");

    // But sometimes the compiler needs help — that's where turbofish comes in:
    //   larger::<f64>(1.0, 2.0)
    // Turbofish is covered in detail in Section 11 (`s11_turbofish`).

    // --- 4. The Option and Result enums are generic ---
    // `Option<T>` is `Some(T)` or `None`.
    // `Result<T, E>` is `Ok(T)` or `Err(E)`.
    // You've been using generics all along without realizing it.
    let maybe: Option<i32> = Some(42);
    println!("maybe = {maybe:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_larger_integers() {
        assert_eq!(larger(3, 7), 7);
        assert_eq!(larger(10, 2), 10);
    }

    #[test]
    fn test_larger_floats() {
        assert!((larger(1.5_f64, 2.5_f64) - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_larger_chars() {
        assert_eq!(larger('a', 'z'), 'z');
    }

    #[test]
    fn test_generic_struct() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn test_generic_struct_float() {
        let p = Point { x: 1.0, y: 2.0 };
        assert!((p.x - 1.0f64).abs() < f64::EPSILON);
    }
}
