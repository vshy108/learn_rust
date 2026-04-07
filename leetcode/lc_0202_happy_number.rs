// LeetCode 202: Happy Number [Easy]
// https://leetcode.com/problems/happy-number/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.482us
//   AI solution avg: 0.061us
//   Ratio: AI 7.87x faster
//
// A happy number is defined by: starting with any positive integer,
// replace the number by the sum of the squares of its digits. Repeat
// until the number equals 1 (happy) or loops endlessly (not happy).
//
// Constraints: 1 <= n <= 2^31 - 1
//
// Run:  cargo run --example lc_0202_happy_number
// Test: cargo test --example lc_0202_happy_number
// Bench: cargo test --example lc_0202_happy_number bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if n is a happy number.
        pub fn is_happy(n: i32) -> bool {
            // HashSet approach to detect cycles
            use std::collections::HashSet;
            fn digit_sq_sum(mut n: i32) -> i32 {
                let mut sum = 0;
                while n > 0 {
                    let d = n % 10;
                    sum += d * d;
                    n /= 10;
                }
                sum
            }
            let mut seen = HashSet::new();
            let mut n = n;
            while n != 1 {
                if !seen.insert(n) {
                    return false;
                }
                n = digit_sq_sum(n);
            }
            true
        }
    }
}

/// AI solution — Floyd's cycle detection (tortoise and hare).
///
/// Compute digit-square-sum. Use slow (1 step) and fast (2 steps)
/// pointers. If fast reaches 1, it's happy. If slow == fast, it's a cycle.
///
/// Time: O(log n) per step, O(?) steps (bounded by cycle length).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Floyd's cycle detection to check for happy number.
        pub fn is_happy(n: i32) -> bool {
            fn digit_square_sum(mut n: i32) -> i32 {
                let mut sum = 0;
                while n > 0 {
                    let d = n % 10;
                    sum += d * d;
                    n /= 10;
                }
                sum
            }

            let mut slow = n;
            let mut fast = digit_square_sum(n);
            while fast != 1 && slow != fast {
                slow = digit_square_sum(slow);
                fast = digit_square_sum(digit_square_sum(fast));
            }
            fast == 1
        }
    }
}

fn main() {
    let my = my_solution::Solution::is_happy(19);
    let ai = ai_solution::Solution::is_happy(19);
    println!("is 19 happy? my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_happy {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_19() {
                    assert!(<$solution>::is_happy(19));
                }

                #[test]
                fn test_2() {
                    // 2 -> 4 -> 16 -> 37 -> 58 -> 89 -> 145 -> 42 -> 20 -> 4 (cycle)
                    assert!(!<$solution>::is_happy(2));
                }

                #[test]
                fn test_1() {
                    assert!(<$solution>::is_happy(1));
                }

                #[test]
                fn test_7() {
                    // 7 -> 49 -> 97 -> 130 -> 10 -> 1
                    assert!(<$solution>::is_happy(7));
                }

                #[test]
                fn test_4() {
                    assert!(!<$solution>::is_happy(4));
                }
            }
        };
    }

    test_happy!(my, crate::my_solution::Solution);
    test_happy!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::is_happy(
                std::hint::black_box(19),
            ));
            std::hint::black_box(crate::my_solution::Solution::is_happy(
                std::hint::black_box(2),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::is_happy(
                std::hint::black_box(19),
            ));
            std::hint::black_box(crate::ai_solution::Solution::is_happy(
                std::hint::black_box(2),
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
