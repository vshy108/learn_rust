// LeetCode 374: Guess Number Higher or Lower [Easy]
// https://leetcode.com/problems/guess-number-higher-or-lower/
//
// We are playing the Guess Game. The game is as follows:
// I pick a number from 1 to n. You have to guess which number I picked.
//
// Every time you guess wrong, I will tell you whether the number I
// picked is higher or lower than your guess.
//
// You call a pre-defined API int guess(int num), which returns three
// possible results:
//   -1: Your guess is higher than the number I picked (num > pick).
//    1: Your guess is lower than the number I picked (num < pick).
//    0: Your guess is equal to the number I picked (num == pick).
//
// Return the number that I picked.
//
// LeetCode Rust signature:
//   unsafe fn guess(num: i32) -> i32 {}
//   impl Solution { unsafe fn guessNumber(n: i32) -> i32 {} }
// Note: we use safe fn guess() locally; guessNumber matches LeetCode's camelCase.
//
// Examples:
//   n = 10, pick = 6 -> 6
//   n = 1,  pick = 1 -> 1
//   n = 2,  pick = 1 -> 1
//
// Constraints:
//   - 1 <= n <= 2^31 - 1
//   - 1 <= pick <= n
//
// Run:  cargo run --example lc_0374_guess_number_higher_or_lower
// Test: cargo test --example lc_0374_guess_number_higher_or_lower
// Bench: cargo test --example lc_0374_guess_number_higher_or_lower bench -- --nocapture
//
// --- Rust concepts used ---
// - i64 widening to avoid i32 midpoint overflow: cast n to i64, compute
//   mid in 64-bit, cast back to i32 for the API call
//   (Previously covered: match on Ordering -> lc_0374 (introduced above in
//    the guess() helper), binary search -> lc_0004, thread_local! + Cell
//    -> lc_0278, as usize/as i32 casts -> lc_0004)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  51.90ms (avg 519ns)
// AI solution:  55.03ms (avg 550ns)
// -> My solution is 1.06x faster
// Reason: pure i32 arithmetic avoids the i32 <-> i64 casts on every
//   iteration that the AI solution pays for the widening safety net.

/// Simulates the `guess` API. The picked number is stored in a
/// thread-local for safe per-thread configuration in tests.
use std::cell::Cell;

thread_local! {
    static PICK: Cell<i32> = const { Cell::new(1) };
}

/// Set which number was picked (call before running solution).
fn set_pick(p: i32) {
    PICK.set(p);
}

/// Simulated API: returns -1 if num > pick, 1 if num < pick, 0 if equal.
fn guess(num: i32) -> i32 {
    let pick = PICK.get();
    match num.cmp(&pick) {
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
    }
}

/// My solution module — classic binary search with inclusive bounds.
///
/// Time:  O(log n)
/// Space: O(1)
#[allow(non_snake_case)]
mod my_solution {
    use super::guess;

    pub struct Solution;

    impl Solution {
        /// Find the picked number in `[1..=n]` using the `guess` API.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound of the guessing range
        ///
        /// # Returns
        ///
        /// The picked number
        pub fn guessNumber(n: i32) -> i32 {
            let mut left = 1;
            let mut right = n;
            while left <= right {
                // Avoids overflow: left + (right - left) / 2 instead of (left + right) / 2
                let mid = left + (right - left) / 2;
                let result = guess(mid);
                if result == 0 {
                    return mid;
                } else if result < 0 {
                    // guess returned -1: our guess is too high
                    right = mid - 1;
                } else {
                    // guess returned 1: our guess is too low
                    left = mid + 1;
                }
            }
            // Should never reach here given valid constraints (1 <= pick <= n)
            -1
        }
    }
}

/// AI solution module — binary search using `match` on `Ordering`.
///
/// Same O(log n) algorithm but uses `std::cmp::Ordering` matching
/// instead of numeric comparison, making the intent clearer and
/// avoiding the subtle sign semantics of -1 / 0 / 1.
///
/// Time:  O(log n)
/// Space: O(1)
#[allow(non_snake_case)]
mod ai_solution {
    use super::guess;
    use std::cmp::Ordering;

    pub struct Solution;

    impl Solution {
        /// Find the picked number in `[1..=n]` using the `guess` API.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound of the guessing range
        ///
        /// # Returns
        ///
        /// The picked number
        pub fn guessNumber(n: i32) -> i32 {
            let mut lo = 1_i64;
            let mut hi = n as i64;
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                // Re-interpret the i32 result as Ordering for pattern matching.
                // guess() returns -1 (too high), 0 (correct), 1 (too low).
                match guess(mid as i32).cmp(&0) {
                    Ordering::Equal => return mid as i32,
                    Ordering::Less => hi = mid - 1, // guess too high
                    Ordering::Greater => lo = mid + 1, // guess too low
                }
            }
            -1
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32, i32)> = vec![(10, 6, 6), (1, 1, 1), (2, 1, 1)];

    println!("--- My Solution ---");
    for &(n, pick, expected) in &test_cases {
        set_pick(pick);
        let result = my_solution::Solution::guessNumber(n);
        println!("  n={n}, pick={pick} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for &(n, pick, expected) in &test_cases {
        set_pick(pick);
        let result = ai_solution::Solution::guessNumber(n);
        println!("  n={n}, pick={pick} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::set_pick;

    macro_rules! test_guessNumber {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::set_pick;
                use $solution as Solution;

                #[test]
                fn test_pick_6_of_10() {
                    set_pick(6);
                    assert_eq!(Solution::guessNumber(10), 6);
                }

                #[test]
                fn test_pick_1_of_1() {
                    set_pick(1);
                    assert_eq!(Solution::guessNumber(1), 1);
                }

                #[test]
                fn test_pick_1_of_2() {
                    set_pick(1);
                    assert_eq!(Solution::guessNumber(2), 1);
                }

                #[test]
                fn test_pick_at_n() {
                    set_pick(100);
                    assert_eq!(Solution::guessNumber(100), 100);
                }

                #[test]
                fn test_pick_at_1() {
                    set_pick(1);
                    assert_eq!(Solution::guessNumber(100), 1);
                }

                #[test]
                fn test_pick_middle() {
                    set_pick(50);
                    assert_eq!(Solution::guessNumber(100), 50);
                }

                #[test]
                fn test_large_n() {
                    set_pick(1_000_000);
                    assert_eq!(Solution::guessNumber(2_000_000), 1_000_000);
                }
            }
        };
    }

    test_guessNumber!(my_tests, crate::my_solution::Solution);
    test_guessNumber!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(i32, i32)> = vec![
            (10, 6),
            (1, 1),
            (1_000_000, 500_000),
            (i32::MAX, i32::MAX / 2),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(n, pick) in &inputs {
                set_pick(pick);
                std::hint::black_box(crate::my_solution::Solution::guessNumber(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(n, pick) in &inputs {
                set_pick(pick);
                std::hint::black_box(crate::ai_solution::Solution::guessNumber(
                    std::hint::black_box(n),
                ));
            }
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
