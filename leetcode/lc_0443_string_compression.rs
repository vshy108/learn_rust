// LeetCode 443: String Compression [Medium]
// https://leetcode.com/problems/string-compression/
//
// Given an array of characters, compress it in-place. For each group of
// consecutive repeating characters, if length is 1 append just the char,
// otherwise append the char followed by the group length digits.
// Return the new length of the array.
//
// Examples:
//   ["a","a","b","b","c","c","c"] -> 6, chars = ["a","2","b","2","c","3"]
//   ["a"] -> 1
//   ["a","b","b","b","b","b","b","b","b","b","b","b","b"] -> 4, chars = ["a","b","1","2"]
//
// Constraints:
//   1 <= chars.length <= 2000
//   chars[i] is a lowercase/uppercase English letter, digit, or symbol
//
// Run:  cargo run --example lc_0443_string_compression
// Test: cargo test --example lc_0443_string_compression
// Bench: cargo test --example lc_0443_string_compression bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 115ns
//   AI solution: avg 114ns
//   Ratio: 1.01x (same algorithm)

/// My solution module — two-pointer in-place compression.
///
/// Walk through groups of consecutive equal characters.
/// Write the char and, if count > 1, write each digit of the count.
///
/// Time:  O(n)
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compress the character array in-place.
        pub fn compress(chars: &mut Vec<char>) -> i32 {
            // Two-pointer in-place compression
            let mut write = 0;
            let mut read = 0;
            while read < chars.len() {
                let ch = chars[read];
                let mut count = 0;
                while read < chars.len() && chars[read] == ch {
                    read += 1;
                    count += 1;
                }
                chars[write] = ch;
                write += 1;
                if count > 1 {
                    for c in count.to_string().chars() {
                        chars[write] = c;
                        write += 1;
                    }
                }
            }
            chars.truncate(write);
            write as i32
        }
    }
}

/// AI solution module — two-pointer with digit writing.
///
/// Use i (read) and k (write) pointers. For each group, write the char
/// at k, then if group length > 1, convert length to string and write digits.
///
/// Time:  O(n)
/// Space: O(1) auxiliary (count string is at most 4 chars for n <= 2000)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compress the character array in-place.
        pub fn compress(chars: &mut Vec<char>) -> i32 {
            let (mut i, mut k, n) = (0, 0, chars.len());
            while i < n {
                let mut j = i + 1;
                // Find end of this group of identical characters
                while j < n && chars[j] == chars[i] {
                    j += 1;
                }
                // Write the character
                chars[k] = chars[i];
                k += 1;
                // If group length > 1, write each digit of the count
                if j - i > 1 {
                    let cnt = (j - i).to_string();
                    for c in cnt.chars() {
                        chars[k] = c;
                        k += 1;
                    }
                }
                i = j;
            }
            k as i32
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_example1() {
            let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
            let len = Solution::compress(&mut chars) as usize;
            assert_eq!(len, 6);
            assert_eq!(&chars[..len], &['a', '2', 'b', '2', 'c', '3']);
        }

        #[test]
        fn test_single_char() {
            let mut chars = vec!['a'];
            let len = Solution::compress(&mut chars) as usize;
            assert_eq!(len, 1);
            assert_eq!(chars[0], 'a');
        }

        #[test]
        fn test_twelve_bs() {
            let mut chars = vec![
                'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
            ];
            let len = Solution::compress(&mut chars) as usize;
            assert_eq!(len, 4);
            assert_eq!(&chars[..len], &['a', 'b', '1', '2']);
        }

        #[test]
        fn test_no_repeats() {
            let mut chars = vec!['a', 'b', 'c'];
            let len = Solution::compress(&mut chars) as usize;
            assert_eq!(len, 3);
            assert_eq!(&chars[..len], &['a', 'b', 'c']);
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
    F: Fn(&mut Vec<char>) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        std::hint::black_box(f(&mut chars));
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
    bench_solution("My solution", my_solution::Solution::compress);
    bench_solution("AI solution", ai_solution::Solution::compress);
}

fn main() {
    let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let len = ai_solution::Solution::compress(&mut chars);
    println!(
        "Compressed length: {}, chars: {:?}",
        len,
        &chars[..len as usize]
    );
}
