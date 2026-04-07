// LeetCode 89: Gray Code [Medium]
// https://leetcode.com/problems/gray-code/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 11.633us
//   AI solution avg: 8.276us
//   Ratio: AI 1.41x faster
//
// An n-bit gray code sequence is a sequence of 2^n integers where:
// - Every integer is in [0, 2^n - 1]
// - The first integer is 0
// - Adjacent integers differ by exactly one bit
// - First and last integers also differ by one bit
//
// Constraints: 1 <= n <= 16
//
// Run:  cargo run --example lc_0089_gray_code
// Test: cargo test --example lc_0089_gray_code
// Bench: cargo test --example lc_0089_gray_code bench -- --nocapture

/// My solution — iterative mirror construction.
///
/// Start with [0]. For each bit, mirror existing sequence and prepend the bit.
///
/// Time: O(2^n).
/// Space: O(2^n).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate an n-bit Gray code sequence.
        ///
        /// # Arguments
        /// * `n` - number of bits
        ///
        /// # Returns
        /// Gray code sequence
        pub fn gray_code(n: i32) -> Vec<i32> {
            let mut result = vec![0];
            for i in 0..n {
                let bit = 1 << i;
                // Mirror: traverse current result in reverse, add bit
                let len = result.len();
                for j in (0..len).rev() {
                    result.push(result[j] | bit);
                }
            }
            result
        }
    }
}

/// AI solution — direct formula: gray(i) = i ^ (i >> 1).
///
/// Time: O(2^n).
/// Space: O(2^n) for output only.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate an n-bit Gray code sequence.
        ///
        /// # Arguments
        /// * `n` - number of bits
        ///
        /// # Returns
        /// Gray code sequence
        pub fn gray_code(n: i32) -> Vec<i32> {
            let total = 1 << n;
            let mut result = Vec::with_capacity(total);
            for i in 0..total as i32 {
                result.push(i ^ (i >> 1)); // Gray code formula
            }
            result
        }
    }
}

fn main() {
    let my = my_solution::Solution::gray_code(3);
    let ai = ai_solution::Solution::gray_code(3);
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    /// Verify gray code property: adjacent elements differ by 1 bit.
    fn is_valid_gray(seq: &[i32], n: i32) -> bool {
        let total = 1usize << n;
        if seq.len() != total {
            return false;
        }
        if seq[0] != 0 {
            return false;
        }
        for i in 0..seq.len() {
            let next = (i + 1) % seq.len();
            let diff = seq[i] ^ seq[next];
            // Must differ by exactly one bit (power of 2)
            if diff == 0 || (diff & (diff - 1)) != 0 {
                return false;
            }
        }
        true
    }

    macro_rules! test_gray_code {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::is_valid_gray;

                #[test]
                fn test_n2() {
                    let result = <$solution>::gray_code(2);
                    assert!(is_valid_gray(&result, 2));
                }

                #[test]
                fn test_n1() {
                    let result = <$solution>::gray_code(1);
                    assert!(is_valid_gray(&result, 1));
                }

                #[test]
                fn test_n3() {
                    let result = <$solution>::gray_code(3);
                    assert!(is_valid_gray(&result, 3));
                }

                #[test]
                fn test_n4() {
                    let result = <$solution>::gray_code(4);
                    assert!(is_valid_gray(&result, 4));
                    assert_eq!(result.len(), 16);
                }
            }
        };
    }

    test_gray_code!(my, crate::my_solution::Solution);
    test_gray_code!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::gray_code(
                std::hint::black_box(10),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::gray_code(
                std::hint::black_box(10),
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
