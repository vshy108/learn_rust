// LeetCode 84: Largest Rectangle in Histogram [Hard]
// https://leetcode.com/problems/largest-rectangle-in-histogram/
//
// First benchmark: 100_000 iterations, 2025-07-08
//   My solution avg: 6.065us
//   AI solution avg: 2.140us
//   Ratio: AI 2.83x faster
//
// Given an array of integers heights representing the histogram's bar height
// where the width of each bar is 1, return the area of the largest rectangle.
//
// Constraints: 1 <= heights.length <= 10^5, 0 <= heights[i] <= 10^4
//
// Run:  cargo run --example lc_0084_largest_rectangle_histogram
// Test: cargo test --example lc_0084_largest_rectangle_histogram
// Bench: cargo test --example lc_0084_largest_rectangle_histogram bench -- --nocapture

/// My solution — brute force checking all pairs.
///
/// For each bar, expand left and right while height >= current bar.
///
/// Time: O(n^2).
/// Space: O(1).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the area of the largest rectangle in the histogram.
        ///
        /// # Arguments
        /// * `heights` - bar heights
        ///
        /// # Returns
        /// Maximum rectangle area
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            let n = heights.len();
            let mut max_area = 0;
            for i in 0..n {
                let h = heights[i];
                // Expand left
                let mut left = i;
                while left > 0 && heights[left - 1] >= h {
                    left -= 1;
                }
                // Expand right
                let mut right = i;
                while right < n - 1 && heights[right + 1] >= h {
                    right += 1;
                }
                let area = h * (right - left + 1) as i32;
                if area > max_area {
                    max_area = area;
                }
            }
            max_area
        }
    }
}

/// AI solution — monotonic stack.
///
/// Maintain a stack of increasing heights. When a smaller bar is found,
/// pop and calculate areas using the popped bar as the shortest.
///
/// Time: O(n).
/// Space: O(n).
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the area of the largest rectangle in the histogram.
        ///
        /// # Arguments
        /// * `heights` - bar heights
        ///
        /// # Returns
        /// Maximum rectangle area
        pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
            let n = heights.len();
            let mut stack: Vec<usize> = Vec::with_capacity(n);
            let mut max_area = 0i32;

            for i in 0..=n {
                // Use height 0 as sentinel to flush remaining bars
                let curr_h = if i < n { heights[i] } else { 0 };
                while let Some(&top) = stack.last() {
                    if heights[top] <= curr_h {
                        break;
                    }
                    stack.pop();
                    let h = heights[top];
                    // Width extends from element after new stack top to i-1
                    let w = if stack.is_empty() {
                        i as i32
                    } else {
                        (i - stack.last().expect("checked") - 1) as i32
                    };
                    max_area = max_area.max(h * w);
                }
                stack.push(i);
            }
            max_area
        }
    }
}

fn main() {
    let heights = vec![2, 1, 5, 6, 2, 3];
    let my = my_solution::Solution::largest_rectangle_area(heights.clone());
    let ai = ai_solution::Solution::largest_rectangle_area(heights);
    println!("my: {my}, ai: {ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_largest_rect {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_example1() {
                    assert_eq!(
                        <$solution>::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]),
                        10
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(<$solution>::largest_rectangle_area(vec![2, 4]), 4);
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::largest_rectangle_area(vec![5]), 5);
                }

                #[test]
                fn test_ascending() {
                    assert_eq!(
                        <$solution>::largest_rectangle_area(vec![1, 2, 3, 4, 5]),
                        9 // 3*3
                    );
                }

                #[test]
                fn test_descending() {
                    assert_eq!(<$solution>::largest_rectangle_area(vec![5, 4, 3, 2, 1]), 9);
                }

                #[test]
                fn test_uniform() {
                    assert_eq!(<$solution>::largest_rectangle_area(vec![3, 3, 3, 3]), 12);
                }
            }
        };
    }

    test_largest_rect!(my, crate::my_solution::Solution);
    test_largest_rect!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let heights: Vec<i32> = (1..=50).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::largest_rectangle_area(
                std::hint::black_box(heights.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::largest_rectangle_area(
                std::hint::black_box(heights.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution: {:.3}us avg ({:.0}ms total)",
            my_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            my_duration.as_millis(),
        );
        println!(
            "AI solution: {:.3}us avg ({:.0}ms total)",
            ai_duration.as_secs_f64() * 1_000_000.0 / iterations as f64,
            ai_duration.as_millis(),
        );

        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.1 {
            println!("AI is {ratio:.2}x faster");
        } else if ratio < 0.9 {
            println!("My solution is {:.2}x faster", 1.0 / ratio);
        } else {
            println!("~tied (ratio: {ratio:.2})");
        }
    }
}
