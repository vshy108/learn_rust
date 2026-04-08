// LeetCode 295: Find Median from Data Stream [Hard]
// https://leetcode.com/problems/find-median-from-data-stream/
//
// The median is the middle value in an ordered integer list. If the size
// of the list is even, there is no middle value, and the median is the
// mean of the two middle values.
//
// Implement the MedianFinder struct:
//   - MedianFinder::new()       -- initializes the object
//   - add_num(num: i32)         -- adds the integer num to the data structure
//   - find_median() -> f64      -- returns the median of all elements so far
//
// Examples:
//   add_num(1), add_num(2), find_median() -> 1.5
//   add_num(3), find_median() -> 2.0
//
// Constraints:
//   - -10^5 <= num <= 10^5
//   - There will be at least one element before calling find_median
//   - At most 5 * 10^4 calls will be made to add_num and find_median
//
// Run:  cargo run --example lc_0295_find_median_from_data_stream
// Test: cargo test --example lc_0295_find_median_from_data_stream
// Bench: cargo test --example lc_0295_find_median_from_data_stream bench -- --nocapture
//
// --- Benchmark (10_000 iterations, 2026-04-06) ---
//   My solution: 374.2ms (avg 37.4us)
//   AI solution: 91.7ms (avg 9.2us)
//   -> AI solution is 4.08x faster
//   Reason: For 100-element streams in debug mode, the sorted Vec's
//     cache-friendly contiguous memory + simple memcpy shifts beat
//     the two-heap's pointer-chasing sift-up/sift-down. BinaryHeap
//     has per-element Reverse wrapper overhead and more branch-heavy
//     comparisons. At larger n the O(log n) heap would win over O(n)
//     Vec::insert.
/// My solution module.
///
/// Two-heap approach: keep lower half in a max-heap, upper half in a
/// min-heap. Rebalance after each insert so max_heap.len() is equal
/// to or one greater than min_heap.len().
///
/// Time:  O(log n) per add_num, O(1) per find_median
/// Space: O(n)
mod my_solution {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    /// Tracks a running stream of integers and efficiently returns the median.
    ///
    /// Rust has no separate MaxHeap/MinHeap types. `BinaryHeap<T>` is a
    /// max-heap by default. To get a min-heap, wrap values in
    /// `std::cmp::Reverse` — this flips the `Ord` comparison so the
    /// smallest value is popped first.
    pub struct MedianFinder {
        // Ruby: @max_heap = MaxHeap.new
        // Rust: BinaryHeap is already a max-heap — pops the largest value.
        max_heap: BinaryHeap<i32>,
        // Ruby: @min_heap = MinHeap.new
        // Rust: BinaryHeap<Reverse<i32>> — Reverse flips ordering so
        // the smallest value sits at the top (popped first).
        min_heap: BinaryHeap<Reverse<i32>>,
    }

    impl MedianFinder {
        /// Initialize the MedianFinder.
        pub fn new() -> Self {
            Self {
                max_heap: BinaryHeap::new(),
                min_heap: BinaryHeap::new(),
            }
        }

        /// Add a number to the data stream.
        ///
        /// # Arguments
        ///
        /// * `num` - The integer to add
        pub fn add_num(&mut self, num: i32) {
            self.max_heap.push(num);

            // Ruby: @min_heap.push(@max_heap.pop)
            // Rust: pop() returns Option<i32> — .unwrap() is safe here
            // because we just pushed above, so max_heap is never empty.
            // Must wrap in Reverse() when pushing to the min-heap.
            self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));

            // Rebalance: max_heap should be >= min_heap in size
            // Ruby: @min_heap.size  ->  Rust: .len()
            if self.min_heap.len() > self.max_heap.len() {
                // Pop from min-heap returns Reverse(val) — unwrap the
                // inner i32 with .0 before pushing to max-heap.
                self.max_heap.push(self.min_heap.pop().unwrap().0);
            }
        }

        /// Return the median of all numbers added so far.
        ///
        /// # Returns
        ///
        /// The median as f64. If even count, return mean of the two middle values.
        pub fn find_median(&self) -> f64 {
            if self.max_heap.len() == self.min_heap.len() {
                // Ruby: @max_heap.max + @min_heap.min
                // Rust: peek() returns Option<&T> — dereference with *.
                // Min-heap peek returns &Reverse(val) — access inner with .0.
                let max_top = *self.max_heap.peek().unwrap();
                let min_top = self.min_heap.peek().unwrap().0;
                (max_top + min_top) as f64 / 2.0
            } else {
                // Ruby: @max_heap.max
                // Rust: peek() returns Option<&i32>, dereference + cast to f64.
                *self.max_heap.peek().unwrap() as f64
            }
        }
    }
}

/// AI solution module.
///
/// Sorted-Vec approach: maintain a sorted vector via binary search
/// insertion. Median is O(1) — direct index into the sorted array.
/// Insertion is O(n) due to shifting, but benefits from cache
/// locality for small/medium n.
///
/// Time:  O(n) per add_num (binary search O(log n) + shift O(n)),
///        O(1) per find_median
/// Space: O(n)
mod ai_solution {
    /// Tracks a running stream of integers and efficiently returns the median.
    ///
    /// Uses a sorted `Vec<i32>`. On each insert, `binary_search` finds
    /// the correct position in O(log n), then `Vec::insert` shifts
    /// elements to make room in O(n). Median is a direct index lookup.
    pub struct MedianFinder {
        sorted: Vec<i32>,
    }

    impl MedianFinder {
        /// Initialize the MedianFinder.
        pub fn new() -> Self {
            Self { sorted: Vec::new() }
        }

        /// Add a number to the data stream.
        ///
        /// # Arguments
        ///
        /// * `num` - The integer to add
        pub fn add_num(&mut self, num: i32) {
            // binary_search returns:
            //   Ok(index)  — found exact match at index
            //   Err(index) — not found, index is where it would be inserted
            // Both cases give the correct insertion point to maintain sort order.
            let pos = self.sorted.binary_search(&num).unwrap_or_else(|i| i);
            // Vec::insert shifts elements right — O(n) but cache-friendly
            // since elements are contiguous in memory.
            self.sorted.insert(pos, num);
        }

        /// Return the median of all numbers added so far.
        ///
        /// # Returns
        ///
        /// The median as f64. If even count, return mean of the two middle values.
        pub fn find_median(&self) -> f64 {
            let n = self.sorted.len();
            if n % 2 == 1 {
                self.sorted[n / 2] as f64
            } else {
                (self.sorted[n / 2 - 1] + self.sorted[n / 2]) as f64 / 2.0
            }
        }
    }
}

fn main() {
    println!("--- My Solution ---");
    {
        let mut mf = my_solution::MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        println!(
            "  add(1), add(2), median -> {} (expected 1.5)",
            mf.find_median()
        );
        mf.add_num(3);
        println!("  add(3), median -> {} (expected 2.0)", mf.find_median());
    }

    println!("--- AI Solution ---");
    {
        let mut mf = ai_solution::MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        println!(
            "  add(1), add(2), median -> {} (expected 1.5)",
            mf.find_median()
        );
        mf.add_num(3);
        println!("  add(3), median -> {} (expected 2.0)", mf.find_median());
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_median_finder {
        ($mod_name:ident, $mf_path:path) => {
            mod $mod_name {
                use $mf_path as MedianFinder;

                #[test]
                fn test_basic_odd() {
                    let mut mf = MedianFinder::new();
                    mf.add_num(1);
                    mf.add_num(2);
                    mf.add_num(3);
                    assert!((mf.find_median() - 2.0).abs() < 1e-5);
                }

                #[test]
                fn test_basic_even() {
                    let mut mf = MedianFinder::new();
                    mf.add_num(1);
                    mf.add_num(2);
                    assert!((mf.find_median() - 1.5).abs() < 1e-5);
                }

                #[test]
                fn test_single_element() {
                    let mut mf = MedianFinder::new();
                    mf.add_num(5);
                    assert!((mf.find_median() - 5.0).abs() < 1e-5);
                }

                #[test]
                fn test_negative_numbers() {
                    let mut mf = MedianFinder::new();
                    mf.add_num(-1);
                    mf.add_num(-2);
                    mf.add_num(-3);
                    assert!((mf.find_median() - (-2.0)).abs() < 1e-5);
                }

                #[test]
                fn test_mixed_sign() {
                    let mut mf = MedianFinder::new();
                    mf.add_num(-1);
                    mf.add_num(0);
                    mf.add_num(1);
                    assert!((mf.find_median() - 0.0).abs() < 1e-5);
                }

                #[test]
                fn test_duplicates() {
                    let mut mf = MedianFinder::new();
                    mf.add_num(2);
                    mf.add_num(2);
                    mf.add_num(2);
                    mf.add_num(2);
                    assert!((mf.find_median() - 2.0).abs() < 1e-5);
                }

                #[test]
                fn test_incremental() {
                    let mut mf = MedianFinder::new();
                    mf.add_num(1);
                    assert!((mf.find_median() - 1.0).abs() < 1e-5);
                    mf.add_num(2);
                    assert!((mf.find_median() - 1.5).abs() < 1e-5);
                    mf.add_num(3);
                    assert!((mf.find_median() - 2.0).abs() < 1e-5);
                    mf.add_num(4);
                    assert!((mf.find_median() - 2.5).abs() < 1e-5);
                }
            }
        };
    }

    test_median_finder!(my_tests, crate::my_solution::MedianFinder);
    test_median_finder!(ai_tests, crate::ai_solution::MedianFinder);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;
        let nums: Vec<i32> = (0..100).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut mf = crate::my_solution::MedianFinder::new();
            for &n in &nums {
                mf.add_num(std::hint::black_box(n));
            }
            std::hint::black_box(mf.find_median());
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut mf = crate::ai_solution::MedianFinder::new();
            for &n in &nums {
                mf.add_num(std::hint::black_box(n));
            }
            std::hint::black_box(mf.find_median());
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
