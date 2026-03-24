// Demonstrates the 'static lifetime.
//
// `'static` means the reference is valid for the ENTIRE program duration.
// String literals have 'static lifetime because they're baked into the binary.
//
// Gotcha: `'static` doesn't mean "lives forever" — it means "CAN live
// for the entire program." Owned types satisfy T: 'static too!
//
// Run: cargo run --example s19_static_lifetime

/// Returns a string literal — always 'static.
fn greeting() -> &'static str {
    "Hello, world!" // compiled into the binary
}

/// A const is always 'static.
const VERSION: &str = "1.0.0";

/// Accepts anything that can live for 'static.
fn print_static(s: &'static str) {
    println!("static: {s}");
}

/// The T: 'static bound means T contains no non-static references.
/// Owned types (String, Vec, i32) satisfy this!
fn take_owned<T: std::fmt::Debug + 'static>(val: T) {
    println!("owned: {val:?}");
}

/// A struct that holds a 'static reference.
#[derive(Debug)]
struct AppInfo {
    name: &'static str,
    version: &'static str,
}

static APP_NAME: &str = "MyApp";

fn main() {
    // --- 1. String literals are 'static ---
    let s: &'static str = "I live forever";
    println!("{s}");
    print_static(s);
    print_static(greeting());

    // --- 2. Constants and statics ---
    println!("version: {VERSION}");
    println!("app: {APP_NAME}");

    // --- 3. T: 'static with owned types ---
    take_owned(String::from("owned string")); // String: 'static (owned, no refs)
    take_owned(42_i32); // i32: 'static (no refs)
    take_owned(vec![1, 2, 3]); // Vec: 'static (owned)

    // --- 4. Static struct ---
    let info = AppInfo {
        name: APP_NAME,
        version: VERSION,
    };
    println!("info: {info:?}");
    println!("  name={}, version={}", info.name, info.version);

    // --- 5. Leaked data becomes 'static ---
    // Box::leak intentionally leaks memory to get a 'static reference:
    let leaked: &'static str = Box::leak(String::from("leaked").into_boxed_str());
    print_static(leaked);

    println!();
    println!("Key insight: 'static means 'can live forever', not 'must live forever'");
    println!("String, i32, Vec all satisfy T: 'static because they own their data.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_static() {
        let g = greeting();
        assert_eq!(g, "Hello, world!");
    }

    #[test]
    fn test_app_info() {
        let info = AppInfo {
            name: "test",
            version: "0.0",
        };
        assert_eq!(info.name, "test");
    }
}
