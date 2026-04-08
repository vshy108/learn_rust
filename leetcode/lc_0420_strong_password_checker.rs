// Benchmark: 2,000,000 iterations (2026-04-08)
// My solution:  avg 63ns   | Inline case-based greedy, single pass
// AI solution:  avg 133ns  | Collect runs then 3-phase deletion
// -> My solution is 2.11x faster
//
// LeetCode 420: Strong Password Checker [Hard]
// https://leetcode.com/problems/strong-password-checker/
//
// A password is considered strong if:
//   - It has at least 6 and at most 20 characters.
//   - It contains at least one lowercase, one uppercase, and one digit.
//   - It does not contain three repeating characters in a row.
//
// Given a string `password`, return the minimum number of steps to make it
// strong. In one step you can insert, delete, or replace one character.
//
// Examples:
//   "a" -> 5
//   "aA1" -> 3
//   "1337C0d3" -> 0
//
// Constraints:
//   1 <= password.length <= 50
//   password consists of letters, digits, dot '.' or exclamation mark '!'
//
// Run:  cargo run --example lc_0420_strong_password_checker
// Test: cargo test --example lc_0420_strong_password_checker
// Bench: cargo test --example lc_0420_strong_password_checker bench -- --nocapture

/// My solution module — case-based greedy (translated from Python/Java).
///
/// Three cases based on length `n`:
/// 1. n < 6: need max(6-n, 3-types) insertions
/// 2. 6 <= n <= 20: count replacements needed for repeating runs (run/3),
///    answer is max(replacements, 3-types)
/// 3. n > 20: must delete (n-20) chars. Deletions can reduce replacements
///    — a run of length k needs k/3 replacements, but deleting from it
///    can save replacements depending on k%3. Prioritize deleting from
///    runs where k%3==0 (1 delete saves 1 replace), then k%3==1 (2 deletes
///    save 1 replace), then k%3==2 (3 deletes save 1 replace).
///
/// Time:  O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum steps to make the password strong.
        ///
        /// # Arguments
        ///
        /// * `password` - The password string to check
        ///
        /// # Returns
        ///
        /// Minimum number of insert/delete/replace operations
        pub fn strong_password_checker(password: String) -> i32 {
            let n = password.len() as i32;
            let types = Self::count_types(&password);

            if n < 6 {
                return (6 - n).max(3 - types);
            }

            let bytes = password.as_bytes();

            if n <= 20 {
                // Only replacements needed for repeating runs
                let mut replace = 0;
                let mut cnt = 1;
                for i in 1..bytes.len() {
                    if bytes[i] == bytes[i - 1] {
                        cnt += 1;
                    } else {
                        replace += cnt / 3;
                        cnt = 1;
                    }
                }
                replace += cnt / 3;
                return replace.max(3 - types);
            }

            // n > 20: need deletions + replacements
            let mut replace = 0_i32;
            let mut remove = n - 20;
            let mut remove2 = 0_i32; // runs where cnt%3==1 (need 2 deletes to save 1 replace)

            let mut cnt = 1_i32;
            for i in 1..bytes.len() {
                if bytes[i] == bytes[i - 1] {
                    cnt += 1;
                } else {
                    if remove > 0 && cnt >= 3 {
                        if cnt % 3 == 0 {
                            // 1 delete saves 1 replace
                            remove -= 1;
                            replace -= 1;
                        } else if cnt % 3 == 1 {
                            remove2 += 1;
                        }
                    }
                    replace += cnt / 3;
                    cnt = 1;
                }
            }
            // Handle the last run
            if remove > 0 && cnt >= 3 {
                if cnt % 3 == 0 {
                    remove -= 1;
                    replace -= 1;
                } else if cnt % 3 == 1 {
                    remove2 += 1;
                }
            }
            replace += cnt / 3;

            // Phase 2: use remaining deletions on k%3==1 runs (2 deletes each)
            let use2 = replace.min(remove2).min(remove / 2);
            replace -= use2;
            remove -= use2 * 2;

            // Phase 3: use remaining deletions on k%3==2 runs (3 deletes each)
            let use3 = replace.min(remove / 3);
            replace -= use3;

            (n - 20) + replace.max(3 - types)
        }

        /// Count how many character types (lowercase, uppercase, digit) are present.
        fn count_types(s: &str) -> i32 {
            let mut lower = 0;
            let mut upper = 0;
            let mut digit = 0;
            for b in s.bytes() {
                if b.is_ascii_lowercase() {
                    lower = 1;
                } else if b.is_ascii_uppercase() {
                    upper = 1;
                } else if b.is_ascii_digit() {
                    digit = 1;
                }
            }
            lower + upper + digit
        }
    }
}

/// AI solution module — same algorithm, cleaner run collection.
///
/// Collects all run lengths first, then applies the three-phase
/// deletion optimization on the collected runs. Separating run
/// collection from deletion logic makes the code easier to follow.
///
/// Time:  O(n)
/// Space: O(r) where r = number of runs (bounded by n/3)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return minimum steps to make the password strong.
        ///
        /// # Arguments
        ///
        /// * `password` - The password string to check
        ///
        /// # Returns
        ///
        /// Minimum number of insert/delete/replace operations
        pub fn strong_password_checker(password: String) -> i32 {
            let n = password.len() as i32;
            let types = Self::count_types(&password);

            if n < 6 {
                return (6 - n).max(3 - types);
            }

            // Collect all consecutive-character run lengths >= 3
            let bytes = password.as_bytes();
            let mut runs: Vec<i32> = Vec::new();
            let mut cnt = 1;
            for i in 1..bytes.len() {
                if bytes[i] == bytes[i - 1] {
                    cnt += 1;
                } else {
                    if cnt >= 3 {
                        runs.push(cnt);
                    }
                    cnt = 1;
                }
            }
            if cnt >= 3 {
                runs.push(cnt);
            }

            let total_replace: i32 = runs.iter().map(|&r| r / 3).sum();

            if n <= 20 {
                return total_replace.max(3 - types);
            }

            // n > 20: optimize deletions to reduce replacements
            let mut replace = total_replace;
            let mut remove = n - 20;

            // Phase 1: runs where len%3==0 — 1 delete saves 1 replace
            // Sort by run%3 to process 0, then 1, then 2
            // But we can do it in 3 passes without sorting
            for run in &mut runs {
                if remove <= 0 {
                    break;
                }
                if *run >= 3 && *run % 3 == 0 {
                    remove -= 1;
                    replace -= 1;
                    *run -= 1;
                }
            }

            // Phase 2: runs where len%3==1 — 2 deletes save 1 replace
            for run in &mut runs {
                if remove <= 1 {
                    break;
                }
                if *run >= 3 && *run % 3 == 1 {
                    remove -= 2;
                    replace -= 1;
                    *run -= 2;
                }
            }

            // Phase 3: remaining deletions, each 3 saves 1 replace
            let use3 = replace.min(remove / 3);
            replace -= use3;

            (n - 20) + replace.max(3 - types)
        }

        /// Count how many character types are present.
        fn count_types(s: &str) -> i32 {
            let has_lower = s.bytes().any(|b| b.is_ascii_lowercase()) as i32;
            let has_upper = s.bytes().any(|b| b.is_ascii_uppercase()) as i32;
            let has_digit = s.bytes().any(|b| b.is_ascii_digit()) as i32;
            has_lower + has_upper + has_digit
        }
    }
}

fn main() {
    let cases = [
        ("a", 5),
        ("aA1", 3),
        ("1337C0d3", 0),
    ];

    println!("--- My Solution ---");
    for (pw, expected) in &cases {
        let result =
            my_solution::Solution::strong_password_checker(pw.to_string());
        println!("  \"{pw}\" -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (pw, expected) in &cases {
        let result =
            ai_solution::Solution::strong_password_checker(pw.to_string());
        println!("  \"{pw}\" -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_password {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_too_short() {
                    assert_eq!(
                        Solution::strong_password_checker("a".to_string()),
                        5,
                    );
                }

                #[test]
                fn test_short_missing_types() {
                    assert_eq!(
                        Solution::strong_password_checker("aA1".to_string()),
                        3,
                    );
                }

                #[test]
                fn test_already_strong() {
                    assert_eq!(
                        Solution::strong_password_checker(
                            "1337C0d3".to_string(),
                        ),
                        0,
                    );
                }

                #[test]
                fn test_repeating_short() {
                    // "aaa" -> length 3 < 6, need 3 more chars and 2 more types
                    // max(6-3, 3-1) = max(3, 2) = 3
                    assert_eq!(
                        Solution::strong_password_checker(
                            "aaa".to_string(),
                        ),
                        3,
                    );
                }

                #[test]
                fn test_repeating_mid() {
                    // "aaaaaa" -> length 6, types=1 (need 2 more)
                    // runs: 6 chars -> 6/3 = 2 replacements
                    // max(2, 2) = 2
                    assert_eq!(
                        Solution::strong_password_checker(
                            "aaaaaa".to_string(),
                        ),
                        2,
                    );
                }

                #[test]
                fn test_long_password() {
                    // 21 chars, all 'a' -> need 1 delete + fix repeats + types
                    let pw = "a".repeat(21);
                    // n=21, remove=1, types=1(need 2 more)
                    // run: 21 -> 21/3=7 replacements
                    // After 1 delete: 20 chars, run=20 -> 20/3=6 replacements
                    //   (21%3==0, so 1 delete saves 1 replace -> 6)
                    // answer: 1 + max(6, 2) = 7
                    assert_eq!(
                        Solution::strong_password_checker(pw),
                        7,
                    );
                }

                #[test]
                fn test_exact_six() {
                    // "Aa1aaa" -> length 6, all types present
                    // run of 3 'a's at end -> 1 replacement
                    assert_eq!(
                        Solution::strong_password_checker(
                            "Aa1aaa".to_string(),
                        ),
                        1,
                    );
                }
            }
        };
    }

    test_password!(my_tests, crate::my_solution::Solution);
    test_password!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 2_000_000_u32;
        let pw = "aaa111QQQbbb222WWW!!!".to_string();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::strong_password_checker(
                    std::hint::black_box(pw.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::strong_password_checker(
                    std::hint::black_box(pw.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
