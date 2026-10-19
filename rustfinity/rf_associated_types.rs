//! # Rustfinity: Associated Types
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/associated-types
//!
//! Associated types in Rust allow you to define traits with a type placeholder that implementors of the trait can specify. This enables more flexible and type-safe abstractions.
//!
//! For example, instead of passing types as generic parameters every time, you can use associated types to simplify the interface. Associated types are particularly useful when working with collections, iterators, or data processing pipelines.
//!
//! In this challenge, you'll define a trait with an associated type and implement it for a struct. The goal is to create a structure for managing a simple Key-Value store where keys and values can have different types specified by the trait implementation.
//!
//! You can solve the problem using either generics or associated types, but the challenge is made to make you familiar with associated types, so try to solve the challenge by using them instead of generics.
//!
//! ## Your Task
//!
//! Define a trait `KeyValueStore` with an associated type `Key` and `Value`. Implement this trait for the struct `InMemoryStore`. The implementation should allow storing and retrieving key-value pairs of the specified types.
//!
//! ### Requirements
//!
//! 1. Define a trait `KeyValueStore` with:
//!    - An associated type `Key`.
//!    - An associated type `Value`.
//!    - Methods:
//!      - `set` to add a key-value pair.
//!      - `get` takes a reference of `&Key` and returns `Option<&Value>`.
//!
//! 2. Create a struct `InMemoryStore` that uses a `HashMap` to store key-value pairs. Implement the `KeyValueStore` trait for this struct.
//! 3. Make sure all relevant values are public so that they can be accessed from outside the module (essential to pass the tests).
//!
//! ## Hints
//!
//! If you're stuck, here are some hints to help you solve the challenge:
//!
//! <details>
//!     <summary>Click here to reveal hints</summary>
//!
//! - Define associated types in the `KeyValueStore` trait like this:
//!
//!   ```rust
//!   pub trait KeyValueStore {
//!       type Key;
//!       type Value;
//!
//!       fn set(&mut self, key: Self::Key, value: Self::Value);
//!       fn get(&self, key: &Self::Key)
//!           -> Option<&Self::Value>;
//!   }
//!   ```
//!
//! </details>
//!
//! Run: `cargo run --example rf_associated_types`
//! Test: `cargo test --example rf_associated_types`
//! Filter: `cargo test --example rf_associated_types <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::hash::Hash;

pub trait KeyValueStore {
    type Key;
    type Value;

    fn set(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

pub struct InMemoryStore<K, V> {
    pub storage: HashMap<K, V>,
}

impl<K, V> KeyValueStore for InMemoryStore<K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;

    fn set(&mut self, key: K, value: V) {
        todo!("store a key-value pair")
    }

    fn get(&self, key: &K) -> Option<&V> {
        todo!("look up a key")
    }
}

pub fn main() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };
    store.set("name".to_string(), "Rust".to_string());
    assert_eq!(store.get(&"name".to_string()), Some(&"Rust".to_string()));
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    use std::collections::HashMap;

    #[test]
    fn test_set_and_get() {
        let mut store: InMemoryStore<String, String> = InMemoryStore {
            storage: HashMap::new(),
        };

        store.set("key1".to_string(), "value1".to_string());
        assert_eq!(store.get(&"key1".to_string()), Some(&"value1".to_string()));

        store.set("key2".to_string(), "value2".to_string());
        assert_eq!(store.get(&"key2".to_string()), Some(&"value2".to_string()));
    }

    #[test]
    fn test_non_existent_key() {
        let store: InMemoryStore<String, String> = InMemoryStore {
            storage: HashMap::new(),
        };

        assert_eq!(store.get(&"missing".to_string()), None);
    }

    #[test]
    fn test_overwrite_value() {
        let mut store: InMemoryStore<String, String> = InMemoryStore {
            storage: HashMap::new(),
        };

        store.set("key".to_string(), "value1".to_string());
        store.set("key".to_string(), "value2".to_string());
        assert_eq!(store.get(&"key".to_string()), Some(&"value2".to_string()));
    }

    #[test]
    fn test_different_key_value_types() {
        let mut store: InMemoryStore<u32, f64> = InMemoryStore {
            storage: HashMap::new(),
        };

        store.set(1, 10.5);
        store.set(2, 20.0);

        assert_eq!(store.get(&1), Some(&10.5));
        assert_eq!(store.get(&2), Some(&20.0));
        assert_eq!(store.get(&3), None);
    }

    #[test]
    fn test_large_store() {
        let mut store: InMemoryStore<u64, String> = InMemoryStore {
            storage: HashMap::new(),
        };

        for i in 0..1000 {
            store.set(i, format!("value{}", i));
        }

        for i in 0..1000 {
            assert_eq!(store.get(&i), Some(&format!("value{}", i)));
        }

        assert_eq!(store.get(&1001), None);
    }

    #[test]
    fn test_empty_store() {
        let store: InMemoryStore<String, String> = InMemoryStore {
            storage: HashMap::new(),
        };

        assert_eq!(store.get(&"anything".to_string()), None);
    }

    #[test]
    fn test_store_with_complex_key() {
        #[derive(Eq, PartialEq, Hash, Debug, Clone)]
        struct ComplexKey {
            id: u32,
            name: String,
        }

        let mut store: InMemoryStore<ComplexKey, String> = InMemoryStore {
            storage: HashMap::new(),
        };

        let key1 = ComplexKey {
            id: 1,
            name: "key1".to_string(),
        };
        let key2 = ComplexKey {
            id: 2,
            name: "key2".to_string(),
        };

        store.set(key1.clone(), "value1".to_string());
        store.set(key2.clone(), "value2".to_string());

        assert_eq!(store.get(&key1), Some(&"value1".to_string()));
        assert_eq!(store.get(&key2), Some(&"value2".to_string()));
    }

    #[test]
    fn test_store_remove_key_behavior() {
        let mut store: InMemoryStore<String, String> = InMemoryStore {
            storage: HashMap::new(),
        };

        store.set("key".to_string(), "value".to_string());
        assert_eq!(store.get(&"key".to_string()), Some(&"value".to_string()));

        store.storage.remove(&"key".to_string());
        assert_eq!(store.get(&"key".to_string()), None);
    }
}
