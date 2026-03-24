// Demonstrates multiple lifetime parameters.
//
// When a struct or function involves references with different
// independent lifetimes, use separate lifetime parameters.
//
// Run: cargo run --example s19_multiple_lifetimes

/// Two references that may live for different durations.
#[derive(Debug)]
struct Context<'a, 'b> {
    query: &'a str,
    data: &'b str,
}

impl<'a, 'b> Context<'a, 'b> {
    fn new(query: &'a str, data: &'b str) -> Self {
        Self { query, data }
    }

    fn search(&self) -> Option<&'b str> {
        if self.data.contains(self.query) {
            Some(self.data)
        } else {
            None
        }
    }
}

/// Function with two independent lifetimes.
fn first_of<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a
}

/// When lifetimes should be the SAME (constrained together):
fn pick<'a>(a: &'a str, b: &'a str, first: bool) -> &'a str {
    if first { a } else { b }
}

fn main() {
    let query = String::from("Rust");

    let result;
    {
        let data = String::from("I love Rust programming");
        let ctx = Context::new(&query, &data);
        result = ctx.search();
        println!("found: {result:?}");
    }
    // result can't be used here because data was dropped,
    // and result borrows from data

    // Independent lifetimes:
    let a = String::from("alpha");
    let f;
    {
        let _b = String::from("beta");
        f = first_of(&a, &_b); // result tied to a, not b
    }
    println!("first: {f}"); // OK: a is still alive

    // Same lifetime:
    let x = "hello";
    let y = "world!";
    println!("pick: {}", pick(x, y, true));
    println!("pick: {}", pick(x, y, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_found() {
        let ctx = Context::new("rust", "I love rust");
        assert!(ctx.search().is_some());
    }

    #[test]
    fn test_context_not_found() {
        let ctx = Context::new("python", "I love rust");
        assert!(ctx.search().is_none());
    }

    #[test]
    fn test_first_of() {
        assert_eq!(first_of("a", "b"), "a");
    }

    #[test]
    fn test_pick() {
        assert_eq!(pick("a", "b", true), "a");
        assert_eq!(pick("a", "b", false), "b");
    }
}
