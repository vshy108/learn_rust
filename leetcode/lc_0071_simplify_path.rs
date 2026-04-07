// LeetCode 71: Simplify Path [Medium]
// https://leetcode.com/problems/simplify-path/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 1.907us
//   AI solution avg: 1.883us
//   Ratio: ~tied
//
// Given an absolute Unix-style file path starting with '/', return the
// simplified canonical path. Rules: ".." goes up one dir, "." stays,
// multiple slashes treated as one, trailing slash removed.
//
// Constraints: 1 <= path.length <= 3000
//
// Run:  cargo run --example lc_0071_simplify_path
// Test: cargo test --example lc_0071_simplify_path
// Bench: cargo test --example lc_0071_simplify_path bench -- --nocapture

/// My solution — split by '/' and use a stack.
///
/// Time: O(n) where n = path length.
/// Space: O(n) for the stack.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Simplify a Unix-style file path.
        ///
        /// # Arguments
        /// * `path` - absolute Unix path string
        ///
        /// # Returns
        /// Simplified canonical path
        pub fn simplify_path(path: String) -> String {
            let mut stack: Vec<&str> = Vec::new();
            for part in path.split('/') {
                match part {
                    "" | "." => {} // skip empty segments and current-dir
                    ".." => {
                        stack.pop(); // go up one directory
                    }
                    _ => stack.push(part),
                }
            }
            format!("/{}", stack.join("/"))
        }
    }
}

/// AI solution — same stack approach (canonical solution).
///
/// Time: O(n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Simplify a Unix-style file path.
        ///
        /// # Arguments
        /// * `path` - absolute Unix path string
        ///
        /// # Returns
        /// Simplified canonical path
        pub fn simplify_path(path: String) -> String {
            let mut stack = Vec::new();
            for part in path.split('/') {
                match part {
                    "" | "." => {}
                    ".." => { stack.pop(); }
                    dir => stack.push(dir),
                }
            }
            let mut result = String::with_capacity(path.len());
            result.push('/');
            result.push_str(&stack.join("/"));
            result
        }
    }
}

fn main() {
    let paths = ["/home/", "/home//foo/", "/home/user/Documents/../Pictures",
                 "/../", "/.../a/../b/c/../d/./"];
    for p in paths {
        let my = my_solution::Solution::simplify_path(p.to_string());
        let ai = ai_solution::Solution::simplify_path(p.to_string());
        println!("{p:40} => my: {my:20} ai: {ai}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_simplify_path {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_home() {
                    assert_eq!(
                        <$solution>::simplify_path("/home/".to_string()),
                        "/home"
                    );
                }

                #[test]
                fn test_double_slash() {
                    assert_eq!(
                        <$solution>::simplify_path("/home//foo/".to_string()),
                        "/home/foo"
                    );
                }

                #[test]
                fn test_dotdot() {
                    assert_eq!(
                        <$solution>::simplify_path(
                            "/home/user/Documents/../Pictures".to_string(),
                        ),
                        "/home/user/Pictures"
                    );
                }

                #[test]
                fn test_root_dotdot() {
                    assert_eq!(
                        <$solution>::simplify_path("/../".to_string()),
                        "/"
                    );
                }

                #[test]
                fn test_triple_dot() {
                    // "..." is a valid directory name
                    assert_eq!(
                        <$solution>::simplify_path("/.../a/../b/c/../d/./".to_string()),
                        "/.../b/d"
                    );
                }
            }
        };
    }

    test_simplify_path!(my, crate::my_solution::Solution);
    test_simplify_path!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let path = "/home/user/../user/Documents/./Projects///rust/../go/../../.hidden/./file.txt"
            .to_string();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::simplify_path(
                std::hint::black_box(path.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::simplify_path(
                std::hint::black_box(path.clone()),
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
