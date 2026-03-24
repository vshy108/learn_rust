// Demonstrates defining a method that accepts an FnOnce closure.
//
// FnOnce closures can be called at most once (they may consume captured values).
// Use FnOnce when your method only needs to call the closure once.
//
// Run: cargo run --example s20_fn_once_trait

/// A configuration builder that applies a setup closure once.
struct App {
    name: String,
    configured: bool,
}

impl App {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            configured: false,
        }
    }

    /// Accepts FnOnce — the setup closure is called exactly once.
    fn setup(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self.configured = true;
        self
    }

    fn describe(&self) -> String {
        format!("App '{}' (configured: {})", self.name, self.configured)
    }
}

/// Generic: run a producer once and return its result.
fn produce<T>(f: impl FnOnce() -> T) -> T {
    f()
}

/// Run with owned data.
fn with_data<T>(data: Vec<T>, f: impl FnOnce(Vec<T>) -> usize) -> usize {
    f(data) // data is moved into f
}

fn main() {
    // --- 1. App setup ---
    let app = App::new("MyApp").setup(|app| {
        app.name = format!("{} v2", app.name);
    });
    println!("{}", app.describe());

    // --- 2. produce ---
    let greeting = produce(|| {
        let parts = vec!["Hello", "World"];
        parts.join(", ")
    });
    println!("produced: {greeting}");

    // --- 3. with_data consumes the vec ---
    let data = vec![1, 2, 3, 4, 5];
    let len = with_data(data, |v| v.len());
    println!("data length: {len}");
    // data is no longer accessible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_setup() {
        let app = App::new("Test").setup(|a| {
            a.name = "Configured".to_string();
        });
        assert!(app.configured);
        assert_eq!(app.name, "Configured");
    }

    #[test]
    fn test_produce() {
        let val = produce(|| 42);
        assert_eq!(val, 42);
    }

    #[test]
    fn test_with_data() {
        let result = with_data(vec![1, 2, 3], |v| v.iter().sum());
        assert_eq!(result, 6);
    }
}
