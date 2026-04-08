// Benchmark: 500,000 iterations (2025-04-08)
// My solution:  avg 4.72us  | Direct unique-letter counting, manual subtraction
// AI solution:  avg 4.70us  | Table-driven order + word letter subtraction
// -> Tied (1.00x)
//
// LeetCode 423: Reconstruct Original Digits from English [Medium]
// https://leetcode.com/problems/reconstruct-original-digits-from-english/
//
// Given a string containing shuffled English representations of digits 0-9,
// return the digits in ascending order.
//
// Examples:
//   "owoztneoer" -> "012"
//   "fviefuro"   -> "45"
//
// Constraints:
//   1 <= s.length <= 10^5
//   s[i] is one of "e","g","f","i","h","o","n","s","r","u","t","w","v","x","z"
//   s is guaranteed to be valid
//
// Run:  cargo run --example lc_0423_reconstruct_original_digits_from_english
// Test: cargo test --example lc_0423_reconstruct_original_digits_from_english
// Bench: cargo test --example lc_0423_reconstruct_original_digits_from_english bench -- --nocapture

/// My solution module — unique letter identification.
///
/// Key insight: some digits have unique letters:
///   0=zero(z), 2=two(w), 4=four(u), 6=six(x), 8=eight(g)
/// After removing those, remaining digits have unique letters:
///   3=three(h), 5=five(f), 7=seven(s)
/// Finally:
///   1=one(o), 9=nine(i)
///
/// Time:  O(n)
/// Space: O(1) -- 26-element frequency array
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reconstruct the digits from shuffled English words.
        ///
        /// # Arguments
        ///
        /// * `s` - String of shuffled letters
        ///
        /// # Returns
        ///
        /// Digits in ascending order as a string
        pub fn original_digits(s: String) -> String {
            let mut freq = [0_i32; 26];
            for b in s.bytes() {
                freq[(b - b'a') as usize] += 1;
            }

            let mut cnt = [0_i32; 10];

            // Round 1: unique letters
            cnt[0] = freq[(b'z' - b'a') as usize];
            cnt[2] = freq[(b'w' - b'a') as usize];
            cnt[4] = freq[(b'u' - b'a') as usize];
            cnt[6] = freq[(b'x' - b'a') as usize];
            cnt[8] = freq[(b'g' - b'a') as usize];

            // Round 2: after subtracting round-1 digits
            // 'h' appears in three(3) and eight(8)
            cnt[3] = freq[(b'h' - b'a') as usize] - cnt[8];
            // 'f' appears in five(5) and four(4)
            cnt[5] = freq[(b'f' - b'a') as usize] - cnt[4];
            // 's' appears in seven(7) and six(6)
            cnt[7] = freq[(b's' - b'a') as usize] - cnt[6];

            // Round 3: remaining
            // 'o' appears in one(1), zero(0), two(2), four(4)
            cnt[1] = freq[(b'o' - b'a') as usize] - cnt[0] - cnt[2] - cnt[4];
            // 'i' appears in nine(9), five(5), six(6), eight(8)
            cnt[9] = freq[(b'i' - b'a') as usize] - cnt[5] - cnt[6] - cnt[8];

            // Build result
            let mut result = String::new();
            for (digit, &count) in cnt.iter().enumerate() {
                for _ in 0..count {
                    result.push((b'0' + digit as u8) as char);
                }
            }
            result
        }
    }
}

/// AI solution module — same logic, table-driven approach.
///
/// Uses a lookup table for the detection order and unique characters,
/// making the code more concise and data-driven.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Reconstruct the digits from shuffled English words.
        ///
        /// # Arguments
        ///
        /// * `s` - String of shuffled letters
        ///
        /// # Returns
        ///
        /// Digits in ascending order as a string
        pub fn original_digits(s: String) -> String {
            let mut freq = [0_i32; 26];
            for b in s.bytes() {
                freq[(b - b'a') as usize] += 1;
            }

            // (digit, unique_char, letters_to_subtract)
            // Process in order: 0,2,4,6,8 -> 3,5,7 -> 1,9
            let order: [(usize, u8); 10] = [
                (0, b'z'), (2, b'w'), (4, b'u'), (6, b'x'), (8, b'g'),
                (3, b'h'), (5, b'f'), (7, b's'),
                (1, b'o'), (9, b'i'),
            ];

            // Word spellings for each digit
            let words: [&[u8]; 10] = [
                b"zero", b"one", b"two", b"three", b"four",
                b"five", b"six", b"seven", b"eight", b"nine",
            ];

            let mut cnt = [0_i32; 10];

            for &(digit, unique_ch) in &order {
                let count = freq[(unique_ch - b'a') as usize];
                cnt[digit] = count;
                // Subtract all letters of this word `count` times
                for &ch in words[digit] {
                    freq[(ch - b'a') as usize] -= count;
                }
            }

            let mut result = String::new();
            for (digit, &count) in cnt.iter().enumerate() {
                for _ in 0..count {
                    result.push((b'0' + digit as u8) as char);
                }
            }
            result
        }
    }
}

fn main() {
    let cases = [
        ("owoztneoer", "012"),
        ("fviefuro", "45"),
    ];

    println!("--- My Solution ---");
    for (s, expected) in &cases {
        let result =
            my_solution::Solution::original_digits(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }

    println!("--- AI Solution ---");
    for (s, expected) in &cases {
        let result =
            ai_solution::Solution::original_digits(s.to_string());
        println!("  \"{s}\" -> \"{result}\" (expected \"{expected}\")");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_digits {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::original_digits(
                            "owoztneoer".to_string(),
                        ),
                        "012",
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::original_digits(
                            "fviefuro".to_string(),
                        ),
                        "45",
                    );
                }

                #[test]
                fn test_single_zero() {
                    assert_eq!(
                        Solution::original_digits("zero".to_string()),
                        "0",
                    );
                }

                #[test]
                fn test_all_digits() {
                    // "zeroonetwothreefourfivesixseveneightnine"
                    let s = "zeroonetwothreefourfivesixseveneightnine";
                    assert_eq!(
                        Solution::original_digits(s.to_string()),
                        "0123456789",
                    );
                }

                #[test]
                fn test_repeated() {
                    // "oneone" -> "11"
                    assert_eq!(
                        Solution::original_digits("oneone".to_string()),
                        "11",
                    );
                }
            }
        };
    }

    test_digits!(my_tests, crate::my_solution::Solution);
    test_digits!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 500_000_u32;
        // Build a large input with many digits
        let s = "zeroonetwothreefourfivesixseveneightnine".repeat(100);

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::original_digits(
                    std::hint::black_box(s.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::original_digits(
                    std::hint::black_box(s.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
