// Demonstrates string concatenation in Rust.
//
// Several ways to combine strings:
// 1. `+` operator (takes ownership of left side)
// 2. `format!` macro (no ownership transfer)
// 3. `push_str` / `push` (in-place mutation)
// 4. Iterator methods
//
// Run: cargo run --example s15_concatenation

fn main() {
    // --- 1. The + operator ---
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2; // s1 is MOVED (can't use it anymore!)
    println!("+ operator: {s3}");
    // println!("{s1}"); // ERROR: s1 was moved
    println!("s2 still usable: {s2}");

    // The + operator calls: fn add(self, s: &str) -> String
    // Left side: takes ownership (self)
    // Right side: borrows (&str)

    // --- 2. Chaining + ---
    let a = String::from("a");
    let b = String::from("b");
    let c = String::from("c");
    let abc = a + "-" + &b + "-" + &c;
    println!("chained: {abc}");

    // --- 3. format! macro — cleaner, no ownership issues ---
    let first = String::from("Rust");
    let last = String::from("Lang");
    let full = format!("{first} {last}");
    println!("format!: {full}");
    // Both first and last are still usable:
    println!("still have: {first}, {last}");

    // --- 4. push_str and push ---
    let mut s = String::from("Hello");
    s.push_str(", ");
    s.push_str("World");
    s.push('!');
    println!("push_str/push: {s}");

    // --- 5. concat on slices ---
    let parts: Vec<&str> = vec!["one", "two", "three"];
    let joined: String = parts.concat();
    println!("concat: {joined}");

    // --- 6. join with separator ---
    let joined = parts.join(", ");
    println!("join: {joined}");

    // --- 7. Iterator collect ---
    let words = vec!["Rust", "is", "great"];
    let sentence: String = words.iter().copied().collect::<Vec<&str>>().join(" ");
    println!("iter join: {sentence}");

    // --- 8. Repeat ---
    let repeated = "ha".repeat(3);
    println!("repeat: {repeated}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_plus_operator() {
        let s1 = String::from("Hello");
        let s2 = String::from(" World");
        assert_eq!(s1 + &s2, "Hello World");
    }

    #[test]
    fn test_format() {
        let a = "x";
        let b = "y";
        assert_eq!(format!("{a}-{b}"), "x-y");
    }

    #[test]
    fn test_join() {
        let v = vec!["a", "b", "c"];
        assert_eq!(v.join("-"), "a-b-c");
    }

    #[test]
    fn test_repeat() {
        assert_eq!("ab".repeat(3), "ababab");
    }
}
