// Demonstrates using trait bounds to conditionally implement methods.
//
// You can add methods to a generic type ONLY when the type parameter
// satisfies certain trait bounds.
//
// Run: cargo run --example s18_conditional_methods

use std::fmt::Display;

/// A generic container.
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

// Methods available for ALL Pair<T>:
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

// Methods ONLY available when T: Display:
impl<T: Display> Pair<T> {
    fn show(&self) {
        println!("({}, {})", self.first, self.second);
    }
}

// Methods ONLY available when T: Display + PartialOrd:
impl<T: Display + PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

// Blanket implementation: implement a trait for all types that satisfy bounds.
// The standard library does this: impl<T: Display> ToString for T { ... }
trait Shout {
    fn shout(&self) -> String;
}

impl<T: Display> Shout for T {
    fn shout(&self) -> String {
        format!("{self}!!!")
    }
}

fn main() {
    // new() works for any T:
    let p1 = Pair::new(5, 10);
    let p2 = Pair::new(vec![1], vec![2]); // Vec doesn't implement Display

    // show() only works when T: Display:
    p1.show();
    // p2.show(); // ERROR: Vec<i32> doesn't implement Display
    println!("p2: {p2:?}"); // Debug works though

    // larger() only when T: Display + PartialOrd:
    println!("larger: {}", p1.larger());

    // Blanket impl: anything Display gets shout():
    println!("{}", 42.shout());
    println!("{}", "hello".shout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_any_type() {
        let p = Pair::new(vec![1], vec![2]);
        assert_eq!(p.first.len(), 1);
    }

    #[test]
    fn test_larger() {
        let p = Pair::new(3, 7);
        assert_eq!(*p.larger(), 7);
    }

    #[test]
    fn test_blanket_shout() {
        assert_eq!(42.shout(), "42!!!");
        assert_eq!("hi".shout(), "hi!!!");
    }
}
