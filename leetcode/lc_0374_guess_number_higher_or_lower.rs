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

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
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
        #[allow(unused_variables)]
        pub fn guess_number(n: i32) -> i32 {
            // left = 1; right = n
            // while left <= right
            //   mid = left + (right - left) / 2
            //   result = guess(mid)
            //   if result == 0: return mid
            //   elsif result < 0: right = mid - 1
            //   else: left = mid + 1
            //   end
            // end
            // -1
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
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
        #[allow(unused_variables)]
        pub fn guess_number(n: i32) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32, i32)> = vec![(10, 6, 6), (1, 1, 1), (2, 1, 1)];

    println!("--- My Solution ---");
    for &(n, pick, expected) in &test_cases {
        set_pick(pick);
        let result = my_solution::Solution::guess_number(n);
        println!("  n={n}, pick={pick} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for &(n, pick, expected) in &test_cases {
        set_pick(pick);
        let result = ai_solution::Solution::guess_number(n);
        println!("  n={n}, pick={pick} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::set_pick;

    macro_rules! test_guess_number {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::set_pick;
                use $solution as Solution;

                #[test]
                fn test_pick_6_of_10() {
                    set_pick(6);
                    assert_eq!(Solution::guess_number(10), 6);
                }

                #[test]
                fn test_pick_1_of_1() {
                    set_pick(1);
                    assert_eq!(Solution::guess_number(1), 1);
                }

                #[test]
                fn test_pick_1_of_2() {
                    set_pick(1);
                    assert_eq!(Solution::guess_number(2), 1);
                }

                #[test]
                fn test_pick_at_n() {
                    set_pick(100);
                    assert_eq!(Solution::guess_number(100), 100);
                }

                #[test]
                fn test_pick_at_1() {
                    set_pick(1);
                    assert_eq!(Solution::guess_number(100), 1);
                }

                #[test]
                fn test_pick_middle() {
                    set_pick(50);
                    assert_eq!(Solution::guess_number(100), 50);
                }

                #[test]
                fn test_large_n() {
                    set_pick(1_000_000);
                    assert_eq!(Solution::guess_number(2_000_000), 1_000_000);
                }
            }
        };
    }

    test_guess_number!(my_tests, crate::my_solution::Solution);
    test_guess_number!(ai_tests, crate::ai_solution::Solution);

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
                std::hint::black_box(crate::my_solution::Solution::guess_number(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(n, pick) in &inputs {
                set_pick(pick);
                std::hint::black_box(crate::ai_solution::Solution::guess_number(
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
