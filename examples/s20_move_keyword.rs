// Demonstrates the `move` keyword with closures.
//
// `move` forces the closure to take ownership of all captured variables,
// even if it only reads them. Essential when the closure outlives the
// current scope (e.g., threads, async tasks).
//
// Run: cargo run --example s20_move_keyword

use std::thread;

fn main() {
    println!("=== The move Keyword ===");

    // --- 1. Without move: closure borrows ---
    let x = 10;
    let read = || println!("x = {x}"); // borrows x
    read();
    println!("x still: {x}"); // x is fine

    // --- 2. With move: closure takes ownership ---
    let name = String::from("Alice");
    let greet = move || {
        println!("Hello, {name}!");
        // `name` is now owned by the closure
    };
    greet();
    // println!("{name}"); // ERROR: name was moved into closure

    // --- 3. move is REQUIRED for threads ---
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // The thread might outlive the current function,
        // so it MUST own its data.
        println!("thread data: {data:?}");
        data.iter().sum::<i32>()
    });
    // println!("{data:?}"); // ERROR: data was moved
    let sum = handle.join().expect("thread panicked");
    println!("thread sum: {sum}");

    // --- 4. move with Copy types ---
    let n = 42;
    let closure = move || n * 2; // copies n (i32 is Copy)
    println!("closure: {}", closure());
    println!("n still: {n}"); // n is still valid! (copied, not moved)

    // --- 5. Returning closures requires move ---
    fn make_greeter(name: String) -> impl Fn() {
        move || println!("Hello, {name}!")
    }
    let greet_bob = make_greeter("Bob".into());
    greet_bob();
    greet_bob();
}

#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn test_move_to_thread() {
        let v = vec![10, 20];
        let handle = thread::spawn(move || v.iter().sum::<i32>());
        assert_eq!(handle.join().unwrap(), 30);
    }

    #[test]
    fn test_move_copy_type() {
        let x = 5;
        let f = move || x + 1;
        assert_eq!(f(), 6);
        assert_eq!(x, 5); // still valid (Copy)
    }
}
