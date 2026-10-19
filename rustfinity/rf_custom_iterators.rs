//! # Rustfinity: Custom Iterators
//!
//! Source: https://github.com/rustfinity/rustfinity/tree/main/challenges/custom-iterators
//!
//! Iterators are one of Rust's most powerful abstractions. While you've used many built-in iterators and combinators, creating your own custom iterators unlocks the full potential of Rust's iteration patterns. By implementing the `Iterator` trait, you can create lazy, composable sequences that integrate seamlessly with all of Rust's iterator methods.
//!
//! The `Iterator` trait requires only one method: `next()`. This method returns `Option<Self::Item>`, yielding `Some(value)` for each element and `None` when the sequence is exhausted. The key to implementing custom iterators is managing internal state—tracking where you are in the sequence and knowing when to stop.
//!
//! ## The Iterator Trait
//!
//! ```rust
//! pub trait Iterator {
//!     type Item;
//!     fn next(&mut self) -> Option<Self::Item>;
//! }
//! ```
//!
//! Once you implement `next()`, you get all the iterator adapters (`.map()`, `.filter()`, `.fold()`, etc.) for free!
//!
//! ## Example: A Counter Iterator
//!
//! ```rust
//! struct Counter {
//!     current: u32,
//!     max: u32,
//! }
//!
//! impl Counter {
//!     fn new(max: u32) -> Self {
//!         Counter { current: 0, max }
//!     }
//! }
//!
//! impl Iterator for Counter {
//!     type Item = u32;
//!
//!     fn next(&mut self) -> Option<Self::Item> {
//!         if self.current < self.max {
//!             let value = self.current;
//!             self.current += 1;
//!             Some(value)
//!         } else {
//!             None
//!         }
//!     }
//! }
//!
//! // Usage
//! let counter = Counter::new(3);
//! let values: Vec<u32> = counter.collect();
//! assert_eq!(values, vec![0, 1, 2]);
//! ```
//!
//! ## Your Task
//!
//! Implement the following custom iterators:
//!
//! ### 1. Fibonacci Iterator
//!
//! Create a `Fibonacci` struct that generates Fibonacci numbers indefinitely:
//!
//! - `Fibonacci::new() -> Fibonacci` - Create a new Fibonacci iterator starting with 0, 1
//! - Yields: 0, 1, 1, 2, 3, 5, 8, 13, 21, ...
//!
//! ### 2. StepRange Iterator
//!
//! Create a `StepRange` struct that iterates from `start` to `end` (exclusive) with a custom step:
//!
//! - `StepRange::new(start: i32, end: i32, step: i32) -> StepRange`
//! - Handles positive steps (ascending) and negative steps (descending)
//! - Yields nothing if step would never reach end (e.g., start=0, end=10, step=-1)
//!
//! ### 3. Cycle Iterator
//!
//! Create a `CycleN<T>` struct that cycles through a slice a fixed number of times:
//!
//! - `CycleN::new(items: &[T], times: usize) -> CycleN<T>` where `T: Clone`
//! - Yields each element in order, repeating the sequence `times` times total
//!
//! ### 4. Collatz Iterator
//!
//! Create a `Collatz` struct that generates the Collatz sequence starting from a given number:
//!
//! - `Collatz::new(start: u64) -> Collatz`
//! - If n is even: next = n / 2
//! - If n is odd: next = 3n + 1
//! - Stops when it reaches 1 (yields 1 as the final element)
//!
//! ### 5. Windows Iterator
//!
//! Create a `Windows<T>` struct that yields overlapping windows of a fixed size:
//!
//! - `Windows::new(items: &[T], size: usize) -> Windows<T>` where `T: Clone`
//! - Yields slices (as `Vec<T>`) of length `size` from the items
//! - Each window overlaps with the previous by `size - 1` elements
//!
//! ### 6. Unfold Iterator
//!
//! Create an `Unfold<T, F>` struct that generates values from a state and function:
//!
//! - `Unfold::new(initial: T, f: F) -> Unfold<T, F>` where `F: FnMut(&T) -> Option<T>`
//! - The function takes the current state and returns `Some(next_state)` or `None` to stop
//! - Yields each state value (including the initial value)
//!
//! ## Examples
//!
//! ```rust
//! // Fibonacci
//! let fib = Fibonacci::new();
//! let first_ten: Vec<u64> = fib.take(10).collect();
//! assert_eq!(
//!     first_ten,
//!     vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
//! );
//!
//! // StepRange
//! let range = StepRange::new(0, 10, 2);
//! let evens: Vec<i32> = range.collect();
//! assert_eq!(evens, vec![0, 2, 4, 6, 8]);
//!
//! let countdown = StepRange::new(5, 0, -1);
//! let nums: Vec<i32> = countdown.collect();
//! assert_eq!(nums, vec![5, 4, 3, 2, 1]);
//!
//! // CycleN
//! let cycle = CycleN::new(&[1, 2, 3], 2);
//! let repeated: Vec<i32> = cycle.collect();
//! assert_eq!(
//!     repeated,
//!     vec![1, 2, 3, 1, 2, 3]
//! );
//!
//! // Collatz
//! let collatz = Collatz::new(6);
//! let sequence: Vec<u64> = collatz.collect();
//! assert_eq!(
//!     sequence,
//!     vec![6, 3, 10, 5, 16, 8, 4, 2, 1]
//! );
//!
//! // Windows
//! let windows = Windows::new(&[1, 2, 3, 4, 5], 3);
//! let groups: Vec<Vec<i32>> = windows.collect();
//! assert_eq!(
//!     groups,
//!     vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]
//! );
//!
//! // Unfold (powers of 2 until > 100)
//! let powers = Unfold::new(1u32, |&n| {
//!     let next = n * 2;
//!     if next <= 100 { Some(next) } else { None }
//! });
//! let result: Vec<u32> = powers.collect();
//! assert_eq!(
//!     result,
//!     vec![1, 2, 4, 8, 16, 32, 64]
//! );
//! ```
//!
//! ## Hints
//!
//! <details>
//!   <summary>Click here for hints</summary>
//!
//! - For `Fibonacci`, store two consecutive numbers and update them each iteration
//! - For `StepRange`, validate step direction matches the relationship between start and end
//! - For `CycleN`, track both the current index in the slice and how many complete cycles remain
//! - For `Collatz`, use a flag or Option to track when you've yielded 1 and should stop
//! - For `Windows`, track the starting index and yield a slice/clone from that position
//! - For `Unfold`, the tricky part is yielding the initial value first, then subsequent values
//! - Remember that `next()` takes `&mut self`—you need mutable state to track progress
//! - Use `Option` fields when you need to represent "already exhausted" states
//!
//! </details>
//!
//! Run: `cargo run --example rf_custom_iterators`
//! Test: `cargo test --example rf_custom_iterators`
//! Filter: `cargo test --example rf_custom_iterators <test-name>`

#![allow(unused)]
#![allow(unused_variables)]

/// A Fibonacci sequence iterator that yields Fibonacci numbers indefinitely.
///
/// The sequence starts with 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
pub struct Fibonacci {
    // TODO: Add fields
}

impl Fibonacci {
    /// Creates a new Fibonacci iterator starting from 0.
    pub fn new() -> Self {
        // TODO: Implement this function
        todo!()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement this function
        todo!()
    }
}

/// An iterator that steps through a range with a custom step size.
///
/// Supports both positive and negative steps for ascending and descending ranges.
pub struct StepRange {
    // TODO: Add fields
}

impl StepRange {
    /// Creates a new StepRange iterator.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting value (inclusive)
    /// * `end` - The ending value (exclusive)
    /// * `step` - The step size (positive or negative)
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        // TODO: Implement this function
        todo!()
    }
}

impl Iterator for StepRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement this function
        todo!()
    }
}

/// An iterator that cycles through a slice a fixed number of times.
pub struct CycleN<T> {
    // TODO: Add fields
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone> CycleN<T> {
    /// Creates a new CycleN iterator.
    ///
    /// # Arguments
    ///
    /// * `items` - The slice of items to cycle through
    /// * `times` - How many complete cycles to perform
    pub fn new(_items: &[T], _times: usize) -> Self {
        // TODO: Implement this function
        todo!()
    }
}

impl<T: Clone> Iterator for CycleN<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement this function
        todo!()
    }
}

/// An iterator that generates the Collatz sequence starting from a given number.
///
/// The Collatz sequence is defined as:
/// - If n is even: next = n / 2
/// - If n is odd: next = 3n + 1
///
/// The sequence stops when it reaches 1.
pub struct Collatz {
    // TODO: Add fields
}

impl Collatz {
    /// Creates a new Collatz iterator starting from the given number.
    pub fn new(start: u64) -> Self {
        // TODO: Implement this function
        todo!()
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement this function
        todo!()
    }
}

/// An iterator that yields overlapping windows of a fixed size.
pub struct Windows<T> {
    // TODO: Add fields
    _marker: std::marker::PhantomData<T>,
}

impl<T: Clone> Windows<T> {
    /// Creates a new Windows iterator.
    ///
    /// # Arguments
    ///
    /// * `items` - The slice to create windows from
    /// * `size` - The size of each window
    pub fn new(_items: &[T], _size: usize) -> Self {
        // TODO: Implement this function
        todo!()
    }
}

impl<T: Clone> Iterator for Windows<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement this function
        todo!()
    }
}

/// An iterator that generates values from a state and a function.
///
/// The function takes the current state and returns `Some(next_state)` to continue
/// or `None` to stop. Each state value is yielded, including the initial value.
pub struct Unfold<T, F> {
    // TODO: Add fields
    _marker: std::marker::PhantomData<(T, F)>,
}

impl<T, F> Unfold<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    /// Creates a new Unfold iterator.
    ///
    /// # Arguments
    ///
    /// * `initial` - The initial state value (will be yielded first)
    /// * `f` - A function that takes the current state and returns the next state
    pub fn new(_initial: T, _f: F) -> Self {
        // TODO: Implement this function
        todo!()
    }
}

impl<T: Clone, F> Iterator for Unfold<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement this function
        todo!()
    }
}

pub fn main() {
    // Fibonacci example
    println!("First 10 Fibonacci numbers:");
    let fib = Fibonacci::new();
    let first_ten: Vec<u64> = fib.take(10).collect();
    println!("{:?}", first_ten);

    // StepRange example
    println!("\nEven numbers 0-10:");
    let evens: Vec<i32> = StepRange::new(0, 10, 2).collect();
    println!("{:?}", evens);

    println!("\nCountdown from 5:");
    let countdown: Vec<i32> = StepRange::new(5, 0, -1).collect();
    println!("{:?}", countdown);

    // CycleN example
    println!("\nCycle [1, 2, 3] twice:");
    let cycled: Vec<i32> = CycleN::new(&[1, 2, 3], 2).collect();
    println!("{:?}", cycled);

    // Collatz example
    println!("\nCollatz sequence starting from 6:");
    let collatz: Vec<u64> = Collatz::new(6).collect();
    println!("{:?}", collatz);

    // Windows example
    println!("\nWindows of size 3 over [1, 2, 3, 4, 5]:");
    let windows: Vec<Vec<i32>> = Windows::new(&[1, 2, 3, 4, 5], 3).collect();
    println!("{:?}", windows);

    // Unfold example
    println!("\nPowers of 2 up to 100:");
    let powers: Vec<u32> = Unfold::new(1u32, |&n| {
        let next = n * 2;
        if next <= 100 { Some(next) } else { None }
    })
    .collect();
    println!("{:?}", powers);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::*;

    // =============================================================================
    // Fibonacci Tests
    // =============================================================================

    mod fibonacci_tests {
        use super::*;

        #[test]
        fn first_ten_fibonacci() {
            let fib = Fibonacci::new();
            let result: Vec<u64> = fib.take(10).collect();
            assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        }

        #[test]
        fn first_fibonacci_is_zero() {
            let mut fib = Fibonacci::new();
            assert_eq!(fib.next(), Some(0));
        }

        #[test]
        fn second_fibonacci_is_one() {
            let mut fib = Fibonacci::new();
            fib.next(); // 0
            assert_eq!(fib.next(), Some(1));
        }

        #[test]
        fn fibonacci_take_one() {
            let fib = Fibonacci::new();
            let result: Vec<u64> = fib.take(1).collect();
            assert_eq!(result, vec![0]);
        }

        #[test]
        fn fibonacci_take_zero() {
            let fib = Fibonacci::new();
            let result: Vec<u64> = fib.take(0).collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn fibonacci_fifteenth() {
            let fib = Fibonacci::new();
            let result: Vec<u64> = fib.take(15).collect();
            assert_eq!(result[14], 377);
        }

        #[test]
        fn fibonacci_with_skip() {
            let fib = Fibonacci::new();
            let result: Vec<u64> = fib.skip(5).take(5).collect();
            assert_eq!(result, vec![5, 8, 13, 21, 34]);
        }

        #[test]
        fn fibonacci_sum_first_ten() {
            let fib = Fibonacci::new();
            let sum: u64 = fib.take(10).sum();
            assert_eq!(sum, 88);
        }
    }

    // =============================================================================
    // StepRange Tests
    // =============================================================================

    mod step_range_tests {
        use super::*;

        #[test]
        fn ascending_step_two() {
            let range = StepRange::new(0, 10, 2);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![0, 2, 4, 6, 8]);
        }

        #[test]
        fn ascending_step_one() {
            let range = StepRange::new(1, 5, 1);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![1, 2, 3, 4]);
        }

        #[test]
        fn descending_step() {
            let range = StepRange::new(5, 0, -1);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![5, 4, 3, 2, 1]);
        }

        #[test]
        fn descending_step_two() {
            let range = StepRange::new(10, 0, -2);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![10, 8, 6, 4, 2]);
        }

        #[test]
        fn step_range_empty_ascending_wrong_direction() {
            // Start < end but step is negative
            let range = StepRange::new(0, 10, -1);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn step_range_empty_descending_wrong_direction() {
            // Start > end but step is positive
            let range = StepRange::new(10, 0, 1);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn step_range_empty_same_start_end() {
            let range = StepRange::new(5, 5, 1);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn step_range_zero_step() {
            let range = StepRange::new(0, 10, 0);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn step_range_negative_numbers() {
            let range = StepRange::new(-5, 0, 1);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![-5, -4, -3, -2, -1]);
        }

        #[test]
        fn step_range_negative_to_negative() {
            let range = StepRange::new(-10, -5, 2);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![-10, -8, -6]);
        }

        #[test]
        fn step_range_large_step() {
            let range = StepRange::new(0, 100, 30);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![0, 30, 60, 90]);
        }

        #[test]
        fn step_range_single_element() {
            let range = StepRange::new(5, 6, 1);
            let result: Vec<i32> = range.collect();
            assert_eq!(result, vec![5]);
        }
    }

    // =============================================================================
    // CycleN Tests
    // =============================================================================

    mod cycle_n_tests {
        use super::*;

        #[test]
        fn cycle_twice() {
            let cycle = CycleN::new(&[1, 2, 3], 2);
            let result: Vec<i32> = cycle.collect();
            assert_eq!(result, vec![1, 2, 3, 1, 2, 3]);
        }

        #[test]
        fn cycle_once() {
            let cycle = CycleN::new(&[1, 2, 3], 1);
            let result: Vec<i32> = cycle.collect();
            assert_eq!(result, vec![1, 2, 3]);
        }

        #[test]
        fn cycle_zero_times() {
            let cycle = CycleN::new(&[1, 2, 3], 0);
            let result: Vec<i32> = cycle.collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn cycle_empty_slice() {
            let cycle: CycleN<i32> = CycleN::new(&[], 5);
            let result: Vec<i32> = cycle.collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn cycle_single_element() {
            let cycle = CycleN::new(&[42], 3);
            let result: Vec<i32> = cycle.collect();
            assert_eq!(result, vec![42, 42, 42]);
        }

        #[test]
        fn cycle_three_times() {
            let cycle = CycleN::new(&['a', 'b'], 3);
            let result: Vec<char> = cycle.collect();
            assert_eq!(result, vec!['a', 'b', 'a', 'b', 'a', 'b']);
        }

        #[test]
        fn cycle_strings() {
            let cycle = CycleN::new(&["hello".to_string(), "world".to_string()], 2);
            let result: Vec<String> = cycle.collect();
            assert_eq!(result, vec!["hello", "world", "hello", "world"]);
        }

        #[test]
        fn cycle_with_take() {
            let cycle = CycleN::new(&[1, 2, 3], 10);
            let result: Vec<i32> = cycle.take(5).collect();
            assert_eq!(result, vec![1, 2, 3, 1, 2]);
        }
    }

    // =============================================================================
    // Collatz Tests
    // =============================================================================

    mod collatz_tests {
        use super::*;

        #[test]
        fn collatz_from_6() {
            let collatz = Collatz::new(6);
            let result: Vec<u64> = collatz.collect();
            assert_eq!(result, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
        }

        #[test]
        fn collatz_from_1() {
            let collatz = Collatz::new(1);
            let result: Vec<u64> = collatz.collect();
            assert_eq!(result, vec![1]);
        }

        #[test]
        fn collatz_from_2() {
            let collatz = Collatz::new(2);
            let result: Vec<u64> = collatz.collect();
            assert_eq!(result, vec![2, 1]);
        }

        #[test]
        fn collatz_from_7() {
            let collatz = Collatz::new(7);
            let result: Vec<u64> = collatz.collect();
            assert_eq!(
                result,
                vec![7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
            );
        }

        #[test]
        fn collatz_from_27() {
            let collatz = Collatz::new(27);
            let result: Vec<u64> = collatz.collect();
            assert_eq!(result.len(), 112); // Famous for having 111 steps
            assert_eq!(result[0], 27);
            assert_eq!(result[result.len() - 1], 1);
        }

        #[test]
        fn collatz_from_0() {
            let collatz = Collatz::new(0);
            let result: Vec<u64> = collatz.collect();
            assert_eq!(result, vec![]);
        }

        #[test]
        fn collatz_from_power_of_two() {
            let collatz = Collatz::new(16);
            let result: Vec<u64> = collatz.collect();
            assert_eq!(result, vec![16, 8, 4, 2, 1]);
        }

        #[test]
        fn collatz_count_steps() {
            let collatz = Collatz::new(6);
            assert_eq!(collatz.count(), 9);
        }

        #[test]
        fn collatz_max_value() {
            let collatz = Collatz::new(7);
            let max = collatz.max();
            assert_eq!(max, Some(52));
        }
    }

    // =============================================================================
    // Windows Tests
    // =============================================================================

    mod windows_tests {
        use super::*;

        #[test]
        fn windows_size_three() {
            let windows = Windows::new(&[1, 2, 3, 4, 5], 3);
            let result: Vec<Vec<i32>> = windows.collect();
            assert_eq!(result, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        }

        #[test]
        fn windows_size_two() {
            let windows = Windows::new(&[1, 2, 3, 4], 2);
            let result: Vec<Vec<i32>> = windows.collect();
            assert_eq!(result, vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
        }

        #[test]
        fn windows_size_one() {
            let windows = Windows::new(&[1, 2, 3], 1);
            let result: Vec<Vec<i32>> = windows.collect();
            assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
        }

        #[test]
        fn windows_size_equal_to_length() {
            let windows = Windows::new(&[1, 2, 3], 3);
            let result: Vec<Vec<i32>> = windows.collect();
            assert_eq!(result, vec![vec![1, 2, 3]]);
        }

        #[test]
        fn windows_size_larger_than_length() {
            let windows = Windows::new(&[1, 2, 3], 5);
            let result: Vec<Vec<i32>> = windows.collect();
            assert_eq!(result, Vec::<Vec<i32>>::new());
        }

        #[test]
        fn windows_empty_input() {
            let windows: Windows<i32> = Windows::new(&[], 2);
            let result: Vec<Vec<i32>> = windows.collect();
            assert_eq!(result, Vec::<Vec<i32>>::new());
        }

        #[test]
        fn windows_size_zero() {
            let windows = Windows::new(&[1, 2, 3], 0);
            let result: Vec<Vec<i32>> = windows.collect();
            assert_eq!(result, Vec::<Vec<i32>>::new());
        }

        #[test]
        fn windows_with_strings() {
            let windows = Windows::new(&["a".to_string(), "b".to_string(), "c".to_string()], 2);
            let result: Vec<Vec<String>> = windows.collect();
            assert_eq!(
                result,
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()]
                ]
            );
        }

        #[test]
        fn windows_count() {
            let windows = Windows::new(&[1, 2, 3, 4, 5, 6], 3);
            assert_eq!(windows.count(), 4);
        }
    }

    // =============================================================================
    // Unfold Tests
    // =============================================================================

    mod unfold_tests {
        use super::*;

        #[test]
        fn unfold_powers_of_two() {
            let powers = Unfold::new(1u32, |&n| {
                let next = n * 2;
                if next <= 100 { Some(next) } else { None }
            });
            let result: Vec<u32> = powers.collect();
            assert_eq!(result, vec![1, 2, 4, 8, 16, 32, 64]);
        }

        #[test]
        fn unfold_countdown() {
            let countdown = Unfold::new(5i32, |&n| if n > 0 { Some(n - 1) } else { None });
            let result: Vec<i32> = countdown.collect();
            assert_eq!(result, vec![5, 4, 3, 2, 1, 0]);
        }

        #[test]
        fn unfold_fibonacci_like() {
            // Using unfold to generate pairs and extract first element
            let fib = Unfold::new((0u64, 1u64), |&(a, b)| Some((b, a + b)));
            let result: Vec<u64> = fib.take(10).map(|(a, _)| a).collect();
            assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        }

        #[test]
        fn unfold_immediate_stop() {
            let iter = Unfold::new(42i32, |_| None);
            let result: Vec<i32> = iter.collect();
            assert_eq!(result, vec![42]);
        }

        #[test]
        fn unfold_single_step() {
            let iter = Unfold::new(1i32, |&n| if n < 2 { Some(n + 1) } else { None });
            let result: Vec<i32> = iter.collect();
            assert_eq!(result, vec![1, 2]);
        }

        #[test]
        fn unfold_with_strings() {
            let iter = Unfold::new("a".to_string(), |s| {
                if s.len() < 4 {
                    Some(format!("{}a", s))
                } else {
                    None
                }
            });
            let result: Vec<String> = iter.collect();
            assert_eq!(result, vec!["a", "aa", "aaa", "aaaa"]);
        }

        #[test]
        fn unfold_geometric_sequence() {
            let geometric = Unfold::new(1.0f64, |&x| {
                let next = x * 0.5;
                if next >= 0.1 { Some(next) } else { None }
            });
            let result: Vec<f64> = geometric.collect();
            assert_eq!(result.len(), 4);
            assert!((result[0] - 1.0).abs() < f64::EPSILON);
            assert!((result[1] - 0.5).abs() < f64::EPSILON);
            assert!((result[2] - 0.25).abs() < f64::EPSILON);
            assert!((result[3] - 0.125).abs() < f64::EPSILON);
        }

        #[test]
        fn unfold_with_take() {
            let naturals = Unfold::new(1u32, |&n| Some(n + 1));
            let result: Vec<u32> = naturals.take(5).collect();
            assert_eq!(result, vec![1, 2, 3, 4, 5]);
        }
    }

    // =============================================================================
    // Integration Tests
    // =============================================================================

    mod integration_tests {
        use super::*;

        #[test]
        fn fibonacci_filter_even() {
            let fib = Fibonacci::new();
            let even_fibs: Vec<u64> = fib.take(15).filter(|&n| n % 2 == 0).collect();
            assert_eq!(even_fibs, vec![0, 2, 8, 34, 144]);
        }

        #[test]
        fn step_range_sum() {
            let range = StepRange::new(1, 11, 2);
            let sum: i32 = range.sum();
            assert_eq!(sum, 25); // 1 + 3 + 5 + 7 + 9
        }

        #[test]
        fn cycle_and_enumerate() {
            let cycle = CycleN::new(&['a', 'b'], 2);
            let indexed: Vec<(usize, char)> = cycle.enumerate().collect();
            assert_eq!(indexed, vec![(0, 'a'), (1, 'b'), (2, 'a'), (3, 'b')]);
        }

        #[test]
        fn collatz_all_greater_than_zero() {
            let mut collatz = Collatz::new(100);
            assert!(collatz.all(|n| n > 0));
        }

        #[test]
        fn windows_map_sum() {
            let windows = Windows::new(&[1, 2, 3, 4, 5], 3);
            let sums: Vec<i32> = windows.map(|w| w.iter().sum()).collect();
            assert_eq!(sums, vec![6, 9, 12]); // 1+2+3, 2+3+4, 3+4+5
        }

        #[test]
        fn unfold_fold() {
            let iter = Unfold::new(1i32, |&n| if n < 5 { Some(n + 1) } else { None });
            let sum: i32 = iter.fold(0, |acc, n| acc + n);
            assert_eq!(sum, 15); // 1 + 2 + 3 + 4 + 5
        }

        #[test]
        fn combine_step_range_and_windows() {
            let range = StepRange::new(0, 6, 1);
            let values: Vec<i32> = range.collect();
            let windows = Windows::new(&values, 2);
            let pairs: Vec<Vec<i32>> = windows.collect();
            assert_eq!(
                pairs,
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
            );
        }

        #[test]
        fn fibonacci_zip_with_indices() {
            let fib = Fibonacci::new();
            let indexed: Vec<(usize, u64)> = fib.take(5).enumerate().collect();
            assert_eq!(indexed, vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 3)]);
        }

        #[test]
        fn chain_multiple_iterators() {
            let range1 = StepRange::new(0, 3, 1);
            let range2 = StepRange::new(10, 13, 1);
            let combined: Vec<i32> = range1.chain(range2).collect();
            assert_eq!(combined, vec![0, 1, 2, 10, 11, 12]);
        }
    }
}
