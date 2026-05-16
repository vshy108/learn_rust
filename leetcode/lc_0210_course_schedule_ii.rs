// LeetCode 210: Course Schedule II [Medium]
// https://leetcode.com/problems/course-schedule-ii/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 11.242us
//   AI solution avg: 10.895us
//   Ratio: ~tied
//
// There are numCourses courses (0 to numCourses-1). prerequisites[i] = [a, b]
// means you must take b before a. Return a valid ordering to finish all
// courses, or an empty array if impossible.
//
// Constraints: 1 <= numCourses <= 2000, 0 <= prerequisites.length <= numCourses * (numCourses-1)
//
// Run:  cargo run --example lc_0210_course_schedule_ii
// Test: cargo test --example lc_0210_course_schedule_ii
// Bench: cargo test --example lc_0210_course_schedule_ii bench -- --nocapture

/// My solution — DFS topological sort with 3-state colouring;
/// nodes are appended to the order Vec after all descendants are
/// processed, then the Vec is reversed.
///
/// Time: O(V + E).
/// Space: O(V + E) for the adjacency list, state array, and result.
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return a valid course ordering, or empty if impossible.
        pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
            // DFS-based topological sort
            let n = num_courses as usize;
            let mut adj = vec![vec![]; n];
            for e in &prerequisites {
                adj[e[1] as usize].push(e[0] as usize);
            }
            let mut state = vec![0u8; n]; // 0=unvisited, 1=visiting, 2=visited
            let mut order = Vec::with_capacity(n);
            fn dfs(
                node: usize,
                adj: &[Vec<usize>],
                state: &mut [u8],
                order: &mut Vec<i32>,
            ) -> bool {
                state[node] = 1;
                for &next in &adj[node] {
                    if state[next] == 1 {
                        return false;
                    }
                    if state[next] == 0 && !dfs(next, adj, state, order) {
                        return false;
                    }
                }
                state[node] = 2;
                order.push(node as i32);
                true
            }
            for i in 0..n {
                if state[i] == 0 && !dfs(i, &adj, &mut state, &mut order) {
                    return vec![];
                }
            }
            order.reverse();
            order
        }
    }
}

/// AI solution — Kahn's algorithm (BFS topological sort).
///
/// Build adjacency list + in-degree. Process nodes with in-degree 0.
/// If the order has all nodes, return it; otherwise cycle detected.
///
/// Time: O(V + E).
/// Space: O(V + E).
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// BFS topological sort returning a valid ordering.
        pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
            let n = num_courses as usize;
            let mut adj = vec![vec![]; n];
            let mut in_degree = vec![0usize; n];

            for edge in &prerequisites {
                let (course, prereq) = (edge[0] as usize, edge[1] as usize);
                adj[prereq].push(course);
                in_degree[course] += 1;
            }

            let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
            let mut order = Vec::with_capacity(n);

            while let Some(node) = queue.pop_front() {
                order.push(node as i32);
                for &neighbor in &adj[node] {
                    in_degree[neighbor] -= 1;
                    if in_degree[neighbor] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }

            if order.len() == n { order } else { vec![] }
        }
    }
}

fn main() {
    let my =
        my_solution::Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
    let ai =
        ai_solution::Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
    println!("order: my={my:?}, ai={ai:?}");
}

#[cfg(test)]
mod tests {
    /// Validate that ordering respects all prerequisites.
    fn is_valid_order(n: i32, prereqs: &[Vec<i32>], order: &[i32]) -> bool {
        if order.len() != n as usize {
            return false;
        }
        let mut pos = vec![0usize; n as usize];
        for (i, &c) in order.iter().enumerate() {
            pos[c as usize] = i;
        }
        // For each [a, b], b must come before a
        prereqs
            .iter()
            .all(|e| pos[e[1] as usize] < pos[e[0] as usize])
    }

    macro_rules! test_cs2 {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                use super::is_valid_order;

                #[test]
                fn test_example1() {
                    let prereqs = vec![vec![1, 0]];
                    let order = <$solution>::find_order(2, prereqs.clone());
                    assert!(is_valid_order(2, &prereqs, &order));
                }

                #[test]
                fn test_example2() {
                    let prereqs = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
                    let order = <$solution>::find_order(4, prereqs.clone());
                    assert!(is_valid_order(4, &prereqs, &order));
                }

                #[test]
                fn test_no_prereqs() {
                    let order = <$solution>::find_order(3, vec![]);
                    assert_eq!(order.len(), 3);
                }

                #[test]
                fn test_cycle() {
                    let order = <$solution>::find_order(2, vec![vec![1, 0], vec![0, 1]]);
                    assert!(order.is_empty());
                }

                #[test]
                fn test_single() {
                    assert_eq!(<$solution>::find_order(1, vec![]), vec![0]);
                }
            }
        };
    }

    test_cs2!(my, crate::my_solution::Solution);
    test_cs2!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let prereqs: Vec<Vec<i32>> = (0..99).map(|i| vec![i + 1, i]).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::find_order(
                std::hint::black_box(100),
                std::hint::black_box(prereqs.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::find_order(
                std::hint::black_box(100),
                std::hint::black_box(prereqs.clone()),
            ));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        println!("\nBenchmark ({iterations} iterations):");
        println!("  My solution avg: {my_avg:.3}us");
        println!("  AI solution avg: {ai_avg:.3}us");
        if my_avg < ai_avg {
            println!("  Ratio: My {:.2}x faster", ai_avg / my_avg);
        } else {
            println!("  Ratio: AI {:.2}x faster", my_avg / ai_avg);
        }
    }
}
