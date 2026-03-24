// Demonstrates multiple generic type parameters.
//
// Functions and types can have more than one generic parameter:
//
//   fn convert<F, T>(from: F) -> T { ... }
//   struct Pair<A, B> { first: A, second: B }
//
// Convention: use single uppercase letters (T, U, V, E, K, V...).
//
// Run: cargo run --example s11_multiple_generics

/// A pair of two potentially different types.
#[derive(Debug, PartialEq)]
struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    /// Creates a new Pair.
    fn new(first: A, second: B) -> Self {
        Pair { first, second }
    }

    /// Swaps the elements, returning a Pair<B, A>.
    fn swap(self) -> Pair<B, A> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

/// Returns the larger of two values (requires PartialOrd).
///
/// # Arguments
///
/// * `a` - First value
/// * `b` - Second value
///
/// # Returns
///
/// The larger of the two
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

/// Zips two slices into a Vec of Pairs.
///
/// # Arguments
///
/// * `a` - First slice
/// * `b` - Second slice
///
/// # Returns
///
/// Vec of Pairs, length = min(a.len(), b.len())
fn zip_pairs<A: Clone, B: Clone>(a: &[A], b: &[B]) -> Vec<Pair<A, B>> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| Pair::new(x.clone(), y.clone()))
        .collect()
}

fn main() {
    // --- 1. Same type for both parameters ---
    let p1 = Pair::new(1, 2);
    println!("p1: {p1:?}");

    // --- 2. Different types ---
    let p2 = Pair::new("hello", 42);
    println!("p2: {p2:?}");

    // --- 3. Swap ---
    let swapped = p2.swap();
    println!("swapped: {swapped:?}");

    // --- 4. Generic function ---
    println!("max(3, 7): {}", max_of(3, 7));
    println!("max(3.14, 2.72): {}", max_of(3.14, 2.72));
    println!("max('a', 'z'): {}", max_of('a', 'z'));

    // --- 5. Zipping ---
    let names = ["Alice", "Bob", "Charlie"];
    let ages = [30, 25, 35];
    let pairs = zip_pairs(&names, &ages);
    for p in &pairs {
        println!("  {} is {}", p.first, p.second);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_new() {
        let p = Pair::new(1, "a");
        assert_eq!(p.first, 1);
        assert_eq!(p.second, "a");
    }

    #[test]
    fn test_pair_swap() {
        let p = Pair::new("x", 99);
        let s = p.swap();
        assert_eq!(s.first, 99);
        assert_eq!(s.second, "x");
    }

    #[test]
    fn test_max_of_int() {
        assert_eq!(max_of(1, 5), 5);
        assert_eq!(max_of(10, 3), 10);
    }

    #[test]
    fn test_zip_pairs() {
        let a = [1, 2];
        let b = ["a", "b", "c"]; // longer — truncated
        let result = zip_pairs(&a, &b);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Pair::new(1, "a"));
    }
}
