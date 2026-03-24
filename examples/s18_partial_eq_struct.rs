// Demonstrates implementing PartialEq for structs.
//
// `PartialEq` enables `==` and `!=` operators.
// Can be derived or implemented manually for custom comparison logic.
//
// Run: cargo run --example s18_partial_eq_struct

/// Derived PartialEq — compares all fields.
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// Manual PartialEq — custom comparison logic.
#[derive(Debug, Clone)]
struct CaseInsensitiveName {
    name: String,
}

impl PartialEq for CaseInsensitiveName {
    fn eq(&self, other: &Self) -> bool {
        // Compare ignoring case:
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

/// Manual PartialEq — compare only some fields.
#[derive(Debug, Clone)]
struct User {
    id: u64,
    #[allow(dead_code)] // stored but not used in comparison
    name: String,
    #[allow(dead_code)] // stored but not used in comparison
    last_login: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        // Two users are equal if they have the same ID:
        self.id == other.id
    }
}

fn main() {
    // --- Derived PartialEq ---
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = Point { x: 3.0, y: 4.0 };
    println!("p1 == p2: {}", p1 == p2); // true
    println!("p1 == p3: {}", p1 == p3); // false
    println!("p1 != p3: {}", p1 != p3); // true

    println!();

    // --- Case-insensitive comparison ---
    let a = CaseInsensitiveName {
        name: "Alice".into(),
    };
    let b = CaseInsensitiveName {
        name: "alice".into(),
    };
    let c = CaseInsensitiveName { name: "BOB".into() };
    println!("Alice == alice: {}", a == b); // true!
    println!("Alice == BOB: {}", a == c); // false

    println!();

    // --- Compare by ID only ---
    let u1 = User {
        id: 1,
        name: "Alice".into(),
        last_login: "2024-01-01".into(),
    };
    let u2 = User {
        id: 1,
        name: "Alice Updated".into(),
        last_login: "2024-06-01".into(),
    };
    println!("Same user? {}", u1 == u2); // true (same id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_eq() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_case_insensitive() {
        let a = CaseInsensitiveName {
            name: "Hello".into(),
        };
        let b = CaseInsensitiveName {
            name: "HELLO".into(),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_user_by_id() {
        let u1 = User {
            id: 42,
            name: "A".into(),
            last_login: "x".into(),
        };
        let u2 = User {
            id: 42,
            name: "B".into(),
            last_login: "y".into(),
        };
        assert_eq!(u1, u2);
    }

    #[test]
    fn test_user_different_id() {
        let u1 = User {
            id: 1,
            name: "A".into(),
            last_login: "x".into(),
        };
        let u2 = User {
            id: 2,
            name: "A".into(),
            last_login: "x".into(),
        };
        assert_ne!(u1, u2);
    }
}
