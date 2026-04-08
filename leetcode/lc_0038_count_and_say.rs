// LeetCode 38: Count and Say [Medium]
// https://leetcode.com/problems/count-and-say/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 55.616µs
//   AI solution avg: 15.465µs
//   Ratio: AI solution is 3.60x faster
//
// The count-and-say sequence is a sequence of digit strings defined by the
// recursive formula:
//   countAndSay(1) = "1"
//   countAndSay(n) = RLE of countAndSay(n-1)
//
// RLE (run-length encoding): for consecutive identical digits, state the
// count followed by the digit. E.g., "3322251" -> "23321511".
//
// Example 1: n = 4 -> "1211"
//   countAndSay(1) = "1"
//   countAndSay(2) = "11"    (one 1)
//   countAndSay(3) = "21"    (two 1s)
//   countAndSay(4) = "1211"  (one 2, one 1)
//
// Constraints:
//   1 <= n <= 30
//
// Run:  cargo run --example lc_0038_count_and_say
// Test: cargo test --example lc_0038_count_and_say
// Bench: cargo test --example lc_0038_count_and_say bench -- --nocapture

/// My solution — iterative string building.
///
/// Starting from "1", repeatedly build the next string by scanning runs
/// of identical characters.
///
/// Time: O(n * L) where L is the length of the string at step n (grows exponentially).
/// Space: O(L) for the current and next strings.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Generate the n-th term of the count-and-say sequence.
        ///
        /// # Arguments
        /// * `n` - 1-indexed term number (1..=30)
        ///
        /// # Returns
        /// The n-th count-and-say string.
        pub fn count_and_say(n: i32) -> String {
            let mut current = String::from("1");

            for _ in 1..n {
                let bytes = current.as_bytes();
                let mut next = String::with_capacity(bytes.len() * 2);
                let mut i = 0;

                while i < bytes.len() {
                    let ch = bytes[i];
                    let mut count = 1u32;
                    // Count consecutive identical digits.
                    while i + (count as usize) < bytes.len() && bytes[i + (count as usize)] == ch {
                        count += 1;
                    }
                    // Append "count" then "digit".
                    next.push_str(&count.to_string());
                    next.push(ch as char);
                    i += count as usize;
                }

                current = next;
            }

            current
        }
    }
}

/// AI-optimized solution — byte buffer with format_args avoidance.
///
/// Use Vec<u8> directly instead of String to avoid UTF-8 overhead and
/// count.to_string() allocation. Since counts are small (max run of
/// identical digits is limited), we write the count digit directly.
///
/// Time: O(n * L) — same asymptotic.
/// Space: O(L) — byte buffer.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Generate the n-th term of the count-and-say sequence.
        ///
        /// # Arguments
        /// * `n` - 1-indexed term number (1..=30)
        ///
        /// # Returns
        /// The n-th count-and-say string.
        pub fn count_and_say(n: i32) -> String {
            let mut current: Vec<u8> = vec![b'1'];

            for _ in 1..n {
                let mut next: Vec<u8> = Vec::with_capacity(current.len() * 2);
                let mut i = 0;

                while i < current.len() {
                    let ch = current[i];
                    let mut count = 1usize;
                    while i + count < current.len() && current[i + count] == ch {
                        count += 1;
                    }
                    // Count will be small (in practice <= 3 for this sequence).
                    // Write count as ASCII digit(s).
                    if count >= 10 {
                        // Extremely rare, but handle it correctly.
                        for b in count.to_string().bytes() {
                            next.push(b);
                        }
                    } else {
                        next.push(b'0' + count as u8);
                    }
                    next.push(ch);
                    i += count;
                }

                current = next;
            }

            // SAFETY: current only contains ASCII digits.
            unsafe { String::from_utf8_unchecked(current) }
        }
    }
}

fn main() {
    for n in 1..=6 {
        println!(
            "n={n}: my:{}, ai:{}",
            my_solution::Solution::count_and_say(n),
            ai_solution::Solution::count_and_say(n),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_count_and_say {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_n1() {
                    assert_eq!(Solution::count_and_say(1), "1");
                }

                #[test]
                fn test_n2() {
                    assert_eq!(Solution::count_and_say(2), "11");
                }

                #[test]
                fn test_n3() {
                    assert_eq!(Solution::count_and_say(3), "21");
                }

                #[test]
                fn test_n4() {
                    assert_eq!(Solution::count_and_say(4), "1211");
                }

                #[test]
                fn test_n5() {
                    assert_eq!(Solution::count_and_say(5), "111221");
                }

                #[test]
                fn test_n6() {
                    assert_eq!(Solution::count_and_say(6), "312211");
                }

                #[test]
                fn test_both_agree_n10() {
                    // Just check both solutions produce the same output.
                    let my_val = crate::my_solution::Solution::count_and_say(10);
                    let ai_val = crate::ai_solution::Solution::count_and_say(10);
                    assert_eq!(my_val, ai_val);
                }
            }
        };
    }

    test_count_and_say!(my_tests, crate::my_solution::Solution);
    test_count_and_say!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000;
        let n = 20; // produces a string of ~5000+ chars

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::count_and_say(
                std::hint::black_box(n),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::count_and_say(
                std::hint::black_box(n),
            ));
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
