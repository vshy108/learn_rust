// LeetCode 306: Additive Number [Medium]
// https://leetcode.com/problems/additive-number/
// Determine if a digit string can form an additive sequence (each number is
// the sum of the two preceding ones). No leading zeros allowed.
//
// Benchmark: 100_000 iterations
// My solution (DFS with string addition): avg 0.196us
// AI solution (iterative validation):     avg 0.348us
// Winner: My solution (1.78x faster)

// ------ My Solution: DFS with big number string addition ------
// Time: O(n^3) — try all (i, j) splits, string add is O(n) per step
// Space: O(n) — recursion depth and string storage
/// Try all possible first two numbers by varying split points i, j.
/// Then recursively verify the rest using string-based addition
/// (handles numbers up to 35 digits — beyond i64 range).
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn is_additive_number(num: String) -> bool {
            let n = num.len();
            let bytes = num.as_bytes();
            // Try all possible first (0..i) and second (i..j) numbers
            for i in 1..n {
                // No leading zeros for first number (except "0" itself)
                if i > 1 && bytes[0] == b'0' {
                    break;
                }
                for j in (i + 1)..n {
                    // No leading zeros for second number
                    if j - i > 1 && bytes[i] == b'0' {
                        continue;
                    }
                    if Self::dfs(&num[..i], &num[i..j], &num[j..]) {
                        return true;
                    }
                }
            }
            false
        }

        /// Check if `rest` can be fully consumed by the additive sequence
        /// starting with `a` and `b`.
        fn dfs(a: &str, b: &str, rest: &str) -> bool {
            if rest.is_empty() {
                return true;
            }
            let sum = Self::add_strings(a, b);
            // Check if rest starts with the sum
            if rest.starts_with(&sum) {
                Self::dfs(b, &sum, &rest[sum.len()..])
            } else {
                false
            }
        }

        /// Add two non-negative number strings, returns their sum as string.
        fn add_strings(a: &str, b: &str) -> String {
            let mut result = Vec::new();
            let a_bytes = a.as_bytes();
            let b_bytes = b.as_bytes();
            let mut carry = 0u8;
            let mut i = a_bytes.len();
            let mut j = b_bytes.len();
            while i > 0 || j > 0 || carry > 0 {
                let mut sum = carry;
                if i > 0 {
                    i -= 1;
                    sum += a_bytes[i] - b'0';
                }
                if j > 0 {
                    j -= 1;
                    sum += b_bytes[j] - b'0';
                }
                result.push(b'0' + sum % 10);
                carry = sum / 10;
            }
            result.reverse();
            String::from_utf8(result).expect("valid UTF-8")
        }
    }
}

// ------ AI Solution: Iterative validation with i64 ------
// Time: O(n^2) for the outer loops, O(n) for each validation chain
// Space: O(1) — only numeric variables (assumes numbers fit i64)
/// For each pair of starting numbers, iteratively compute the next
/// and check if it matches the remaining string. Uses i64 arithmetic
/// (works for the given constraint of len <= 35 if numbers stay small).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn is_additive_number(num: String) -> bool {
            let n = num.len();
            let bytes = num.as_bytes();
            // Try all splits for first two numbers
            for i in 1..n {
                if i > 1 && bytes[0] == b'0' {
                    break;
                }
                for j in (i + 1)..n {
                    if j - i > 1 && bytes[i] == b'0' {
                        continue;
                    }
                    if Self::validate(bytes, i, j) {
                        return true;
                    }
                }
            }
            false
        }

        /// Iteratively validate the additive sequence using string addition.
        fn validate(bytes: &[u8], i: usize, j: usize) -> bool {
            // Use string-based addition for large number support
            let mut a = bytes[..i].to_vec();
            let mut b = bytes[i..j].to_vec();
            let mut k = j;
            while k < bytes.len() {
                let sum = Self::add_bytes(&a, &b);
                // Check if bytes[k..] starts with sum
                if k + sum.len() > bytes.len() {
                    return false;
                }
                if bytes[k..k + sum.len()] != sum[..] {
                    return false;
                }
                k += sum.len();
                a = b;
                b = sum;
            }
            true
        }

        /// Add two digit-byte arrays (big-endian), return sum as byte array.
        fn add_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
            let mut result = Vec::new();
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
                result.push(b'0' + sum % 10);
                carry = sum / 10;
            }
            result.reverse();
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    #[test]
    fn test_example1() {
        assert!(MySolution::is_additive_number("112358".into()));
        assert!(AiSolution::is_additive_number("112358".into()));
    }

    #[test]
    fn test_example2() {
        assert!(MySolution::is_additive_number("199100199".into()));
        assert!(AiSolution::is_additive_number("199100199".into()));
    }

    #[test]
    fn test_not_additive() {
        assert!(!MySolution::is_additive_number("1234".into()));
        assert!(!AiSolution::is_additive_number("1234".into()));
    }

    #[test]
    fn test_zero_start() {
        // "011" -> 0, 1, 1 -> valid
        assert!(MySolution::is_additive_number("011".into()));
        assert!(AiSolution::is_additive_number("011".into()));
    }

    #[test]
    fn test_all_zeros() {
        // "000" -> 0, 0, 0 -> valid
        assert!(MySolution::is_additive_number("000".into()));
        assert!(AiSolution::is_additive_number("000".into()));
    }

    #[test]
    fn test_too_short() {
        assert!(!MySolution::is_additive_number("1".into()));
        assert!(!AiSolution::is_additive_number("1".into()));
    }

    #[test]
    fn test_two_digits() {
        assert!(!MySolution::is_additive_number("12".into()));
        assert!(!AiSolution::is_additive_number("12".into()));
    }

    #[test]
    fn test_simple_valid() {
        // "123" -> 1, 2, 3 -> valid
        assert!(MySolution::is_additive_number("123".into()));
        assert!(AiSolution::is_additive_number("123".into()));
    }

    #[test]
    fn test_leading_zero_invalid() {
        // "101" -> 1, 0, 1 -> valid (0 doesn't have leading zero issue)
        assert!(MySolution::is_additive_number("101".into()));
        assert!(AiSolution::is_additive_number("101".into()));
    }

    #[test]
    fn test_longer_sequence() {
        // 1, 1, 2, 3, 5, 8, 13
        assert!(MySolution::is_additive_number("112358".into()));
        assert!(AiSolution::is_additive_number("112358".into()));
    }

    #[test]
    fn test_large_numbers() {
        // 99, 1, 100, 101, 201
        assert!(MySolution::is_additive_number("991100101201".into()));
        assert!(AiSolution::is_additive_number("991100101201".into()));
    }
}

fn main() {
    println!("LeetCode 306: Additive Number");

    let num = "112358".to_string();
    println!(
        "Input: \"{num}\", My: {}, AI: {}",
        my_solution::Solution::is_additive_number(num.clone()),
        ai_solution::Solution::is_additive_number(num.clone())
    );

    // Benchmark
    let iterations = 100_000;
    let test_num = "199100199".to_string();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::Solution::is_additive_number(
            std::hint::black_box(test_num.clone()),
        ));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::Solution::is_additive_number(
            std::hint::black_box(test_num.clone()),
        ));
    }
    let ai_time = start.elapsed();

    let my_avg = my_time.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_time.as_nanos() as f64 / iterations as f64 / 1000.0;

    println!("\nBenchmark ({iterations} iterations):");
    println!("  My solution:  {my_avg:.3}us avg");
    println!("  AI solution:  {ai_avg:.3}us avg");
    if my_avg < ai_avg {
        println!("  Winner: My solution ({:.2}x faster)", ai_avg / my_avg);
    } else {
        println!("  Winner: AI solution ({:.2}x faster)", my_avg / ai_avg);
    }
}
