// Demonstrates traits with generics.
//
// Traits can have generic type parameters, allowing implementors
// to work with different types.
//
// Run: cargo run --example s18_traits_generics

/// A generic converter trait.
trait Converter<T> {
    /// Converts self into type T.
    fn convert(&self) -> T;
}

struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);

// Celsius can convert to Fahrenheit:
impl Converter<Fahrenheit> for Celsius {
    fn convert(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

// Celsius can convert to Kelvin:
impl Converter<Kelvin> for Celsius {
    fn convert(&self) -> Kelvin {
        Kelvin(self.0 + 273.15)
    }
}

/// A generic container trait.
trait Container<T> {
    fn items(&self) -> &[T];
    fn count(&self) -> usize {
        self.items().len()
    }
}

struct Basket {
    fruits: Vec<String>,
}

impl Container<String> for Basket {
    fn items(&self) -> &[String] {
        &self.fruits
    }
}

struct NumberBag {
    numbers: Vec<i32>,
}

impl Container<i32> for NumberBag {
    fn items(&self) -> &[i32] {
        &self.numbers
    }
}

fn main() {
    let c = Celsius(100.0);
    let f: Fahrenheit = c.convert();
    let k: Kelvin = c.convert();
    println!("100 C = {:.1} F", f.0);
    println!("100 C = {:.2} K", k.0);

    let b = Basket {
        fruits: vec!["apple".into(), "banana".into()],
    };
    println!("Basket has {} items", b.count());

    let n = NumberBag {
        numbers: vec![1, 2, 3],
    };
    println!("NumberBag has {} items", n.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let f: Fahrenheit = Celsius(0.0).convert();
        assert!((f.0 - 32.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let k: Kelvin = Celsius(0.0).convert();
        assert!((k.0 - 273.15).abs() < f64::EPSILON);
    }

    #[test]
    fn test_container_count() {
        let b = Basket {
            fruits: vec!["a".into()],
        };
        assert_eq!(b.count(), 1);
    }
}
