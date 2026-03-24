// Demonstrates traits' role in project structure.
//
// In a real project, traits live in their own module and types
// implement them across the codebase. This decouples behavior
// from concrete types.
//
// Run: cargo run --example s18_project_structure

/// Represents a storage backend (trait = the interface).
mod storage {
    pub trait Store {
        /// Saves a key-value pair.
        fn save(&mut self, key: &str, value: &str);
        /// Retrieves a value by key.
        fn load(&self, key: &str) -> Option<&str>;
    }
}

/// In-memory implementation.
mod memory_store {
    use super::storage::Store;
    use std::collections::HashMap;

    pub struct MemoryStore {
        data: HashMap<String, String>,
    }

    impl MemoryStore {
        pub fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }
    }

    impl Store for MemoryStore {
        fn save(&mut self, key: &str, value: &str) {
            self.data.insert(key.to_string(), value.to_string());
        }
        fn load(&self, key: &str) -> Option<&str> {
            self.data.get(key).map(|s| s.as_str())
        }
    }
}

/// Application logic only depends on the trait, not a specific type.
mod app {
    use super::storage::Store;

    pub fn run(store: &mut dyn Store) {
        store.save("name", "Rust");
        store.save("version", "2024");

        println!("name: {:?}", store.load("name"));
        println!("version: {:?}", store.load("version"));
        println!("missing: {:?}", store.load("missing"));
    }
}

fn main() {
    println!("=== Traits in Project Structure ===");
    let mut store = memory_store::MemoryStore::new();
    app::run(&mut store);
}

#[cfg(test)]
mod tests {
    use super::memory_store::MemoryStore;
    use super::storage::Store;

    #[test]
    fn test_memory_store() {
        let mut s = MemoryStore::new();
        s.save("k", "v");
        assert_eq!(s.load("k"), Some("v"));
        assert_eq!(s.load("x"), None);
    }
}
