// LeetCode 158: Read N Characters Given read4 II - Call Multiple Times [Hard] (Premium)
// https://leetcode.com/problems/read-n-characters-given-read4-ii-call-multiple-times/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 0.308us
//   AI solution avg: 0.250us
//   Ratio: AI 1.23x faster
//
// Given a file, implement read to read n characters using read4.
// The read method may be called MULTIPLE TIMES.
//
// read4 reads 4 consecutive chars from file into buf4, returns actual read count.
// read reads n chars from file into buf, returns actual read count.
//
// Example 1: file = "abc", queries = [1,2,1] -> [1,2,0]
// Example 2: file = "abc", queries = [4,1] -> [3,0]
//
// Constraints:
//   - 1 <= file.length <= 500
//   - file consists of English letters and digits
//   - 1 <= queries.length <= 10
//   - 1 <= queries[i] <= 500
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0158.Read%20N%20Characters%20Given%20read4%20II%20-%20Call%20Multiple%20Times/README_EN.md
//
// Run:  cargo run --example lc_0158_read_n_characters_given_read4_ii
// Test: cargo test --example lc_0158_read_n_characters_given_read4_ii
// Bench: cargo test --example lc_0158_read_n_characters_given_read4_ii bench -- --nocapture

/// Simulated read4 API — file reader with position tracking.
struct FileReader {
    data: Vec<u8>,
    pos: usize,
}

impl FileReader {
    /// Create a new file reader.
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

/// My solution — buffered reader (translated from Java in doocs/leetcode).
///
/// Maintain an internal buf4 plus index/size from previous read4 calls.
/// On each read(n), first drain leftover from the internal buffer, then
/// call read4 for more data as needed. Supports multiple calls.
///
/// Time: O(n) per call.
/// Space: O(1) — fixed 4-byte internal buffer.
mod my_solution {
    use super::FileReader;

    pub struct Solution {
        buf4: [u8; 4],
        /// Current position in buf4 (how much we've consumed).
        i: usize,
        /// How many valid bytes are in buf4 from last read4 call.
        size: usize,
    }

    impl Solution {
        /// Create a new Solution (must persist across multiple read calls).
        pub fn new() -> Self {
            Self {
                buf4: [0; 4],
                i: 0,
                size: 0,
            }
        }

        /// Read n characters from file into buf. Returns actual count.
        pub fn read(&mut self, reader: &mut FileReader, buf: &mut Vec<u8>, n: usize) -> usize {
            let mut j = 0;
            while j < n {
                // If internal buffer is exhausted, fetch more via read4
                if self.i == self.size {
                    self.size = reader.read4(&mut self.buf4);
                    self.i = 0;
                    if self.size == 0 {
                        break;
                    }
                }
                // Copy from internal buffer
                while j < n && self.i < self.size {
                    buf.push(self.buf4[self.i]);
                    self.i += 1;
                    j += 1;
                }
            }
            j
        }
    }
}

/// AI solution — buffered reader with bulk copy.
///
/// Same concept but uses slice operations to copy multiple bytes at once
/// from the internal buffer rather than one-at-a-time.
///
/// Time: O(n) per call.
/// Space: O(1) — fixed 4-byte internal buffer.
mod ai_solution {
    use super::FileReader;

    pub struct Solution {
        buf4: [u8; 4],
        buf_pos: usize,
        buf_len: usize,
    }

    impl Solution {
        /// Create a new Solution.
        pub fn new() -> Self {
            Self {
                buf4: [0; 4],
                buf_pos: 0,
                buf_len: 0,
            }
        }

        /// Read n characters from file into buf. Returns actual count.
        pub fn read(&mut self, reader: &mut FileReader, buf: &mut Vec<u8>, n: usize) -> usize {
            let mut total = 0;

            while total < n {
                // Refill internal buffer if exhausted
                if self.buf_pos == self.buf_len {
                    self.buf_len = reader.read4(&mut self.buf4);
                    self.buf_pos = 0;
                    if self.buf_len == 0 {
                        break;
                    }
                }
                // Bulk copy: take min(remaining_in_buf, remaining_requested)
                let available = self.buf_len - self.buf_pos;
                let needed = n - total;
                let to_copy = available.min(needed);
                buf.extend_from_slice(&self.buf4[self.buf_pos..self.buf_pos + to_copy]);
                self.buf_pos += to_copy;
                total += to_copy;
            }
            total
        }
    }
}

fn main() {
    // Example 1: file="abc", queries=[1,2,1] -> [1,2,0]
    {
        let mut reader = FileReader::new("abc");
        let mut sol = my_solution::Solution::new();
        for &q in &[1, 2, 1] {
            let mut buf = Vec::new();
            let result = sol.read(&mut reader, &mut buf, q);
            println!(
                "my: read({q}) = {result}, buf={:?}",
                String::from_utf8_lossy(&buf)
            );
        }
    }
    println!("---");
    {
        let mut reader = FileReader::new("abc");
        let mut sol = ai_solution::Solution::new();
        for &q in &[1, 2, 1] {
            let mut buf = Vec::new();
            let result = sol.read(&mut reader, &mut buf, q);
            println!(
                "ai: read({q}) = {result}, buf={:?}",
                String::from_utf8_lossy(&buf)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_read_multi {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::*;

                #[test]
                fn test_example1() {
                    let mut reader = FileReader::new("abc");
                    let mut sol = <$solution>::new();

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 1), 1);
                    assert_eq!(&buf, b"a");

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 2), 2);
                    assert_eq!(&buf, b"bc");

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 1), 0);
                    assert!(buf.is_empty());
                }

                #[test]
                fn test_example2() {
                    let mut reader = FileReader::new("abc");
                    let mut sol = <$solution>::new();

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 4), 3);
                    assert_eq!(&buf, b"abc");

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 1), 0);
                }

                #[test]
                fn test_across_read4_boundary() {
                    // File = "abcdefgh" (8 chars), read 3 then 3 then 3
                    let mut reader = FileReader::new("abcdefgh");
                    let mut sol = <$solution>::new();

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 3), 3);
                    assert_eq!(&buf, b"abc");

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 3), 3);
                    assert_eq!(&buf, b"def");

                    let mut buf = Vec::new();
                    assert_eq!(sol.read(&mut reader, &mut buf, 3), 2);
                    assert_eq!(&buf, b"gh");
                }

                #[test]
                fn test_read_one_at_a_time() {
                    let mut reader = FileReader::new("hello");
                    let mut sol = <$solution>::new();
                    let mut collected = Vec::new();
                    for _ in 0..6 {
                        let mut buf = Vec::new();
                        let n = sol.read(&mut reader, &mut buf, 1);
                        collected.extend_from_slice(&buf);
                        if n == 0 {
                            break;
                        }
                    }
                    assert_eq!(&collected, b"hello");
                }
            }
        };
    }

    test_read_multi!(my, crate::my_solution::Solution);
    test_read_multi!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let file_content = "abcdefghijklmnopqrstuvwxyz";
        let queries = [3, 5, 2, 7, 10];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut reader = FileReader::new(file_content);
            let mut sol = crate::my_solution::Solution::new();
            for &q in &queries {
                let mut buf = Vec::new();
                std::hint::black_box(sol.read(&mut reader, &mut buf, q));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let mut reader = FileReader::new(file_content);
            let mut sol = crate::ai_solution::Solution::new();
            for &q in &queries {
                let mut buf = Vec::new();
                std::hint::black_box(sol.read(&mut reader, &mut buf, q));
            }
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
