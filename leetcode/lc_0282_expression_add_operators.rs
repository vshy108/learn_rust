// LeetCode 282: Expression Add Operators [Hard]
// https://leetcode.com/problems/expression-add-operators/
//
// First benchmark: 1_000 iterations, 2025-07-07
//   My solution avg: 88.365us
//   AI solution avg: 86.416us
//   Ratio: AI 1.02x faster
//
// Given a string `num` of digits and an integer `target`, return all
// possibilities to insert '+', '-', '*' between digits so the expression
// evaluates to target. No leading zeros in operands.
//
// Example 1: num = "123", target = 6 -> ["1*2*3","1+2+3"]
// Example 2: num = "232", target = 8 -> ["2*3+2","2+3*2"]
// Example 3: num = "3456237490", target = 9191 -> []
//
// Constraints:
//   - 1 <= num.length <= 10
//   - num consists of only digits
//   - -2^31 <= target <= 2^31 - 1
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0282.Expression%20Add%20Operators/README_EN.md
//
// Run:  cargo run --example lc_0282_expression_add_operators
// Test: cargo test --example lc_0282_expression_add_operators
// Bench: cargo test --example lc_0282_expression_add_operators bench -- --nocapture

/// My solution — DFS with prev tracking (from doocs Python/Java).
///
/// Backtrack over all positions to split the digit string. Track `curr`
/// (running evaluation) and `prev` (last operand, needed to undo addition
/// for multiplication: curr - prev + prev * next). Skip numbers with
/// leading zeros.
///
/// Time:  O(4^n * n) — each gap has 4 choices (nothing, +, -, *)
/// Space: O(n) recursion depth + O(4^n) for results
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn add_operators(num: String, target: i32) -> Vec<String> {
            let digits = num.as_bytes();
            let target = target as i64;
            let mut ans = Vec::new();
            let mut path = String::new();
            Self::dfs(digits, target, 0, 0, 0, &mut path, &mut ans);
            ans
        }

        fn dfs(
            digits: &[u8],
            target: i64,
            pos: usize,
            prev: i64,
            curr: i64,
            path: &mut String,
            ans: &mut Vec<String>,
        ) {
            if pos == digits.len() {
                if curr == target {
                    ans.push(path.clone());
                }
                return;
            }

            let mut num_val: i64 = 0;
            let path_len = path.len();

            for i in pos..digits.len() {
                // Skip numbers with leading zeros (e.g., "05")
                if i != pos && digits[pos] == b'0' {
                    break;
                }
                num_val = num_val * 10 + (digits[i] - b'0') as i64;

                if pos == 0 {
                    // First number: no operator needed
                    path.push_str(&num_val.to_string());
                    Self::dfs(digits, target, i + 1, num_val, num_val, path, ans);
                    path.truncate(path_len);
                } else {
                    let num_str = num_val.to_string();

                    // Try '+'
                    path.push('+');
                    path.push_str(&num_str);
                    Self::dfs(
                        digits,
                        target,
                        i + 1,
                        num_val,
                        curr + num_val,
                        path,
                        ans,
                    );
                    path.truncate(path_len);

                    // Try '-'
                    path.push('-');
                    path.push_str(&num_str);
                    Self::dfs(
                        digits,
                        target,
                        i + 1,
                        -num_val,
                        curr - num_val,
                        path,
                        ans,
                    );
                    path.truncate(path_len);

                    // Try '*' — undo the last operand, apply multiplication
                    path.push('*');
                    path.push_str(&num_str);
                    Self::dfs(
                        digits,
                        target,
                        i + 1,
                        prev * num_val,
                        curr - prev + prev * num_val,
                        path,
                        ans,
                    );
                    path.truncate(path_len);
                }
            }
        }
    }
}

/// AI solution — Same DFS algorithm but using byte slices for path building.
///
/// Avoids String allocation during recursion by using a Vec<u8> buffer
/// for the path and converting to String only when a valid expression
/// is found. Slightly more efficient due to fewer heap allocations.
///
/// Time:  O(4^n * n)
/// Space: O(n) recursion + O(4^n) results
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn add_operators(num: String, target: i32) -> Vec<String> {
            let digits = num.as_bytes();
            let target = target as i64;
            let mut ans = Vec::new();
            let mut buf: Vec<u8> = Vec::with_capacity(digits.len() * 2);
            Self::dfs(digits, target, 0, 0, 0, &mut buf, &mut ans);
            ans
        }

        fn dfs(
            digits: &[u8],
            target: i64,
            pos: usize,
            prev: i64,
            curr: i64,
            buf: &mut Vec<u8>,
            ans: &mut Vec<String>,
        ) {
            if pos == digits.len() {
                if curr == target {
                    // Convert buffer to String only on success
                    ans.push(
                        String::from_utf8(buf.clone())
                            .expect("valid ASCII"),
                    );
                }
                return;
            }

            let mut num_val: i64 = 0;
            let buf_len = buf.len();

            for i in pos..digits.len() {
                if i != pos && digits[pos] == b'0' {
                    break;
                }
                num_val = num_val * 10 + (digits[i] - b'0') as i64;

                // Write the number digits into a small stack buffer
                let num_bytes = Self::i64_to_bytes(num_val);

                if pos == 0 {
                    buf.extend_from_slice(&num_bytes);
                    Self::dfs(
                        digits, target, i + 1, num_val, num_val, buf, ans,
                    );
                    buf.truncate(buf_len);
                } else {
                    // '+'
                    buf.push(b'+');
                    buf.extend_from_slice(&num_bytes);
                    Self::dfs(
                        digits,
                        target,
                        i + 1,
                        num_val,
                        curr + num_val,
                        buf,
                        ans,
                    );
                    buf.truncate(buf_len);

                    // '-'
                    buf.push(b'-');
                    buf.extend_from_slice(&num_bytes);
                    Self::dfs(
                        digits,
                        target,
                        i + 1,
                        -num_val,
                        curr - num_val,
                        buf,
                        ans,
                    );
                    buf.truncate(buf_len);

                    // '*'
                    buf.push(b'*');
                    buf.extend_from_slice(&num_bytes);
                    Self::dfs(
                        digits,
                        target,
                        i + 1,
                        prev * num_val,
                        curr - prev + prev * num_val,
                        buf,
                        ans,
                    );
                    buf.truncate(buf_len);
                }
            }
        }

        /// Convert a non-negative i64 to its ASCII byte representation.
        fn i64_to_bytes(mut n: i64) -> Vec<u8> {
            if n == 0 {
                return vec![b'0'];
            }
            let mut digits = Vec::new();
            while n > 0 {
                digits.push(b'0' + (n % 10) as u8);
                n /= 10;
            }
            digits.reverse();
            digits
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    macro_rules! test_expr {
        ($name:ident, $num:expr, $target:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let expected: HashSet<String> =
                    $expected.iter().map(|s| s.to_string()).collect();

                let my_res: HashSet<String> =
                    my_solution::Solution::add_operators(
                        $num.to_string(),
                        $target,
                    )
                    .into_iter()
                    .collect();
                assert_eq!(my_res, expected, "my_solution failed");

                let ai_res: HashSet<String> =
                    ai_solution::Solution::add_operators(
                        $num.to_string(),
                        $target,
                    )
                    .into_iter()
                    .collect();
                assert_eq!(ai_res, expected, "ai_solution failed");
            }
        };
    }

    // Example 1
    test_expr!(test_example_1, "123", 6, ["1*2*3", "1+2+3"]);
    // Example 2
    test_expr!(test_example_2, "232", 8, ["2*3+2", "2+3*2"]);
    // Example 3: no solution
    test_expr!(test_no_solution, "3456237490", 9191, Vec::<&str>::new());
    // Single digit equals target
    test_expr!(test_single_digit, "5", 5, ["5"]);
    // Single digit, no match
    test_expr!(test_single_no_match, "5", 3, Vec::<&str>::new());
    // Leading zeros
    test_expr!(test_leading_zeros, "105", 5, ["1*0+5", "10-5"]);
    // All same digits
    test_expr!(test_all_ones, "111", 3, ["1+1+1"]);
    // Zero target
    test_expr!(test_zero_target, "123", 0, ["1+2-3"]);
    // Two digits
    test_expr!(test_two_digits, "12", 3, ["1+2"]);
    // Multi-digit operand
    test_expr!(test_multi_digit, "123", 15, ["12+3"]);
    // Leading zeros: "00" with target 0
    test_expr!(test_double_zero, "00", 0, ["0+0", "0-0", "0*0"]);
}

fn main() {
    println!("LeetCode 282: Expression Add Operators");
    let num = "123".to_string();
    let target = 6;
    println!(
        "my_solution: add_operators({:?}, {}) = {:?}",
        num,
        target,
        my_solution::Solution::add_operators(num.clone(), target)
    );
    let ai_result =
        ai_solution::Solution::add_operators(num, target);
    println!("ai_solution: {:?}", ai_result);
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let num = "1234567".to_string();
        let target = 100;
        let iterations = 1_000; // fewer iterations — this is exponential

        // Warm up
        for _ in 0..10 {
            std::hint::black_box(my_solution::Solution::add_operators(
                num.clone(),
                target,
            ));
            std::hint::black_box(ai_solution::Solution::add_operators(
                num.clone(),
                target,
            ));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::add_operators(
                num.clone(),
                target,
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::add_operators(
                num.clone(),
                target,
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
