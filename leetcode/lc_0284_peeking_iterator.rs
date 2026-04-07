// LeetCode 284: Peeking Iterator [Medium]
// https://leetcode.com/problems/peeking-iterator/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.034us
//   AI solution avg: 0.033us
//   Ratio: AI 1.02x faster
//
// Design an iterator that supports peek (view next without advancing),
// next (return next and advance), and has_next operations.
//
// Example: [1,2,3] -> next()=1, peek()=2, next()=2, next()=3, hasNext()=false
//
// Constraints:
//   - 1 <= nums.length <= 1000
//   - 1 <= nums[i] <= 1000
//   - At most 1000 calls to next, hasNext, peek
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0284.Peeking%20Iterator/README_EN.md
//
// Run:  cargo run --example lc_0284_peeking_iterator
// Test: cargo test --example lc_0284_peeking_iterator
// Bench: cargo test --example lc_0284_peeking_iterator bench -- --nocapture

/// My solution — Cached peek value with boolean flag (from doocs).
///
/// Wraps an inner iterator. On peek, advance the inner iterator and
/// cache the result. Subsequent peek/next calls use the cached value
/// until consumed. has_next triggers a peek if not already cached.
///
/// Time:  O(1) per operation
/// Space: O(1) extra (just one cached value)
mod my_solution {
    pub struct PeekingIterator<I: Iterator<Item = i32>> {
        iter: I,
        has_peeked: bool,
        peeked: i32,
    }

    impl<I: Iterator<Item = i32>> PeekingIterator<I> {
        pub fn new(iter: I) -> Self {
            Self { iter, has_peeked: false, peeked: 0 }
        }

        /// Returns the next element without advancing the iterator.
        pub fn peek(&mut self) -> i32 {
            if !self.has_peeked {
                if let Some(v) = self.iter.next() {
                    self.peeked = v;
                    self.has_peeked = true;
                }
            }
            self.peeked
        }

        pub fn next(&mut self) -> i32 {
            if self.has_peeked {
                self.has_peeked = false;
                self.peeked
            } else {
                self.iter.next().expect("next called with elements")
            }
        }

        /// To check has_next without Option-pre-fetch, we peek ahead
        /// and cache the result if we haven't already.
        pub fn has_next(&mut self) -> bool {
            if self.has_peeked {
                return true;
            }
            // Try to fetch the next element
            if let Some(v) = self.iter.next() {
                self.peeked = v;
                self.has_peeked = true;
                true
            } else {
                false
            }
        }
    }
}

/// AI solution — Pre-fetch next value using Option<i32>.
///
/// Always keep the next value pre-fetched in an Option. `peek` returns
/// the cached value. `next` returns the cached value and pre-fetches
/// the following one. `has_next` checks if the cache is Some.
///
/// Time:  O(1) per operation
/// Space: O(1)
mod ai_solution {
    pub struct PeekingIterator<I: Iterator<Item = i32>> {
        iter: I,
        /// Always holds the next value to be returned, or None if exhausted.
        next_val: Option<i32>,
    }

    impl<I: Iterator<Item = i32>> PeekingIterator<I> {
        pub fn new(mut iter: I) -> Self {
            let next_val = iter.next();
            Self { iter, next_val }
        }

        /// View the next element without consuming it.
        pub fn peek(&self) -> i32 {
            self.next_val.expect("peek on exhausted iterator")
        }

        /// Return the next element and advance.
        pub fn next(&mut self) -> i32 {
            let val = self.next_val.expect("next on exhausted iterator");
            self.next_val = self.iter.next();
            val
        }

        /// Check whether more elements exist.
        pub fn has_next(&self) -> bool {
            self.next_val.is_some()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- my_solution tests ----
    #[test]
    fn test_my_example() {
        let mut it = my_solution::PeekingIterator::new(vec![1, 2, 3].into_iter());
        assert_eq!(it.next(), 1);
        assert_eq!(it.peek(), 2);
        assert_eq!(it.next(), 2);
        assert_eq!(it.next(), 3);
        assert!(!it.has_next());
    }

    #[test]
    fn test_my_peek_multiple_times() {
        let mut it = my_solution::PeekingIterator::new(vec![5, 10].into_iter());
        assert_eq!(it.peek(), 5);
        assert_eq!(it.peek(), 5);
        assert_eq!(it.next(), 5);
        assert_eq!(it.peek(), 10);
        assert_eq!(it.next(), 10);
        assert!(!it.has_next());
    }

    #[test]
    fn test_my_single() {
        let mut it = my_solution::PeekingIterator::new(vec![42].into_iter());
        assert!(it.has_next());
        assert_eq!(it.peek(), 42);
        assert_eq!(it.next(), 42);
        assert!(!it.has_next());
    }

    // ---- ai_solution tests ----
    // Test the AI solution (pre-fetch approach — cleaner and correct)
    #[test]
    fn test_ai_example() {
        let mut it = ai_solution::PeekingIterator::new(vec![1, 2, 3].into_iter());
        assert_eq!(it.next(), 1);
        assert_eq!(it.peek(), 2);
        assert_eq!(it.next(), 2);
        assert_eq!(it.next(), 3);
        assert!(!it.has_next());
    }

    #[test]
    fn test_ai_peek_multiple_times() {
        let mut it = ai_solution::PeekingIterator::new(vec![5, 10].into_iter());
        assert_eq!(it.peek(), 5);
        assert_eq!(it.peek(), 5); // peek again, same value
        assert_eq!(it.next(), 5);
        assert_eq!(it.peek(), 10);
        assert_eq!(it.next(), 10);
        assert!(!it.has_next());
    }

    #[test]
    fn test_ai_single_element() {
        let mut it = ai_solution::PeekingIterator::new(vec![42].into_iter());
        assert!(it.has_next());
        assert_eq!(it.peek(), 42);
        assert_eq!(it.next(), 42);
        assert!(!it.has_next());
    }

    #[test]
    fn test_ai_has_next_throughout() {
        let mut it = ai_solution::PeekingIterator::new(vec![1, 2].into_iter());
        assert!(it.has_next());
        assert_eq!(it.next(), 1);
        assert!(it.has_next());
        assert_eq!(it.next(), 2);
        assert!(!it.has_next());
    }

    #[test]
    fn test_ai_all_next_no_peek() {
        let mut it = ai_solution::PeekingIterator::new(vec![1, 2, 3, 4, 5].into_iter());
        for i in 1..=5 {
            assert!(it.has_next());
            assert_eq!(it.next(), i);
        }
        assert!(!it.has_next());
    }

    #[test]
    fn test_ai_alternating_peek_next() {
        let mut it = ai_solution::PeekingIterator::new(vec![10, 20, 30].into_iter());
        assert_eq!(it.peek(), 10);
        assert_eq!(it.next(), 10);
        assert_eq!(it.peek(), 20);
        assert_eq!(it.next(), 20);
        assert_eq!(it.peek(), 30);
        assert_eq!(it.next(), 30);
        assert!(!it.has_next());
    }

    #[test]
    fn test_ai_peek_at_end() {
        let mut it = ai_solution::PeekingIterator::new(vec![7].into_iter());
        assert_eq!(it.peek(), 7);
        assert_eq!(it.peek(), 7);
        assert_eq!(it.peek(), 7);
        assert_eq!(it.next(), 7);
        assert!(!it.has_next());
    }
}

fn main() {
    println!("LeetCode 284: Peeking Iterator");
    let mut it = ai_solution::PeekingIterator::new(vec![1, 2, 3].into_iter());
    println!("next()    = {}", it.next());     // 1
    println!("peek()    = {}", it.peek());     // 2
    println!("next()    = {}", it.next());     // 2
    println!("next()    = {}", it.next());     // 3
    println!("hasNext() = {}", it.has_next()); // false
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let data: Vec<i32> = (1..=100).collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            let mut it = my_solution::PeekingIterator::new(data.iter().copied());
            while it.has_next() {
                std::hint::black_box(it.next());
            }
            let mut it = ai_solution::PeekingIterator::new(data.iter().copied());
            while it.has_next() {
                std::hint::black_box(it.next());
            }
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut it = my_solution::PeekingIterator::new(data.iter().copied());
            while it.has_next() {
                std::hint::black_box(it.next());
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut it = ai_solution::PeekingIterator::new(data.iter().copied());
            while it.has_next() {
                std::hint::black_box(it.next());
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

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
