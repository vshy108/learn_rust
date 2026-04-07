// LeetCode 150: Evaluate Reverse Polish Notation [Medium]
// https://leetcode.com/problems/evaluate-reverse-polish-notation/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.988us
//   AI solution avg: 0.803us
//   Ratio: AI 1.23x faster
//
// You are given an array of strings tokens representing an arithmetic expression
// in Reverse Polish Notation. Evaluate it and return the result.
// Valid operators are +, -, *, /. Division truncates toward zero.
//
// Constraints: 1 <= tokens.length <= 10^4, each token is an operator or
//   integer in range [-200, 200]
//
// Run:  cargo run --example lc_0150_eval_rpn
// Test: cargo test --example lc_0150_eval_rpn
// Bench: cargo test --example lc_0150_eval_rpn bench -- --nocapture

/// My solution — standard stack-based evaluation.
///
/// Time: O(n).
/// Space: O(n) for the stack.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate the RPN expression.
        pub fn eval_rpn(tokens: Vec<String>) -> i32 {
            let mut stack: Vec<i32> = Vec::new();

            for token in &tokens {
                match token.as_str() {
                    "+" | "-" | "*" | "/" => {
                        let b = stack.pop().expect("valid RPN");
                        let a = stack.pop().expect("valid RPN");
                        let result = match token.as_str() {
                            "+" => a + b,
                            "-" => a - b,
                            "*" => a * b,
                            "/" => a / b, // Rust i32 division truncates toward zero
                            _ => unreachable!(),
                        };
                        stack.push(result);
                    }
                    num => {
                        stack.push(
                            num.parse::<i32>().expect("valid integer token"),
                        );
                    }
                }
            }
            stack.pop().expect("valid RPN has single result")
        }
    }
}

/// AI solution — same stack approach, slightly tighter with first-byte check.
///
/// Time: O(n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Evaluate RPN with first-byte operator detection.
        pub fn eval_rpn(tokens: Vec<String>) -> i32 {
            let mut stack = Vec::with_capacity(tokens.len() / 2 + 1);

            for token in &tokens {
                let bytes = token.as_bytes();
                // An operator is exactly 1 byte and one of + - * /
                // (negative numbers like "-11" have len > 1)
                if bytes.len() == 1 && matches!(bytes[0], b'+' | b'-' | b'*' | b'/') {
                    let b = stack.pop().expect("valid RPN");
                    let a: i32 = stack.pop().expect("valid RPN");
                    stack.push(match bytes[0] {
                        b'+' => a + b,
                        b'-' => a - b,
                        b'*' => a * b,
                        b'/' => a / b,
                        _ => unreachable!(),
                    });
                } else {
                    stack.push(
                        token.parse::<i32>().expect("valid integer"),
                    );
                }
            }
            stack[0]
        }
    }
}

fn main() {
    let tokens: Vec<String> = ["2", "1", "+", "3", "*"]
        .iter().map(|s| s.to_string()).collect();
    let my = my_solution::Solution::eval_rpn(tokens.clone());
    let ai = ai_solution::Solution::eval_rpn(tokens);
    println!("eval RPN: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    fn s(tokens: &[&str]) -> Vec<String> {
        tokens.iter().map(|t| t.to_string()).collect()
    }

    macro_rules! test_rpn {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::eval_rpn(s(&["2", "1", "+", "3", "*"])),
                        9,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        <$solution>::eval_rpn(s(&["4", "13", "5", "/", "+"])),
                        6,
                    );
                }

                #[test]
                fn test_example3() {
                    assert_eq!(
                        <$solution>::eval_rpn(s(&[
                            "10", "6", "9", "3", "+", "-11", "*",
                            "/", "*", "17", "+", "5", "+",
                        ])),
                        22,
                    );
                }

                #[test]
                fn test_single_number() {
                    assert_eq!(<$solution>::eval_rpn(s(&["42"])), 42);
                }

                #[test]
                fn test_negative_division() {
                    // -1 / 2 should truncate toward zero = 0
                    assert_eq!(
                        <$solution>::eval_rpn(s(&["-1", "2", "/"])),
                        0,
                    );
                }
            }
        };
    }

    test_rpn!(my, crate::my_solution::Solution);
    test_rpn!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let tokens = s(&[
            "10", "6", "9", "3", "+", "-11", "*",
            "/", "*", "17", "+", "5", "+",
        ]);

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::eval_rpn(
                std::hint::black_box(tokens.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::eval_rpn(
                std::hint::black_box(tokens.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
