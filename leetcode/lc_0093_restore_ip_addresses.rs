// LeetCode 93: Restore IP Addresses [Medium]
// https://leetcode.com/problems/restore-ip-addresses/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 3.260us
//   AI solution avg: 1.247us
//   Ratio: AI 2.61x faster
//
// Given a string s containing only digits, return all possible valid IP
// addresses that can be formed by inserting dots. Each integer is between
// 0 and 255, cannot have leading zeros.
//
// Constraints: 1 <= s.length <= 20, s consists of digits only
//
// Run:  cargo run --example lc_0093_restore_ip_addresses
// Test: cargo test --example lc_0093_restore_ip_addresses
// Bench: cargo test --example lc_0093_restore_ip_addresses bench -- --nocapture

/// My solution — backtracking with 4 segments.
///
/// Time: O(1) bounded — at most 3^4 = 81 placements for a 12-char string.
/// Space: O(1) bounded.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Restore valid IP addresses from digit string.
        ///
        /// # Arguments
        /// * `s` - string of digits
        ///
        /// # Returns
        /// All valid IP address strings
        pub fn restore_ip_addresses(s: String) -> Vec<String> {
            let mut result = Vec::new();
            let mut segments = Vec::with_capacity(4);
            Self::backtrack(s.as_bytes(), 0, &mut segments, &mut result);
            result
        }

        fn backtrack(s: &[u8], start: usize, segments: &mut Vec<String>, result: &mut Vec<String>) {
            if segments.len() == 4 {
                if start == s.len() {
                    result.push(segments.join("."));
                }
                return;
            }

            let remaining_segments = 4 - segments.len();
            let remaining_chars = s.len() - start;
            // Prune: each segment needs 1-3 chars
            if remaining_chars < remaining_segments || remaining_chars > remaining_segments * 3 {
                return;
            }

            for len in 1..=3 {
                if start + len > s.len() {
                    break;
                }
                let seg = &s[start..start + len];
                // No leading zeros (except "0" itself)
                if len > 1 && seg[0] == b'0' {
                    break;
                }
                let val: u32 = seg.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as u32);
                if val > 255 {
                    break;
                }
                segments.push(String::from_utf8_lossy(seg).into_owned());
                Self::backtrack(s, start + len, segments, result);
                segments.pop();
            }
        }
    }
}

/// AI solution — three nested loops for dot placement.
///
/// Time: O(1) bounded.
/// Space: O(1) bounded.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Restore valid IP addresses from digit string.
        ///
        /// # Arguments
        /// * `s` - string of digits
        ///
        /// # Returns
        /// All valid IP address strings
        pub fn restore_ip_addresses(s: String) -> Vec<String> {
            let n = s.len();
            if n < 4 || n > 12 {
                return vec![];
            }
            let b = s.as_bytes();
            let mut result = Vec::new();

            for i in 1..=3.min(n - 3) {
                for j in (i + 1)..=(i + 3).min(n - 2) {
                    for k in (j + 1)..=(j + 3).min(n - 1) {
                        if n - k > 3 {
                            continue;
                        }
                        if let (Some(a), Some(b_val), Some(c), Some(d)) = (
                            Self::parse(&b[..i]),
                            Self::parse(&b[i..j]),
                            Self::parse(&b[j..k]),
                            Self::parse(&b[k..]),
                        ) {
                            result.push(format!("{a}.{b_val}.{c}.{d}"));
                        }
                    }
                }
            }
            result
        }

        /// Parse a segment, returning None if invalid (leading zero or > 255).
        fn parse(seg: &[u8]) -> Option<u32> {
            if seg.len() > 1 && seg[0] == b'0' {
                return None; // leading zero
            }
            let val = seg
                .iter()
                .fold(0u32, |acc, &b| acc * 10 + (b - b'0') as u32);
            if val > 255 { None } else { Some(val) }
        }
    }
}

fn main() {
    let my = my_solution::Solution::restore_ip_addresses("25525511135".to_string());
    let ai = ai_solution::Solution::restore_ip_addresses("25525511135".to_string());
    println!("my: {my:?}");
    println!("ai: {ai:?}");
}

#[cfg(test)]
mod tests {
    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    macro_rules! test_ip {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::sorted;

                #[test]
                fn test_example1() {
                    let result =
                        sorted(<$solution>::restore_ip_addresses("25525511135".to_string()));
                    assert_eq!(
                        result,
                        sorted(vec![
                            "255.255.11.135".to_string(),
                            "255.255.111.35".to_string(),
                        ])
                    );
                }

                #[test]
                fn test_zeros() {
                    let result = sorted(<$solution>::restore_ip_addresses("0000".to_string()));
                    assert_eq!(result, vec!["0.0.0.0".to_string()]);
                }

                #[test]
                fn test_101023() {
                    let result = sorted(<$solution>::restore_ip_addresses("101023".to_string()));
                    let expected = sorted(vec![
                        "1.0.10.23".to_string(),
                        "1.0.102.3".to_string(),
                        "10.1.0.23".to_string(),
                        "10.10.2.3".to_string(),
                        "101.0.2.3".to_string(),
                    ]);
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_too_short() {
                    let result = <$solution>::restore_ip_addresses("1".to_string());
                    assert!(result.is_empty());
                }
            }
        };
    }

    test_ip!(my, crate::my_solution::Solution);
    test_ip!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let s = "25525511135".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::restore_ip_addresses(
                std::hint::black_box(s.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::restore_ip_addresses(
                std::hint::black_box(s.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
