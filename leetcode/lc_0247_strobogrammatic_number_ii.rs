// LeetCode 247: Strobogrammatic Number II [Medium] (Premium)
// https://leetcode.com/problems/strobogrammatic-number-ii/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 1.743us
//   AI solution avg: 1.544us
//   Ratio: AI 1.13x faster
//
// Given an integer n, return all strobogrammatic numbers of length n.
// A strobogrammatic number looks the same when rotated 180 degrees.
//
// Example 1: n = 2 -> ["11","69","88","96"]
// Example 2: n = 1 -> ["0","1","8"]
//
// Constraints:
//   - 1 <= n <= 14
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0247.Strobogrammatic%20Number%20II/README_EN.md
//
// Run:  cargo run --example lc_0247_strobogrammatic_number_ii
// Test: cargo test --example lc_0247_strobogrammatic_number_ii
// Bench: cargo test --example lc_0247_strobogrammatic_number_ii bench -- --nocapture

/// My solution — recursive build from center outward (translated from Python in doocs).
///
/// Base cases: length 0 -> [""], length 1 -> ["0","1","8"].
/// For length u, take all results of length u-2 and wrap with valid
/// pairs (1,1), (8,8), (6,9), (9,6). Also wrap with (0,0) if u != n
/// (no leading zeros for the outermost layer).
///
/// Time: O(5^(n/2)) — output size dominates
/// Space: O(5^(n/2))
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return all strobogrammatic numbers of length n.
        pub fn find_strobogrammatic(n: i32) -> Vec<String> {
            fn dfs(u: i32, n: i32) -> Vec<String> {
                if u == 0 {
                    return vec![String::new()];
                }
                if u == 1 {
                    return vec!["0".into(), "1".into(), "8".into()];
                }
                let mut ans = Vec::new();
                // Valid strobogrammatic pairs (left, right)
                let pairs: [(char, char); 4] = [('1', '1'), ('8', '8'), ('6', '9'), ('9', '6')];
                for v in dfs(u - 2, n) {
                    for &(l, r) in &pairs {
                        let mut s = String::with_capacity(v.len() + 2);
                        s.push(l);
                        s.push_str(&v);
                        s.push(r);
                        ans.push(s);
                    }
                    // Only add '0' wrapping for inner layers (no leading zeros)
                    if u != n {
                        let mut s = String::with_capacity(v.len() + 2);
                        s.push('0');
                        s.push_str(&v);
                        s.push('0');
                        ans.push(s);
                    }
                }
                ans
            }
            dfs(n, n)
        }
    }
}

/// AI solution — iterative build from center outward.
///
/// Start with the base case for the center (depending on odd/even n),
/// then iteratively wrap each existing number with valid pairs,
/// growing by 2 characters each step.
///
/// Time: O(5^(n/2))
/// Space: O(5^(n/2))
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return all strobogrammatic numbers of length n.
        pub fn find_strobogrammatic(n: i32) -> Vec<String> {
            let n = n as usize;
            // Start from center: odd-length starts with single digits,
            // even-length starts with empty string
            let mut cur: Vec<String> = if n % 2 == 1 {
                vec!["0".into(), "1".into(), "8".into()]
            } else {
                vec![String::new()]
            };
            // Current length of strings in `cur`
            let mut len = n % 2;
            // All valid strobogrammatic pairs
            let pairs: [(char, char); 5] =
                [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
            // Iteratively grow outward by wrapping pairs
            while len < n {
                len += 2;
                let mut next = Vec::with_capacity(cur.len() * 5);
                for v in &cur {
                    for &(l, r) in &pairs {
                        // Skip leading zeros on the outermost layer
                        if len == n && l == '0' {
                            continue;
                        }
                        let mut s = String::with_capacity(v.len() + 2);
                        s.push(l);
                        s.push_str(v);
                        s.push(r);
                        next.push(s);
                    }
                }
                cur = next;
            }
            cur
        }
    }
}

fn main() {
    println!(
        "my n=2: {:?}",
        my_solution::Solution::find_strobogrammatic(2)
    );
    println!(
        "my n=1: {:?}",
        my_solution::Solution::find_strobogrammatic(1)
    );
    println!(
        "ai n=2: {:?}",
        ai_solution::Solution::find_strobogrammatic(2)
    );
    println!(
        "ai n=1: {:?}",
        ai_solution::Solution::find_strobogrammatic(1)
    );
}

#[cfg(test)]
mod tests {
    macro_rules! test_strobogrammatic_ii {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Sol;

                fn sorted(mut v: Vec<String>) -> Vec<String> {
                    v.sort();
                    v
                }

                #[test]
                fn test_n1() {
                    let mut result = Sol::find_strobogrammatic(1);
                    result.sort();
                    assert_eq!(result, ["0", "1", "8"]);
                }

                #[test]
                fn test_n2() {
                    let result = sorted(Sol::find_strobogrammatic(2));
                    assert_eq!(result, ["11", "69", "88", "96"]);
                }

                #[test]
                fn test_n3() {
                    let result = sorted(Sol::find_strobogrammatic(3));
                    // 3-digit: center must be 0,1,8; outer pairs: 11,69,88,96
                    let expected = sorted(
                        vec![
                            "101", "111", "181", "609", "619", "689", "808", "818", "888", "906",
                            "916", "986",
                        ]
                        .into_iter()
                        .map(String::from)
                        .collect(),
                    );
                    assert_eq!(result, expected);
                }

                #[test]
                fn test_n4() {
                    let result = sorted(Sol::find_strobogrammatic(4));
                    // Each result should have length 4
                    assert!(result.iter().all(|s| s.len() == 4));
                    // No leading zeros
                    assert!(result.iter().all(|s| !s.starts_with('0')));
                    // All should be strobogrammatic
                    let d: [i8; 10] = [0, 1, -1, -1, -1, -1, 9, -1, 8, 6];
                    for s in &result {
                        let b = s.as_bytes();
                        let (mut i, mut j) = (0, b.len() - 1);
                        while i <= j {
                            let a = (b[i] - b'0') as usize;
                            let bv = (b[j] - b'0') as i8;
                            assert_eq!(d[a], bv, "not strobogrammatic: {s}");
                            i += 1;
                            if j > 0 {
                                j -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                    // n=4 should produce 4*4 = 16 results? No:
                    // outer pairs: 4 choices (11,69,88,96)
                    // inner pairs: 5 choices (00,11,69,88,96)
                    // 4 * 5 = 20
                    // Wait: inner is length 2, so inner results are the
                    // 5 two-digit strobogrammatic nums including 00
                    assert_eq!(result.len(), 20);
                }

                #[test]
                fn test_count_n5() {
                    let result = Sol::find_strobogrammatic(5);
                    // n=5: outer 4 * inner(3-digit) which is 4*3=12 => 4*12=48?
                    // Actually: dfs(5) = for each in dfs(3): 4 pairs
                    // dfs(3) = for each in dfs(1): 4 pairs + 1 zero pair = 5 pairs * 3 = 15
                    // dfs(5) = 15 * 4 = 60? No, dfs(3) has 4*3=12 (no zero for outer)
                    // n=5, u=5=n so no zero. dfs(3, 5) has u=3!=n=5 so 5 pairs * 3 = 15
                    // Wait: dfs(3, 5): u=3, n=5, so u!=n, adds zero wrap.
                    // dfs(1, 5) returns ["0","1","8"]
                    // dfs(3, 5): for each of 3 base: 4 pairs + 1 zero = 5 => 15
                    // dfs(5, 5): for each of 15: 4 pairs (no zero since u==n) => 60
                    assert_eq!(result.len(), 60);
                }
            }
        };
    }

    test_strobogrammatic_ii!(my, crate::my_solution::Solution);
    test_strobogrammatic_ii!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_strobogrammatic(4));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_strobogrammatic(4));
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
