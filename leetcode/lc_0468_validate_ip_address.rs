// LeetCode 468: Validate IP Address [Medium]
// https://leetcode.com/problems/validate-ip-address/
//
// Given a string queryIP, return "IPv4" if it is a valid IPv4 address,
// "IPv6" if it is a valid IPv6 address, or "Neither" if it is neither.
//
// IPv4: four decimal numbers (0-255), separated by '.', no leading zeros.
// IPv6: eight groups of four hexadecimal digits, separated by ':'.
//       Leading zeros are allowed; no empty groups.
//
// Examples:
//   "172.16.254.1"                  -> "IPv4"
//   "2001:0db8:85a3:0:0:8A2E:0370:7334" -> "IPv6"
//   "256.256.256.256"               -> "Neither"
//
// Run:  cargo run --example lc_0468_validate_ip_address
// Test: cargo test --example lc_0468_validate_ip_address
// Bench: cargo test --example lc_0468_validate_ip_address bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 150ns
//   AI solution: avg 150ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — string parsing.
///
/// Time:  O(n) where n = queryIP.len()
/// Space: O(n) — split allocations

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn valid_ip_address(query_ip: String) -> String {
            if query_ip.contains('.') {
                let parts: Vec<&str> = query_ip.split('.').collect();
                if parts.len() != 4 {
                    return "Neither".into();
                }
                for p in &parts {
                    if p.is_empty() || p.len() > 3 {
                        return "Neither".into();
                    }
                    if p.len() > 1 && p.starts_with('0') {
                        return "Neither".into();
                    }
                    if !p.bytes().all(|b| b.is_ascii_digit()) {
                        return "Neither".into();
                    }
                    if p.parse::<u32>().unwrap_or(256) > 255 {
                        return "Neither".into();
                    }
                }
                return "IPv4".into();
            } else if query_ip.contains(':') {
                let parts: Vec<&str> = query_ip.split(':').collect();
                if parts.len() != 8 {
                    return "Neither".into();
                }
                for p in &parts {
                    if p.is_empty() || p.len() > 4 {
                        return "Neither".into();
                    }
                    if !p.bytes().all(|b| b.is_ascii_hexdigit()) {
                        return "Neither".into();
                    }
                }
                return "IPv6".into();
            }
            "Neither".into()
        }
    }
}

/// AI solution module — parse and validate each format.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn valid_ip_address(query_ip: String) -> String {
            if query_ip.contains('.') {
                if Self::is_ipv4(&query_ip) {
                    return "IPv4".to_string();
                }
            } else if query_ip.contains(':') {
                if Self::is_ipv6(&query_ip) {
                    return "IPv6".to_string();
                }
            }
            "Neither".to_string()
        }

        fn is_ipv4(s: &str) -> bool {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() != 4 {
                return false;
            }
            for part in &parts {
                if part.is_empty() || part.len() > 3 {
                    return false;
                }
                // No leading zeros (except "0" itself)
                if part.len() > 1 && part.starts_with('0') {
                    return false;
                }
                // Must be all digits
                if !part.bytes().all(|b| b.is_ascii_digit()) {
                    return false;
                }
                // Value 0..=255
                let val: u32 = match part.parse() {
                    Ok(v) => v,
                    Err(_) => return false,
                };
                if val > 255 {
                    return false;
                }
            }
            true
        }

        fn is_ipv6(s: &str) -> bool {
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() != 8 {
                return false;
            }
            for part in &parts {
                // Each group must have 1-4 hex digits
                if part.is_empty() || part.len() > 4 {
                    return false;
                }
                if !part.bytes().all(|b| b.is_ascii_hexdigit()) {
                    return false;
                }
            }
            true
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_ipv4() {
            assert_eq!(
                Solution::valid_ip_address("172.16.254.1".to_string()),
                "IPv4"
            );
        }
        #[test]
        fn test_ipv6() {
            assert_eq!(
                Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()),
                "IPv6"
            );
        }
        #[test]
        fn test_neither() {
            assert_eq!(
                Solution::valid_ip_address("256.256.256.256".to_string()),
                "Neither"
            );
        }
        #[test]
        fn test_leading_zero() {
            assert_eq!(
                Solution::valid_ip_address("01.01.01.01".to_string()),
                "Neither"
            );
        }
        #[test]
        fn test_ipv6_short() {
            assert_eq!(
                Solution::valid_ip_address("2001:db8:85a3:0:0:8A2E:0370:7334".to_string()),
                "IPv6"
            );
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(String) -> String,
{
    let iterations = 500_000;
    let input = "172.16.254.1".to_string();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(input.clone())));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution("My solution", my_solution::Solution::valid_ip_address);
    bench_solution("AI solution", ai_solution::Solution::valid_ip_address);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::valid_ip_address("172.16.254.1".to_string())
    );
}
