// Demonstrates implementing the Clone trait.
//
// `Clone` provides `.clone()` for explicit deep copies.
// Can be derived or implemented manually for custom behavior.
//
// Gotcha: Clone can be expensive — it allocates new heap memory.
// Always call `.clone()` explicitly; Rust never clones implicitly.
//
// Run: cargo run --example s18_clone_trait

/// Derived Clone — the common case.
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// Manual Clone — custom behavior.
#[derive(Debug, PartialEq)]
struct UniqueId {
    id: u64,
    label: String,
}

// Static counter for generating unique IDs:
// (In real code, use AtomicU64.)
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl Clone for UniqueId {
    fn clone(&self) -> Self {
        // Clone gets a NEW id, unlike a bitwise copy:
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            label: format!("{} (clone)", self.label),
        }
    }
}

/// Clone for types with heap data.
#[derive(Debug, Clone, PartialEq)]
struct Config {
    name: String,     // String is on the heap
    values: Vec<i32>, // Vec is on the heap
    enabled: bool,    // bool is Copy (cheap)
}

fn main() {
    // --- Derived Clone ---
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone(); // explicit deep copy
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
    assert_eq!(p1, p2);

    println!();

    // --- Manual Clone ---
    let uid1 = UniqueId {
        id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        label: "original".into(),
    };
    let uid2 = uid1.clone();
    println!("uid1: {uid1:?}");
    println!("uid2: {uid2:?}");
    // Different IDs!
    assert_ne!(uid1.id, uid2.id);

    println!();

    // --- Heap data Clone ---
    let c1 = Config {
        name: "prod".into(),
        values: vec![1, 2, 3],
        enabled: true,
    };
    let mut c2 = c1.clone();
    c2.values.push(4);
    // c1 is unaffected — deep copy:
    println!("c1.values: {:?}", c1.values);
    println!("c2.values: {:?}", c2.values);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_clone() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(p, p.clone());
    }

    #[test]
    fn test_manual_clone_different_id() {
        let a = UniqueId {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            label: "test".into(),
        };
        let b = a.clone();
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn test_deep_copy() {
        let c1 = Config {
            name: "a".into(),
            values: vec![1],
            enabled: true,
        };
        let mut c2 = c1.clone();
        c2.values.push(2);
        assert_eq!(c1.values.len(), 1);
        assert_eq!(c2.values.len(), 2);
    }
}
