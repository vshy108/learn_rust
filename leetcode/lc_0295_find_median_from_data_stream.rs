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

/// My solution module.
///
/// Time:  O(?) per add_num, O(?) per find_median
/// Space: O(?)
mod my_solution {
    /// Tracks a running stream of integers and efficiently returns the median.
    pub struct MedianFinder {
        // TODO: choose internal data structure
    }

    impl MedianFinder {
        /// Initialize the MedianFinder.
        pub fn new() -> Self {
            todo!()
        }

        /// Add a number to the data stream.
        ///
        /// # Arguments
        ///
        /// * `num` - The integer to add
        pub fn add_num(&mut self, num: i32) {
            todo!()
        }

        /// Return the median of all numbers added so far.
        ///
        /// # Returns
        ///
        /// The median as f64. If even count, return mean of the two middle values.
        pub fn find_median(&self) -> f64 {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?) per add_num, O(?) per find_median
/// Space: O(?)
mod ai_solution {
    /// Tracks a running stream of integers and efficiently returns the median.
    pub struct MedianFinder {
        // TODO: choose internal data structure
    }

    impl MedianFinder {
        /// Initialize the MedianFinder.
        pub fn new() -> Self {
            todo!()
        }

        /// Add a number to the data stream.
        ///
        /// # Arguments
        ///
        /// * `num` - The integer to add
        pub fn add_num(&mut self, num: i32) {
            todo!()
        }

        /// Return the median of all numbers added so far.
        ///
        /// # Returns
        ///
        /// The median as f64. If even count, return mean of the two middle values.
        pub fn find_median(&self) -> f64 {
            todo!()
        }
    }
}

fn main() {
    println!("--- My Solution ---");
    {
        let mut mf = my_solution::MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        println!("  add(1), add(2), median -> {} (expected 1.5)", mf.find_median());
        mf.add_num(3);
        println!("  add(3), median -> {} (expected 2.0)", mf.find_median());
    }

    println!("--- AI Solution ---");
    {
        let mut mf = ai_solution::MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        println!("  add(1), add(2), median -> {} (expected 1.5)", mf.find_median());
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
