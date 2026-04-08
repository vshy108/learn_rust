// LeetCode 137: Single Number II [Medium]
// https://leetcode.com/problems/single-number-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 2.017us
//   AI solution avg: 0.671us
//   Ratio: AI 3.00x faster
//
// Given an integer array nums where every element appears three times except
// for one, which appears exactly once. Find the single element.
// Must have O(n) time and O(1) extra space.
//
// Constraints: 1 <= nums.length <= 3*10^4, -2^31 <= nums[i] <= 2^31-1
//
// Run:  cargo run --example lc_0137_single_number_ii
// Test: cargo test --example lc_0137_single_number_ii
// Bench: cargo test --example lc_0137_single_number_ii bench -- --nocapture

/// My solution — count each bit position mod 3.
///
/// Time: O(32n) = O(n).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return the element that appears only once; all others appear 3 times.
        pub fn single_number(nums: Vec<i32>) -> i32 {
            let mut result = 0i32;
            // For each of the 32 bits, count how many numbers have that bit set.
            // If count % 3 != 0, the unique number has that bit set.
            for bit in 0..32 {
                let count: i32 = nums.iter().map(|&n| (n >> bit) & 1).sum();
                if count % 3 != 0 {
                    result |= 1 << bit;
                }
            }
            result
        }
    }
}

/// AI solution — digital logic: two-variable state machine (ones, twos).
///
/// Time: O(n).
/// Space: O(1).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the unique element using bitwise state machine.
        pub fn single_number(nums: Vec<i32>) -> i32 {
            // `ones` and `twos` track bits that have appeared 1 and 2 times (mod 3).
            // When a bit appears a 3rd time, it's cleared from both.
            let mut ones = 0i32;
            let mut twos = 0i32;
            for num in nums {
                ones = (ones ^ num) & !twos;
                twos = (twos ^ num) & !ones;
            }
            ones
        }
    }
}

fn main() {
    let my = my_solution::Solution::single_number(vec![2, 2, 3, 2]);
    let ai = ai_solution::Solution::single_number(vec![2, 2, 3, 2]);
    println!("single number II: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_single {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(<$solution>::single_number(vec![2, 2, 3, 2]), 3,);
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99,);
                }

                #[test]
                fn test_single_element() {
                    assert_eq!(<$solution>::single_number(vec![1]), 1);
                }

                #[test]
                fn test_negative() {
                    assert_eq!(<$solution>::single_number(vec![-2, -2, 1, -2]), 1,);
                }

                #[test]
                fn test_all_negative() {
                    assert_eq!(<$solution>::single_number(vec![-1, -1, -1, -5]), -5,);
                }
            }
        };
    }

    test_single!(my, crate::my_solution::Solution);
    test_single!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let nums: Vec<i32> = {
            let mut v: Vec<i32> = (1..=300).flat_map(|x| [x, x, x]).collect();
            v.push(999);
            v
        };

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::single_number(
                std::hint::black_box(nums.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::single_number(
                std::hint::black_box(nums.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );
        println!(
            "AI solution: {:.3}us avg",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("Both solutions are roughly equal");
        }
    }
}
