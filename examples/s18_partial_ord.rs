// Demonstrates implementing PartialOrd.
//
// `PartialOrd` enables `<`, `>`, `<=`, `>=` operators and `.partial_cmp()`.
// Like `PartialEq`, it's "partial" because not all values may be comparable
// (e.g., floating-point NaN).
//
// `Ord` (full ordering) additionally requires that ALL values are comparable.
//
// Run: cargo run --example s18_partial_ord

use std::cmp::Ordering;

/// Derived PartialOrd — lexicographic comparison of fields (in declaration order).
#[derive(Debug, PartialEq, PartialOrd)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

/// Manual PartialOrd — compare by a specific field.
#[derive(Debug, PartialEq)]
struct Student {
    name: String,
    gpa: f64,
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare by GPA (higher is "greater"):
        self.gpa.partial_cmp(&other.gpa)
    }
}

/// Full Ord (requires Eq + PartialOrd):
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u8);

fn main() {
    // --- Derived: lexicographic ---
    let v1 = Version {
        major: 1,
        minor: 9,
        patch: 0,
    };
    let v2 = Version {
        major: 2,
        minor: 0,
        patch: 0,
    };
    let v3 = Version {
        major: 1,
        minor: 9,
        patch: 1,
    };
    println!("{v1:?} < {v2:?}: {}", v1 < v2);
    println!("{v1:?} < {v3:?}: {}", v1 < v3);

    println!();

    // --- Manual: by GPA ---
    let alice = Student {
        name: "Alice".into(),
        gpa: 3.8,
    };
    let bob = Student {
        name: "Bob".into(),
        gpa: 3.5,
    };
    println!(
        "{} vs {}: {:?}",
        alice.name,
        bob.name,
        alice.partial_cmp(&bob)
    );
    println!("Alice > Bob: {}", alice > bob);

    // --- Sorting with Ord ---
    let mut priorities = vec![Priority(3), Priority(1), Priority(2)];
    priorities.sort(); // Ord enables sort()
    println!("Sorted: {priorities:?}");

    // --- PartialOrd only: sort with sort_by ---
    let mut students = vec![
        Student {
            name: "C".into(),
            gpa: 3.0,
        },
        Student {
            name: "A".into(),
            gpa: 3.9,
        },
        Student {
            name: "B".into(),
            gpa: 3.5,
        },
    ];
    students.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    for s in &students {
        println!("  {} (GPA: {:.1})", s.name, s.gpa);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_ordering() {
        let v1 = Version {
            major: 1,
            minor: 0,
            patch: 0,
        };
        let v2 = Version {
            major: 1,
            minor: 0,
            patch: 1,
        };
        assert!(v1 < v2);
    }

    #[test]
    fn test_student_ordering() {
        let a = Student {
            name: "A".into(),
            gpa: 4.0,
        };
        let b = Student {
            name: "B".into(),
            gpa: 3.0,
        };
        assert!(a > b);
    }

    #[test]
    fn test_priority_sort() {
        let mut p = vec![Priority(3), Priority(1), Priority(2)];
        p.sort();
        assert_eq!(p, vec![Priority(1), Priority(2), Priority(3)]);
    }
}
