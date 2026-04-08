// LeetCode 433: Minimum Genetic Mutation [Medium]
// https://leetcode.com/problems/minimum-genetic-mutation/
//
// A gene string can be represented by an 8-character long string, with choices
// from 'A', 'C', 'G', and 'T'. Given startGene, endGene, and a gene bank,
// return the minimum number of mutations needed to mutate from startGene to
// endGene. Each mutation changes exactly one character. A gene must be in the
// bank to be valid. Return -1 if no such mutation path exists.
//
// Examples:
//   startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"] -> 1
//   startGene = "AACCGGTT", endGene = "AAACGGTA",
//     bank = ["AACCGGTA","AACCGCTA","AAACGGTA"] -> 2
//
// Constraints:
//   0 <= bank.length <= 10
//   startGene.length == endGene.length == bank[i].length == 8
//   startGene, endGene, and bank[i] consist of only 'A', 'C', 'G', 'T'
//
// Run:  cargo run --example lc_0433_minimum_genetic_mutation
// Test: cargo test --example lc_0433_minimum_genetic_mutation
// Bench: cargo test --example lc_0433_minimum_genetic_mutation bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 1.014us
//   AI solution: avg 1.023us
//   Ratio: 0.99x (My ~= AI)

/// My solution module — BFS approach.
///
/// BFS from startGene, exploring each bank gene that differs by exactly 1 char.
/// Track visited genes to avoid cycles. Return depth when endGene is reached.
///
/// Time:  O(n * m * C) -- n = bank length, m = gene length (8), C = charset (4)
/// Space: O(n * m)     -- visited set and queue

#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum number of mutations from start to end.
        ///
        /// # Arguments
        ///
        /// * `start_gene` - Starting gene string
        /// * `end_gene` - Target gene string
        /// * `bank` - Valid gene mutations
        ///
        /// # Returns
        ///
        /// Minimum mutations, or -1 if impossible
        pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
            use std::collections::{HashSet, VecDeque};
            let mut q = VecDeque::new();
            q.push_back((start_gene.clone(), 0));
            let mut vis = HashSet::new();
            vis.insert(start_gene);
            while let Some((gene, depth)) = q.pop_front() {
                if gene == end_gene {
                    return depth;
                }
                for next in &bank {
                    let mut c = 2;
                    for k in 0..8 {
                        if gene.as_bytes()[k] != next.as_bytes()[k] {
                            c -= 1;
                        }
                        if c == 0 {
                            break;
                        }
                    }
                    if c > 0 && !vis.contains(next) {
                        vis.insert(next.clone());
                        q.push_back((next.clone(), depth + 1));
                    }
                }
            }
            -1
        }
    }
}

/// AI solution module — BFS with byte comparison and early exit.
///
/// Same BFS strategy but uses byte slices for faster comparison
/// and counts differences with an early-exit threshold of 2.
///
/// Time:  O(n * m * C) -- n = bank length, m = gene length (8), C = charset (4)
/// Space: O(n * m)     -- visited set and queue
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Find the minimum number of mutations from start to end.
        ///
        /// # Arguments
        ///
        /// * `start_gene` - Starting gene string
        /// * `end_gene` - Target gene string
        /// * `bank` - Valid gene mutations
        ///
        /// # Returns
        ///
        /// Minimum mutations, or -1 if impossible
        pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
            use std::collections::{HashSet, VecDeque};

            let mut q = VecDeque::new();
            q.push_back((start_gene.clone(), 0));
            let mut vis = HashSet::new();
            vis.insert(start_gene);

            while let Some((gene, depth)) = q.pop_front() {
                if gene == end_gene {
                    return depth;
                }
                for next in &bank {
                    // Count character differences, bail early if > 1
                    let mut diff = 0;
                    for k in 0..8 {
                        if gene.as_bytes()[k] != next.as_bytes()[k] {
                            diff += 1;
                        }
                        if diff > 1 {
                            break;
                        }
                    }
                    if diff == 1 && !vis.contains(next) {
                        vis.insert(next.clone());
                        q.push_back((next.clone(), depth + 1));
                    }
                }
            }
            -1
        }
    }
}

/// Generate tests for both solution modules.
#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::$module::Solution;

        #[test]
        fn test_one_mutation() {
            assert_eq!(
                Solution::min_mutation(
                    "AACCGGTT".to_string(),
                    "AACCGGTA".to_string(),
                    vec!["AACCGGTA".to_string()]
                ),
                1
            );
        }

        #[test]
        fn test_two_mutations() {
            assert_eq!(
                Solution::min_mutation(
                    "AACCGGTT".to_string(),
                    "AAACGGTA".to_string(),
                    vec![
                        "AACCGGTA".to_string(),
                        "AACCGCTA".to_string(),
                        "AAACGGTA".to_string()
                    ]
                ),
                2
            );
        }

        #[test]
        fn test_no_path() {
            assert_eq!(
                Solution::min_mutation(
                    "AACCGGTT".to_string(),
                    "AAACGGTA".to_string(),
                    vec!["AACCGGTA".to_string()]
                ),
                -1
            );
        }

        #[test]
        fn test_empty_bank() {
            assert_eq!(
                Solution::min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), vec![]),
                -1
            );
        }

        #[test]
        fn test_start_equals_end() {
            assert_eq!(
                Solution::min_mutation(
                    "AACCGGTT".to_string(),
                    "AACCGGTT".to_string(),
                    vec!["AACCGGTA".to_string()]
                ),
                0
            );
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
    F: Fn(String, String, Vec<String>) -> i32,
{
    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box("AACCGGTT".to_string()),
            std::hint::black_box("AAACGGTA".to_string()),
            std::hint::black_box(vec![
                "AACCGGTA".to_string(),
                "AACCGCTA".to_string(),
                "AAACGGTA".to_string(),
            ]),
        ));
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
    bench_solution("My solution", |s, e, b| {
        my_solution::Solution::min_mutation(s, e, b)
    });
    bench_solution("AI solution", |s, e, b| {
        ai_solution::Solution::min_mutation(s, e, b)
    });
}

fn main() {
    let result = ai_solution::Solution::min_mutation(
        "AACCGGTT".to_string(),
        "AAACGGTA".to_string(),
        vec![
            "AACCGGTA".to_string(),
            "AACCGCTA".to_string(),
            "AAACGGTA".to_string(),
        ],
    );
    println!("Minimum mutations: {}", result);
}
