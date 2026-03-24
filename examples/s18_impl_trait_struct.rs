// Demonstrates implementing a trait for a struct.
//
// `impl TraitName for StructName { ... }`
//
// Run: cargo run --example s18_impl_trait_struct

/// A trait for greeting.
trait Greetable {
    /// Returns a greeting string.
    fn greet(&self) -> String;
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct Robot {
    id: u32,
}

// Implement Greetable for Person
impl Greetable for Person {
    fn greet(&self) -> String {
        format!("Hi, I'm {} and I'm {} years old.", self.name, self.age)
    }
}

// Implement Greetable for Robot
impl Greetable for Robot {
    fn greet(&self) -> String {
        format!("BEEP BOOP. Unit #{} online.", self.id)
    }
}

fn main() {
    let alice = Person {
        name: "Alice".into(),
        age: 30,
    };
    let bot = Robot { id: 42 };

    // Same method name, different implementations:
    println!("{}", alice.greet());
    println!("{}", bot.greet());

    // Both satisfy the trait contract.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_greet() {
        let p = Person {
            name: "Bob".into(),
            age: 25,
        };
        assert!(p.greet().contains("Bob"));
    }

    #[test]
    fn test_robot_greet() {
        let r = Robot { id: 7 };
        assert!(r.greet().contains("7"));
    }
}
