// LeetCode 66: Plus One [Easy]
// https://leetcode.com/problems/plus-one/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 1.290us
//   AI solution avg: 0.949us
//   Ratio: AI 1.36x faster
//
// Given a large integer represented as an integer array `digits`, increment
// by one. The most-significant digit is at index 0.
//
// Example 1: digits = [1,2,3] -> [1,2,4]
// Example 2: digits = [4,3,2,1] -> [4,3,2,2]
// Example 3: digits = [9] -> [1,0]
//
// Constraints: 1 <= digits.length <= 100, 0 <= digits[i] <= 9
//
// Run:  cargo run --example lc_0066_plus_one
// Test: cargo test --example lc_0066_plus_one
// Bench: cargo test --example lc_0066_plus_one bench -- --nocapture

/// My solution — iterate from right, handle carry.
///
/// Time: O(n) worst case (all 9s).
/// Space: O(1) extra (may grow vec by 1 for all-9s case).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Add one to the number represented by digits.
        ///
        /// # Arguments
        /// * `digits` - array of digits (MSB first)
        ///
        /// # Returns
        /// Digits after adding one
        pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
            for i in (0..digits.len()).rev() {
                if digits[i] < 9 {
                    digits[i] += 1;
                    return digits;
                }
                // This digit is 9 -> becomes 0, carry continues
                digits[i] = 0;
            }
            // All digits were 9 -> prepend 1 (e.g., 999 -> 1000)
            digits.insert(0, 1);
            digits
        }
    }
}

/// AI solution — same approach (canonical optimal).
///
/// Time: O(n).
/// Space: O(1) extra.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Add one to the number represented by digits.
        ///
        /// # Arguments
        /// * `digits` - array of digits (MSB first)
        ///
        /// # Returns
        /// Digits after adding one
        pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
            let mut carry = 1;
            for d in digits.iter_mut().rev() {
                let sum = *d + carry;
                *d = sum % 10;
                carry = sum / 10;
                if carry == 0 {
                    return digits;
                }
            }
            if carry > 0 {
                digits.insert(0, carry);
            }
            digits
        }
    }
}

fn main() {
    let cases = [vec![1, 2, 3], vec![4, 3, 2, 1], vec![9], vec![9, 9, 9]];
    for d in &cases {
        println!(
            "{d:?} -> my:{:?}, ai:{:?}",
            my_solution::Solution::plus_one(d.clone()),
            ai_solution::Solution::plus_one(d.clone()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_plus_one {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    assert_eq!(<$solution>::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
                }

                #[test]
                fn test_no_carry() {
                    assert_eq!(<$solution>::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
                }

                #[test]
                fn test_carry() {
                    assert_eq!(<$solution>::plus_one(vec![9]), vec![1, 0]);
                }

                #[test]
                fn test_all_nines() {
                    assert_eq!(<$solution>::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
                }

                #[test]
                fn test_single_zero() {
                    assert_eq!(<$solution>::plus_one(vec![0]), vec![1]);
                }

                #[test]
                fn test_partial_carry() {
                    assert_eq!(<$solution>::plus_one(vec![1, 9, 9]), vec![2, 0, 0]);
                }
            }
        };
    }

    test_plus_one!(my, crate::my_solution::Solution);
    test_plus_one!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let digits: Vec<i32> = vec![9; 100]; // worst case: all 9s

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::plus_one(
                std::hint::black_box(digits.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::plus_one(
                std::hint::black_box(digits.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
