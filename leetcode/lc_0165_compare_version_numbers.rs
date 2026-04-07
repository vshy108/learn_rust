// LeetCode 165: Compare Version Numbers [Medium]
// https://leetcode.com/problems/compare-version-numbers/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.596us
//   AI solution avg: 0.434us
//   Ratio: AI 1.37x faster
//
// Given two version strings version1 and version2, compare them.
// Version strings consist of revisions separated by dots '.'.
// A revision is an integer that may have leading zeros (which are ignored).
// Compare revisions left to right; missing revisions treated as 0.
// Return -1 if version1 < version2, 1 if version1 > version2, else 0.
//
// Constraints: 1 <= version1/2.length <= 500, only digits and '.'
//
// Run:  cargo run --example lc_0165_compare_version_numbers
// Test: cargo test --example lc_0165_compare_version_numbers
// Bench: cargo test --example lc_0165_compare_version_numbers bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compare two version strings and return -1, 0, or 1.
        pub fn compare_version(version1: String, version2: String) -> i32 {
            // Split, parse, and compare revision by revision
            let v1: Vec<i32> = version1.split('.').map(|s| s.parse().expect("digit")).collect();
            let v2: Vec<i32> = version2.split('.').map(|s| s.parse().expect("digit")).collect();
            let max_len = v1.len().max(v2.len());
            for i in 0..max_len {
                let r1 = if i < v1.len() { v1[i] } else { 0 };
                let r2 = if i < v2.len() { v2[i] } else { 0 };
                if r1 < r2 { return -1; }
                if r1 > r2 { return 1; }
            }
            0
        }
    }
}

/// AI solution — split by '.', parse each revision, compare left to right.
///
/// Time: O(n + m) where n, m are lengths of the version strings.
/// Space: O(n + m) for the split slices.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compare two version strings, ignoring leading zeros.
        pub fn compare_version(version1: String, version2: String) -> i32 {
            let v1: Vec<i32> = version1.split('.').map(|s| s.parse().expect("digit")).collect();
            let v2: Vec<i32> = version2.split('.').map(|s| s.parse().expect("digit")).collect();
            let max_len = v1.len().max(v2.len());

            for i in 0..max_len {
                // Missing revision defaults to 0
                let r1 = if i < v1.len() { v1[i] } else { 0 };
                let r2 = if i < v2.len() { v2[i] } else { 0 };
                match r1.cmp(&r2) {
                    std::cmp::Ordering::Less => return -1,
                    std::cmp::Ordering::Greater => return 1,
                    std::cmp::Ordering::Equal => {}
                }
            }
            0
        }
    }
}

fn main() {
    let my = my_solution::Solution::compare_version("1.01".into(), "1.001".into());
    let ai = ai_solution::Solution::compare_version("1.01".into(), "1.001".into());
    println!("compare: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_cmp {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_equal() {
                    assert_eq!(
                        <$solution>::compare_version("1.01".into(), "1.001".into()),
                        0,
                    );
                }

                #[test]
                fn test_less() {
                    assert_eq!(
                        <$solution>::compare_version("1.0".into(), "1.1".into()),
                        -1,
                    );
                }

                #[test]
                fn test_trailing_zeros() {
                    assert_eq!(
                        <$solution>::compare_version("1.0.0".into(), "1".into()),
                        0,
                    );
                }

                #[test]
                fn test_greater() {
                    assert_eq!(
                        <$solution>::compare_version("0.1".into(), "0.0.1".into()),
                        1,
                    );
                }

                #[test]
                fn test_leading_zeros() {
                    assert_eq!(
                        <$solution>::compare_version("0001.1".into(), "1.001".into()),
                        0, // 0001 parses to 1, 001 parses to 1 → 1.1 vs 1.1
                    );
                }
            }
        };
    }

    test_cmp!(my, crate::my_solution::Solution);
    test_cmp!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let v1 = "1.23.45.67.89.100.200.300".to_string();
        let v2 = "1.23.45.67.89.100.200.301".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::compare_version(
                std::hint::black_box(v1.clone()),
                std::hint::black_box(v2.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::compare_version(
                std::hint::black_box(v1.clone()),
                std::hint::black_box(v2.clone()),
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
