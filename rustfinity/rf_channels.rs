//! # Rustfinity: Channels
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/channels
//!
//! Concurrency is an essential concept in modern programming. This challenge focuses on building a message processing system using Rust's channels and threads. You'll create a system where multiple producers send messages with different priority levels, and a single consumer processes and formats these messages.
//!
//! ## Your Task
//!
//! Implement three key functions that work with a message processing system:
//!
//! 1. `create_message_channel()` - Creates a channel for sending Message structs
//! 2. `create_producer_thread()` - Creates a thread that analyzes and forwards messages
//! 3. `create_consumer_thread()` - Creates a thread that formats and collects messages
//!
//! ### Requirements
//!
//! #### Producer Thread
//!
//! - Receives a vector of messages and a sender channel
//! - Updates priority based on content keywords:
//!   - "ERROR" → Critical
//!   - "WARNING" → High
//!   - "DEBUG" → Medium
//!   - Others become Low
//! - Sends each updated message through the channel
//!
//! #### Consumer Thread
//!
//! - Receives messages until the channel is closed
//! - Formats each message as: `[PRIORITY|SENDER_ID] CONTENT`
//!   where PRIORITY is one of: LOW, MED, HIGH, CRIT
//! - Returns a vector of formatted message strings
//!
//! ### Notes
//!
//! - Ignore error handling, you can simply `unwrap()`
//! - Have a look at the `main` function to see how the functions are used.
//!
//! ## Hints
//!
//! Here are some tips to help you get started:
//!
//! <details>
//!     <summary>Click here for implementation hints</summary>
//!
//! - Use `mpsc::channel()` to create the message channel
//! - Use `thread::spawn` and move closures
//! - Use `while let Ok(msg) = rx.recv()` for receiving
//! - Format with `format!("[{}|{}] {}", priority, id, content)`
//!
//! </details>
//!
//! Run: `cargo run --example rf_channels`
//! Test: `cargo test --example rf_channels`
//! Filter: `cargo test --example rf_channels <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    // 1. Implement this function to create and return a message channel
    unimplemented!()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    // TODO: Create a thread that:
    // - Updates the priority based on content
    // - Sends the updated message through the channel
    unimplemented!()
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    // TODO: Create a thread that:
    // - Receives messages from the channel
    // - Formats them as "[PRIORITY|SENDER_ID] CONTENT"
    // - Returns a vector of formatted messages
    unimplemented!()
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    #[test]
    fn test_single_producer() {
        let (tx, rx) = create_message_channel();
        let messages = vec![Message {
            content: "Normal message".to_string(),
            sender_id: 0,
            priority: Priority::Low,
        }];

        let handle = create_producer_thread(messages, tx);
        handle.join().unwrap();

        let consumer_handle = create_consumer_thread(rx);
        let results = consumer_handle.join().unwrap();

        assert_eq!(results.len(), 1, "Should process exactly one message");
        assert!(
            results[0].contains("LOW") && results[0].contains("Normal message"),
            "Message should contain priority LOW and the original content"
        );
    }

    #[test]
    fn test_priority_updates() {
        let (tx, rx) = create_message_channel();
        let messages = vec![
            Message {
                content: "Normal message".to_string(),
                sender_id: 1,
                priority: Priority::Low,
            },
            Message {
                content: "WARNING: Test warning".to_string(),
                sender_id: 1,
                priority: Priority::Low,
            },
            Message {
                content: "ERROR: Test error".to_string(),
                sender_id: 1,
                priority: Priority::Low,
            },
        ];

        let handle = create_producer_thread(messages, tx);
        handle.join().unwrap();

        let consumer_handle = create_consumer_thread(rx);
        let results = consumer_handle.join().unwrap();

        assert_eq!(results.len(), 3, "Should process all messages");
        assert!(
            results[0].contains("LOW") && results[0].contains("Normal message"),
            "Normal message should contain LOW priority"
        );
        assert!(
            results[1].contains("HIGH") && results[1].contains("WARNING"),
            "Warning message should contain HIGH priority"
        );
        assert!(
            results[2].contains("CRIT") && results[2].contains("ERROR"),
            "Error message should contain CRITICAL priority"
        );
    }

    #[test]
    fn test_multiple_producers() {
        let (tx, rx) = create_message_channel();

        let mut handles = vec![];
        for id in 0..3 {
            let tx_clone = tx.clone();
            let messages = vec![Message {
                content: format!("Message from {}", id),
                sender_id: id,
                priority: Priority::Low,
            }];
            handles.push(create_producer_thread(messages, tx_clone));
        }
        drop(tx);

        for handle in handles {
            handle.join().unwrap();
        }

        let consumer_handle = create_consumer_thread(rx);
        let results = consumer_handle.join().unwrap();

        assert_eq!(
            results.len(),
            3,
            "Should process one message from each producer"
        );
        assert!(
            results.iter().any(|msg| msg.contains("|0]")),
            "Should have message from producer 0"
        );
        assert!(
            results.iter().any(|msg| msg.contains("|1]")),
            "Should have message from producer 1"
        );
        assert!(
            results.iter().any(|msg| msg.contains("|2]")),
            "Should have message from producer 2"
        );
    }

    #[test]
    fn test_empty_channel() {
        let (tx, rx) = create_message_channel();
        drop(tx);

        let consumer_handle = create_consumer_thread(rx);
        let results = consumer_handle.join().unwrap();
        assert!(
            results.is_empty(),
            "Should return empty vector when no messages are sent"
        );
    }

    #[test]
    fn test_message_format() {
        let (tx, rx) = create_message_channel();
        let messages = vec![Message {
            content: "Test message".to_string(),
            sender_id: 42,
            priority: Priority::Medium, // This priority will be overridden
        }];

        let producer_handle = create_producer_thread(messages, tx);
        producer_handle.join().unwrap();

        let consumer_handle = create_consumer_thread(rx);
        let results = consumer_handle.join().unwrap();

        assert_eq!(results.len(), 1);
        assert!(
            results[0].contains("LOW")  // Changed from MED to LOW since content has no keywords
            && results[0].contains("42")
            && results[0].contains("Test message"),
            "Message should contain correct priority based on content, ID and content"
        );
    }
}
