// LeetCode 248: Strobogrammatic Number III [Hard] (Premium)
// https://leetcode.com/problems/strobogrammatic-number-iii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 7.518us
//   AI solution avg: 7.293us
//   Ratio: AI 1.03x faster
//
// Given two strings `low` and `high` representing integers where low <= high,
// return the number of strobogrammatic numbers in [low, high].
// A strobogrammatic number looks the same when rotated 180 degrees.
//
// Example 1: low = "50", high = "100" -> 3   (69, 88, 96)
// Example 2: low = "0",  high = "0"   -> 1   (0)
//
// Constraints:
//   - 1 <= low.length, high.length <= 15
//   - low and high consist of only digits
//   - low <= high
//   - No leading zeros except for zero itself
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0248.Strobogrammatic%20Number%20III/README_EN.md
//
// Run:  cargo run --example lc_0248_strobogrammatic_number_iii
// Test: cargo test --example lc_0248_strobogrammatic_number_iii
// Bench: cargo test --example lc_0248_strobogrammatic_number_iii bench -- --nocapture

/// My solution — generate all + filter by range (translated from Python in doocs).
///
/// Reuse the recursive generation from LC 247 for each length in [len(low)..len(high)].
/// Parse each result and check if it falls in [low, high].
/// Works for values up to 15 digits — fits in i64.
///
/// Time: O(5^(n/2) * n) — generate + parse each number
/// Space: O(5^(n/2))
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return count of strobogrammatic numbers in [low, high].
        pub fn strobogrammatic_in_range(low: String, high: String) -> i32 {
            let a = low.len();
            let b = high.len();
            let lo: i64 = low.parse().expect("valid integer");
            let hi: i64 = high.parse().expect("valid integer");
            let mut ans = 0;
            for n in a..=b {
                for s in Self::dfs(n, n) {
                    // Parse the generated number and check range
                    let v: i64 = s.parse().expect("valid integer");
                    if lo <= v && v <= hi {
                        ans += 1;
                    }
                }
            }
            ans
        }

        /// Recursively generate all strobogrammatic numbers of length u,
        /// where n is the target outermost length (to avoid leading zeros).
        fn dfs(u: usize, n: usize) -> Vec<String> {
            if u == 0 {
                return vec![String::new()];
            }
            if u == 1 {
                return vec!["0".into(), "1".into(), "8".into()];
            }
            let pairs: [(char, char); 4] = [('1', '1'), ('8', '8'), ('6', '9'), ('9', '6')];
            let mut ans = Vec::new();
            for v in Self::dfs(u - 2, n) {
                for &(l, r) in &pairs {
                    let mut s = String::with_capacity(v.len() + 2);
                    s.push(l);
                    s.push_str(&v);
                    s.push(r);
                    ans.push(s);
                }
                // Only add '0' wrapping for inner layers (no leading zeros)
                if u != n {
                    let mut s = String::with_capacity(v.len() + 2);
                    s.push('0');
                    s.push_str(&v);
                    s.push('0');
                    ans.push(s);
                }
            }
            ans
        }
    }
}

/// AI solution — counting without full generation using digit DP style.
///
/// Instead of generating all strings, count strobogrammatic numbers
/// with string comparison to handle the 15-digit range (which fits i64
/// but this approach uses string compare to avoid overflow risk).
/// Generate candidates per length, compare as strings when same length
/// as low/high.
///
/// Time: O(5^(n/2) * n)
/// Space: O(5^(n/2))
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return count of strobogrammatic numbers in [low, high].
        pub fn strobogrammatic_in_range(low: String, high: String) -> i32 {
            let a = low.len();
            let b = high.len();
            let mut ans = 0;
            for n in a..=b {
                for s in Self::generate(n, n) {
                    // For same-length boundaries, use string comparison
                    // (strings of equal length compare lexicographically
                    // the same as their numeric values)
                    if s.len() == a && s.len() == b {
                        // Both bounds have same length as generated
                        if s.as_str() >= low.as_str() && s.as_str() <= high.as_str() {
                            ans += 1;
                        }
                    } else if s.len() == a {
                        // Only lower bound has same length
                        if s.as_str() >= low.as_str() {
                            ans += 1;
                        }
                    } else if s.len() == b {
                        // Only upper bound has same length
                        if s.as_str() <= high.as_str() {
                            ans += 1;
                        }
                    } else {
                        // Strictly between lengths — always valid
                        ans += 1;
                    }
                }
            }
            ans
        }

        /// Iteratively build strobogrammatic numbers of length n.
        fn generate(n: usize, target: usize) -> Vec<String> {
            let mut cur: Vec<String> = if n % 2 == 1 {
                vec!["0".into(), "1".into(), "8".into()]
            } else {
                vec![String::new()]
            };
            let mut len = n % 2;
            let pairs: [(char, char); 5] =
                [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
            while len < target {
                len += 2;
                let mut next = Vec::with_capacity(cur.len() * 5);
                for v in &cur {
                    for &(l, r) in &pairs {
                        // Skip leading zeros on outermost layer
                        if len == target && l == '0' {
                            continue;
                        }
                        let mut s = String::with_capacity(v.len() + 2);
                        s.push(l);
                        s.push_str(v);
                        s.push(r);
                        next.push(s);
                    }
                }
                cur = next;
            }
            cur
        }
    }
}

fn main() {
    println!(
        "my: 50..100 = {}",
        my_solution::Solution::strobogrammatic_in_range("50".into(), "100".into())
    );
    println!(
        "ai: 50..100 = {}",
        ai_solution::Solution::strobogrammatic_in_range("50".into(), "100".into())
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_strobogrammatic_iii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_example_1() {
                    assert_eq!(Sol::strobogrammatic_in_range("50".into(), "100".into()), 3);
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(Sol::strobogrammatic_in_range("0".into(), "0".into()), 1);
                }

                #[test]
                fn test_single_digit_range() {
                    // Strobogrammatic single digits: 0, 1, 8
                    assert_eq!(Sol::strobogrammatic_in_range("0".into(), "9".into()), 3);
                }

                #[test]
                fn test_two_digit_range() {
                    // Two-digit strobogrammatic: 11, 69, 88, 96
                    assert_eq!(Sol::strobogrammatic_in_range("10".into(), "99".into()), 4);
                }

                #[test]
                fn test_cross_length() {
                    // 0-99: single digit (0,1,8) + two digit (11,69,88,96)
                    assert_eq!(Sol::strobogrammatic_in_range("0".into(), "99".into()), 7);
                }

                #[test]
                fn test_large_range() {
                    // 0-999: singles(3) + doubles(4) + triples(12)
                    assert_eq!(Sol::strobogrammatic_in_range("0".into(), "999".into()), 19);
                }

                #[test]
                fn test_just_one() {
                    assert_eq!(Sol::strobogrammatic_in_range("88".into(), "88".into()), 1);
                }

                #[test]
                fn test_no_match() {
                    assert_eq!(Sol::strobogrammatic_in_range("12".into(), "15".into()), 0);
                }
            }
        };
    }

    test_strobogrammatic_iii!(my, crate::my_solution::Solution);
    test_strobogrammatic_iii!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::strobogrammatic_in_range(
                "50".into(),
                "10000".into(),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::strobogrammatic_in_range(
                "50".into(),
                "10000".into(),
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
        } else if ai_avg < my_avg {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        } else {
            println!("  Ratio: ~tied");
        }
    }
}
