// LeetCode 412: Fizz Buzz [Easy]
// https://leetcode.com/problems/fizz-buzz/
//
// Given an integer `n`, return a string array `answer` (1-indexed) where:
//   - answer[i] == "FizzBuzz" if i is divisible by 3 and 5
//   - answer[i] == "Fizz"     if i is divisible by 3
//   - answer[i] == "Buzz"     if i is divisible by 5
//   - answer[i] == i          (as a string) if none of the above
//
// Examples:
//   n = 3  -> ["1","2","Fizz"]
//   n = 5  -> ["1","2","Fizz","4","Buzz"]
//   n = 15 -> ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz",
//              "11","Fizz","13","14","FizzBuzz"]
//
// Constraints:
//   - 1 <= n <= 10^4
//
// Run:  cargo run --example lc_0412_fizz_buzz
// Test: cargo test --example lc_0412_fizz_buzz
// Bench: cargo test --example lc_0412_fizz_buzz bench -- --nocapture
//
// --- Rust concepts used ---
// - match on tuple: `match (i % 3, i % 5)` pattern-matches two values
//   simultaneously, cleaner than cascading if-else
// - `(1..=n).map().collect()`: inclusive range + map + collect to build
//   Vec<String> in functional style
//   (Previously covered: .to_string() -> lc_0012, String::push_str -> lc_0006,
//    .is_empty() -> lc_0014, .collect() turbofish -> lc_0151)
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  634.92ms (avg 6.34us)
// AI solution:  691.46ms (avg 6.91us)
// -> My solution is 1.09x faster
// Reason: match on tuple avoids repeated String::new() + conditional
//   push_str allocations; each branch returns a pre-formed literal.

/// My solution module — match on tuple of remainders.
///
/// Time:  O(n)
/// Space: O(n) for the result vector
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the FizzBuzz answer array for 1..=n.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound (inclusive)
        ///
        /// # Returns
        ///
        /// Vector of strings following FizzBuzz rules
        pub fn fizz_buzz(n: i32) -> Vec<String> {
            (1..=n)
                .map(|i| match (i % 3, i % 5) {
                    (0, 0) => "FizzBuzz".to_string(),
                    (0, _) => "Fizz".to_string(),
                    (_, 0) => "Buzz".to_string(),
                    _ => i.to_string(),
                })
                .collect()
        }
    }
}

/// AI solution module — string concatenation approach.
///
/// Builds each entry by conditionally appending "Fizz" and "Buzz",
/// falling back to the number string when neither applies.
///
/// Time:  O(n)
/// Space: O(n) for the result vector
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the FizzBuzz answer array for 1..=n.
        ///
        /// # Arguments
        ///
        /// * `n` - Upper bound (inclusive)
        ///
        /// # Returns
        ///
        /// Vector of strings following FizzBuzz rules
        pub fn fizz_buzz(n: i32) -> Vec<String> {
            (1..=n)
                .map(|i| {
                    let mut s = String::new();
                    if i % 3 == 0 {
                        s.push_str("Fizz");
                    }
                    if i % 5 == 0 {
                        s.push_str("Buzz");
                    }
                    if s.is_empty() {
                        s = i.to_string();
                    }
                    s
                })
                .collect()
        }
    }
}

fn main() {
    let test_cases: Vec<(i32, Vec<&str>)> = vec![
        (3, vec!["1", "2", "Fizz"]),
        (5, vec!["1", "2", "Fizz", "4", "Buzz"]),
        (
            15,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz",
            ],
        ),
    ];

    println!("--- My Solution ---");
    for (n, expected) in &test_cases {
        let result = my_solution::Solution::fizz_buzz(*n);
        println!("  n={n} -> {result:?} (expected {expected:?})");
    }

    println!("--- AI Solution ---");
    for (n, expected) in &test_cases {
        let result = ai_solution::Solution::fizz_buzz(*n);
        println!("  n={n} -> {result:?} (expected {expected:?})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_fizz_buzz {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                fn s(v: Vec<&str>) -> Vec<String> {
                    v.into_iter().map(String::from).collect()
                }

                #[test]
                fn test_n_3() {
                    assert_eq!(Solution::fizz_buzz(3), s(vec!["1", "2", "Fizz"]));
                }

                #[test]
                fn test_n_5() {
                    assert_eq!(
                        Solution::fizz_buzz(5),
                        s(vec!["1", "2", "Fizz", "4", "Buzz"])
                    );
                }

                #[test]
                fn test_n_15() {
                    assert_eq!(
                        Solution::fizz_buzz(15),
                        s(vec![
                            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11",
                            "Fizz", "13", "14", "FizzBuzz"
                        ])
                    );
                }

                #[test]
                fn test_n_1() {
                    assert_eq!(Solution::fizz_buzz(1), s(vec!["1"]));
                }

                #[test]
                fn test_n_30() {
                    let result = Solution::fizz_buzz(30);
                    assert_eq!(result.len(), 30);
                    assert_eq!(result[14], "FizzBuzz");
                    assert_eq!(result[29], "FizzBuzz");
                }

                #[test]
                fn test_fizz_at_6() {
                    let result = Solution::fizz_buzz(6);
                    assert_eq!(result[5], "Fizz");
                }

                #[test]
                fn test_buzz_at_10() {
                    let result = Solution::fizz_buzz(10);
                    assert_eq!(result[9], "Buzz");
                }
            }
        };
    }

    test_fizz_buzz!(my_tests, crate::my_solution::Solution);
    test_fizz_buzz!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<i32> = vec![3, 5, 15, 1, 30];

        let start = Instant::now();
        for _ in 0..iterations {
            for n in &inputs {
                std::hint::black_box(crate::my_solution::Solution::fizz_buzz(
                    std::hint::black_box(*n),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for n in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::fizz_buzz(
                    std::hint::black_box(*n),
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
