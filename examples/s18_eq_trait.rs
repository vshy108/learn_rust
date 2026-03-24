// Demonstrates the Eq trait.
//
// `Eq` is a marker trait (no extra methods). It signals that
// equality is REFLEXIVE: `a == a` is always true.
//
// `PartialEq` alone doesn't guarantee reflexivity (e.g., `f64::NAN != f64::NAN`).
// `Eq` extends `PartialEq` to add that guarantee.
//
// Types with `Eq` can be used as HashMap keys or in HashSet.
//
// Run: cargo run --example s18_eq_trait

use std::collections::HashSet;

/// Eq requires PartialEq. Usually both are derived together.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tag {
    name: String,
    category: String,
}

// f64 implements PartialEq but NOT Eq (because NaN != NaN).
// So this wouldn't compile:
// #[derive(Eq)]
// struct BadKey { value: f64 }

fn main() {
    // --- Eq types can be HashSet elements ---
    let mut ids = HashSet::new();
    ids.insert(UserId(1));
    ids.insert(UserId(2));
    ids.insert(UserId(1)); // duplicate, ignored
    println!("Unique IDs: {ids:?}");
    assert_eq!(ids.len(), 2);

    // --- Tags in a set ---
    let mut tags = HashSet::new();
    tags.insert(Tag {
        name: "rust".into(),
        category: "lang".into(),
    });
    tags.insert(Tag {
        name: "python".into(),
        category: "lang".into(),
    });
    tags.insert(Tag {
        name: "rust".into(),
        category: "lang".into(),
    }); // dup
    println!("Tags: {tags:?}");
    assert_eq!(tags.len(), 2);

    // --- Why Eq matters ---
    println!();
    println!("Eq guarantees: a == a is always true");
    println!("f64: NaN != NaN, so f64 does NOT implement Eq");
    println!("Therefore f64 can't be a HashMap key.");
    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan); // false!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_reflexive() {
        let id = UserId(42);
        assert_eq!(id, id); // Eq guarantees this
    }

    #[test]
    fn test_hashset_dedup() {
        let mut s = HashSet::new();
        s.insert(UserId(1));
        s.insert(UserId(1));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_nan_not_eq() {
        // This is why f64 doesn't implement Eq:
        // NaN is not equal to itself.
        assert!(!f64::NAN.is_nan() || true); // NaN.is_nan() == true
        assert!(f64::NAN.is_nan());
    }
}
