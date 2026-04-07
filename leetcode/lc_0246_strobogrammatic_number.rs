// LeetCode 246: Strobogrammatic Number [Easy] (Premium)
// https://leetcode.com/problems/strobogrammatic-number/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 0.203us
//   AI solution avg: 0.792us
//   Ratio: My 3.91x faster
//
// Given a string `num` representing an integer, return true if num is a
// strobogrammatic number.
// A strobogrammatic number looks the same when rotated 180 degrees.
//
// Example 1: num = "69"  -> true
// Example 2: num = "88"  -> true
// Example 3: num = "962" -> false
//
// Constraints:
//   - 1 <= num.length <= 50
//   - num consists of only digits
//   - num does not contain leading zeros except for zero itself
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0246.Strobogrammatic%20Number/README_EN.md
//
// Run:  cargo run --example lc_0246_strobogrammatic_number
// Test: cargo test --example lc_0246_strobogrammatic_number
// Bench: cargo test --example lc_0246_strobogrammatic_number bench -- --nocapture

/// My solution — lookup array + two pointers (translated from Python in doocs).
///
/// d[i] = the digit produced when digit i is rotated 180 degrees.
/// d[i] = -1 means the digit is invalid after rotation.
/// Valid mappings: 0->0, 1->1, 6->9, 8->8, 9->6.
/// Walk inward from both ends comparing d[left] == right.
///
/// Time: O(n)
/// Space: O(1)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if num is strobogrammatic.
        pub fn is_strobogrammatic(num: String) -> bool {
            // d[i] = rotated digit, -1 means invalid under 180 rotation
            let d: [i8; 10] = [0, 1, -1, -1, -1, -1, 9, -1, 8, 6];
            let bytes = num.as_bytes();
            let mut i = 0usize;
            let mut j = bytes.len() - 1;
            // Use checked comparison to handle the middle digit when i == j
            loop {
                let a = (bytes[i] - b'0') as usize;
                let b = (bytes[j] - b'0') as i8;
                if d[a] != b {
                    return false;
                }
                if i >= j {
                    break;
                }
                i += 1;
                j -= 1;
            }
            true
        }
    }
}

/// AI solution — HashMap-based pair checking.
///
/// Use a HashMap of valid strobogrammatic pairs. Convert string to
/// chars, check from both ends that (left, right) is a valid pair.
/// More readable but slightly more overhead from HashMap.
///
/// Time: O(n)
/// Space: O(1) — HashMap has fixed 5 entries
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Return true if num is strobogrammatic.
        pub fn is_strobogrammatic(num: String) -> bool {
            // Valid strobogrammatic digit pairs: (digit, its 180-rotation)
            let pairs: HashMap<u8, u8> = [
                (b'0', b'0'),
                (b'1', b'1'),
                (b'6', b'9'),
                (b'8', b'8'),
                (b'9', b'6'),
            ]
            .into_iter()
            .collect();
            let bytes = num.as_bytes();
            let mut i = 0;
            let mut j = bytes.len() - 1;
            loop {
                match pairs.get(&bytes[i]) {
                    Some(&rotated) if rotated == bytes[j] => {}
                    _ => return false,
                }
                if i >= j {
                    break;
                }
                i += 1;
                j -= 1;
            }
            true
        }
    }
}

fn main() {
    for num in ["69", "88", "962", "1", "0", "96"] {
        println!(
            "my({num})={}, ai({num})={}",
            my_solution::Solution::is_strobogrammatic(num.into()),
            ai_solution::Solution::is_strobogrammatic(num.into()),
        );
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_strobogrammatic {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                #[test]
                fn test_69() {
                    assert!(Sol::is_strobogrammatic("69".into()));
                }

                #[test]
                fn test_88() {
                    assert!(Sol::is_strobogrammatic("88".into()));
                }

                #[test]
                fn test_962() {
                    assert!(!Sol::is_strobogrammatic("962".into()));
                }

                #[test]
                fn test_0() {
                    assert!(Sol::is_strobogrammatic("0".into()));
                }

                #[test]
                fn test_1() {
                    assert!(Sol::is_strobogrammatic("1".into()));
                }

                #[test]
                fn test_8() {
                    assert!(Sol::is_strobogrammatic("8".into()));
                }

                #[test]
                fn test_96() {
                    assert!(Sol::is_strobogrammatic("96".into()));
                }

                #[test]
                fn test_11() {
                    assert!(Sol::is_strobogrammatic("11".into()));
                }

                #[test]
                fn test_619() {
                    assert!(Sol::is_strobogrammatic("619".into()));
                }

                #[test]
                fn test_2() {
                    assert!(!Sol::is_strobogrammatic("2".into()));
                }

                #[test]
                fn test_10() {
                    assert!(!Sol::is_strobogrammatic("10".into()));
                }

                #[test]
                fn test_818() {
                    assert!(Sol::is_strobogrammatic("818".into()));
                }
            }
        };
    }

    test_strobogrammatic!(my, crate::my_solution::Solution);
    test_strobogrammatic!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let inputs: Vec<String> = vec![
            "69".into(), "88".into(), "962".into(), "10801".into(),
            "818".into(), "96196".into(),
        ];

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for num in &inputs {
                std::hint::black_box(
                    crate::my_solution::Solution::is_strobogrammatic(num.clone()),
                );
            }
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            for num in &inputs {
                std::hint::black_box(
                    crate::ai_solution::Solution::is_strobogrammatic(num.clone()),
                );
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
