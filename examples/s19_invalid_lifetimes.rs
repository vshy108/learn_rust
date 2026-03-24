// Demonstrates invalid lifetimes — situations the borrow checker rejects.
//
// Understanding WHY something fails helps you write correct Rust.
//
// Run: cargo run --example s19_invalid_lifetimes

fn main() {
    println!("=== Invalid Lifetimes (what the compiler rejects) ===");

    // --- 1. Dangling reference (would be invalid) ---
    // let r;
    // {
    //     let x = 5;
    //     r = &x;   // ERROR: x will be dropped
    // }
    // println!("{r}"); // dangling!

    // Fix: ensure the referenced value lives long enough:
    let x = 5;
    let r = &x;
    println!("r = {r}"); // OK: x is still alive

    // --- 2. Use after move ---
    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{s1}"); // ERROR: s1 was moved to s2

    // Fix: clone or borrow:
    let s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("s1 still valid: {s1}");

    // --- 3. Mutable + immutable borrow overlap ---
    let mut v = vec![1, 2, 3];
    // let r = &v[0];
    // v.push(4);       // ERROR: can't mutate while r exists
    // println!("{r}");  // r used here

    // Fix: use immutable borrow BEFORE mutating:
    let first = v[0]; // Copy the value
    v.push(4);
    println!("first = {first}, v = {v:?}");

    // --- 4. Returning reference to local variable ---
    // fn bad() -> &String {
    //     let s = String::from("local");
    //     &s  // ERROR: s is dropped when function returns
    // }

    // Fix: return owned value:
    fn good() -> String {
        String::from("owned")
    }
    println!("good() = {}", good());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clone_avoids_move() {
        let s1 = String::from("test");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_copy_before_mutate() {
        let mut v = vec![1, 2, 3];
        let first = v[0]; // i32 is Copy
        v.push(4);
        assert_eq!(first, 1);
    }
}
