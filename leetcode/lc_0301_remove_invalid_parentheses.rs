// LeetCode 301: Remove Invalid Parentheses [Hard]
// https://leetcode.com/problems/remove-invalid-parentheses/
// Given a string s with parentheses and letters, remove the minimum number of
// invalid parentheses to make the input string valid. Return all unique valid strings.
//
// Benchmark: 100_000 iterations
// My solution (DFS + pruning):  avg 0.580us
// AI solution (BFS level-by-level): avg 2.988us
// Winner: My solution (5.16x faster)

// ------ My Solution: DFS with pruning ------
// Time: O(2^n) worst case — try skip/keep for each parenthesis
// Space: O(n) — recursion depth + string building
/// Count minimum removals needed (l unmatched '(', r unmatched ')'), then
/// DFS tries skipping each removable parenthesis. Prunes when remaining
/// chars can't satisfy removal quota or when `rcnt > lcnt` (invalid prefix).
mod my_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
            let chars: Vec<char> = s.chars().collect();
            let n = chars.len();
            // Count unmatched '(' (l) and ')' (r) that must be removed
            let (mut l, mut r) = (0i32, 0i32);
            for &c in &chars {
                if c == '(' {
                    l += 1;
                } else if c == ')' {
                    if l > 0 {
                        l -= 1;
                    } else {
                        r += 1;
                    }
                }
            }

            let mut ans = HashSet::new();

            /// Recursive DFS exploring skip/keep decisions for each character.
            /// `l` = remaining '(' to remove, `r` = remaining ')' to remove,
            /// `lcnt`/`rcnt` = running count of '('/')' kept so far.
            fn dfs(
                chars: &[char],
                n: usize,
                i: usize,
                l: i32,
                r: i32,
                lcnt: i32,
                rcnt: i32,
                t: &mut String,
                ans: &mut HashSet<String>,
            ) {
                if i == n {
                    if l == 0 && r == 0 {
                        ans.insert(t.clone());
                    }
                    return;
                }
                // Prune: not enough chars left to remove, or invalid prefix
                if (n - i) < (l + r) as usize || rcnt > lcnt {
                    return;
                }
                let c = chars[i];
                // Option 1: skip this '(' if we still need to remove some
                if c == '(' && l > 0 {
                    dfs(chars, n, i + 1, l - 1, r, lcnt, rcnt, t, ans);
                }
                // Option 2: skip this ')' if we still need to remove some
                if c == ')' && r > 0 {
                    dfs(chars, n, i + 1, l, r - 1, lcnt, rcnt, t, ans);
                }
                // Option 3: keep this character
                let x = if c == '(' { 1 } else { 0 };
                let y = if c == ')' { 1 } else { 0 };
                t.push(c);
                dfs(chars, n, i + 1, l, r, lcnt + x, rcnt + y, t, ans);
                t.pop();
            }

            let mut t = String::new();
            dfs(&chars, n, 0, l, r, 0, 0, &mut t, &mut ans);
            ans.into_iter().collect()
        }
    }
}

// ------ AI Solution: BFS level-by-level removal ------
// Time: O(n * 2^n) worst case — BFS generates substrings by removing one char
// Space: O(2^n) — visited set of generated strings
/// BFS starting from the original string. At each level, try removing one
/// parenthesis at every position. If any valid string is found at the current
/// level, collect all valid strings at that level and stop (minimum removals).
mod ai_solution {
    use std::collections::{HashSet, VecDeque};

    pub struct Solution;

    impl Solution {
        pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
            /// Check if a string has valid (balanced) parentheses.
            fn is_valid(s: &str) -> bool {
                let mut count = 0i32;
                for c in s.chars() {
                    if c == '(' {
                        count += 1;
                    } else if c == ')' {
                        count -= 1;
                        if count < 0 {
                            return false;
                        }
                    }
                }
                count == 0
            }

            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            let mut ans = Vec::new();
            let mut found = false;

            visited.insert(s.clone());
            queue.push_back(s);

            while let Some(curr) = queue.pop_front() {
                if is_valid(&curr) {
                    ans.push(curr.clone());
                    found = true;
                }
                if found {
                    continue;
                }
                // Try removing each parenthesis character
                let chars: Vec<char> = curr.chars().collect();
                for i in 0..chars.len() {
                    if chars[i] != '(' && chars[i] != ')' {
                        continue;
                    }
                    let next: String = chars[..i].iter().chain(chars[i + 1..].iter()).collect();
                    if visited.insert(next.clone()) {
                        queue.push_back(next);
                    }
                }
            }
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn test_example1() {
        let s = "()())()".to_string();
        let expected = sorted(vec!["(())()".into(), "()()()".into()]);
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_example2() {
        let s = "(a)())()".to_string();
        let expected = sorted(vec!["(a())()".into(), "(a)()()".into()]);
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_example3() {
        let s = ")(".to_string();
        let expected = vec!["".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_already_valid() {
        let s = "(a)(b)".to_string();
        let expected = vec!["(a)(b)".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_all_letters() {
        let s = "abc".to_string();
        let expected = vec!["abc".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_single_open() {
        let s = "(".to_string();
        let expected = vec!["".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_single_close() {
        let s = ")".to_string();
        let expected = vec!["".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_nested() {
        let s = "((a))".to_string();
        let expected = vec!["((a))".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_extra_open() {
        let s = "(a(b)".to_string();
        // Remove either '(' -> "a(b)" or "(ab)" — wait, "(ab)" isn't valid pairing
        // "(a(b)" -> remove first '(' -> "a(b)" or remove second '(' -> "(ab)"
        // "(ab)" has balanced parens: yes. "a(b)" also balanced.
        let expected = sorted(vec!["(ab)".into(), "a(b)".into()]);
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_multiple_removals() {
        let s = ")((".to_string();
        let expected = vec!["".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }

    #[test]
    fn test_empty() {
        // s.length >= 1 per constraints, but empty should work
        let s = "".to_string();
        let expected = vec!["".to_string()];
        assert_eq!(
            sorted(MySolution::remove_invalid_parentheses(s.clone())),
            expected
        );
        assert_eq!(sorted(AiSolution::remove_invalid_parentheses(s)), expected);
    }
}

fn main() {
    println!("LeetCode 301: Remove Invalid Parentheses");

    let s = "()())()".to_string();
    println!("Input: \"{s}\"");
    println!(
        "  My: {:?}",
        my_solution::Solution::remove_invalid_parentheses(s.clone())
    );
    println!(
        "  AI: {:?}",
        ai_solution::Solution::remove_invalid_parentheses(s)
    );

    // Benchmark
    let iterations = 100_000;
    let test_s = "()())()".to_string();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::Solution::remove_invalid_parentheses(
            std::hint::black_box(test_s.clone()),
        ));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::Solution::remove_invalid_parentheses(
            std::hint::black_box(test_s.clone()),
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
