// Demonstrates building Option<T> from scratch to understand its internals.
//
// Option is just an enum with two variants. There's no compiler magic.
// Let's rebuild it to see how it works.
//
// Run: cargo run --example s12_build_option

/// Our custom Option — functionally identical to std::option::Option.
#[derive(Debug, PartialEq)]
enum MyOption<T> {
    MySome(T),
    MyNone,
}

impl<T> MyOption<T> {
    /// Returns true if the option contains a value.
    fn is_some(&self) -> bool {
        matches!(self, MyOption::MySome(_))
    }

    /// Returns true if the option is empty.
    fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Extracts the value, panicking if None.
    fn unwrap(self) -> T {
        match self {
            MyOption::MySome(v) => v,
            MyOption::MyNone => panic!("called unwrap on MyNone"),
        }
    }

    /// Returns the contained value or a default.
    fn unwrap_or(self, default: T) -> T {
        match self {
            MyOption::MySome(v) => v,
            MyOption::MyNone => default,
        }
    }

    /// Applies a function to the contained value (if any).
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MyOption<U> {
        match self {
            MyOption::MySome(v) => MyOption::MySome(f(v)),
            MyOption::MyNone => MyOption::MyNone,
        }
    }

    /// Returns MyNone if empty, otherwise calls `f` with the value.
    fn and_then<U, F: FnOnce(T) -> MyOption<U>>(self, f: F) -> MyOption<U> {
        match self {
            MyOption::MySome(v) => f(v),
            MyOption::MyNone => MyOption::MyNone,
        }
    }
}

fn main() {
    // --- 1. Creating our custom Option ---
    let some = MyOption::MySome(42);
    let none: MyOption<i32> = MyOption::MyNone;
    println!("some = {some:?}, none = {none:?}");

    // --- 2. is_some / is_none ---
    println!("some.is_some() = {}", some.is_some());
    println!("none.is_none() = {}", none.is_none());

    // --- 3. unwrap ---
    println!("unwrap: {}", MyOption::MySome(10).unwrap());

    // --- 4. unwrap_or ---
    println!("unwrap_or: {}", none.unwrap_or(99));

    // --- 5. map ---
    let doubled = MyOption::MySome(5).map(|x| x * 2);
    println!("map: {doubled:?}");

    // --- 6. and_then ---
    let result = MyOption::MySome(10).and_then(|x| {
        if x > 0 {
            MyOption::MySome(x.to_string())
        } else {
            MyOption::MyNone
        }
    });
    println!("and_then: {result:?}");

    // The standard library's Option works exactly the same way —
    // it's just an enum with methods!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_some() {
        assert!(MyOption::MySome(1).is_some());
        assert!(!MyOption::<i32>::MyNone.is_some());
    }

    #[test]
    fn test_unwrap_some() {
        assert_eq!(MyOption::MySome(42).unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "called unwrap on MyNone")]
    fn test_unwrap_none() {
        MyOption::<i32>::MyNone.unwrap();
    }

    #[test]
    fn test_unwrap_or() {
        assert_eq!(MyOption::MySome(5).unwrap_or(0), 5);
        assert_eq!(MyOption::<i32>::MyNone.unwrap_or(10), 10);
    }

    #[test]
    fn test_map() {
        let doubled = MyOption::MySome(3).map(|x| x * 2);
        assert_eq!(doubled, MyOption::MySome(6));

        let none_mapped = MyOption::<i32>::MyNone.map(|x| x * 2);
        assert_eq!(none_mapped, MyOption::MyNone);
    }

    #[test]
    fn test_and_then() {
        let r = MyOption::MySome(5).and_then(|x| MyOption::MySome(x + 1));
        assert_eq!(r, MyOption::MySome(6));

        let r = MyOption::<i32>::MyNone.and_then(|x| MyOption::MySome(x + 1));
        assert_eq!(r, MyOption::MyNone);
    }
}
