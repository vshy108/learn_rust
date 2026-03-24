// Demonstrates dependency injection for testable code.
//
// Dependency injection uses traits to decouple components,
// making them testable by swapping real implementations with mocks.
//
// Run: cargo test --example s22_dependency_injection

/// Trait for a notification service.
trait Notifier {
    /// Send a notification message.
    fn send(&self, message: &str) -> Result<(), String>;
}

/// Real email notifier (would send actual emails).
#[allow(dead_code)] // Shown for educational purposes; tests use MockNotifier
struct EmailNotifier {
    smtp_server: String,
}

impl Notifier for EmailNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        // In production, this would actually send an email
        println!("Sending email via {}: {message}", self.smtp_server);
        Ok(())
    }
}

/// Order processor that depends on a Notifier (injected).
struct OrderProcessor<N: Notifier> {
    notifier: N,
    orders_processed: u32,
}

impl<N: Notifier> OrderProcessor<N> {
    fn new(notifier: N) -> Self {
        Self {
            notifier,
            orders_processed: 0,
        }
    }

    /// Process an order and send a notification.
    fn process_order(&mut self, order_id: u32) -> Result<(), String> {
        // Business logic...
        self.orders_processed += 1;
        let msg = format!("Order #{order_id} processed");
        self.notifier.send(&msg)?;
        Ok(())
    }

    fn total_processed(&self) -> u32 {
        self.orders_processed
    }
}

fn main() {
    let notifier = EmailNotifier {
        smtp_server: "smtp.example.com".to_string(),
    };
    let mut processor = OrderProcessor::new(notifier);
    processor.process_order(1001).expect("failed to process");
    println!("Total processed: {}", processor.total_processed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// Mock notifier for testing — records messages instead of sending.
    struct MockNotifier {
        messages: RefCell<Vec<String>>,
        should_fail: bool,
    }

    impl MockNotifier {
        fn new() -> Self {
            Self {
                messages: RefCell::new(Vec::new()),
                should_fail: false,
            }
        }

        fn failing() -> Self {
            Self {
                messages: RefCell::new(Vec::new()),
                should_fail: true,
            }
        }

        fn sent_messages(&self) -> Vec<String> {
            self.messages.borrow().clone()
        }
    }

    impl Notifier for MockNotifier {
        fn send(&self, message: &str) -> Result<(), String> {
            if self.should_fail {
                return Err("notification failed".to_string());
            }
            self.messages.borrow_mut().push(message.to_string());
            Ok(())
        }
    }

    #[test]
    fn test_process_order() {
        let mock = MockNotifier::new();
        let mut processor = OrderProcessor::new(mock);

        processor.process_order(101).unwrap();
        assert_eq!(processor.total_processed(), 1);
    }

    #[test]
    fn test_notification_sent() {
        let mock = MockNotifier::new();
        let mut processor = OrderProcessor::new(mock);

        processor.process_order(42).unwrap();
        let messages = processor.notifier.sent_messages();
        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("42"));
    }

    #[test]
    fn test_notification_failure() {
        let mock = MockNotifier::failing();
        let mut processor = OrderProcessor::new(mock);

        let result = processor.process_order(1);
        assert!(result.is_err());
    }
}
