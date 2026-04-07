// LeetCode 157: Read N Characters Given Read4 [Easy] (Premium)
// https://leetcode.com/problems/read-n-characters-given-read4/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.157us
//   AI solution avg: 0.158us
//   Ratio: ~tied
//
// Given a file and assume you can only read it using read4, implement
// a method read to read n characters.
//
// read4 reads 4 consecutive characters from file into buf4, returns
// the number of actual characters read.
//
// read reads n characters from file into buf, returns actual chars read.
// The read function will only be called once per test case.
//
// Example 1: file = "abc", n = 4 -> 3 (buf = "abc")
// Example 2: file = "abcde", n = 5 -> 5 (buf = "abcde")
// Example 3: file = "abcdABCD1234", n = 12 -> 12
//
// Constraints:
//   - 1 <= file.length <= 500
//   - file consists of English letters and digits
//   - 1 <= n <= 1000
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0157.Read%20N%20Characters%20Given%20Read4/README_EN.md
//
// Run:  cargo run --example lc_0157_read_n_characters_given_read4
// Test: cargo test --example lc_0157_read_n_characters_given_read4
// Bench: cargo test --example lc_0157_read_n_characters_given_read4 bench -- --nocapture

/// Simulated read4 API — reads up to 4 chars from the file at current position.
///
/// In real LeetCode, this is provided by the platform. We simulate it
/// using a struct that tracks the file pointer position.
struct FileReader {
    data: Vec<u8>,
    pos: usize,
}

impl FileReader {
    /// Create a new file reader with the given content.
    fn new(content: &str) -> Self {
        Self {
            data: content.bytes().collect(),
            pos: 0,
        }
    }

    /// Read up to 4 characters into buf4; returns number actually read.
    fn read4(&mut self, buf4: &mut [u8; 4]) -> usize {
        let mut count = 0;
        while count < 4 && self.pos < self.data.len() {
            buf4[count] = self.data[self.pos];
            self.pos += 1;
            count += 1;
        }
        count
    }
}

/// My solution — loop calling read4 (translated from Python in doocs/leetcode).
///
/// Repeatedly call read4, copying characters into buf until we've read
/// n characters or the file is exhausted.
///
/// Time: O(n) — read at most n characters.
/// Space: O(1) — only a 4-byte buffer.
mod my_solution {
    use super::FileReader;

    pub struct Solution;

    impl Solution {
        /// Read n characters from file via read4 into buf.
        /// Returns number of characters actually read.
        pub fn read(reader: &mut FileReader, buf: &mut Vec<u8>, n: usize) -> usize {
            let mut buf4 = [0u8; 4];
            let mut total = 0;
            // Keep reading while read4 returned a full chunk (might have more)
            let mut v = 4;
            while v >= 4 {
                v = reader.read4(&mut buf4);
                for j in 0..v {
                    buf.push(buf4[j]);
                    total += 1;
                    if total >= n {
                        return n;
                    }
                }
            }
            total
        }
    }
}

/// AI solution — read4 with min-copy approach.
///
/// Instead of checking after each character, read a whole buf4 chunk
/// and copy only min(v, n - total) characters at once. This reduces
/// per-character branch overhead.
///
/// Time: O(n) — read at most n characters.
/// Space: O(1) — only a 4-byte buffer.
mod ai_solution {
    use super::FileReader;

    pub struct Solution;

    impl Solution {
        /// Read n characters from file via read4 into buf.
        /// Returns number of characters actually read.
        pub fn read(reader: &mut FileReader, buf: &mut Vec<u8>, n: usize) -> usize {
            let mut buf4 = [0u8; 4];
            let mut total = 0;

            loop {
                let v = reader.read4(&mut buf4);
                if v == 0 {
                    break;
                }
                // Copy only what we need: min(v, n - total)
                let to_copy = v.min(n - total);
                buf.extend_from_slice(&buf4[..to_copy]);
                total += to_copy;
                if total >= n {
                    break;
                }
            }
            total
        }
    }
}

fn main() {
    let cases = [("abc", 4, 3), ("abcde", 5, 5), ("abcdABCD1234", 12, 12)];
    for (file, n, expected) in &cases {
        let mut reader1 = FileReader::new(file);
        let mut buf1 = Vec::new();
        let my = my_solution::Solution::read(&mut reader1, &mut buf1, *n);

        let mut reader2 = FileReader::new(file);
        let mut buf2 = Vec::new();
        let ai = ai_solution::Solution::read(&mut reader2, &mut buf2, *n);

        println!(
            "file={file:?}, n={n}: my={my} ({:?}), ai={ai} ({:?}) (expected {expected})",
            String::from_utf8_lossy(&buf1),
            String::from_utf8_lossy(&buf2),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_read {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_read_less_than_file() {
                    let mut reader = FileReader::new("abc");
                    let mut buf = Vec::new();
                    let result = <$solution>::read(&mut reader, &mut buf, 4);
                    assert_eq!(result, 3);
                    assert_eq!(&buf, b"abc");
                }

                #[test]
                fn test_read_exact() {
                    let mut reader = FileReader::new("abcde");
                    let mut buf = Vec::new();
                    let result = <$solution>::read(&mut reader, &mut buf, 5);
                    assert_eq!(result, 5);
                    assert_eq!(&buf, b"abcde");
                }

                #[test]
                fn test_read_long() {
                    let mut reader = FileReader::new("abcdABCD1234");
                    let mut buf = Vec::new();
                    let result = <$solution>::read(&mut reader, &mut buf, 12);
                    assert_eq!(result, 12);
                    assert_eq!(&buf, b"abcdABCD1234");
                }

                #[test]
                fn test_read_partial() {
                    // Request only 2 from a 5-char file
                    let mut reader = FileReader::new("hello");
                    let mut buf = Vec::new();
                    let result = <$solution>::read(&mut reader, &mut buf, 2);
                    assert_eq!(result, 2);
                    assert_eq!(&buf, b"he");
                }

                #[test]
                fn test_read_single_char() {
                    let mut reader = FileReader::new("x");
                    let mut buf = Vec::new();
                    let result = <$solution>::read(&mut reader, &mut buf, 1);
                    assert_eq!(result, 1);
                    assert_eq!(&buf, b"x");
                }
            }
        };
    }

    test_read!(my, crate::my_solution::Solution);
    test_read!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let file_content = "abcdABCD1234efghIJKL5678";
        let n = 20;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut reader = FileReader::new(file_content);
            let mut buf = Vec::new();
            std::hint::black_box(crate::my_solution::Solution::read(
                &mut reader,
                &mut buf,
                std::hint::black_box(n),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut reader = FileReader::new(file_content);
            let mut buf = Vec::new();
            std::hint::black_box(crate::ai_solution::Solution::read(
                &mut reader,
                &mut buf,
                std::hint::black_box(n),
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
