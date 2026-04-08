// LeetCode 401: Binary Watch [Easy]
// https://leetcode.com/problems/binary-watch/
// Benchmark: 100000 iterations
//   My solution (enumerate (h,m) pairs):     avg 6.70us
//   AI solution (10-bit binary enumeration): avg 6.94us
//   -> Roughly tied (My 1.03x faster)
//
// A binary watch has 4 LEDs for hours (0-11) and 6 LEDs for minutes (0-59).
// Given `turnedOn` LEDs currently on, return all possible times.
//
// Examples:
//   turnedOn=1 -> ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
//   turnedOn=9 -> []
//
// Constraints:
//   - 0 <= turnedOn <= 10
//
// Run:  cargo run --example lc_0401_binary_watch
// Test: cargo test --example lc_0401_binary_watch
// Bench: cargo test --example lc_0401_binary_watch bench -- --nocapture

/// My solution module — enumerate all (hour, minute) pairs.
///
/// For each (h, m) where h in [0,12) and m in [0,60), check if the
/// total number of 1-bits in h and m equals turnedOn.
///
/// Time:  O(12 * 60) = O(1)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all possible times with `turned_on` LEDs lit.
        ///
        /// # Arguments
        ///
        /// * `turned_on` - Number of LEDs currently on
        ///
        /// # Returns
        ///
        /// Vec of time strings like "h:mm"
        pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
            let mut ans = Vec::new();
            for h in 0u32..12 {
                for m in 0u32..60 {
                    if (h.count_ones() + m.count_ones()) as i32 == turned_on {
                        ans.push(format!("{h}:{m:02}"));
                    }
                }
            }
            ans
        }
    }
}

/// AI solution module — 10-bit binary enumeration.
///
/// Treat the watch as a single 10-bit number: top 4 bits = hours,
/// bottom 6 bits = minutes. Enumerate 0..1024 and filter.
///
/// Time:  O(1024) = O(1)
/// Space: O(1)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all possible times with `turned_on` LEDs lit.
        ///
        /// # Arguments
        ///
        /// * `turned_on` - Number of LEDs currently on
        ///
        /// # Returns
        ///
        /// Vec of time strings like "h:mm"
        pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
            let mut ans = Vec::new();
            for i in 0u32..(1 << 10) {
                let h = i >> 6;
                let m = i & 0b111111;
                if h < 12 && m < 60 && i.count_ones() as i32 == turned_on {
                    ans.push(format!("{h}:{m:02}"));
                }
            }
            ans
        }
    }
}

fn main() {
    let cases = [1, 0, 2];

    for turned_on in &cases {
        println!("turnedOn = {turned_on}:");
        println!(
            "  My:  {:?}",
            my_solution::Solution::read_binary_watch(*turned_on)
        );
        println!(
            "  AI:  {:?}",
            ai_solution::Solution::read_binary_watch(*turned_on)
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_watch {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_turned_on_1() {
                    let mut result = Solution::read_binary_watch(1);
                    result.sort();
                    let mut expected = vec![
                        "0:01", "0:02", "0:04", "0:08", "0:16", "0:32",
                        "1:00", "2:00", "4:00", "8:00",
                    ];
                    expected.sort();
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_turned_on_0() {
                    assert_eq!(
                        Solution::read_binary_watch(0),
                        vec!["0:00"]
                    );
                }

                #[test]
                fn test_turned_on_9() {
                    assert!(Solution::read_binary_watch(9).is_empty());
                }

                #[test]
                fn test_turned_on_2() {
                    let result = Solution::read_binary_watch(2);
                    // Should contain "0:03" (minutes 01+02)
                    assert!(result.contains(&"0:03".to_string()));
                    // Should contain "1:01" (hour 1, minute 01)
                    assert!(result.contains(&"1:01".to_string()));
                }
            }
        };
    }

    test_watch!(my_tests, crate::my_solution::Solution);
    test_watch!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 1_000_000_u32;

        let start = Instant::now();
        for turned_on in 0..=10 {
            for _ in 0..(iterations / 11) {
                std::hint::black_box(
                    crate::my_solution::Solution::read_binary_watch(
                        std::hint::black_box(turned_on),
                    ),
                );
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for turned_on in 0..=10 {
            for _ in 0..(iterations / 11) {
                std::hint::black_box(
                    crate::ai_solution::Solution::read_binary_watch(
                        std::hint::black_box(turned_on),
                    ),
                );
            }
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
