// LeetCode 406: Queue Reconstruction by Height [Medium]
// https://leetcode.com/problems/queue-reconstruction-by-height/
// Benchmark: 100000 iterations
//   My solution (sort + Vec::insert):  avg 12.71us
//   AI solution (sort + splice):       avg 13.54us
//   -> Roughly tied (My 1.07x faster)
//
// Given array `people` where people[i] = [h_i, k_i]:
//   h_i = height, k_i = number of people in front with height >= h_i.
// Reconstruct the queue.
//
// Examples:
//   [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
//     -> [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
//
// Constraints:
//   1 <= people.length <= 2000
//   0 <= h_i <= 10^6
//   0 <= k_i < people.length
//
// Run:  cargo run --example lc_0406_queue_reconstruction_by_height
// Test: cargo test --example lc_0406_queue_reconstruction_by_height
// Bench: cargo test --example lc_0406_queue_reconstruction_by_height bench -- --nocapture

/// My solution module — sort descending by height, insert by k index.
///
/// Greedy: sort people by height descending (ties broken by k ascending).
/// Then insert each person at their k index. Taller people are placed
/// first, so inserting shorter people later doesn't disturb the k counts
/// of already-placed taller people.
///
/// Time:  O(n^2) -- each insert can shift up to n elements
/// Space: O(n)
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Reconstruct the queue from people's height and k values.
        ///
        /// # Arguments
        ///
        /// * `people` - Each element is [height, k]
        ///
        /// # Returns
        ///
        /// The reconstructed queue
        pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            // Sort by height descending; if same height, k ascending
            people.sort_unstable_by(|a, b| b[0].cmp(&a[0]).then(a[1].cmp(&b[1])));

            let mut result: Vec<Vec<i32>> = Vec::with_capacity(people.len());
            for p in people {
                let k = p[1] as usize;
                // Insert at position k -- all previously placed people
                // are taller or same height, so this person sees exactly
                // k of them in front.
                result.insert(k, p);
            }

            result
        }
    }
}

/// AI solution module — same greedy, uses `splice` for insertion.
///
/// Identical algorithm but uses `splice` with an empty range to insert,
/// which may have different performance characteristics on some allocators.
///
/// Time:  O(n^2)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Reconstruct the queue from people's height and k values.
        ///
        /// # Arguments
        ///
        /// * `people` - Each element is [height, k]
        ///
        /// # Returns
        ///
        /// The reconstructed queue
        pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            // Sort by height descending; ties broken by k ascending
            people.sort_unstable_by(|a, b| {
                if a[0] == b[0] {
                    a[1].cmp(&b[1])
                } else {
                    b[0].cmp(&a[0])
                }
            });

            let mut result: Vec<Vec<i32>> = Vec::with_capacity(people.len());
            for p in people {
                let k = p[1] as usize;
                // `splice` replaces range k..k (empty range) with one element
                result.splice(k..k, std::iter::once(p));
            }

            result
        }
    }
}

fn main() {
    let cases = [
        (
            vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2],
            ],
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1],
            ],
        ),
        (
            vec![
                vec![6, 0],
                vec![5, 0],
                vec![4, 0],
                vec![3, 2],
                vec![2, 2],
                vec![1, 4],
            ],
            vec![
                vec![4, 0],
                vec![5, 0],
                vec![2, 2],
                vec![3, 2],
                vec![1, 4],
                vec![6, 0],
            ],
        ),
    ];

    println!("--- My Solution ---");
    for (people, expected) in &cases {
        let result = my_solution::Solution::reconstruct_queue(people.clone());
        println!("  {people:?} -> {result:?}");
        println!("  expected: {expected:?}");
    }

    println!("--- AI Solution ---");
    for (people, expected) in &cases {
        let result = ai_solution::Solution::reconstruct_queue(people.clone());
        println!("  {people:?} -> {result:?}");
        println!("  expected: {expected:?}");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_reconstruct {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    let people = vec![
                        vec![7, 0],
                        vec![4, 4],
                        vec![7, 1],
                        vec![5, 0],
                        vec![6, 1],
                        vec![5, 2],
                    ];
                    let expected = vec![
                        vec![5, 0],
                        vec![7, 0],
                        vec![5, 2],
                        vec![6, 1],
                        vec![4, 4],
                        vec![7, 1],
                    ];
                    assert_eq!(Solution::reconstruct_queue(people), expected);
                }

                #[test]
                fn test_example2() {
                    let people = vec![
                        vec![6, 0],
                        vec![5, 0],
                        vec![4, 0],
                        vec![3, 2],
                        vec![2, 2],
                        vec![1, 4],
                    ];
                    let expected = vec![
                        vec![4, 0],
                        vec![5, 0],
                        vec![2, 2],
                        vec![3, 2],
                        vec![1, 4],
                        vec![6, 0],
                    ];
                    assert_eq!(Solution::reconstruct_queue(people), expected);
                }

                #[test]
                fn test_single() {
                    let people = vec![vec![1, 0]];
                    assert_eq!(Solution::reconstruct_queue(people), vec![vec![1, 0]]);
                }

                #[test]
                fn test_all_same_height() {
                    let people = vec![vec![5, 0], vec![5, 1], vec![5, 2]];
                    let expected = vec![vec![5, 0], vec![5, 1], vec![5, 2]];
                    assert_eq!(Solution::reconstruct_queue(people), expected);
                }
            }
        };
    }

    test_reconstruct!(my_tests, crate::my_solution::Solution);
    test_reconstruct!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;

        // Build a large input
        let mut people = Vec::new();
        for h in (1..=20).rev() {
            for k in 0..h.min(10) {
                people.push(vec![h * 10, k]);
            }
        }

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::reconstruct_queue(
                std::hint::black_box(people.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::reconstruct_queue(
                std::hint::black_box(people.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64 / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64 / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
