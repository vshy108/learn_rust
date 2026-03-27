// LeetCode 278: First Bad Version [Easy]
// https://leetcode.com/problems/first-bad-version/
//
// You are a product manager and currently leading a team to develop a
// new product. Unfortunately, the latest version of your product fails
// the quality check. Since each version is developed based on the
// previous version, all versions after a bad version are also bad.
//
// You are given an API bool isBadVersion(version) which returns whether
// version is bad. Implement a function to find the first bad version.
// You should minimize the number of calls to the API.
//
// Examples:
//   n = 5, bad = 4 -> 4  (versions 1..3 good, 4..5 bad)
//   n = 1, bad = 1 -> 1
//
// Constraints:
//   - 1 <= bad <= n <= 2^31 - 1
//
// Run:  cargo run --example lc_0278_first_bad_version
// Test: cargo test --example lc_0278_first_bad_version
// Bench: cargo test --example lc_0278_first_bad_version bench -- --nocapture

/// Simulates the `isBadVersion` API. In LeetCode, this is provided
/// externally. Here we store the bad version in a thread-local so
/// tests can configure it per-thread without data races.
///
/// `std::cell::Cell` is used because `thread_local!` requires a
/// non-`Sync` type for interior mutability. Unlike `RefCell`, `Cell`
/// works for `Copy` types (like `i32`) with no runtime borrow checks.
use std::cell::Cell;

thread_local! {
    static BAD_VERSION: Cell<i32> = const { Cell::new(1) };
}

/// Set which version is the first bad one (call before running solution).
fn set_bad_version(v: i32) {
    BAD_VERSION.set(v);
}

/// Simulated API: returns `true` if `version >= first_bad_version`.
fn is_bad_version(version: i32) -> bool {
    BAD_VERSION.get() <= version
}

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    use super::is_bad_version;

    pub struct Solution;

    impl Solution {
        /// Find the first bad version in `[1..=n]`.
        ///
        /// Uses the `is_bad_version` API to check each version.
        /// Minimize the number of API calls.
        ///
        /// # Arguments
        ///
        /// * `n` - Total number of versions
        ///
        /// # Returns
        ///
        /// The first bad version number
        #[allow(unused_variables)]
        pub fn first_bad_version(n: i32) -> i32 {
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    use super::is_bad_version;

    pub struct Solution;

    impl Solution {
        /// Find the first bad version in `[1..=n]`.
        ///
        /// Uses the `is_bad_version` API to check each version.
        /// Minimize the number of API calls.
        ///
        /// # Arguments
        ///
        /// * `n` - Total number of versions
        ///
        /// # Returns
        ///
        /// The first bad version number
        #[allow(unused_variables)]
        pub fn first_bad_version(n: i32) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, i32, i32)> = vec![(5, 4, 4), (1, 1, 1), (10, 7, 7)];

    println!("--- My Solution ---");
    for &(n, bad, expected) in &test_cases {
        set_bad_version(bad);
        let result = my_solution::Solution::first_bad_version(n);
        println!("  n={n}, bad={bad} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for &(n, bad, expected) in &test_cases {
        set_bad_version(bad);
        let result = ai_solution::Solution::first_bad_version(n);
        println!("  n={n}, bad={bad} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::set_bad_version;

    macro_rules! test_first_bad_version {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::super::set_bad_version;
                use $solution as Solution;

                #[test]
                fn test_bad_at_4_of_5() {
                    set_bad_version(4);
                    assert_eq!(Solution::first_bad_version(5), 4);
                }

                #[test]
                fn test_bad_at_1_of_1() {
                    set_bad_version(1);
                    assert_eq!(Solution::first_bad_version(1), 1);
                }

                #[test]
                fn test_bad_at_first() {
                    set_bad_version(1);
                    assert_eq!(Solution::first_bad_version(10), 1);
                }

                #[test]
                fn test_bad_at_last() {
                    set_bad_version(10);
                    assert_eq!(Solution::first_bad_version(10), 10);
                }

                #[test]
                fn test_bad_at_middle() {
                    set_bad_version(5);
                    assert_eq!(Solution::first_bad_version(10), 5);
                }

                #[test]
                fn test_large_n() {
                    set_bad_version(1_000_000);
                    assert_eq!(Solution::first_bad_version(2_000_000), 1_000_000);
                }

                #[test]
                fn test_bad_at_2_of_3() {
                    set_bad_version(2);
                    assert_eq!(Solution::first_bad_version(3), 2);
                }
            }
        };
    }

    test_first_bad_version!(my_tests, crate::my_solution::Solution);
    test_first_bad_version!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<(i32, i32)> = vec![
            (5, 4),
            (1, 1),
            (1_000_000, 500_000),
            (i32::MAX, i32::MAX / 2),
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for &(n, bad) in &inputs {
                set_bad_version(bad);
                std::hint::black_box(crate::my_solution::Solution::first_bad_version(
                    std::hint::black_box(n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for &(n, bad) in &inputs {
                set_bad_version(bad);
                std::hint::black_box(crate::ai_solution::Solution::first_bad_version(
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
