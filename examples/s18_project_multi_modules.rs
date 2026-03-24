// Demonstrates project structure with multiple trait-implementing modules.
//
// Pattern: one trait module, many implementors in separate modules.
//
// Run: cargo run --example s18_project_multi_modules

/// The core trait module.
mod notifier {
    pub trait Notifier {
        /// Sends a notification with message.
        fn notify(&self, message: &str) -> String;
        /// Returns the notifier's name.
        fn name(&self) -> &str;
    }
}

/// Email notifier module.
mod email {
    use super::notifier::Notifier;

    pub struct EmailNotifier {
        pub address: String,
    }

    impl Notifier for EmailNotifier {
        fn notify(&self, message: &str) -> String {
            format!("Email to {}: {message}", self.address)
        }
        fn name(&self) -> &str {
            "Email"
        }
    }
}

/// SMS notifier module.
mod sms {
    use super::notifier::Notifier;

    pub struct SmsNotifier {
        pub phone: String,
    }

    impl Notifier for SmsNotifier {
        fn notify(&self, message: &str) -> String {
            format!("SMS to {}: {message}", self.phone)
        }
        fn name(&self) -> &str {
            "SMS"
        }
    }
}

/// Slack notifier module.
mod slack {
    use super::notifier::Notifier;

    pub struct SlackNotifier {
        pub channel: String,
    }

    impl Notifier for SlackNotifier {
        fn notify(&self, message: &str) -> String {
            format!("Slack #{}: {message}", self.channel)
        }
        fn name(&self) -> &str {
            "Slack"
        }
    }
}

use email::EmailNotifier;
use notifier::Notifier;
use slack::SlackNotifier;
use sms::SmsNotifier;

/// Sends to all notifiers — depends on trait, not concrete types.
fn broadcast(notifiers: &[&dyn Notifier], message: &str) {
    for n in notifiers {
        println!("[{}] {}", n.name(), n.notify(message));
    }
}

fn main() {
    let e = EmailNotifier {
        address: "user@example.com".into(),
    };
    let s = SmsNotifier {
        phone: "+1234567890".into(),
    };
    let sl = SlackNotifier {
        channel: "general".into(),
    };

    let all: Vec<&dyn Notifier> = vec![&e, &s, &sl];
    broadcast(&all, "Server deployed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email() {
        let e = EmailNotifier {
            address: "a@b.com".into(),
        };
        assert!(e.notify("hi").contains("a@b.com"));
    }

    #[test]
    fn test_sms() {
        let s = SmsNotifier {
            phone: "123".into(),
        };
        assert!(s.notify("hi").contains("123"));
    }

    #[test]
    fn test_slack() {
        let sl = SlackNotifier {
            channel: "dev".into(),
        };
        assert!(sl.notify("hi").contains("#dev"));
    }
}
