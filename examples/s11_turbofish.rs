// Demonstrates the turbofish operator `::<>`.
//
// When Rust can't infer a generic type, you can specify it explicitly
// using the "turbofish" syntax: `function::<Type>(args)`.
//
// Named "turbofish" because `::<>` looks like a fish: ::<>
//
// Common places you'll need it:
// - `str::parse::<i32>()`
// - `Vec::<i32>::new()`
// - `Iterator::collect::<Vec<_>>()`
//
// Run: cargo run --example s11_turbofish

fn main() {
    // --- 1. parse() needs to know the target type ---
    let n = "42".parse::<i32>().unwrap();
    println!("Parsed i32: {n}");

    // Alternative: annotate the variable type instead
    let m: f64 = "3.14".parse().unwrap();
    println!("Parsed f64: {m}");

    // --- 2. collect() needs to know the collection type ---
    let chars: Vec<char> = "hello".chars().collect();
    println!("chars: {chars:?}");

    // Or with turbofish on collect:
    let nums = (1..=5).collect::<Vec<i32>>();
    println!("nums: {nums:?}");

    // --- 3. Turbofish on Vec::new ---
    // Usually not needed because push() infers the type.
    let mut v = Vec::<String>::new();
    v.push("turbofish".to_string());
    println!("v: {v:?}");

    // --- 4. Generic function requiring turbofish ---
    fn identity<T>(x: T) -> T {
        x
    }
    let val = identity::<u64>(42);
    println!("identity: {val}");

    // --- 5. When turbofish is required ---
    // This WON'T compile without a type hint:
    // let x = "100".parse().unwrap(); // error: type unclear
    // Fix with turbofish:
    let x = "100".parse::<u32>().unwrap();
    println!("x = {x}");

    // --- 6. Turbofish with multiple generics ---
    fn pair<A, B>(a: A, b: B) -> (A, B) {
        (a, b)
    }
    let p = pair::<i32, &str>(1, "two");
    println!("pair: {p:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_turbofish() {
        let n = "123".parse::<i32>().unwrap();
        assert_eq!(n, 123);
    }

    #[test]
    fn test_collect_turbofish() {
        let v = (1..4).collect::<Vec<i32>>();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_float() {
        let f = "2.5".parse::<f64>().unwrap();
        assert!((f - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_error() {
        let r = "abc".parse::<i32>();
        assert!(r.is_err());
    }
}
