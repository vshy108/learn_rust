// LeetCode 227: Basic Calculator II [Medium]
// https://leetcode.com/problems/basic-calculator-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.252us
//   AI solution avg: 0.085us
//   Ratio: AI 2.95x faster
//
// Given a string s which represents an expression, evaluate this
// expression and return its value. The integer division should truncate
// toward zero. You may assume the given expression is always valid.
// All intermediate results will be in the range of [-2^31, 2^31 - 1].
//
// The expression contains only non-negative integers, +, -, *, /
// operators, and empty spaces. No parentheses.
//
// Constraints: 1 <= s.length <= 3 * 10^5
//
// Run:  cargo run --example lc_0227_basic_calculator_ii
// Test: cargo test --example lc_0227_basic_calculator_ii
// Bench: cargo test --example lc_0227_basic_calculator_ii bench -- --nocapture

/// My solution — stack-based, two-pass in spirit.
///
/// Scan left to right, building numbers. When we encounter an operator
/// (or reach the end), apply the *previous* operator:
///   '+'/'-': push +num or -num onto the stack
///   '*'/'/': pop the top, combine with current num, push result
/// The final answer is the sum of the stack.
///
/// Time: O(n) — single pass plus stack sum.
/// Space: O(n) — stack in the worst case (all +/- operators).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate the expression in `s`.
        pub fn calculate(s: String) -> i32 {
            let bytes = s.as_bytes();
            let mut stack: Vec<i32> = Vec::new();
            let mut num: i32 = 0;
            // The operator that precedes the current number; starts as '+'
            // because the first number is implicitly positive.
            let mut prev_op: u8 = b'+';

            for i in 0..bytes.len() {
                let ch = bytes[i];

                if ch.is_ascii_digit() {
                    num = num * 10 + (ch - b'0') as i32;
                }

                // Process when we hit an operator or the end of the string.
                // Skip spaces that are not at the end.
                if (ch == b'+' || ch == b'-' || ch == b'*' || ch == b'/') || i == bytes.len() - 1 {
                    match prev_op {
                        b'+' => stack.push(num),
                        b'-' => stack.push(-num),
                        b'*' => {
                            let top = stack.pop().expect("stack non-empty before *");
                            stack.push(top * num);
                        }
                        b'/' => {
                            let top = stack.pop().expect("stack non-empty before /");
                            // Integer division truncating toward zero (Rust's
                            // default for i32 division)
                            stack.push(top / num);
                        }
                        _ => {}
                    }
                    prev_op = ch;
                    num = 0;
                }
            }

            stack.iter().sum()
        }
    }
}

/// AI solution — O(1) space, no stack.
///
/// Instead of a stack, keep two running accumulators:
///   `result` — the sum of all fully-resolved terms so far
///   `last`   — the most recent term (might still be modified by * or /)
///
/// On '+'/'-', flush `last` into `result` and start a new term.
/// On '*'/'/', update `last` in place.
///
/// Time: O(n) — single pass.
/// Space: O(1) — only three integer variables.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate the expression in `s`.
        pub fn calculate(s: String) -> i32 {
            let bytes = s.as_bytes();
            let mut result: i32 = 0;
            let mut last: i32 = 0; // the current term (may be updated by * /)
            let mut num: i32 = 0;
            let mut prev_op: u8 = b'+';

            for i in 0..bytes.len() {
                let ch = bytes[i];

                if ch.is_ascii_digit() {
                    num = num * 10 + (ch - b'0') as i32;
                }

                if (ch == b'+' || ch == b'-' || ch == b'*' || ch == b'/') || i == bytes.len() - 1 {
                    match prev_op {
                        b'+' => {
                            // Flush previous term, start a new positive term
                            result += last;
                            last = num;
                        }
                        b'-' => {
                            result += last;
                            last = -num;
                        }
                        b'*' => {
                            // Extend the current term
                            last *= num;
                        }
                        b'/' => {
                            last /= num;
                        }
                        _ => {}
                    }
                    prev_op = ch;
                    num = 0;
                }
            }

            // Flush the final term
            result + last
        }
    }
}

fn main() {
    let cases = [("3+2*2", 7), (" 3/2 ", 1), (" 3+5 / 2 ", 5)];
    for (expr, expected) in &cases {
        let my = my_solution::Solution::calculate(expr.to_string());
        let ai = ai_solution::Solution::calculate(expr.to_string());
        println!("{expr:20} => my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_calc {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::calculate("3+2*2".into()), 7);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::calculate(" 3/2 ".into()), 1);
                }

                #[test]
                fn test_example3() {
                    assert_eq!(<$solution>::calculate(" 3+5 / 2 ".into()), 5);
                }

                #[test]
                fn test_single_number() {
                    assert_eq!(<$solution>::calculate("42".into()), 42);
                }

                #[test]
                fn test_subtraction() {
                    assert_eq!(<$solution>::calculate("10-3-2".into()), 5);
                }

                #[test]
                fn test_multiply_then_add() {
                    // 2*3+4 = 6+4 = 10
                    assert_eq!(<$solution>::calculate("2*3+4".into()), 10);
                }

                #[test]
                fn test_division_truncates_toward_zero() {
                    // 14/3 = 4 (truncated)
                    assert_eq!(<$solution>::calculate("14/3".into()), 4);
                }

                #[test]
                fn test_complex() {
                    // 1+2*3-4/2 = 1+6-2 = 5
                    assert_eq!(<$solution>::calculate("1+2*3-4/2".into()), 5);
                }
            }
        };
    }

    test_calc!(my, crate::my_solution::Solution);
    test_calc!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let expr = "1+2*3-4/2+10*5-3+7/2*4-1+100".to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::calculate(
                std::hint::black_box(expr.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::calculate(
                std::hint::black_box(expr.clone()),
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
