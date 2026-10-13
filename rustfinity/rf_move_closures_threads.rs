//! # Rustfinity: Move Closures Threads
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/move-closures-threads
//!
//! `thread::spawn` accepts any closure that is `Send + 'static`. The `'static` part is the strict one: the spawned thread might still be running long after the function that created it has returned, so the closure is not allowed to borrow anything from that function's stack.
//!
//! The `move` keyword is how you satisfy that. It changes capture mode: instead of borrowing the variables it mentions, the closure takes ownership of them. The data now lives inside the closure, which lives inside the thread, so there is no dangling reference to worry about.
//!
//! Ownership being ownership, only one thread can have it. If two threads each need the same `String`, one of them needs its own copy. That is not a workaround, it is the rule being consistent: `move` really does hand the value over.
//!
//! ## Your Task
//!
//! Implement two functions.
//!
//! ### Implement `count_chars_in_thread`
//!
//! ```rust
//! pub fn count_chars_in_thread(
//!     words: Vec<String>,
//! ) -> usize
//! ```
//!
//! Move `words` into a spawned thread, count the characters across all of them, join, and return the total.
//!
//! ```rust
//! let words = vec![
//!     "ab".to_string(),
//!     "cde".to_string(),
//! ];
//! assert_eq!(count_chars_in_thread(words), 5);
//! ```
//!
//! ### Implement `greet_each`
//!
//! ```rust
//! pub fn greet_each(
//!     prefix: String,
//!     names: Vec<String>,
//! ) -> Vec<String>
//! ```
//!
//! Spawn one thread per name. Each thread builds `"{prefix}, {name}!"`. Return the greetings **in the same order as `names`**, whatever order the threads happen to finish in.
//!
//! ```rust
//! let names = vec![
//!     "Ada".to_string(),
//!     "Bo".to_string(),
//! ];
//! let out = greet_each("Hi".to_string(), names);
//! assert_eq!(out, vec!["Hi, Ada!", "Hi, Bo!"]);
//! ```
//!
//! ### Notes
//!
//! - Count characters, not bytes, so `"héllo"` is 5.
//! - Order is part of the contract. Spawning all the threads first and then joining them in the same order is enough to guarantee it, and it also keeps the threads running concurrently.
//!
//! ## Hints
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - `w.chars().count()` counts characters; `w.len()` counts bytes.
//! - Only one thread can own `prefix`, so give each closure its own `prefix.clone()` before the `move`.
//! - Collect into a `Vec` of handles first. If you join inside the same loop that spawns, each thread finishes before the next one starts and you have written a slow sequential loop.
//! - `handles.into_iter().map(|h| h.join().unwrap()).collect()` turns the handles back into results, in order.
//!
//! </details>
//!
//! Run: `cargo run --example rf_move_closures_threads`
//! Test: `cargo test --example rf_move_closures_threads`
//! Filter: `cargo test --example rf_move_closures_threads <test-name>`

#![allow(unused)]

#![allow(unused_variables)]

use std::thread;

/// Counts the characters in every word, on a separate thread.
pub fn count_chars_in_thread(words: Vec<String>) -> usize {
    // TODO: move `words` into a spawned thread, count, and join
    unimplemented!()
}

/// Greets every name on its own thread, preserving input order.
pub fn greet_each(prefix: String, names: Vec<String>) -> Vec<String> {
    // TODO: spawn one thread per name and join them in spawn order
    unimplemented!()
}

// Example usage
pub fn main() {
    let words = vec!["hello".to_string(), "world".to_string()];
    println!("chars: {}", count_chars_in_thread(words));

    let names = vec!["Ada".to_string(), "Bo".to_string()];
    println!("{:?}", greet_each("Hi".to_string(), names));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    mod count_chars_tests {
        use super::*;

        #[test]
        fn counts_two_words() {
            assert_eq!(count_chars_in_thread(strings(&["ab", "cde"])), 5);
        }

        #[test]
        fn empty_input_is_zero() {
            assert_eq!(count_chars_in_thread(vec![]), 0);
        }

        #[test]
        fn empty_strings_count_as_zero() {
            assert_eq!(count_chars_in_thread(strings(&["", ""])), 0);
        }

        #[test]
        fn counts_characters_not_bytes() {
            assert_eq!(count_chars_in_thread(strings(&["héllo"])), 5);
        }

        #[test]
        fn counts_many_words() {
            let words = strings(&["a", "bb", "ccc", "dddd", "eeeee"]);
            assert_eq!(count_chars_in_thread(words), 15);
        }
    }

    mod greet_each_tests {
        use super::*;

        #[test]
        fn greets_two_names() {
            let got = greet_each("Hi".to_string(), strings(&["Ada", "Bo"]));
            assert_eq!(got, vec!["Hi, Ada!", "Hi, Bo!"]);
        }

        #[test]
        fn no_names_gives_no_greetings() {
            let got = greet_each("Hi".to_string(), vec![]);
            assert!(got.is_empty());
        }

        #[test]
        fn preserves_input_order() {
            let names = strings(&["d", "c", "b", "a"]);
            let got = greet_each("Yo".to_string(), names);
            assert_eq!(got, vec!["Yo, d!", "Yo, c!", "Yo, b!", "Yo, a!"]);
        }

        #[test]
        fn every_thread_sees_the_same_prefix() {
            let names = strings(&["one", "two", "three", "four", "five"]);
            let got = greet_each("Hello".to_string(), names);

            assert_eq!(got.len(), 5);
            assert!(got.iter().all(|g| g.starts_with("Hello, ")));
        }

        #[test]
        fn duplicate_names_are_all_kept() {
            let got = greet_each("Hi".to_string(), strings(&["Ada", "Ada"]));
            assert_eq!(got, vec!["Hi, Ada!", "Hi, Ada!"]);
        }

        #[test]
        fn works_with_an_empty_prefix() {
            let got = greet_each(String::new(), strings(&["Ada"]));
            assert_eq!(got, vec![", Ada!"]);
        }
    }
}
