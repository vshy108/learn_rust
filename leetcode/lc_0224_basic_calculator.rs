// LeetCode 224: Basic Calculator [Hard]
// https://leetcode.com/problems/basic-calculator/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.078us
//   AI solution avg: 0.104us
//   Ratio: My 1.33x faster
//
// Given a string s representing a valid expression, implement a basic
// calculator to evaluate it, and return the result of the evaluation.
//
// The expression string may contain open ( and closing ) parentheses,
// the plus + or minus - sign, non-negative integers, and empty spaces.
//
// Note: s is a valid expression. '+' is not used as a unary operation
// (i.e., "+1" and "+(2 + 3)" is invalid). '-' could be used as a unary
// operation (i.e., "-1" and "-(2 + 3)" is valid). There will be no two
// consecutive operators. Every number and running calculation will fit
// in a signed 32-bit integer.
//
// Constraints: 1 <= s.length <= 3 * 10^5, s consists of digits, '+',
// '-', '(', ')', and ' '.
//
// Run:  cargo run --example lc_0224_basic_calculator
// Test: cargo test --example lc_0224_basic_calculator
// Bench: cargo test --example lc_0224_basic_calculator bench -- --nocapture

/// My solution — recursive descent parser.
///
/// Parse the expression recursively: when we encounter '(' we recurse,
/// when we encounter ')' we return. Track current number, running result,
/// and sign. Each recursive call handles one level of parentheses.
///
/// Time: O(n) — each character visited once.
/// Space: O(d) where d is the maximum nesting depth of parentheses.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate the arithmetic expression in `s`.
        pub fn calculate(s: String) -> i32 {
            let bytes = s.as_bytes();
            // `pos` tracks our cursor through the string; shared across
            // recursive calls via mutable reference
            let mut pos = 0;
            Self::eval(bytes, &mut pos)
        }

        /// Recursive evaluator: processes characters starting at `pos`,
        /// returns the value of the (sub-)expression, and advances `pos`
        /// past the consumed characters (including any closing ')').
        fn eval(bytes: &[u8], pos: &mut usize) -> i32 {
            let mut result: i32 = 0;
            let mut num: i32 = 0;
            let mut sign: i32 = 1;
            let mut has_num = false;

            while *pos < bytes.len() {
                let ch = bytes[*pos];
                *pos += 1;

                match ch {
                    b'0'..=b'9' => {
                        // Build the current number digit by digit
                        num = num * 10 + (ch - b'0') as i32;
                        has_num = true;
                    }
                    b'+' => {
                        // Flush accumulated number, set sign for next operand
                        if has_num {
                            result += sign * num;
                            num = 0;
                            has_num = false;
                        }
                        sign = 1;
                    }
                    b'-' => {
                        if has_num {
                            result += sign * num;
                            num = 0;
                            has_num = false;
                        }
                        sign = -1;
                    }
                    b'(' => {
                        // Recurse into the sub-expression; the recursive
                        // call will advance `pos` past the matching ')'
                        let sub = Self::eval(bytes, pos);
                        result += sign * sub;
                        sign = 1;
                    }
                    b')' => {
                        // End of this sub-expression; flush and return
                        if has_num {
                            result += sign * num;
                        }
                        return result;
                    }
                    // Skip spaces and any other characters
                    _ => {}
                }
            }

            // End of string; flush any trailing number
            if has_num {
                result += sign * num;
            }
            result
        }
    }
}

/// AI solution — iterative stack-based approach.
///
/// Use a stack to store the result-so-far and the sign whenever we enter
/// a new parenthesis level. On '(' push (result, sign) and reset.
/// On ')' pop and combine. No recursion needed.
///
/// Time: O(n) — single pass.
/// Space: O(d) — stack depth proportional to nesting depth.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate the arithmetic expression in `s`.
        pub fn calculate(s: String) -> i32 {
            let mut result: i32 = 0;
            let mut num: i32 = 0;
            let mut sign: i32 = 1;
            // Each entry: (result_before_paren, sign_before_paren)
            let mut stack: Vec<(i32, i32)> = Vec::new();

            for &ch in s.as_bytes() {
                match ch {
                    b'0'..=b'9' => {
                        num = num * 10 + (ch - b'0') as i32;
                    }
                    b'+' => {
                        // Flush current number
                        result += sign * num;
                        num = 0;
                        sign = 1;
                    }
                    b'-' => {
                        result += sign * num;
                        num = 0;
                        sign = -1;
                    }
                    b'(' => {
                        // Push context: the accumulated result and the sign
                        // that precedes this parenthesized sub-expression
                        stack.push((result, sign));
                        // Reset for the sub-expression
                        result = 0;
                        sign = 1;
                    }
                    b')' => {
                        // Flush the last number inside parens
                        result += sign * num;
                        num = 0;
                        // Pop the outer context and combine:
                        //   outer_result + outer_sign * inner_result
                        let (outer_result, outer_sign) = stack
                            .pop()
                            .expect("Unbalanced parentheses — should not happen with valid input");
                        result = outer_result + outer_sign * result;
                        // Reset sign; the next operator will set it
                        sign = 1;
                    }
                    _ => {} // skip spaces
                }
            }

            // Flush trailing number (not followed by operator or ')')
            result + sign * num
        }
    }
}

fn main() {
    let cases = [("1 + 1", 2), (" 2-1 + 2 ", 3), ("(1+(4+5+2)-3)+(6+8)", 23)];

    for (expr, expected) in &cases {
        let my = my_solution::Solution::calculate(expr.to_string());
        let ai = ai_solution::Solution::calculate(expr.to_string());
        println!("{expr:30} => my={my}, ai={ai} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_calc {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::calculate("1 + 1".into()), 2);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::calculate(" 2-1 + 2 ".into()), 3);
                }

                #[test]
                fn test_example3() {
                    assert_eq!(<$solution>::calculate("(1+(4+5+2)-3)+(6+8)".into()), 23,);
                }

                #[test]
                fn test_single_number() {
                    assert_eq!(<$solution>::calculate("42".into()), 42);
                }

                #[test]
                fn test_unary_minus() {
                    // "-1" is valid per the problem
                    assert_eq!(<$solution>::calculate("-1".into()), -1);
                }

                #[test]
                fn test_nested_parens() {
                    assert_eq!(<$solution>::calculate("((1+2)+(3+4))".into()), 10,);
                }

                #[test]
                fn test_minus_in_parens() {
                    // "-(3+2)" should evaluate to -5
                    assert_eq!(<$solution>::calculate("1-(3+2)".into()), -4,);
                }

                #[test]
                fn test_complex_nesting() {
                    assert_eq!(<$solution>::calculate("1-(-(2+3))".into()), 6,);
                }
            }
        };
    }

    test_calc!(my, crate::my_solution::Solution);
    test_calc!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let expr = "(1+(4+5+2)-3)+(6+8)-(1-(2+3))+(10-20+30)".to_string();

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
