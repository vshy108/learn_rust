// LeetCode 201: Bitwise AND of Numbers Range [Medium]
// https://leetcode.com/problems/bitwise-and-of-numbers-range/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.008us
//   AI solution avg: 0.008us
//   Ratio: ~tied
//
// Given two integers left and right (left <= right), return the bitwise
// AND of all numbers in the range [left, right], inclusive.
//
// Constraints: 0 <= left <= right <= 2^31 - 1
//
// Run:  cargo run --example lc_0201_bitwise_and_range
// Test: cargo test --example lc_0201_bitwise_and_range
// Bench: cargo test --example lc_0201_bitwise_and_range bench -- --nocapture

/// My solution — right-shift both left and right until they are equal
/// (finding the common binary prefix), then left-shift back by the
/// number of shifts performed.
///
/// Time: O(1) — at most 31 shifts.
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the bitwise AND of all numbers in [left, right].
        pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
            // Same common-prefix approach
            let (mut l, mut r) = (left, right);
            let mut shift = 0;
            while l != r {
                l >>= 1;
                r >>= 1;
                shift += 1;
            }
            l << shift
        }
    }
}

/// AI solution — find common prefix of left and right in binary.
///
/// Key insight: the AND of a range keeps only the bits that are
/// the same in both left and right (the common prefix). Any bit
/// where they differ will have both 0 and 1 in the range, ANDing to 0.
///
/// Time: O(1) — at most 31 shifts.
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Shift both numbers right until they match (common prefix),
        /// then shift back left.
        pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
            let (mut l, mut r) = (left, right);
            let mut shift = 0;
            while l != r {
                l >>= 1;
                r >>= 1;
                shift += 1;
            }
            l << shift
        }
    }
}

fn main() {
    let my = my_solution::Solution::range_bitwise_and(5, 7);
    let ai = ai_solution::Solution::range_bitwise_and(5, 7);
    println!("range AND: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_and {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::range_bitwise_and(5, 7), 4);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::range_bitwise_and(0, 0), 0);
                }

                #[test]
                fn test_example3() {
                    assert_eq!(<$solution>::range_bitwise_and(1, 2147483647), 0);
                }

                #[test]
                fn test_same() {
                    assert_eq!(<$solution>::range_bitwise_and(42, 42), 42);
                }

                #[test]
                fn test_adjacent() {
                    // 6 = 110, 7 = 111 → AND = 110 = 6
                    assert_eq!(<$solution>::range_bitwise_and(6, 7), 6);
                }
            }
        };
    }

    test_and!(my, crate::my_solution::Solution);
    test_and!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::range_bitwise_and(
                std::hint::black_box(1_000_000),
                std::hint::black_box(2_000_000),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::range_bitwise_and(
                std::hint::black_box(1_000_000),
                std::hint::black_box(2_000_000),
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
