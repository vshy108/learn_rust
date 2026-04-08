// LeetCode 492: Construct the Rectangle [Easy]
// https://leetcode.com/problems/construct-the-rectangle/
//
// A web developer needs a rectangular web page with area equal to `area`.
// Return [L, W] where L >= W, L * W == area, and L - W is minimized.
//
// Examples:
//   area = 4  -> [2, 2]
//   area = 37 -> [37, 1]
//   area = 122681 -> [427, 287]
//
// Constraints:
//   1 <= area <= 10^7
//
// Run:  cargo run --example lc_0492_construct_the_rectangle
// Test: cargo test --example lc_0492_construct_the_rectangle
// Bench: cargo test --example lc_0492_construct_the_rectangle bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 257ns
//   AI solution: avg 256ns
//   Ratio: 1.00x (same algorithm)

/// My solution module — iterate from sqrt downwards.
///
/// Time:  O(sqrt(n))
/// Space: O(1)

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;
    impl Solution {
        pub fn construct_rectangle(area: i32) -> Vec<i32> {
            let mut w = (area as f64).sqrt() as i32;
            while area % w != 0 {
                w -= 1;
            }
            vec![area / w, w]
        }
    }
}

/// AI solution module — start from floor(sqrt(area)) and decrement W
/// until area % W == 0.
///
/// Time:  O(sqrt(n))
/// Space: O(1)
mod ai_solution {
    pub struct Solution;
    impl Solution {
        pub fn construct_rectangle(area: i32) -> Vec<i32> {
            let mut w = (area as f64).sqrt() as i32;
            while area % w != 0 {
                w -= 1;
            }
            vec![area / w, w]
        }
    }
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;
        #[test]
        fn test_4() {
            assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
        }
        #[test]
        fn test_prime() {
            assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
        }
        #[test]
        fn test_six() {
            assert_eq!(Solution::construct_rectangle(6), vec![3, 2]);
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
    F: Fn(i32) -> Vec<i32>,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(std::hint::black_box(122681)));
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
    bench_solution("My solution", my_solution::Solution::construct_rectangle);
    bench_solution("AI solution", ai_solution::Solution::construct_rectangle);
}

fn main() {
    println!("{:?}", ai_solution::Solution::construct_rectangle(4));
}
