// LeetCode 455: Assign Cookies [Easy]
// https://leetcode.com/problems/assign-cookies/
//
// Each child i has a greed factor g[i]. Each cookie j has size s[j].
// If s[j] >= g[i], cookie j can satisfy child i. Maximize content children.
//
// Examples:
//   g = [1,2,3], s = [1,1] -> 1
//   g = [1,2], s = [1,2,3] -> 2
//
// Constraints:
//   1 <= g.length <= 3 * 10^4
//   0 <= s.length <= 3 * 10^4
//   1 <= g[i], s[j] <= 2^31 - 1
//
// Run:  cargo run --example lc_0455_assign_cookies
// Test: cargo test --example lc_0455_assign_cookies
// Bench: cargo test --example lc_0455_assign_cookies bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 63ns
//   AI solution: avg 62ns
//   Ratio: 1.02x (same algorithm)

/// My solution module — sort + two pointers.
///
/// Time:  O(m log m + n log n)
/// Space: O(log m + log n) for sorting

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
            // Sort both, greedily assign smallest cookie that satisfies
            g.sort_unstable();
            s.sort_unstable();
            let (mut gi, mut si) = (0, 0);
            while gi < g.len() && si < s.len() {
                if s[si] >= g[gi] {
                    gi += 1;
                }
                si += 1;
            }
            gi as i32
        }
    }
}

/// AI solution module — sort both, greedily assign smallest sufficient cookie.
///
/// Time:  O(m log m + n log n)
/// Space: O(log m + log n)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
            g.sort_unstable();
            s.sort_unstable();
            let (m, n) = (g.len(), s.len());
            let mut j = 0;
            for i in 0..m {
                // Find smallest cookie that satisfies child i
                while j < n && s[j] < g[i] {
                    j += 1;
                }
                if j >= n {
                    return i as i32;
                }
                j += 1; // Use this cookie
            }
            m as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_example1() {
            assert_eq!(
                Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
                1
            );
        }
        #[test]
        fn test_example2() {
            assert_eq!(
                Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
                2
            );
        }
        #[test]
        fn test_no_cookies() {
            assert_eq!(Solution::find_content_children(vec![1, 2], vec![]), 0);
        }
        #[test]
        fn test_all_satisfied() {
            assert_eq!(Solution::find_content_children(vec![1, 1], vec![5, 5]), 2);
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
    F: Fn(Vec<i32>, Vec<i32>) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(vec![1, 2, 3]),
            std::hint::black_box(vec![1, 1]),
        ));
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
    bench_solution("My solution", my_solution::Solution::find_content_children);
    bench_solution("AI solution", ai_solution::Solution::find_content_children);
}

fn main() {
    println!(
        "{}",
        ai_solution::Solution::find_content_children(vec![1, 2, 3], vec![1, 1])
    );
}
