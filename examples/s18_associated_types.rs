// Demonstrates associated types in traits.
//
// Associated types are like generics but defined by the implementor.
// Instead of `trait Foo<T>`, you write `trait Foo { type T; }`.
//
// Key difference:
// - Generic trait: can implement multiple times with different T
// - Associated type: implement exactly ONCE per type
//
// Run: cargo run --example s18_associated_types

/// A trait with an associated type.
/// Each type that implements Iterator has ONE specific Item type.
trait MyIterator {
    /// The type of elements this iterator yields.
    type Item;

    /// Returns the next item, or None if exhausted.
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Self { current: 0, max }
    }
}

impl MyIterator for Counter {
    type Item = u32; // The associated type is u32

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

/// A converter trait using associated types.
trait Converter {
    type Input;
    type Output;
    type Error;

    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

struct StringToInt;

impl Converter for StringToInt {
    type Input = String;
    type Output = i64;
    type Error = std::num::ParseIntError;

    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.parse::<i64>()
    }
}

/// Using associated types in function bounds:
fn sum_all<I>(iter: &mut I) -> u64
where
    I: MyIterator<Item = u32>,
{
    let mut total = 0u64;
    while let Some(val) = iter.next() {
        total += u64::from(val);
    }
    total
}

fn main() {
    // --- Counter iterator ---
    let mut counter = Counter::new(5);
    while let Some(n) = counter.next() {
        print!("{n} ");
    }
    println!();

    // --- Sum with associated type bound ---
    let mut c = Counter::new(10);
    println!("Sum 1..10: {}", sum_all(&mut c));

    // --- Converter ---
    let conv = StringToInt;
    match conv.convert("42".into()) {
        Ok(n) => println!("Converted: {n}"),
        Err(e) => println!("Error: {e}"),
    }
    match conv.convert("abc".into()) {
        Ok(n) => println!("Converted: {n}"),
        Err(e) => println!("Error: {e}"),
    }

    println!();
    println!("Associated type: ONE impl per type (like Iterator::Item)");
    println!("Generic trait:   MANY impls per type (like From<T>)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut c = Counter::new(3);
        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(3));
        assert_eq!(c.next(), None);
    }

    #[test]
    fn test_sum_all() {
        let mut c = Counter::new(5);
        assert_eq!(sum_all(&mut c), 15);
    }

    #[test]
    fn test_converter_ok() {
        let conv = StringToInt;
        assert_eq!(conv.convert("123".into()), Ok(123));
    }

    #[test]
    fn test_converter_err() {
        let conv = StringToInt;
        assert!(conv.convert("xyz".into()).is_err());
    }
}
