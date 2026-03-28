// LeetCode 274: H-Index [Medium]
// https://leetcode.com/problems/h-index/
//
// Given an array of integers citations where citations[i] is the number
// of citations a researcher received for their ith paper, return the
// researcher's h-index.
//
// The h-index is defined as the maximum value of h such that the given
// researcher has published at least h papers that have each been cited
// at least h times.
//
// Examples:
//   citations = [3,0,6,1,5] -> 3
//     (3 papers with >= 3 citations: [3, 6, 5])
//   citations = [1,3,1]     -> 1
//
// Constraints:
//   - n == citations.length
//   - 1 <= n <= 5000
//   - 0 <= citations[i] <= 1000
//
// Run:  cargo run --example lc_0274_h_index
// Test: cargo test --example lc_0274_h_index
// Bench: cargo test --example lc_0274_h_index bench -- --nocapture

/// My solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Compute the h-index from the citation counts.
        ///
        /// # Arguments
        ///
        /// * `citations` - Citation counts per paper
        ///
        /// # Returns
        ///
        /// The h-index value
        pub fn h_index(citations: Vec<i32>) -> i32 {
            // n = citations.length
            // citations.sort!.reverse!
            // h = 0
            // citations.each_with_index do |c, i|
            //   h = i + 1 if c >= i + 1
            // end
            // h
            todo!()
        }
    }
}

/// AI solution module.
///
/// Time:  O(?)
/// Space: O(?)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Compute the h-index from the citation counts.
        ///
        /// # Arguments
        ///
        /// * `citations` - Citation counts per paper
        ///
        /// # Returns
        ///
        /// The h-index value
        pub fn h_index(citations: Vec<i32>) -> i32 {
            todo!()
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![(vec![3, 0, 6, 1, 5], 3), (vec![1, 3, 1], 1)];

    println!("--- My Solution ---");
    for (citations, expected) in &test_cases {
        let result = my_solution::Solution::h_index(citations.clone());
        println!("  citations={citations:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (citations, expected) in &test_cases {
        let result = ai_solution::Solution::h_index(citations.clone());
        println!("  citations={citations:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_h_index {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_basic() {
                    assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
                }

                #[test]
                fn test_small() {
                    assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
                }

                #[test]
                fn test_all_zero() {
                    assert_eq!(Solution::h_index(vec![0, 0, 0]), 0);
                }

                #[test]
                fn test_single_paper_high_citations() {
                    assert_eq!(Solution::h_index(vec![100]), 1);
                }

                #[test]
                fn test_single_paper_zero() {
                    assert_eq!(Solution::h_index(vec![0]), 0);
                }

                #[test]
                fn test_all_high() {
                    assert_eq!(Solution::h_index(vec![10, 10, 10, 10]), 4);
                }

                #[test]
                fn test_descending() {
                    assert_eq!(Solution::h_index(vec![5, 4, 3, 2, 1, 0]), 3);
                }
            }
        };
    }

    test_h_index!(my_tests, crate::my_solution::Solution);
    test_h_index!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<i32>> = vec![
            vec![3, 0, 6, 1, 5],
            vec![1, 3, 1],
            vec![10, 10, 10, 10],
            vec![5, 4, 3, 2, 1, 0],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for citations in &inputs {
                std::hint::black_box(crate::my_solution::Solution::h_index(std::hint::black_box(
                    citations.clone(),
                )));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for citations in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::h_index(std::hint::black_box(
                    citations.clone(),
                )));
            }
        }
        let ai_duration = start.elapsed();

        println!("\n=== Benchmark ({iterations} iterations) ===");
        println!(
            "  My solution: {:?} (avg {:?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "  AI solution: {:?} (avg {:?})",
            ai_duration,
            ai_duration / iterations
        );

        if my_duration < ai_duration {
            let ratio = ai_duration.as_nanos() as f64 / my_duration.as_nanos() as f64;
            println!("  -> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_duration.as_nanos() as f64 / ai_duration.as_nanos() as f64;
            println!("  -> AI solution is {ratio:.2}x faster");
        }
    }
}
