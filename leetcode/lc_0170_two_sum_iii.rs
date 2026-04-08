// LeetCode 170: Two Sum III - Data structure design [Easy] (Premium)
// https://leetcode.com/problems/two-sum-iii-data-structure-design/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.952us
//   AI solution avg: 0.410us
//   Ratio: AI 2.32x faster
//
// Design a data structure that accepts a stream of integers and checks
// if it has a pair of integers that sum up to a particular value.
//
// Implement TwoSum:
//   - TwoSum() initializes with an empty array
//   - void add(int number) adds number to the data structure
//   - boolean find(int value) returns true if there exists a pair
//     that sums to value
//
// Example:
//   add(1), add(3), add(5)
//   find(4) -> true (1+3)
//   find(7) -> false
//
// Constraints:
//   - -10^5 <= number <= 10^5
//   - -2^31 <= value <= 2^31 - 1
//   - At most 10^4 calls to add and find
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0170.Two%20Sum%20III%20-%20Data%20structure%20design/README_EN.md
//
// Run:  cargo run --example lc_0170_two_sum_iii
// Test: cargo test --example lc_0170_two_sum_iii
// Bench: cargo test --example lc_0170_two_sum_iii bench -- --nocapture

/// My solution — HashMap counting (translated from Python in doocs/leetcode).
///
/// Store frequency counts. For find(value), iterate keys and check if
/// complement exists. Handle the case where key == complement by
/// requiring count > 1.
///
/// Time: add O(1), find O(n)
/// Space: O(n)
mod my_solution {
    use std::collections::HashMap;

    pub struct TwoSum {
        cnt: HashMap<i64, i32>,
    }

    impl TwoSum {
        /// Create a new TwoSum instance.
        pub fn new() -> Self {
            Self {
                cnt: HashMap::new(),
            }
        }

        /// Add number to the data structure.
        pub fn add(&mut self, number: i32) {
            *self.cnt.entry(number as i64).or_insert(0) += 1;
        }

        /// Return true if there exists a pair summing to value.
        pub fn find(&self, value: i32) -> bool {
            let val = value as i64;
            for (&x, &v) in &self.cnt {
                let y = val - x;
                if let Some(&yv) = self.cnt.get(&y) {
                    if x != y || v > 1 {
                        let _ = yv; // suppress unused warning
                        return true;
                    }
                }
            }
            false
        }
    }
}

/// AI solution — sorted Vec with two pointers for find.
///
/// Maintain a sorted Vec. add inserts via binary_search to keep sorted.
/// find uses two pointers from both ends for O(n) lookup with no
/// hash overhead, better cache locality.
///
/// Time: add O(n) due to insert, find O(n)
/// Space: O(n)
mod ai_solution {
    pub struct TwoSum {
        nums: Vec<i32>,
    }

    impl TwoSum {
        /// Create a new TwoSum instance.
        pub fn new() -> Self {
            Self { nums: Vec::new() }
        }

        /// Add number, maintaining sorted order.
        pub fn add(&mut self, number: i32) {
            let pos = self.nums.binary_search(&number).unwrap_or_else(|e| e);
            self.nums.insert(pos, number);
        }

        /// Return true if there exists a pair summing to value.
        pub fn find(&self, value: i32) -> bool {
            if self.nums.len() < 2 {
                return false;
            }
            let val = value as i64;
            let mut lo = 0;
            let mut hi = self.nums.len() - 1;
            while lo < hi {
                let sum = self.nums[lo] as i64 + self.nums[hi] as i64;
                if sum == val {
                    return true;
                } else if sum < val {
                    lo += 1;
                } else {
                    hi -= 1;
                }
            }
            false
        }
    }
}

fn main() {
    let mut my = my_solution::TwoSum::new();
    my.add(1);
    my.add(3);
    my.add(5);
    println!("my: find(4)={}, find(7)={}", my.find(4), my.find(7));

    let mut ai = ai_solution::TwoSum::new();
    ai.add(1);
    ai.add(3);
    ai.add(5);
    println!("ai: find(4)={}, find(7)={}", ai.find(4), ai.find(7));
}

#[cfg(test)]
mod tests {
    macro_rules! test_two_sum_iii {
        ($mod_name:ident, $solution:ident) => {
            mod $mod_name {
                use crate::$solution::TwoSum;

                #[test]
                fn test_example() {
                    let mut ts = TwoSum::new();
                    ts.add(1);
                    ts.add(3);
                    ts.add(5);
                    assert!(ts.find(4)); // 1 + 3
                    assert!(!ts.find(7)); // no pair
                }

                #[test]
                fn test_duplicate_needed() {
                    let mut ts = TwoSum::new();
                    ts.add(3);
                    assert!(!ts.find(6)); // only one 3
                    ts.add(3);
                    assert!(ts.find(6)); // 3 + 3
                }

                #[test]
                fn test_negative() {
                    let mut ts = TwoSum::new();
                    ts.add(-1);
                    ts.add(1);
                    assert!(ts.find(0));
                }

                #[test]
                fn test_empty() {
                    let ts = TwoSum::new();
                    assert!(!ts.find(0));
                }

                #[test]
                fn test_single_element() {
                    let mut ts = TwoSum::new();
                    ts.add(5);
                    assert!(!ts.find(10));
                }
            }
        };
    }

    test_two_sum_iii!(my, my_solution);
    test_two_sum_iii!(ai, ai_solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut ts = crate::my_solution::TwoSum::new();
            for i in 0..20 {
                ts.add(i);
            }
            std::hint::black_box(ts.find(15));
            std::hint::black_box(ts.find(100));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut ts = crate::ai_solution::TwoSum::new();
            for i in 0..20 {
                ts.add(i);
            }
            std::hint::black_box(ts.find(15));
            std::hint::black_box(ts.find(100));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
