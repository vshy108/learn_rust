// LeetCode 273: Integer to English Words [Hard]
// https://leetcode.com/problems/integer-to-english-words/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 3.312us
//   AI solution avg: 3.894us
//   Ratio: My 1.18x faster
//
// Convert a non-negative integer num to its English words representation.
//
// Example 1: 123     -> "One Hundred Twenty Three"
// Example 2: 12345   -> "Twelve Thousand Three Hundred Forty Five"
// Example 3: 1234567 -> "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
//
// Constraints:
//   - 0 <= num <= 2^31 - 1
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0273.Integer%20to%20English%20Words/README_EN.md
//
// Run:  cargo run --example lc_0273_integer_to_english_words
// Test: cargo test --example lc_0273_integer_to_english_words
// Bench: cargo test --example lc_0273_integer_to_english_words bench -- --nocapture

/// My solution — Recursive chunk-based conversion (from doocs).
///
/// Process the number in chunks of 3 digits from the most significant group
/// (billions, millions, thousands, units). For each chunk, recursively convert
/// the 1-999 range using lookup tables for ones (<20) and tens.
///
/// Time:  O(1) — bounded input range means bounded recursion depth
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    // Words for 0-19
    const LT20: [&str; 20] = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
        "Sixteen", "Seventeen", "Eighteen", "Nineteen",
    ];

    // Words for tens: 0, 10, 20, ..., 90
    const TENS: [&str; 10] = [
        "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy",
        "Eighty", "Ninety",
    ];

    // Scale labels from largest to smallest
    const THOUSANDS: [&str; 4] = ["Billion", "Million", "Thousand", ""];

    impl Solution {
        pub fn number_to_words(num: i32) -> String {
            if num == 0 {
                return "Zero".to_string();
            }

            let mut num = num as u32;
            let mut parts = Vec::new();
            // Divisors: 10^9, 10^6, 10^3, 10^0
            let divisors = [1_000_000_000, 1_000_000, 1_000, 1];

            for (j, &div) in divisors.iter().enumerate() {
                let chunk = num / div;
                if chunk > 0 {
                    let chunk_str = Self::transfer(chunk);
                    if !THOUSANDS[j].is_empty() {
                        parts.push(format!("{} {}", chunk_str, THOUSANDS[j]));
                    } else {
                        parts.push(chunk_str);
                    }
                    num %= div;
                }
            }

            parts.join(" ")
        }

        /// Convert a number 1-999 to English words.
        fn transfer(num: u32) -> String {
            if num == 0 {
                return String::new();
            }
            if num < 20 {
                return LT20[num as usize].to_string();
            }
            if num < 100 {
                let t = TENS[(num / 10) as usize];
                let rest = Self::transfer(num % 10);
                if rest.is_empty() {
                    return t.to_string();
                }
                return format!("{} {}", t, rest);
            }
            // num >= 100
            let hundreds = LT20[(num / 100) as usize];
            let rest = Self::transfer(num % 100);
            if rest.is_empty() {
                return format!("{} Hundred", hundreds);
            }
            format!("{} Hundred {}", hundreds, rest)
        }
    }
}

/// AI solution — Iterative right-to-left processing.
///
/// Process groups of 3 digits starting from the least significant. Use a
/// scale index to append the correct label (Thousand, Million, Billion).
/// Build the result string by prepending each group's words.
///
/// Time:  O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    const ONES: [&str; 20] = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
        "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
        "Sixteen", "Seventeen", "Eighteen", "Nineteen",
    ];

    const TENS: [&str; 10] = [
        "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy",
        "Eighty", "Ninety",
    ];

    const SCALES: [&str; 4] = ["", "Thousand", "Million", "Billion"];

    impl Solution {
        pub fn number_to_words(num: i32) -> String {
            if num == 0 {
                return "Zero".to_string();
            }

            let mut num = num as u32;
            let mut parts: Vec<String> = Vec::new();
            let mut scale_idx = 0;

            while num > 0 {
                let chunk = num % 1000;
                if chunk > 0 {
                    let chunk_str = Self::chunk_to_words(chunk);
                    if !SCALES[scale_idx].is_empty() {
                        parts.push(format!(
                            "{} {}",
                            chunk_str, SCALES[scale_idx]
                        ));
                    } else {
                        parts.push(chunk_str);
                    }
                }
                num /= 1000;
                scale_idx += 1;
            }

            // Reverse because we built from least significant to most
            parts.reverse();
            parts.join(" ")
        }

        /// Convert a 1-999 chunk to English words (iterative).
        fn chunk_to_words(mut n: u32) -> String {
            let mut words = Vec::new();

            if n >= 100 {
                words.push(ONES[(n / 100) as usize].to_string());
                words.push("Hundred".to_string());
                n %= 100;
            }

            if n >= 20 {
                words.push(TENS[(n / 10) as usize].to_string());
                n %= 10;
            }

            if n > 0 {
                words.push(ONES[n as usize].to_string());
            }

            words.join(" ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_ntw {
        ($name:ident, $num:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    my_solution::Solution::number_to_words($num),
                    $expected,
                    "my_solution failed for {}",
                    $num
                );
                assert_eq!(
                    ai_solution::Solution::number_to_words($num),
                    $expected,
                    "ai_solution failed for {}",
                    $num
                );
            }
        };
    }

    test_ntw!(test_zero, 0, "Zero");
    test_ntw!(test_example_1, 123, "One Hundred Twenty Three");
    test_ntw!(
        test_example_2,
        12345,
        "Twelve Thousand Three Hundred Forty Five"
    );
    test_ntw!(
        test_example_3,
        1234567,
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
    );
    test_ntw!(test_one, 1, "One");
    test_ntw!(test_ten, 10, "Ten");
    test_ntw!(test_eleven, 11, "Eleven");
    test_ntw!(test_twenty, 20, "Twenty");
    test_ntw!(test_hundred, 100, "One Hundred");
    test_ntw!(test_thousand, 1000, "One Thousand");
    test_ntw!(test_million, 1_000_000, "One Million");
    test_ntw!(test_billion, 1_000_000_000, "One Billion");
    test_ntw!(
        test_max_i32,
        2_147_483_647,
        "Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Seven"
    );
    test_ntw!(test_fifty, 50, "Fifty");
    test_ntw!(
        test_one_million_one,
        1_000_001,
        "One Million One"
    );
    test_ntw!(
        test_gaps,
        1_000_010,
        "One Million Ten"
    );
    test_ntw!(test_teens, 19, "Nineteen");
    test_ntw!(test_ninety_nine, 99, "Ninety Nine");
    test_ntw!(test_nine_ninety_nine, 999, "Nine Hundred Ninety Nine");
}

fn main() {
    println!("LeetCode 273: Integer to English Words");
    for num in [123, 12345, 1234567, 0, 2_147_483_647] {
        println!(
            "  {} -> {:?}",
            num,
            my_solution::Solution::number_to_words(num)
        );
    }
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let nums = [0, 123, 12345, 1234567, 2_147_483_647];
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            for &n in &nums {
                std::hint::black_box(my_solution::Solution::number_to_words(n));
                std::hint::black_box(ai_solution::Solution::number_to_words(n));
            }
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &nums {
                std::hint::black_box(my_solution::Solution::number_to_words(n));
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for &n in &nums {
                std::hint::black_box(ai_solution::Solution::number_to_words(n));
            }
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
