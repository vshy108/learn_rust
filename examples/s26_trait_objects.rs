// Demonstrates trait objects for handling multiple error types.
//
// `Box<dyn Trait>` enables dynamic dispatch — you can store different
// types behind the same interface. This is especially useful for
// error handling where multiple error types must be returned.
//
// Run: cargo run --example s26_trait_objects

use std::fmt;

// --- Define a trait ---
trait Describable: fmt::Display {
    /// Returns a description.
    fn describe(&self) -> String;
}

#[derive(Debug)]
struct Dog {
    name: String,
}

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

impl Describable for Dog {
    fn describe(&self) -> String {
        format!("{} is a good dog", self.name)
    }
}

#[derive(Debug)]
struct Cat {
    name: String,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cat({})", self.name)
    }
}

impl Describable for Cat {
    fn describe(&self) -> String {
        format!("{} is an independent cat", self.name)
    }
}

/// Accepts any Describable via trait object.
fn print_description(item: &dyn Describable) {
    println!("  {item}: {}", item.describe());
}

/// Returns different error types using Box<dyn Error>.
fn parse_and_double(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let n: i32 = input.parse()?; // ParseIntError
    if n < 0 {
        // Custom error message
        return Err("negative numbers not allowed".into());
    }
    Ok(n * 2)
}

fn main() {
    println!("=== Trait Objects ===");

    // --- 1. Heterogeneous collection ---
    let animals: Vec<Box<dyn Describable>> = vec![
        Box::new(Dog {
            name: "Rex".to_string(),
        }),
        Box::new(Cat {
            name: "Whiskers".to_string(),
        }),
        Box::new(Dog {
            name: "Buddy".to_string(),
        }),
    ];
    for a in &animals {
        print_description(a.as_ref());
    }

    // --- 2. Error handling with Box<dyn Error> ---
    println!("--- error handling ---");
    let cases = ["42", "-1", "abc"];
    for case in &cases {
        match parse_and_double(case) {
            Ok(val) => println!("  '{case}' -> {val}"),
            Err(e) => println!("  '{case}' -> error: {e}"),
        }
    }

    // --- 3. Size of trait objects ---
    println!(
        "Box<dyn Describable> size: {} bytes",
        std::mem::size_of::<Box<dyn Describable>>()
    );
    // Two pointers: data ptr + vtable ptr (16 bytes on 64-bit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_object() {
        let dog: Box<dyn Describable> = Box::new(Dog {
            name: "Rex".to_string(),
        });
        assert!(dog.describe().contains("good dog"));
    }

    #[test]
    fn test_parse_ok() {
        assert_eq!(parse_and_double("21").unwrap(), 42);
    }

    #[test]
    fn test_parse_error() {
        assert!(parse_and_double("abc").is_err());
        assert!(parse_and_double("-1").is_err());
    }
}
