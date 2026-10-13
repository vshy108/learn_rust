//! # Rustfinity: If Let Enums
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/if-let-enums
//!
//! Sometimes, you only need to handle a specific case of an enum variant, in this case using a `match` statement can be overkill. Rust provides a more concise way to handle such cases using the `if let` construct. This allows you to match a single variant of an enum and extract its value in a single line.
//!
//! Here's an example:
//!
//! ```rust
//! enum Ip {
//!     V4(String),
//!     V6(String),
//! }
//!
//! if let Ip::V4(ip) = &ip {
//!     println!("IPv4 address: {}", ip);
//! }
//! ```
//!
//! ---
//!
//! ## Your Task
//!
//! You are provided an enum called `Message` with the following variants:
//!
//! - `Text(String)`: Represents a textual message.
//! - `Number(i32)`: Represents a numerical message.
//! - `Quit`: Represents a command to quit.
//! - `None`: Represents no message.
//!
//! Your task is to implement the function `process_text_message` that takes a reference to a `Message` enum. For the `Text` variant, the function should return `"Processed Text: <content>"`, replacing `<content>` with the actual string.
//!
//! If the input is any other variant of the enum, the function should return `"Unhandled Message"`.
//!
//! Try to solve this using the `if let` construct.
//!
//! ---
//!
//! ## Hints
//!
//! <details>
//! <summary>Click here to reveal hints</summary>
//!
//! - You can use the `if let` construct like this:
//!   ```rust
//!   if let EnumVariant(value) = &enum_instance {
//!       // Perform actions with `value`.
//!   }
//!   ```
//! - Ensure you return `"Unhandled Message"` for all other variants not explicitly matched.
//!
//! </details>
//!
//! Run: `cargo run --example rf_if_let_enums`
//! Test: `cargo test --example rf_if_let_enums`
//! Filter: `cargo test --example rf_if_let_enums <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

pub enum Message {
    Text(String),
    Number(i32),
    Quit,
    None,
}

pub fn process_text_message(message: &Message) -> String {
    // Your code here...

    String::from("Unhandled Message")
}

pub fn main() {
    assert_eq!(
        process_text_message(&Message::Text(String::from("Hello"))),
        "Processed Text: Hello"
    );
    assert_eq!(
        process_text_message(&Message::Number(42)),
        "Unhandled Message"
    );
    assert_eq!(process_text_message(&Message::Quit), "Unhandled Message");
    assert_eq!(process_text_message(&Message::None), "Unhandled Message");
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_process_text_message_with_text() {
        let result = process_text_message(&Message::Text(String::from("Hello")));
        assert_eq!(result, "Processed Text: Hello");

        let result = process_text_message(&Message::Text(String::from("Rust is great!")));
        assert_eq!(result, "Processed Text: Rust is great!");

        let result = process_text_message(&Message::Text(String::from("Test message with spaces")));
        assert_eq!(result, "Processed Text: Test message with spaces");

        let result =
            process_text_message(&Message::Text(String::from("Special chars: !@#$%^&*()")));
        assert_eq!(result, "Processed Text: Special chars: !@#$%^&*()");
    }

    #[test]
    fn test_process_text_message_with_other_variants() {
        let result = process_text_message(&Message::Number(42));
        assert_eq!(result, "Unhandled Message");

        let result = process_text_message(&Message::Number(-100));
        assert_eq!(result, "Unhandled Message");

        let result = process_text_message(&Message::Quit);
        assert_eq!(result, "Unhandled Message");

        let result = process_text_message(&Message::None);
        assert_eq!(result, "Unhandled Message");
    }

    #[test]
    fn test_process_text_message_edge_cases() {
        let result = process_text_message(&Message::Text(String::from("")));
        assert_eq!(result, "Processed Text: ");

        let result = process_text_message(&Message::Text(String::from(" ")));
        assert_eq!(result, "Processed Text:  ");

        let result = process_text_message(&Message::Text(String::from("\n")));
        assert_eq!(result, "Processed Text: \n");
    }

    #[test]
    fn test_process_text_message_with_mixed_messages() {
        let text1 = Message::Text(String::from("First message"));
        let text2 = Message::Text(String::from("Second message"));
        let messages = vec![
            &text1,
            &Message::Number(123),
            &Message::Quit,
            &text2,
            &Message::None,
        ];

        let expected = vec![
            "Processed Text: First message",
            "Unhandled Message",
            "Unhandled Message",
            "Processed Text: Second message",
            "Unhandled Message",
        ];

        for (msg, exp) in messages.into_iter().zip(expected.into_iter()) {
            assert_eq!(process_text_message(msg), exp);
        }
    }
}
