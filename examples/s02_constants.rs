// Demonstrates constants in Rust.
//
// Rust has two kinds of compile-time values:
//   `const`  — a true compile-time constant (inlined everywhere it's used)
//   `static` — a global variable with a fixed memory address ('static lifetime)
//
// Unlike `let` bindings, `const` and `static` require explicit type annotations.
//
// Run: cargo run --example s02_constants

// --- 1. `const`: compile-time constant ---
// Must have a type annotation. Value must be a constant expression
// (computable at compile time). Cannot be `mut`.
// Convention: SCREAMING_SNAKE_CASE.
const MAX_PLAYERS: u32 = 100;
const TAU_HALF: f64 = std::f64::consts::PI;
const GREETING: &str = "Welcome to Rust!";

// `const` can use const-evaluable expressions:
const DOUBLED_MAX: u32 = MAX_PLAYERS * 2;

// --- 2. `static`: global variable with fixed address ---
// Like `const`, but has a single memory location for the entire program.
// Can be `mut` (but accessing `static mut` requires `unsafe`).
static APP_VERSION: &str = "1.0.0";

fn main() {
    // --- Using constants ---
    println!("Max players: {MAX_PLAYERS}");
    println!("Doubled max: {DOUBLED_MAX}");
    println!("Pi (from std): {TAU_HALF:.8}");
    println!("{GREETING}");
    println!("App version: {APP_VERSION}");

    // --- 3. const vs let ---
    // `const`: compile-time, no fixed address, inlined by compiler
    // `let`:   runtime, stack-allocated, can be computed dynamically
    //
    // `let` bindings can use runtime values:
    let runtime_val = std::time::SystemTime::now();
    println!("Runtime: {runtime_val:?}");
    // `const` CANNOT:
    // const BAD: ... = std::time::SystemTime::now(); // error: not const

    // --- 4. const inside functions ---
    // Constants can be declared inside functions. Their scope is limited
    // to that function, but they're still compile-time constants.
    const LOCAL_CONST: i32 = 42;
    println!("Local const: {LOCAL_CONST}");

    // --- 5. const vs static ---
    // `const`: inlined at each use site (like C's #define, but type-safe)
    // `static`: single allocation, has a memory address, lives for 'static
    //
    // Use `const` by default. Use `static` only when you need:
    //   - A fixed memory address (e.g., for FFI)
    //   - Interior mutability with `Mutex` or `AtomicU32`

    // --- 6. const in type contexts ---
    // Constants can be used where compile-time values are needed:
    const BUFFER_SIZE: usize = 1024;
    let _buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    println!("Buffer size: {BUFFER_SIZE}");

    // A `let` variable CANNOT be used for array sizes:
    // let size = 1024;
    // let buf: [u8; size] = [0; size];  // error: not a constant

    // --- Gotcha 1: `const` has no address ---
    // You can't take a reference to a `const` that persists.
    // The compiler may create a temporary each time you use `&CONST`.
    // If you need `&'static T`, use `static` instead.

    // --- Gotcha 2: `static mut` is unsafe ---
    // Mutable global state requires `unsafe` because Rust can't enforce
    // thread safety at compile time for globals.
    // Prefer `std::sync::LazyLock`, `std::sync::OnceLock`, or `Atomic*`.

    // --- Gotcha 3: const evaluation limits ---
    // Not all expressions are const-evaluable. Things like heap allocation
    // (String::new()), I/O, and panicking (in some contexts) aren't allowed.
    // The set of const-evaluable operations grows with each Rust release.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_values() {
        assert_eq!(MAX_PLAYERS, 100);
        assert_eq!(DOUBLED_MAX, 200);
    }

    #[test]
    fn test_const_in_array_size() {
        const SIZE: usize = 4;
        let arr = [0u8; SIZE];
        assert_eq!(arr.len(), 4);
    }

    #[test]
    fn test_static_has_stable_address() {
        // Taking the address of a static twice gives the same pointer.
        let ptr1: *const &str = &APP_VERSION;
        let ptr2: *const &str = &APP_VERSION;
        assert_eq!(ptr1, ptr2);
    }

    #[test]
    fn test_const_expression() {
        const A: u32 = 10;
        const B: u32 = A * A + 1;
        assert_eq!(B, 101);
    }
}
