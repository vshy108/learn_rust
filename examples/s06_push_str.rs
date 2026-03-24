// Demonstrates `push_str()` on `String`.
//
// `push_str(&str)` appends a string slice to the end of a `String`.
// It takes a `&str`, meaning it borrows the data — no ownership transfer.
//
// Related methods:
// - `push(char)` — appends a single character.
// - `+` operator — consumes the left String, borrows the right &str.
// - `format!()` — creates a new String without consuming anything.
//
// Run: cargo run --example s06_push_str

fn main() {
    // --- 1. Basic push_str ---
    let mut greeting = String::from("Hello");
    greeting.push_str(", world");
    println!("{greeting}"); // Hello, world

    // --- 2. push_str takes &str, so the source remains usable ---
    let suffix = String::from("!");
    greeting.push_str(&suffix); // &String coerces to &str via Deref
    println!("{greeting}"); // Hello, world!
    println!("suffix still usable: {suffix}");

    // --- 3. push(char) for single characters ---
    let mut s = String::from("abc");
    s.push('d');
    println!("{s}"); // abcd

    // --- 4. + operator consumes the left operand ---
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = s1 + &s2; // s1 is MOVED into s3, s2 is borrowed
    // println!("{s1}"); // error: s1 was moved
    println!("{s3}, s2 still ok: {s2}");

    // --- 5. format! macro — no ownership transfer ---
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    let result = format!("{a}-{b}-{c}");
    println!("{result}");
    // a, b, c are all still valid because format! only borrows.

    // --- 6. Capacity growth ---
    let mut buf = String::with_capacity(4);
    println!("cap before: {}", buf.capacity());
    buf.push_str("hello world, this is a longer string");
    println!("cap after:  {}", buf.capacity());
    // When push_str exceeds capacity, String reallocates (roughly doubles).
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_push_str_appends() {
        let mut s = String::from("hello");
        s.push_str(" world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_push_char() {
        let mut s = String::from("ab");
        s.push('c');
        assert_eq!(s, "abc");
    }

    #[test]
    fn test_plus_operator() {
        let s1 = String::from("foo");
        let s2 = String::from("bar");
        let s3 = s1 + &s2;
        assert_eq!(s3, "foobar");
        // s2 still valid
        assert_eq!(s2, "bar");
    }

    #[test]
    fn test_format_no_move() {
        let a = String::from("x");
        let b = String::from("y");
        let result = format!("{a}{b}");
        assert_eq!(result, "xy");
        // a, b still valid
        assert_eq!(a, "x");
        assert_eq!(b, "y");
    }
}
