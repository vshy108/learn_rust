// LeetCode 172: Factorial Trailing Zeroes [Medium]
// https://leetcode.com/problems/factorial-trailing-zeroes/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.003us
//   AI solution avg: 0.003us
//   Ratio: ~tied
//
// Given an integer n, return the number of trailing zeroes in n!.
// Trailing zeroes come from factors of 10 = 2*5. Since there are always
// more 2s than 5s, count factors of 5.
//
// Constraints: 0 <= n <= 10^4
//
// Run:  cargo run --example lc_0172_factorial_trailing_zeroes
// Test: cargo test --example lc_0172_factorial_trailing_zeroes
// Bench: cargo test --example lc_0172_factorial_trailing_zeroes bench -- --nocapture

/// My solution — count factors of 5 in 1..=n using repeated division:
/// floor(n/5) + floor(n/25) + floor(n/125) + ...
///
/// Time: O(log₅ n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the number of trailing zeroes in n!.
        pub fn trailing_zeroes(n: i32) -> i32 {
            // Count multiples of 5, 25, 125, ...
            let mut count = 0;
            let mut divisor = 5;
            let n_long = n as i64;
            while divisor <= n_long {
                count += (n_long / divisor) as i32;
                divisor *= 5;
            }
            count
        }
    }
}

/// AI solution — count factors of 5 in 1..=n.
///
/// n/5 gives multiples of 5, n/25 adds extra 5s from 25,50,...,
/// n/125 for 125,250,..., etc.
///
/// Time: O(log5 n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Count trailing zeroes by summing floor(n / 5^k).
        pub fn trailing_zeroes(n: i32) -> i32 {
            let mut count = 0;
            let mut power = 5i64; // use i64 to avoid overflow for large n
            let n = n as i64;
            while power <= n {
                count += n / power;
                power *= 5;
            }
            count as i32
        }
    }
}

fn main() {
    let my = my_solution::Solution::trailing_zeroes(100);
    let ai = ai_solution::Solution::trailing_zeroes(100);
    println!("trailing zeroes in 100!: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_tz {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_three() {
                    assert_eq!(<$solution>::trailing_zeroes(3), 0);
                }

                #[test]
                fn test_five() {
                    assert_eq!(<$solution>::trailing_zeroes(5), 1);
                }

                #[test]
                fn test_zero() {
                    assert_eq!(<$solution>::trailing_zeroes(0), 0);
                }

                #[test]
                fn test_ten() {
                    assert_eq!(<$solution>::trailing_zeroes(10), 2);
                }

                #[test]
                fn test_25() {
                    // 25! has 6 trailing zeroes: 5,10,15,20 each give one 5,
                    // 25 gives two 5s → total = 5 + 1 = 6
                    assert_eq!(<$solution>::trailing_zeroes(25), 6);
                }

                #[test]
                fn test_100() {
                    // 100/5=20, 100/25=4, 100/125=0 → 24
                    assert_eq!(<$solution>::trailing_zeroes(100), 24);
                }
            }
        };
    }

    test_tz!(my, crate::my_solution::Solution);
    test_tz!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::trailing_zeroes(
                std::hint::black_box(10_000),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::trailing_zeroes(
                std::hint::black_box(10_000),
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
