// LeetCode 444: Sequence Reconstruction [Medium]
// https://leetcode.com/problems/sequence-reconstruction/
//
// Given an integer array `nums` which is a permutation of [1, n] and a
// 2D array `sequences` where each sequences[i] is a subsequence of nums,
// check if nums is the only shortest common supersequence of all
// sequences. The shortest common supersequence is a shortest sequence
// such that all sequences[i] are subsequences of it. There could be
// multiple valid supersequences; return true only if nums is the unique
// one.
//
// Examples:
//   nums = [1,2,3], sequences = [[1,2],[1,3]]        -> false
//   nums = [1,2,3], sequences = [[1,2],[1,3],[2,3]]  -> true
//   nums = [1,2,3], sequences = [[1,2]]               -> false
//
// Constraints:
//   n == nums.length
//   1 <= n <= 10^4
//   1 <= sequences.length <= 10^4
//   1 <= sequences[i].length <= 10^4
//   1 <= sum(sequences[i].length) <= 10^5
//   1 <= sequences[i][j] <= n
//   All arrays of sequences are unique
//   sequences[i] is a subsequence of nums
//
// Run:  cargo run --example lc_0444_sequence_reconstruction
// Test: cargo test --example lc_0444_sequence_reconstruction
// Bench: cargo test --example lc_0444_sequence_reconstruction bench -- --nocapture
//
// First benchmark (200K iterations, 2026-04-08):
//   My solution: avg 533.059us
//   AI solution: avg 256.187us
//   Ratio: 2.08x (AI position-check faster)

/// Time: O(n + m) where m = total elements across all sequences
/// Space: O(n + m)
/// Topological sort (Kahn's algorithm / BFS). Build a directed graph
/// from consecutive pairs in each subsequence. If at any point during
/// BFS the queue holds more than 1 node, multiple valid orderings
/// exist, so return false. Also verify the resulting order matches nums.
#[allow(dead_code)]
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Check whether `nums` is the unique shortest supersequence.
        ///
        /// # Arguments
        /// * `nums` - A permutation of [1..=n]
        /// * `sequences` - Subsequences of `nums`
        ///
        /// # Returns
        /// `true` if `nums` is the only shortest common supersequence
        pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
            let n = nums.len();
            // Adjacency list and in-degree array (1-indexed)
            let mut adj: Vec<Vec<i32>> = vec![vec![]; n + 1];
            let mut in_deg: Vec<i32> = vec![0; n + 1];

            for seq in &sequences {
                for pair in seq.windows(2) {
                    // Edge from pair[0] -> pair[1]
                    adj[pair[0] as usize].push(pair[1]);
                    in_deg[pair[1] as usize] += 1;
                }
            }

            // Seed queue with all nodes that have zero in-degree
            let mut queue = std::collections::VecDeque::new();
            for node in 1..=n {
                if in_deg[node] == 0 {
                    queue.push_back(node as i32);
                }
            }

            let mut topo_order: Vec<i32> = Vec::with_capacity(n);
            while !queue.is_empty() {
                // If more than one candidate, ordering is not unique
                if queue.len() > 1 {
                    return false;
                }
                let node = queue.pop_front().expect("queue is not empty");
                topo_order.push(node);
                for &next in &adj[node as usize] {
                    in_deg[next as usize] -= 1;
                    if in_deg[next as usize] == 0 {
                        queue.push_back(next);
                    }
                }
            }

            // All nodes must appear and order must match nums
            topo_order == nums
        }
    }
}

/// Time: O(n + m) where m = total elements across all sequences
/// Space: O(n)
/// Position-based checking: record each element's position in `nums`.
/// For every consecutive pair (a, b) in each sequence, a must appear
/// before b. Then, for uniqueness, verify that every adjacent pair
/// in `nums` is explicitly constrained by at least one sequence
/// (i.e., some sequence contains them as consecutive elements).
#[allow(dead_code)]
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Check whether `nums` is the unique shortest supersequence.
        ///
        /// # Arguments
        /// * `nums` - A permutation of [1..=n]
        /// * `sequences` - Subsequences of `nums`
        ///
        /// # Returns
        /// `true` if `nums` is the only shortest common supersequence
        pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
            let n = nums.len();
            // pos[v] = index of value v in nums (1-indexed values)
            let mut pos = vec![0usize; n + 1];
            for (i, &v) in nums.iter().enumerate() {
                pos[v as usize] = i;
            }

            // confirmed[i] = true means the adjacency (nums[i], nums[i+1])
            // is witnessed by at least one sequence containing them
            // consecutively.
            let mut confirmed = vec![false; n];

            for seq in &sequences {
                for pair in seq.windows(2) {
                    let (a, b) = (pair[0] as usize, pair[1] as usize);
                    // a must appear before b in nums for it to be valid
                    if pos[a] >= pos[b] {
                        return false;
                    }
                    // If a and b are adjacent in nums, mark that edge
                    if pos[a] + 1 == pos[b] {
                        confirmed[pos[a]] = true;
                    }
                }
            }

            // Every adjacent pair in nums must be confirmed
            // (confirmed[n-1] is unused / the last element has no successor)
            confirmed[..n.saturating_sub(1)].iter().all(|&c| c)
        }
    }
}

fn main() {
    println!("LeetCode 444: Sequence Reconstruction");
    let cases: Vec<(Vec<i32>, Vec<Vec<i32>>, bool)> = vec![
        (vec![1, 2, 3], vec![vec![1, 2], vec![1, 3]], false),
        (
            vec![1, 2, 3],
            vec![vec![1, 2], vec![1, 3], vec![2, 3]],
            true,
        ),
        (vec![1, 2, 3], vec![vec![1, 2]], false),
    ];
    for (nums, seqs, expected) in &cases {
        let r1 = my_solution::Solution::sequence_reconstruction(nums.clone(), seqs.clone());
        let r2 = ai_solution::Solution::sequence_reconstruction(nums.clone(), seqs.clone());
        println!("  nums={nums:?}, seqs={seqs:?} => my={r1}, ai={r2} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_both {
        ($name:ident, $nums:expr, $seqs:expr, $expected:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    assert_eq!(
                        my_solution::Solution::sequence_reconstruction($nums, $seqs),
                        $expected
                    );
                }
                #[test]
                fn ai() {
                    assert_eq!(
                        ai_solution::Solution::sequence_reconstruction($nums, $seqs),
                        $expected
                    );
                }
            }
        };
    }

    test_both!(example1, vec![1, 2, 3], vec![vec![1, 2], vec![1, 3]], false);
    test_both!(
        example2,
        vec![1, 2, 3],
        vec![vec![1, 2], vec![1, 3], vec![2, 3]],
        true
    );
    test_both!(example3, vec![1, 2, 3], vec![vec![1, 2]], false);
    test_both!(single_element, vec![1], vec![vec![1]], true);
    test_both!(two_elements, vec![1, 2], vec![vec![1, 2]], true);
    test_both!(two_el_no_edge, vec![1, 2], vec![vec![1], vec![2]], false);
    test_both!(
        longer_chain,
        vec![4, 1, 5, 2, 6, 3],
        vec![vec![5, 2, 6, 3], vec![4, 1, 5, 2]],
        true
    );
    test_both!(
        disjoint_seqs,
        vec![1, 2, 3, 4],
        vec![vec![1, 2], vec![3, 4]],
        false
    );
    test_both!(
        fully_covered,
        vec![1, 2, 3, 4],
        vec![vec![1, 2, 3, 4]],
        true
    );
}

#[cfg(test)]
fn bench_solution<F>(name: &str, f: F)
where
    F: Fn(Vec<i32>, Vec<Vec<i32>>) -> bool,
{
    let iterations = 200_000;
    let n = 5_000;
    let nums: Vec<i32> = (1..=n).collect();
    // Build sequences that fully constrain the ordering
    let seqs: Vec<Vec<i32>> = nums.windows(2).map(|w| w.to_vec()).collect();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(
            std::hint::black_box(nums.clone()),
            std::hint::black_box(seqs.clone()),
        ));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    bench_solution(
        "My solution",
        my_solution::Solution::sequence_reconstruction,
    );
    bench_solution(
        "AI solution",
        ai_solution::Solution::sequence_reconstruction,
    );
}
