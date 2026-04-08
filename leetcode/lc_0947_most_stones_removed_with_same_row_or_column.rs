// LeetCode 947: Most Stones Removed with Same Row or Column [Medium]
// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
//
// On a 2D plane, we place n stones at some integer coordinate points. Each
// coordinate point may have at most one stone.
//
// A stone can be removed if it shares either the same row or the same column
// as another stone that has not been removed.
//
// Given an array `stones` where stones[i] = [xi, yi] represents the location
// of the i-th stone, return the largest possible number of stones that can be
// removed.
//
// Examples:
//   stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]] -> 5
//   stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]       -> 3
//   stones = [[0,0]]                               -> 0
//
// Constraints:
//   - 1 <= stones.length <= 1000
//   - 0 <= xi, yi <= 10^4
//   - No two stones are at the same coordinate point
//
// Run:  cargo run --example lc_0947_most_stones_removed_with_same_row_or_column
// Test: cargo test --example lc_0947_most_stones_removed_with_same_row_or_column
// Bench: cargo test --example lc_0947_most_stones_removed_with_same_row_or_column bench -- --nocapture
//
// --- Benchmark (100_000 iterations, 2026-04-07) ---
// My solution:  4817.93ms (avg 48.17us)
// AI solution:  380.19ms (avg 3.80us)
// -> AI solution is 12.67x faster
// Reason: DFS iterates an adjacency check in O(n) per node, but operates
//   on a small visited vec. Union-Find with HashMap has per-stone constant
//   cost amortized, but the HashMap overhead per operation is higher for
//   small inputs. AI wins because small test inputs favour the simple DFS.

/// My solution module — Union-Find with bitwise complement for column separation.
///
/// Time:  O(n * α(n)) amortized, effectively O(n)
/// Space: O(n) for the parent map
mod my_solution {
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Return the largest number of stones that can be removed.
        ///
        /// # Arguments
        ///
        /// * `stones` - Array of [x, y] coordinate pairs
        ///
        /// # Returns
        ///
        /// Maximum number of removable stones
        pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
            let mut parent: HashMap<i32, i32> = HashMap::new();

            fn find(parent: &mut HashMap<i32, i32>, x: i32) -> i32 {
                // clippy::map_entry: `if !map.contains_key(&k) { map.insert(k, v); }`
                // does two hash lookups. The entry API does one lookup, then
                // inserts only if the key is absent (Vacant). Faster and idiomatic.
                if let std::collections::hash_map::Entry::Vacant(e) = parent.entry(x) {
                    e.insert(x);
                    return x;
                }
                if parent[&x] != x {
                    let root = find(parent, parent[&x]);
                    parent.insert(x, root);
                }
                parent[&x]
            }

            fn union(parent: &mut HashMap<i32, i32>, x: i32, y: i32) {
                let rx = find(parent, x);
                let ry = find(parent, y);
                if rx != ry {
                    parent.insert(rx, ry);
                }
            }

            for stone in &stones {
                // !y (bitwise NOT) maps column y to a distinct negative space
                union(&mut parent, stone[0], !stone[1]);
            }

            // Count unique components among stones
            let components: HashSet<i32> = stones.iter().map(|s| find(&mut parent, s[0])).collect();
            stones.len() as i32 - components.len() as i32
        }
    }
}

/// AI solution module — DFS connected components.
///
/// Build adjacency: two stones share an edge if they have the same
/// row or column. Then count connected components via DFS.
///
/// Time:  O(n^2)
/// Space: O(n)
mod ai_solution {
    pub struct Solution;

    impl Solution {
        /// Return the largest number of stones that can be removed.
        ///
        /// # Arguments
        ///
        /// * `stones` - Array of [x, y] coordinate pairs
        ///
        /// # Returns
        ///
        /// Maximum number of removable stones
        pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
            let n = stones.len();
            let mut visited = vec![false; n];
            let mut components = 0;
            for i in 0..n {
                if !visited[i] {
                    components += 1;
                    // DFS from stone i
                    let mut stack = vec![i];
                    while let Some(curr) = stack.pop() {
                        if visited[curr] {
                            continue;
                        }
                        visited[curr] = true;
                        for j in 0..n {
                            if !visited[j]
                                && (stones[curr][0] == stones[j][0]
                                    || stones[curr][1] == stones[j][1])
                            {
                                stack.push(j);
                            }
                        }
                    }
                }
            }
            n as i32 - components
        }
    }
}

fn main() {
    let test_cases: Vec<(Vec<Vec<i32>>, i32)> = vec![
        (
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![2, 2],
            ],
            5,
        ),
        (
            vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]],
            3,
        ),
        (vec![vec![0, 0]], 0),
    ];

    println!("--- My Solution ---");
    for (stones, expected) in &test_cases {
        let result = my_solution::Solution::remove_stones(stones.clone());
        println!("  {stones:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (stones, expected) in &test_cases {
        let result = ai_solution::Solution::remove_stones(stones.clone());
        println!("  {stones:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_remove_stones {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use $solution as Solution;

                #[test]
                fn test_example_1() {
                    assert_eq!(
                        Solution::remove_stones(vec![
                            vec![0, 0],
                            vec![0, 1],
                            vec![1, 0],
                            vec![1, 2],
                            vec![2, 1],
                            vec![2, 2],
                        ]),
                        5
                    );
                }

                #[test]
                fn test_example_2() {
                    assert_eq!(
                        Solution::remove_stones(vec![
                            vec![0, 0],
                            vec![0, 2],
                            vec![1, 1],
                            vec![2, 0],
                            vec![2, 2],
                        ]),
                        3
                    );
                }

                #[test]
                fn test_single_stone() {
                    assert_eq!(Solution::remove_stones(vec![vec![0, 0]]), 0);
                }

                #[test]
                fn test_two_stones_same_row() {
                    assert_eq!(Solution::remove_stones(vec![vec![0, 0], vec![0, 1]]), 1);
                }

                #[test]
                fn test_two_stones_same_col() {
                    assert_eq!(Solution::remove_stones(vec![vec![0, 0], vec![1, 0]]), 1);
                }

                #[test]
                fn test_two_stones_no_shared() {
                    assert_eq!(Solution::remove_stones(vec![vec![0, 0], vec![1, 1]]), 0);
                }

                #[test]
                fn test_line_of_stones() {
                    assert_eq!(
                        Solution::remove_stones(vec![
                            vec![0, 0],
                            vec![0, 1],
                            vec![0, 2],
                            vec![0, 3],
                        ]),
                        3
                    );
                }
            }
        };
    }

    test_remove_stones!(my_tests, crate::my_solution::Solution);
    test_remove_stones!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 100_000_u32;
        let inputs: Vec<Vec<Vec<i32>>> = vec![
            vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![1, 2],
                vec![2, 1],
                vec![2, 2],
            ],
            vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]],
            vec![vec![0, 0]],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for stones in &inputs {
                std::hint::black_box(crate::my_solution::Solution::remove_stones(
                    std::hint::black_box(stones.clone()),
                ));
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            for stones in &inputs {
                std::hint::black_box(crate::ai_solution::Solution::remove_stones(
                    std::hint::black_box(stones.clone()),
                ));
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
