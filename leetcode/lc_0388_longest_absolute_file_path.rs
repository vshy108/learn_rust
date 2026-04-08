// LeetCode 388: Longest Absolute File Path [Medium]
// https://leetcode.com/problems/longest-absolute-file-path/
//
// Benchmark: 100,000 iterations (2025-04-08)
//   My solution (stack + split):       avg 556ns
//   AI solution (stack + byte walk):   avg 382ns
//   -> AI solution is 1.46x faster
//
// Suppose we have a file system represented as a string where '\n' and '\t'
// are newline and tab characters. Each file/directory has a unique absolute
// path. A file is distinguished by having a '.' in its name.
//
// Given a string input representing the file system, return the length of
// the longest absolute path to a **file**. If no file exists, return 0.
//
// Examples:
//   "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext" -> 20
//     ("dir/subdir2/file.ext")
//   "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext" -> 32
//     ("dir/subdir2/subsubdir2/file2.ext")
//   "a" -> 0 (no file, just a directory)
//
// Constraints:
//   - 1 <= input.length <= 10^4
//   - input may contain lowercase/uppercase English letters, '\n', '\t',
//     '.', ' ', and digits
//   - All file and directory names have positive length
//
// Run:  cargo run --example lc_0388_longest_absolute_file_path
// Test: cargo test --example lc_0388_longest_absolute_file_path
// Bench: cargo test --example lc_0388_longest_absolute_file_path bench -- --nocapture

/// My solution module — stack-based simulation.
///
/// Split input by '\n'. For each line, count leading '\t' to determine
/// depth. Maintain a stack of cumulative path lengths. Pop until stack
/// depth matches the current depth, then push the new cumulative length.
/// If the entry is a file (contains '.'), update the answer.
///
/// Time:  O(n) where n = input.len()
/// Space: O(d) where d = max depth
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the longest absolute path to a file.
        ///
        /// # Arguments
        ///
        /// * `input` - The file system string
        ///
        /// # Returns
        ///
        /// Length of the longest file path, or 0 if no file exists
        pub fn length_longest_path(input: String) -> i32 {
            let mut ans = 0i32;
            // Stack stores cumulative path lengths at each depth
            let mut stack: Vec<i32> = Vec::new();

            for line in input.split('\n') {
                let bytes = line.as_bytes();
                // Count leading tabs = depth level
                let depth = bytes.iter().take_while(|&&b| b == b'\t').count();
                let name = &line[depth..]; // strip leading tabs
                let name_len = name.len() as i32;
                let is_file = name.contains('.');

                // Pop back to parent depth
                while stack.len() > depth {
                    stack.pop();
                }

                // Cumulative length = parent length + '/' separator + name
                let cum_len = if let Some(&parent) = stack.last() {
                    parent + 1 + name_len // +1 for the '/' separator
                } else {
                    name_len // root level, no separator
                };

                if is_file {
                    ans = ans.max(cum_len);
                } else {
                    stack.push(cum_len);
                }
            }
            ans
        }
    }
}

/// AI solution module — index-based single pass (no split allocation).
///
/// Walk the input byte-by-byte, counting '\t' for depth and characters
/// for name length, detecting files by '.'. Uses a Vec as a stack of
/// cumulative lengths. Avoids allocating substrings via split().
///
/// Time:  O(n)
/// Space: O(d)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the length of the longest absolute path to a file.
        ///
        /// # Arguments
        ///
        /// * `input` - The file system string
        ///
        /// # Returns
        ///
        /// Length of the longest file path, or 0 if no file exists
        pub fn length_longest_path(input: String) -> i32 {
            let bytes = input.as_bytes();
            let n = bytes.len();
            let mut i = 0;
            let mut ans = 0i32;
            let mut stack: Vec<i32> = Vec::new();

            while i < n {
                // Count tabs to determine depth
                let mut depth = 0;
                while i < n && bytes[i] == b'\t' {
                    depth += 1;
                    i += 1;
                }

                // Measure name length and check for '.' (file indicator)
                let mut name_len = 0i32;
                let mut is_file = false;
                while i < n && bytes[i] != b'\n' {
                    if bytes[i] == b'.' {
                        is_file = true;
                    }
                    name_len += 1;
                    i += 1;
                }
                i += 1; // skip '\n'

                // Pop to parent depth
                while stack.len() > depth {
                    stack.pop();
                }

                let cum_len = if let Some(&parent) = stack.last() {
                    parent + 1 + name_len
                } else {
                    name_len
                };

                if is_file {
                    ans = ans.max(cum_len);
                } else {
                    stack.push(cum_len);
                }
            }
            ans
        }
    }
}

fn main() {
    let cases = [
        ("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext", 20),
        (
            "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext",
            32,
        ),
        ("a", 0),
    ];

    println!("--- My Solution ---");
    for (input, expected) in &cases {
        let result = my_solution::Solution::length_longest_path(input.to_string());
        println!("  -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (input, expected) in &cases {
        let result = ai_solution::Solution::length_longest_path(input.to_string());
        println!("  -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_longest_path {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::length_longest_path(
                            "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"
                                .to_string()
                        ),
                        20
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::length_longest_path(
                            "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"
                                .to_string()
                        ),
                        32
                    );
                }

                #[test]
                fn test_no_file() {
                    assert_eq!(
                        Solution::length_longest_path("a".to_string()),
                        0
                    );
                }

                #[test]
                fn test_single_file() {
                    assert_eq!(
                        Solution::length_longest_path("file.txt".to_string()),
                        8
                    );
                }

                #[test]
                fn test_nested_no_file() {
                    assert_eq!(
                        Solution::length_longest_path(
                            "dir\n\tsubdir1\n\tsubdir2".to_string()
                        ),
                        0
                    );
                }

                #[test]
                fn test_multiple_files() {
                    // dir/a.txt = 9, dir/sub/b.txt = 13
                    assert_eq!(
                        Solution::length_longest_path(
                            "dir\n\ta.txt\n\tsub\n\t\tb.txt".to_string()
                        ),
                        13
                    );
                }
            }
        };
    }

    test_longest_path!(my_tests, crate::my_solution::Solution);
    test_longest_path!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        // Build a moderately deep file system string
        let mut input = String::from("root");
        for i in 0..20 {
            input.push('\n');
            for _ in 0..=i {
                input.push('\t');
            }
            input.push_str(&format!("dir{i}"));
        }
        input.push_str("\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tfile.ext");

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::length_longest_path(
                std::hint::black_box(input.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::length_longest_path(
                std::hint::black_box(input.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
