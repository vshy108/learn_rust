// LeetCode 390: Elimination Game [Medium]
// https://leetcode.com/problems/elimination-game/
//
// Benchmark: 1,000,000 iterations (2025-04-08)
//   My solution (iterative arithmetic): avg 16ns
//   AI solution (recursive mirror):     avg 84ns
//   -> My solution is 5.15x faster
//
// You have a list arr of all integers in the range [1, n] sorted in a
// strictly increasing order. Apply the following algorithm:
//   1. Starting from left, remove the first number and every other number
//      afterward until you reach the end.
//   2. Repeat from right, removing the rightmost number and every other.
//   3. Alternate left-to-right and right-to-left until one number remains.
//
// Given n, return the last remaining number.
//
// Examples:
//   n = 9 -> 6
//     [1,2,3,4,5,6,7,8,9] -> [2,4,6,8] -> [2,6] -> [6]
//   n = 1 -> 1
//
// Constraints:
//   - 1 <= n <= 10^9
//
// Run:  cargo run --example lc_0390_elimination_game
// Test: cargo test --example lc_0390_elimination_game
// Bench: cargo test --example lc_0390_elimination_game bench -- --nocapture

/// My solution module — arithmetic simulation tracking first element.
///
/// Instead of simulating the list, track the first element (`head`), the
/// step between elements, the remaining count, and the direction (L/R).
///
/// On a left pass, `head` always advances by `step`.
/// On a right pass, `head` advances by `step` only if count is odd
/// (because the last element is removed and the pattern shifts the front).
///
/// After each pass: count halves, step doubles, direction flips.
///
/// Time:  O(log n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the last number remaining after the elimination process.
        ///
        /// # Arguments
        ///
        /// * `n` - The upper bound of the range [1..n]
        ///
        /// # Returns
        ///
        /// The last remaining number
        pub fn last_remaining(n: i32) -> i32 {
            let mut head = 1;
            let mut step = 1;
            let mut remaining = n;
            let mut left_to_right = true;

            while remaining > 1 {
                // Head moves forward when:
                // 1) scanning left-to-right (we always remove the first), OR
                // 2) scanning right-to-left and count is odd (removal shifts head)
                if left_to_right || remaining % 2 == 1 {
                    head += step;
                }
                remaining /= 2;
                step *= 2;
                left_to_right = !left_to_right;
            }
            head
        }
    }
}

/// AI solution module — recursive Josephus-style reduction.
///
/// After one left-to-right pass on [1..n], the surviving elements are
/// [2, 4, 6, ...]. Dividing by 2 gives [1, 2, 3, ...n/2]. The next pass
/// is right-to-left, which is a mirror of left-to-right on the reversed
/// list: `last_remaining(n/2)` from the mirrored perspective becomes
/// `n/2 + 1 - last_remaining(n/2)`. Combining:
///   f(n) = 2 * (n/2 + 1 - f(n/2))
///
/// This elegantly captures both directions in a single recurrence.
///
/// Time:  O(log n)
/// Space: O(log n) — recursion stack
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the last number remaining after the elimination process.
        ///
        /// # Arguments
        ///
        /// * `n` - The upper bound of the range [1..n]
        ///
        /// # Returns
        ///
        /// The last remaining number
        pub fn last_remaining(n: i32) -> i32 {
            if n == 1 {
                return 1;
            }
            // After left-to-right pass, survivors are 2,4,...
            // Map to 1..n/2, apply problem recursively with reversed direction.
            // Mirror formula: f(n) = 2 * (n/2 + 1 - f(n/2))
            2 * (n / 2 + 1 - Self::last_remaining(n / 2))
        }
    }
}

fn main() {
    let cases = [(9, 6), (1, 1)];

    println!("--- My Solution ---");
    for (n, expected) in &cases {
        let result = my_solution::Solution::last_remaining(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (n, expected) in &cases {
        let result = ai_solution::Solution::last_remaining(*n);
        println!("  n={n} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_elimination {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(Solution::last_remaining(9), 6);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(Solution::last_remaining(1), 1);
                }

                #[test]
                fn test_n2() {
                    assert_eq!(Solution::last_remaining(2), 2);
                }

                #[test]
                fn test_n3() {
                    assert_eq!(Solution::last_remaining(3), 2);
                }

                #[test]
                fn test_n4() {
                    assert_eq!(Solution::last_remaining(4), 2);
                }

                #[test]
                fn test_n6() {
                    assert_eq!(Solution::last_remaining(6), 4);
                }

                #[test]
                fn test_n10() {
                    assert_eq!(Solution::last_remaining(10), 8);
                }

                #[test]
                fn test_large() {
                    // Known value for n = 100_000_000
                    assert_eq!(Solution::last_remaining(100_000_000), 32_896_342);
                }
            }
        };
    }

    test_elimination!(my_tests, crate::my_solution::Solution);
    test_elimination!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 1_000_000_u32;
        let n = 1_000_000_000;

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::last_remaining(
                std::hint::black_box(n),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::last_remaining(
                std::hint::black_box(n),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
