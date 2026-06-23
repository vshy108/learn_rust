// Demonstrates higher-ranked trait bounds, associated types, and phantom types.

use std::marker::PhantomData;

fn call_with_str<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    f("hello");
}

trait IteratorLike {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: u32,
}

impl IteratorLike for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        Some(self.current)
    }
}

struct Meter;

struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Quantity<Unit> {
    fn new(value: f64) -> Self {
        Self {
            value,
            _unit: PhantomData,
        }
    }
}

fn main() {
    call_with_str(|text| println!("{text}"));

    let mut counter = Counter { current: 0 };
    println!("next = {:?}", counter.next());

    let distance = Quantity::<Meter>::new(12.5);
    println!("distance = {}", distance.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_str_passes_expected_value() {
        let captured = std::cell::RefCell::new(String::new());
        call_with_str(|text| {
            *captured.borrow_mut() = text.to_string();
        });
        assert_eq!(*captured.borrow(), "hello");
    }

    #[test]
    fn counter_next_increments_sequentially() {
        let mut counter = Counter { current: 0 };
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
    }

    #[test]
    fn quantity_new_sets_value() {
        let distance = Quantity::<Meter>::new(42.0);
        assert_eq!(distance.value, 42.0);
    }
}