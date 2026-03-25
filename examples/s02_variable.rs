// Demonstrates variable basics in Rust.
//
// Key points:
//   - Variables are immutable by default (use `mut` to make them mutable).
//   - Rust is statically typed, but the compiler can infer types.
//   - Variables must be initialized before use (no `None` equivalent by default).
//   - Unused variables generate warnings unless prefixed with `_`.
//
// Run: cargo run --example s02_variable

fn main() {
    // --- 1. Immutable by default ---
    // `let` binds a value to a name. The binding is immutable by default.
    // Trying to reassign would cause a compile error:
    //   `x = 10;`  // error[E0384]: cannot assign twice to immutable variable
    let x = 5;
    println!("x = {x}");

    // --- 2. Mutable variables ---
    // Add `mut` to allow reassignment. This is an explicit opt-in.
    let mut y = 10;
    println!("y before = {y}");
    y = 20;
    println!("y after  = {y}");

    // Gotcha: `mut` allows changing the *value*, but NOT the *type*.
    // `y = "hello";` would fail because y is `i32`, not `&str`.

    // --- 3. Type inference ---
    // Rust infers the type from the assigned value. These are equivalent:
    let inferred = 42; // compiler infers `i32` (the default integer type)
    let explicit: i32 = 42; // explicitly annotated
    println!("inferred = {inferred}, explicit = {explicit}");

    // Gotcha: the default integer type is `i32`, and the default float is `f64`.
    // If Rust can't infer the type, you'll get a compile error asking for
    // a type annotation.

    // --- 4. Uninitialized variables ---
    // Rust requires variables to be initialized before use.
    // This prevents undefined behavior that's common in C/C++.
    // Clippy prefers `let later = 99;` but we split declaration and
    // initialization here to show that Rust tracks initialization status.
    #[allow(clippy::needless_late_init)]
    let later: i32;
    // `println!("{later}");` here would fail:
    //   "used binding `later` isn't initialized"
    later = 99;
    println!("later = {later}");

    // --- 5. Unused variable warnings ---
    // Rust warns about unused variables. Prefix with `_` to suppress:
    let _unused = "this won't trigger a warning";

    // Without the underscore, `cargo clippy` or the compiler would warn:
    // "warning: unused variable: `something`"

    // --- 6. Multiple bindings at once (destructuring) ---
    // You can bind multiple values using tuple destructuring.
    let (a, b, c) = (1, 2.0, "three");
    println!("a = {a}, b = {b}, c = {c}");

    // --- 7. Type suffixes on literals ---
    // You can annotate the type directly on the literal value.
    let small = 255u8; // u8: unsigned 8-bit (0..=255)
    let big = 1_000_000i64; // i64: signed 64-bit
    let pi = 2.72f32; // f32: 32-bit float
    println!("small = {small}, big = {big}, pi = {pi}");

    // --- 8. Immutability does NOT mean "constant" ---
    // `let` bindings are stack-allocated at runtime, not compile-time constants.
    // For true compile-time constants, use `const` (see s02_constants).
    // `let` values can vary per function call; `const` cannot.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_immutable_binding() {
        let x = 5;
        assert_eq!(x, 5);
        // x = 10; // Would not compile: E0384
    }

    #[test]
    fn test_mutable_binding() {
        let mut y = 10;
        y += 5;
        assert_eq!(y, 15);
    }

    #[test]
    fn test_type_inference_defaults() {
        // Default integer is i32.
        let n = 42;
        assert_eq!(std::mem::size_of_val(&n), 4); // i32 = 4 bytes

        // Default float is f64.
        let f = 3.14;
        assert_eq!(std::mem::size_of_val(&f), 8); // f64 = 8 bytes
    }

    #[test]
    fn test_destructuring() {
        let (a, b) = (10, 20);
        assert_eq!(a + b, 30);
    }

    #[test]
    fn test_type_suffix_literals() {
        let val = 255u8;
        assert_eq!(val, u8::MAX);
    }

    #[test]
    fn test_deferred_initialization() {
        let x: i32;
        x = 42;
        assert_eq!(x, 42);
    }
}
