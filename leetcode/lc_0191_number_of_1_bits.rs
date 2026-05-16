// LeetCode 191: Number of 1 Bits [Easy]
// https://leetcode.com/problems/number-of-1-bits/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.026us
//   AI solution avg: 0.001us
//   Ratio: AI 22.82x faster
//
// Given a positive integer n, return the number of set bits (1 bits)
// in its binary representation. Also known as the Hamming weight.
//
// Constraints: 1 <= n <= 2^31 - 1
//
// Run:  cargo run --example lc_0191_number_of_1_bits
// Test: cargo test --example lc_0191_number_of_1_bits
// Bench: cargo test --example lc_0191_number_of_1_bits bench -- --nocapture

/// My solution — shift-and-count: check the LSB with `n & 1`,
/// accumulate, then right-shift until n reaches 0.
///
/// Time: O(1) — at most 32 iterations.
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the number of set bits in n.
        pub fn hamming_weight(n: i32) -> i32 {
            // Shift-and-count approach: check each bit
            let mut count = 0;
            let mut n = n;
            while n != 0 {
                count += n & 1;
                n >>= 1; // logical shift not needed since we check != 0
            }
            count
        }
    }
}

/// AI solution — Brian Kernighan's bit trick: n & (n-1) clears the
/// lowest set bit. Count how many times until n becomes 0.
///
/// Time: O(k) where k is the number of set bits.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Brian Kernighan's algorithm: clear lowest set bit each iteration.
        pub fn hamming_weight(n: i32) -> i32 {
            let mut n = n;
            let mut count = 0;
            while n != 0 {
                n &= n - 1; // clear the lowest set bit
                count += 1;
            }
            count
        }
    }
}

fn main() {
    let my = my_solution::Solution::hamming_weight(11);
    let ai = ai_solution::Solution::hamming_weight(11);
    println!("1-bits: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_hw {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_11() {
                    // 11 = 1011 → 3 set bits
                    assert_eq!(<$solution>::hamming_weight(11), 3);
                }

                #[test]
                fn test_128() {
                    // 128 = 10000000 → 1 set bit
                    assert_eq!(<$solution>::hamming_weight(128), 1);
                }

                #[test]
                fn test_one() {
                    assert_eq!(<$solution>::hamming_weight(1), 1);
                }

                #[test]
                fn test_max() {
                    // 2^31 - 1 = 0x7FFFFFFF → 31 set bits
                    assert_eq!(<$solution>::hamming_weight(i32::MAX), 31);
                }

                #[test]
                fn test_power_of_two() {
                    assert_eq!(<$solution>::hamming_weight(1024), 1);
                }
            }
        };
    }

    test_hw!(my, crate::my_solution::Solution);
    test_hw!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::hamming_weight(
                std::hint::black_box(0x7FFF_FFFF),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::hamming_weight(
                std::hint::black_box(0x7FFF_FFFF),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
