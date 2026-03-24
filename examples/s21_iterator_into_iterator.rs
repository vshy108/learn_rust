// Demonstrates the Iterator and IntoIterator traits.
//
// - **Iterator**: has `next()` method, yields items one at a time.
// - **IntoIterator**: can be converted into an Iterator (has `into_iter()`).
//
// `for` loops call `.into_iter()` automatically.
//
// Run: cargo run --example s21_iterator_into_iterator

/// A custom iterator that counts down.
struct Countdown {
    current: u32,
}

impl Countdown {
    fn new(start: u32) -> Self {
        Self { current: start }
    }
}

// `Iterator` trait requires one method: `next()`
impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0 {
            let val = self.current;
            self.current -= 1;
            Some(val)
        } else {
            None
        }
    }
}

/// A collection that implements IntoIterator.
struct NumberList {
    numbers: Vec<i32>,
}

// IntoIterator allows `for item in number_list`
impl IntoIterator for NumberList {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.numbers.into_iter()
    }
}

fn main() {
    // --- 1. Custom Iterator ---
    let cd = Countdown::new(5);
    for n in cd {
        print!("{n} ");
    }
    println!();

    // --- 2. Collect into a Vec ---
    let nums: Vec<u32> = Countdown::new(3).collect();
    println!("collected: {nums:?}");

    // --- 3. IntoIterator ---
    let list = NumberList {
        numbers: vec![10, 20, 30],
    };
    for n in list {
        print!("{n} ");
    }
    println!();

    // --- 4. Standard library types implement IntoIterator ---
    // Vec, &Vec, &mut Vec, HashMap, String (chars), Range, etc.
    let v = vec![1, 2, 3];
    for &n in &v {
        print!("{n} ");
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown() {
        let v: Vec<u32> = Countdown::new(3).collect();
        assert_eq!(v, vec![3, 2, 1]);
    }

    #[test]
    fn test_into_iterator() {
        let list = NumberList {
            numbers: vec![1, 2],
        };
        let v: Vec<i32> = list.into_iter().collect();
        assert_eq!(v, vec![1, 2]);
    }
}
