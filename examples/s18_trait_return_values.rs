// Demonstrates returning traits from functions.
//
// `fn foo() -> impl Trait` returns some concrete type that
// implements the trait, but the caller doesn't know which type.
//
// Limitation: can only return ONE concrete type per function.
//
// Run: cargo run --example s18_trait_return_values

trait Animal {
    fn speak(&self) -> &str;
    fn name(&self) -> &str;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> &str {
        "Woof!"
    }
    fn name(&self) -> &str {
        "Dog"
    }
}

impl Animal for Cat {
    fn speak(&self) -> &str {
        "Meow!"
    }
    fn name(&self) -> &str {
        "Cat"
    }
}

/// Returns impl Animal — caller knows it's Animal but not which type.
fn adopt_dog() -> impl Animal {
    Dog
}

fn adopt_cat() -> impl Animal {
    Cat
}

// THIS DOES NOT WORK:
// fn adopt(kind: &str) -> impl Animal {
//     if kind == "dog" { Dog } else { Cat }
//     // ERROR: `if` and `else` have incompatible types
// }
// For that, use trait objects (Box<dyn Animal>):

/// Returns different types — needs Box<dyn Animal>.
fn adopt(kind: &str) -> Box<dyn Animal> {
    match kind {
        "dog" => Box::new(Dog),
        _ => Box::new(Cat),
    }
}

fn main() {
    // --- 1. impl Trait return ---
    let d = adopt_dog();
    let c = adopt_cat();
    println!("{}: {}", d.name(), d.speak());
    println!("{}: {}", c.name(), c.speak());

    // --- 2. Box<dyn Trait> for dynamic dispatch ---
    let pet = adopt("dog");
    println!("Adopted: {} says {}", pet.name(), pet.speak());

    let pets: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Dog)];
    for pet in &pets {
        println!("  {}: {}", pet.name(), pet.speak());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adopt_dog() {
        let d = adopt_dog();
        assert_eq!(d.speak(), "Woof!");
    }

    #[test]
    fn test_adopt_dynamic() {
        let pet = adopt("cat");
        assert_eq!(pet.speak(), "Meow!");
    }
}
