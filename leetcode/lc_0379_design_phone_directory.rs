// @lc app=leetcode id=379 lang=rust
// #[Medium] - Design Phone Directory
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.263us
// ai_solution  | avg: 10.778us
// Performance: My 40.90x faster

/// # 379. Design Phone Directory
///
/// Design a phone directory with maxNumbers slots supporting:
/// get() - provide an available number (-1 if none), check(number), release(number).
///
/// Constraints:
/// - 1 <= maxNumbers <= 10^4
/// - 0 <= number < maxNumbers
/// - At most 2 * 10^4 calls

/// ## My Solution — Vec-based free list O(1) all operations
///
/// Maintain a vec as a stack of available numbers and a bool array for O(1) check.
///
/// - Time: O(1) for get, check, release
/// - Space: O(n)
mod my_solution {
    pub struct PhoneDirectory {
        available: Vec<usize>,
        is_available: Vec<bool>,
    }

    impl PhoneDirectory {
        pub fn new(max_numbers: usize) -> Self {
            Self {
                available: (0..max_numbers).collect(),
                is_available: vec![true; max_numbers],
            }
        }

        /// Get an available number, or return -1.
        pub fn get(&mut self) -> i32 {
            if let Some(num) = self.available.pop() {
                self.is_available[num] = false;
                num as i32
            } else {
                -1
            }
        }

        /// Check if a number is available.
        pub fn check(&self, number: usize) -> bool {
            self.is_available[number]
        }

        /// Release a number back to available pool.
        pub fn release(&mut self, number: usize) {
            if !self.is_available[number] {
                self.is_available[number] = true;
                self.available.push(number);
            }
        }
    }
}

/// ## AI Solution — HashSet-based O(1) amortized
///
/// Use a HashSet to store available numbers.
///
/// - Time: O(1) amortized for all operations
/// - Space: O(n)
mod ai_solution {
    use std::collections::HashSet;

    pub struct PhoneDirectory {
        available: HashSet<usize>,
    }

    impl PhoneDirectory {
        pub fn new(max_numbers: usize) -> Self {
            Self {
                available: (0..max_numbers).collect(),
            }
        }

        pub fn get(&mut self) -> i32 {
            if let Some(&num) = self.available.iter().next() {
                self.available.remove(&num);
                num as i32
            } else {
                -1
            }
        }

        pub fn check(&self, number: usize) -> bool {
            self.available.contains(&number)
        }

        pub fn release(&mut self, number: usize) {
            self.available.insert(number);
        }
    }
}

#[cfg(test)]
trait PhoneDirectoryApi {
    fn get(&mut self) -> i32;
    fn check(&self, number: usize) -> bool;
    fn release(&mut self, number: usize);
}

#[cfg(test)]
impl PhoneDirectoryApi for my_solution::PhoneDirectory {
    fn get(&mut self) -> i32 {
        self.get()
    }
    fn check(&self, number: usize) -> bool {
        self.check(number)
    }
    fn release(&mut self, number: usize) {
        self.release(number)
    }
}

#[cfg(test)]
impl PhoneDirectoryApi for ai_solution::PhoneDirectory {
    fn get(&mut self) -> i32 {
        self.get()
    }
    fn check(&self, number: usize) -> bool {
        self.check(number)
    }
    fn release(&mut self, number: usize) {
        self.release(number)
    }
}

#[cfg(test)]
fn run_tests<T: PhoneDirectoryApi>(dir: &mut T) {
    let a = dir.get();
    assert!(a >= 0 && a < 3);
    let b = dir.get();
    assert!(b >= 0 && b < 3 && b != a);
    // One number left
    let remaining = 3 - a - b;
    assert!(dir.check(remaining as usize));
    let c = dir.get();
    assert_eq!(c, remaining);
    assert!(!dir.check(remaining as usize));
    dir.release(remaining as usize);
    assert!(dir.check(remaining as usize));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        let mut dir = my_solution::PhoneDirectory::new(3);
        run_tests(&mut dir);
    }

    #[test]
    fn test_ai_solution() {
        let mut dir = ai_solution::PhoneDirectory::new(3);
        run_tests(&mut dir);
    }
}

fn main() {
    let n = 100;
    let iterations = 100_000;

    // Benchmark my_solution
    for _ in 0..1000 {
        let mut d = my_solution::PhoneDirectory::new(n);
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
        std::hint::black_box(d.check(0));
        for i in 0..n {
            d.release(i);
        }
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut d = my_solution::PhoneDirectory::new(n);
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
        std::hint::black_box(d.check(0));
        for i in 0..n {
            d.release(i);
        }
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
    }
    let my_dur = start.elapsed();

    // Benchmark ai_solution
    for _ in 0..1000 {
        let mut d = ai_solution::PhoneDirectory::new(n);
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
        std::hint::black_box(d.check(0));
        for i in 0..n {
            d.release(i);
        }
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut d = ai_solution::PhoneDirectory::new(n);
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
        std::hint::black_box(d.check(0));
        for i in 0..n {
            d.release(i);
        }
        for _ in 0..n {
            std::hint::black_box(d.get());
        }
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 379 - Design Phone Directory");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
