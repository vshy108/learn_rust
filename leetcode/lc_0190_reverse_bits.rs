// LeetCode 190: Reverse Bits [Easy]
// https://leetcode.com/problems/reverse-bits/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.001us
//   AI solution avg: 0.001us
//   Ratio: ~tied
//
// Reverse bits of a given 32-bit unsigned integer.
// E.g. 43261596 (00000010100101000001111010011100)
//   -> 964176192 (00111001011110000010100101000000)
//
// Constraints: input is a 32-bit unsigned integer
//
// Run:  cargo run --example lc_0190_reverse_bits
// Test: cargo test --example lc_0190_reverse_bits
// Bench: cargo test --example lc_0190_reverse_bits bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse bits of a 32-bit unsigned integer.
        pub fn reverse_bits(x: u32) -> u32 {
            let mut n = x;
            let mut result = 0u32;
            for _ in 0..32 {
                result = (result << 1) | (n & 1);
                n >>= 1;
            }
            result
        }
    }
}

/// AI solution — bit-by-bit reversal.
///
/// Iterate 32 times: shift result left, append LSB of n, shift n right.
///
/// Time: O(1) — fixed 32 iterations.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Reverse bits by shifting one bit at a time.
        pub fn reverse_bits(x: u32) -> u32 {
            let mut n = x;
            let mut result = 0u32;
            for _ in 0..32 {
                result = (result << 1) | (n & 1);
                n >>= 1;
            }
            result
        }
    }
}

fn main() {
    let my = my_solution::Solution::reverse_bits(43261596);
    let ai = ai_solution::Solution::reverse_bits(43261596);
    println!("reversed: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_rev {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::reverse_bits(0b00000010100101000001111010011100),
                        964176192,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::reverse_bits(0b11111111111111111111111111111101),
                        0b10111111111111111111111111111111,
                    );
                }

                #[test]
                fn test_zero() {
                    assert_eq!(<$solution>::reverse_bits(0), 0);
                }

                #[test]
                fn test_all_ones() {
                    assert_eq!(<$solution>::reverse_bits(u32::MAX), u32::MAX);
                }

                #[test]
                fn test_one() {
                    // 1 = ...0001, reversed = 1000...0 = 2^31
                    assert_eq!(<$solution>::reverse_bits(1), 1 << 31);
                }
            }
        };
    }

    test_rev!(my, crate::my_solution::Solution);
    test_rev!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::reverse_bits(
                std::hint::black_box(43261596),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::reverse_bits(
                std::hint::black_box(43261596),
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
