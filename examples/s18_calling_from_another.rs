// Demonstrates calling one trait method from another.
//
// Default methods can call other trait methods (even required ones).
// This lets you build rich APIs from a small number of required methods.
//
// Run: cargo run --example s18_calling_from_another

/// A logger trait where only `log` is required.
/// Other methods build on top of it.
trait Logger {
    /// Required: every logger must implement this.
    fn log(&self, message: &str);

    /// Default: calls log with INFO prefix.
    fn info(&self, message: &str) {
        self.log(&format!("[INFO] {message}"));
    }

    /// Default: calls log with WARN prefix.
    fn warn(&self, message: &str) {
        self.log(&format!("[WARN] {message}"));
    }

    /// Default: calls log with ERROR prefix.
    fn error(&self, message: &str) {
        self.log(&format!("[ERROR] {message}"));
    }
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("{message}");
    }
}

struct PrefixLogger {
    prefix: String,
}

impl Logger for PrefixLogger {
    fn log(&self, message: &str) {
        println!("[{}] {message}", self.prefix);
    }
}

fn main() {
    let console = ConsoleLogger;
    console.info("Server started");
    console.warn("High memory usage");
    console.error("Connection refused");

    println!();

    let prefixed = PrefixLogger {
        prefix: "APP".into(),
    };
    prefixed.info("Initialized");
    prefixed.warn("Deprecated API");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLogger {
        messages: std::cell::RefCell<Vec<String>>,
    }

    impl TestLogger {
        fn new() -> Self {
            Self {
                messages: std::cell::RefCell::new(Vec::new()),
            }
        }
    }

    impl Logger for TestLogger {
        fn log(&self, message: &str) {
            self.messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn test_info_calls_log() {
        let logger = TestLogger::new();
        logger.info("test");
        let msgs = logger.messages.borrow();
        assert_eq!(msgs.len(), 1);
        assert!(msgs[0].contains("[INFO]"));
    }

    #[test]
    fn test_error_calls_log() {
        let logger = TestLogger::new();
        logger.error("bad");
        let msgs = logger.messages.borrow();
        assert!(msgs[0].contains("[ERROR]"));
    }
}
