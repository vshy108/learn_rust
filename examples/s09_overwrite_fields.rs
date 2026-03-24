// Demonstrates overwriting struct fields.
//
// To mutate a field, the entire struct binding must be `mut`.
// Rust doesn't support field-level mutability.
//
// Run: cargo run --example s09_overwrite_fields

#[derive(Debug)]
struct Counter {
    name: String,
    count: u64,
    active: bool,
}

fn main() {
    // --- 1. Basic field mutation ---
    let mut c = Counter {
        name: String::from("visitors"),
        count: 0,
        active: true,
    };
    println!("Before: {c:?}");
    c.count += 1;
    c.count += 1;
    println!("After incrementing: {c:?}");

    // --- 2. Replacing a String field ---
    c.name = String::from("page_views");
    println!("Renamed: {c:?}");

    // --- 3. Toggle boolean ---
    c.active = !c.active;
    println!("Toggled: {c:?}");

    // --- 4. Replace entire struct ---
    c = Counter {
        name: String::from("new_counter"),
        count: 100,
        active: true,
    };
    println!("Replaced: {c:?}");

    // --- 5. Mutation through a function ---
    increment(&mut c);
    increment(&mut c);
    println!("After fn increment: {c:?}");
}

/// Increments the counter's count by 1.
///
/// # Arguments
///
/// * `counter` - Mutable reference to a Counter
fn increment(counter: &mut Counter) {
    counter.count += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut c = Counter {
            name: String::from("test"),
            count: 0,
            active: true,
        };
        increment(&mut c);
        assert_eq!(c.count, 1);
        increment(&mut c);
        assert_eq!(c.count, 2);
    }

    #[test]
    fn test_field_overwrite() {
        let mut c = Counter {
            name: String::from("old"),
            count: 5,
            active: true,
        };
        c.name = String::from("new");
        c.count = 0;
        assert_eq!(c.name, "new");
        assert_eq!(c.count, 0);
    }
}
