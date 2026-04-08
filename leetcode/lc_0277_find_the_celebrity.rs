// LeetCode 277: Find the Celebrity [Medium] (Premium)
// https://leetcode.com/problems/find-the-celebrity/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 0.094us
//   AI solution avg: 0.089us
//   Ratio: AI 1.05x faster
//
// There are n people at a party. A celebrity is someone who:
//   - Everyone else knows them.
//   - They don't know anyone else.
// Given a `knows(a, b)` API, find the celebrity or return -1.
//
// Example 1: graph = [[1,1,0],[0,1,0],[1,1,1]] -> 1
// Example 2: graph = [[1,0,1],[1,1,0],[0,1,1]] -> -1
//
// Constraints:
//   - 2 <= n <= 100
//   - graph[i][i] == 1
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0277.Find%20the%20Celebrity/README_EN.md
//
// Run:  cargo run --example lc_0277_find_the_celebrity
// Test: cargo test --example lc_0277_find_the_celebrity
// Bench: cargo test --example lc_0277_find_the_celebrity bench -- --nocapture

/// Simulated `knows` API backed by an adjacency matrix.
/// graph[a][b] == true means person a knows person b.
fn knows(graph: &[Vec<bool>], a: usize, b: usize) -> bool {
    graph[a][b]
}

/// My solution — Two-pass candidate elimination (from doocs).
///
/// Pass 1: Start with candidate = 0. For each i in 1..n, if candidate
/// knows i, then candidate can't be the celebrity — update candidate to i.
/// After this pass, the candidate is the only possible celebrity.
///
/// Pass 2: Verify the candidate by checking that they know no one else
/// and everyone else knows them.
///
/// Time:  O(n) — at most 3(n-1) calls to `knows`
/// Space: O(1)
mod my_solution {
    use super::knows;

    pub struct Solution;

    impl Solution {
        pub fn find_celebrity(graph: &[Vec<bool>], n: usize) -> i32 {
            // Pass 1: find candidate
            let mut candidate = 0;
            for i in 1..n {
                if knows(graph, candidate, i) {
                    candidate = i;
                }
            }
            // Pass 2: verify candidate
            for i in 0..n {
                if i != candidate {
                    // Celebrity must not know i, and i must know celebrity
                    if knows(graph, candidate, i) || !knows(graph, i, candidate) {
                        return -1;
                    }
                }
            }
            candidate as i32
        }
    }
}

/// AI solution — Two-pointer elimination from both ends.
///
/// Use two pointers lo and hi starting at 0 and n-1. If lo knows hi,
/// lo can't be celebrity → lo++. Otherwise hi can't be celebrity → hi--.
/// When they meet, verify the candidate. Achieves the same O(n) but
/// converges from both ends.
///
/// Time:  O(n)
/// Space: O(1)
mod ai_solution {
    use super::knows;

    pub struct Solution;

    impl Solution {
        pub fn find_celebrity(graph: &[Vec<bool>], n: usize) -> i32 {
            let mut lo = 0;
            let mut hi = n - 1;
            // Each comparison eliminates one candidate
            while lo < hi {
                if knows(graph, lo, hi) {
                    lo += 1; // lo knows hi → lo is not celebrity
                } else {
                    hi -= 1; // lo doesn't know hi → hi is not celebrity
                }
            }
            // lo == hi is the only candidate; verify
            let candidate = lo;
            for i in 0..n {
                if i != candidate {
                    if knows(graph, candidate, i) || !knows(graph, i, candidate) {
                        return -1;
                    }
                }
            }
            candidate as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a bool adjacency matrix from a nested i32 array.
    fn to_graph(raw: &[&[i32]]) -> Vec<Vec<bool>> {
        raw.iter()
            .map(|row| row.iter().map(|&v| v == 1).collect())
            .collect()
    }

    macro_rules! test_celebrity {
        ($name:ident, $raw:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let graph = to_graph(&$raw);
                let n = graph.len();
                assert_eq!(
                    my_solution::Solution::find_celebrity(&graph, n),
                    $expected,
                    "my_solution failed"
                );
                assert_eq!(
                    ai_solution::Solution::find_celebrity(&graph, n),
                    $expected,
                    "ai_solution failed"
                );
            }
        };
    }

    // Example 1: person 1 is celebrity
    test_celebrity!(test_example_1, [&[1, 1, 0][..], &[0, 1, 0], &[1, 1, 1]], 1);

    // Example 2: no celebrity
    test_celebrity!(test_example_2, [&[1, 0, 1][..], &[1, 1, 0], &[0, 1, 1]], -1);

    // 2 people, person 1 is celebrity
    test_celebrity!(test_two_people_celebrity, [&[1, 1][..], &[0, 1]], 1);

    // 2 people, both know each other → no celebrity
    test_celebrity!(test_two_mutual, [&[1, 1][..], &[1, 1]], -1);

    // 2 people, neither knows the other → no celebrity
    test_celebrity!(test_two_strangers, [&[1, 0][..], &[0, 1]], -1);

    // 4 people, person 2 is celebrity
    test_celebrity!(
        test_four_person_2,
        [
            &[1, 0, 1, 0][..],
            &[0, 1, 1, 0],
            &[0, 0, 1, 0],
            &[1, 0, 1, 1]
        ],
        2
    );

    // Everyone knows everyone → no celebrity (celebrity must not know others)
    test_celebrity!(
        test_all_know_all,
        [&[1, 1, 1][..], &[1, 1, 1], &[1, 1, 1]],
        -1
    );

    // Person 0 is celebrity
    test_celebrity!(test_celeb_0, [&[1, 0, 0][..], &[1, 1, 0], &[1, 0, 1]], 0);

    // Person n-1 is celebrity
    test_celebrity!(test_celeb_last, [&[1, 0, 1][..], &[0, 1, 1], &[0, 0, 1]], 2);

    // No one knows anyone (diagonal only) → no celebrity
    test_celebrity!(test_isolated, [&[1, 0, 0][..], &[0, 1, 0], &[0, 0, 1]], -1);

    // 5-person graph, person 3 is celebrity
    test_celebrity!(
        test_five_person,
        [
            &[1, 0, 0, 1, 0][..],
            &[0, 1, 0, 1, 0],
            &[0, 0, 1, 1, 0],
            &[0, 0, 0, 1, 0],
            &[0, 0, 0, 1, 1]
        ],
        3
    );
}

fn main() {
    println!("LeetCode 277: Find the Celebrity");
    let graph = vec![
        vec![true, true, false],
        vec![false, true, false],
        vec![true, true, true],
    ];
    println!(
        "my_solution: find_celebrity(3) = {}",
        my_solution::Solution::find_celebrity(&graph, 3)
    );
    println!(
        "ai_solution: find_celebrity(3) = {}",
        ai_solution::Solution::find_celebrity(&graph, 3)
    );
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        // Build a 20-person graph where person 10 is the celebrity
        let n = 20;
        let celebrity = 10;
        let graph: Vec<Vec<bool>> = (0..n)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        if i == j {
                            true
                        } else if i == celebrity {
                            false // celebrity knows no one
                        } else {
                            j == celebrity // everyone else knows celebrity
                        }
                    })
                    .collect()
            })
            .collect();
        let iterations = 100_000;

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(my_solution::Solution::find_celebrity(&graph, n));
            std::hint::black_box(ai_solution::Solution::find_celebrity(&graph, n));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(my_solution::Solution::find_celebrity(&graph, n));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(ai_solution::Solution::find_celebrity(&graph, n));
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
