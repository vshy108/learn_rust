// LeetCode 67: Add Binary [Easy]
// https://leetcode.com/problems/add-binary/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 15.025us
//   AI solution avg: 15.024us
//   Ratio: ~tied
//
// Given two binary strings a and b, return their sum as a binary string.
//
// Example 1: a = "11", b = "1" -> "100"
// Example 2: a = "1010", b = "1011" -> "10101"
//
// Constraints: 1 <= a.length, b.length <= 10^4, a and b consist of '0' or '1'
//
// Run:  cargo run --example lc_0067_add_binary
// Test: cargo test --example lc_0067_add_binary
// Bench: cargo test --example lc_0067_add_binary bench -- --nocapture

/// My solution — iterate from right with carry, build result in reverse.
///
/// Time: O(max(m, n)).
/// Space: O(max(m, n)) for result.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Add two binary strings.
        ///
        /// # Arguments
        /// * `a` - binary string
        /// * `b` - binary string
        ///
        /// # Returns
        /// Binary sum as string
        pub fn add_binary(a: String, b: String) -> String {
            let a = a.as_bytes();
            let b = b.as_bytes();
            let mut result = Vec::with_capacity(a.len().max(b.len()) + 1);
            let mut carry = 0u8;
            let mut i = a.len();
            let mut j = b.len();

            while i > 0 || j > 0 || carry > 0 {
                let mut sum = carry;
                if i > 0 {
                    i -= 1;
                    sum += a[i] - b'0';
                }
                if j > 0 {
                    j -= 1;
                    sum += b[j] - b'0';
                }
                result.push(b'0' + sum % 2);
                carry = sum / 2;
            }
            result.reverse();
            String::from_utf8(result).expect("valid ASCII binary")
        }
    }
}

/// AI solution — same approach (canonical for this problem).
///
/// Time: O(max(m, n)).
/// Space: O(max(m, n)).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Add two binary strings.
        ///
        /// # Arguments
        /// * `a` - binary string
        /// * `b` - binary string
        ///
        /// # Returns
        /// Binary sum as string
        pub fn add_binary(a: String, b: String) -> String {
            let a = a.as_bytes();
            let b = b.as_bytes();
            let mut result = Vec::with_capacity(a.len().max(b.len()) + 1);
            let (mut i, mut j) = (a.len(), b.len());
            let mut carry = 0u8;

            while i > 0 || j > 0 || carry > 0 {
                let mut sum = carry;
                if i > 0 {
                    i -= 1;
                    sum += a[i] - b'0';
                }
                if j > 0 {
                    j -= 1;
                    sum += b[j] - b'0';
                }
                result.push(b'0' + sum % 2);
                carry = sum / 2;
            }
            result.reverse();
            String::from_utf8(result).expect("valid ASCII binary")
        }
    }
}

fn main() {
    let cases = [("11", "1"), ("1010", "1011")];
    for (a, b) in &cases {
        println!(
            "{a} + {b} = my:{}, ai:{}",
            my_solution::Solution::add_binary(a.to_string(), b.to_string()),
            ai_solution::Solution::add_binary(a.to_string(), b.to_string()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_add_binary {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_basic() {
                    assert_eq!(<$solution>::add_binary("11".into(), "1".into()), "100");
                }

                #[test]
                fn test_same_length() {
                    assert_eq!(
                        <$solution>::add_binary("1010".into(), "1011".into()),
                        "10101"
                    );
                }

                #[test]
                fn test_zeros() {
                    assert_eq!(<$solution>::add_binary("0".into(), "0".into()), "0");
                }

                #[test]
                fn test_one_zero() {
                    assert_eq!(<$solution>::add_binary("1".into(), "0".into()), "1");
                }

                #[test]
                fn test_carry_chain() {
                    assert_eq!(<$solution>::add_binary("111".into(), "1".into()), "1000");
                }
            }
        };
    }

    test_add_binary!(my, crate::my_solution::Solution);
    test_add_binary!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let a = "1".repeat(1000);
        let b = "1".repeat(1000);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::add_binary(
                std::hint::black_box(a.clone()),
                std::hint::black_box(b.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::add_binary(
                std::hint::black_box(a.clone()),
                std::hint::black_box(b.clone()),
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
