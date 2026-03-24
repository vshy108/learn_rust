// Demonstrates supertraits (trait inheritance).
//
// A trait can require another trait: `trait Child: Parent`.
// This means any type implementing Child MUST also implement Parent.
//
// Run: cargo run --example s18_supertraits

use std::fmt;

/// Base trait — all printable things.
trait Printable: fmt::Display {
    fn print(&self) {
        // Can use {} because Self: Display
        println!("{self}");
    }
}

/// Supertrait chain: Loggable requires Printable requires Display.
trait Loggable: Printable {
    fn log(&self) {
        println!("[LOG] {self}");
    }
}

#[derive(Debug)]
struct Event {
    name: String,
    severity: u8,
}

// Must implement Display (required by Printable):
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event({}, sev={})", self.name, self.severity)
    }
}

// Must implement Printable (required by Loggable):
impl Printable for Event {}

// Now can implement Loggable:
impl Loggable for Event {}

/// Accepts anything Loggable (which implies Printable + Display).
fn process(item: &impl Loggable) {
    item.print();
    item.log();
}

/// Multiple supertraits:
trait Describable: fmt::Display + fmt::Debug {
    fn describe(&self) -> String {
        format!("Display: {self}, Debug: {self:?}")
    }
}

impl Describable for Event {}

fn main() {
    let e = Event {
        name: "deploy".into(),
        severity: 3,
    };

    // All three trait methods are available:
    e.print();
    e.log();
    process(&e);

    println!();
    println!("Describe: {}", e.describe());

    println!();
    println!("Supertrait chain:");
    println!("  Loggable: Printable -> requires Printable");
    println!("  Printable: Display -> requires Display");
    println!("  So Loggable requires both Printable AND Display");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let e = Event {
            name: "test".into(),
            severity: 1,
        };
        assert!(e.to_string().contains("test"));
    }

    #[test]
    fn test_describe() {
        let e = Event {
            name: "d".into(),
            severity: 0,
        };
        let desc = e.describe();
        assert!(desc.contains("Display:"));
        assert!(desc.contains("Debug:"));
    }
}
