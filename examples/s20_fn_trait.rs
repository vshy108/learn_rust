// Demonstrates the Fn trait — closures that only read captures.
//
// `Fn` is the most restrictive closure trait: the closure won't
// consume or mutate anything. This makes it safe to call many times,
// share across threads, etc.
//
// Run: cargo run --example s20_fn_trait

/// Calls the closure twice and returns both results.
fn call_twice<F, T>(f: F) -> (T, T)
where
    F: Fn() -> T,
{
    (f(), f())
}

/// Stores a callback for later use.
struct EventHandler<F: Fn(&str)> {
    handler: F,
}

impl<F: Fn(&str)> EventHandler<F> {
    fn new(handler: F) -> Self {
        Self { handler }
    }

    fn trigger(&self, event: &str) {
        (self.handler)(event);
    }
}

/// Filter function that accepts Fn (can call predicate many times).
fn filter_all<T>(items: &[T], predicate: impl Fn(&T) -> bool) -> Vec<&T> {
    items.iter().filter(|item| predicate(item)).collect()
}

fn main() {
    // --- 1. Pure closure (reads nothing mutable) ---
    let (a, b) = call_twice(|| 42);
    println!("call_twice: ({a}, {b})");

    // --- 2. Closure capturing immutable reference ---
    let name = "Rust";
    let (g1, g2) = call_twice(|| format!("Hello, {name}!"));
    println!("{g1}");
    println!("{g2}");

    // --- 3. EventHandler stores Fn closure ---
    let prefix = String::from("[LOG]");
    let handler = EventHandler::new(|event| {
        println!("{prefix} {event}");
    });
    handler.trigger("click");
    handler.trigger("keypress");
    handler.trigger("scroll");

    // --- 4. filter_all with Fn predicate ---
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let evens = filter_all(&nums, |n| *n % 2 == 0);
    println!("evens: {evens:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_twice() {
        let (a, b) = call_twice(|| 99);
        assert_eq!(a, 99);
        assert_eq!(b, 99);
    }

    #[test]
    fn test_filter_all() {
        let v = vec![1, 2, 3, 4];
        let result = filter_all(&v, |&n| n > 2);
        assert_eq!(result.len(), 2);
    }
}
