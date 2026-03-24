// Demonstrates the Option<T> enum.
//
// `Option<T>` is Rust's replacement for null/None in other languages.
// A value is either present (`Some(T)`) or absent (`None`).
//
//   enum Option<T> {
//       Some(T),
//       None,
//   }
//
// Rust has NO null pointers. Option forces you to explicitly handle
// the "nothing" case, preventing null pointer errors at compile time.
//
// Run: cargo run --example s12_option_enum

fn main() {
    // --- 1. Creating Option values ---
    let some_num: Option<i32> = Some(42);
    let no_num: Option<i32> = None;
    println!("some_num = {some_num:?}");
    println!("no_num = {no_num:?}");

    // --- 2. You MUST handle both cases ---
    // Can't use `some_num` as an i32 directly:
    // let result = some_num + 1; // error: can't add Option<i32> + i32
    // Must unwrap or pattern-match first.

    // --- 3. Pattern matching ---
    match some_num {
        Some(n) => println!("Got: {n}"),
        None => println!("Got nothing"),
    }

    // --- 4. Common operations ---
    println!("is_some: {}", some_num.is_some());
    println!("is_none: {}", no_num.is_none());

    // --- 5. map: transform the inner value ---
    let doubled = some_num.map(|n| n * 2);
    println!("doubled: {doubled:?}"); // Some(84)
    let none_doubled = no_num.map(|n| n * 2);
    println!("none_doubled: {none_doubled:?}"); // None

    // --- 6. and_then (flatmap): chain Option-returning operations ---
    let result = some_num.and_then(|n| if n > 0 { Some(n.to_string()) } else { None });
    println!("and_then: {result:?}");

    // --- 7. Option with String ---
    let name: Option<String> = Some("Rust".to_string());
    let no_name: Option<String> = None;
    println!("name: {name:?}, no_name: {no_name:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_some() {
        let opt = Some(10);
        assert!(opt.is_some());
        assert!(!opt.is_none());
    }

    #[test]
    fn test_none() {
        let opt: Option<i32> = None;
        assert!(opt.is_none());
        assert!(!opt.is_some());
    }

    #[test]
    fn test_map_some() {
        let opt = Some(5);
        assert_eq!(opt.map(|x| x * 3), Some(15));
    }

    #[test]
    fn test_map_none() {
        let opt: Option<i32> = None;
        assert_eq!(opt.map(|x| x * 3), None);
    }

    #[test]
    fn test_and_then() {
        let positive = Some(5).and_then(|n| if n > 0 { Some(n) } else { None });
        assert_eq!(positive, Some(5));
        let negative = Some(-1).and_then(|n| if n > 0 { Some(n) } else { None });
        assert_eq!(negative, None);
    }
}
