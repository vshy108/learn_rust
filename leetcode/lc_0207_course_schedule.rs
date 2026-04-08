// LeetCode 207: Course Schedule [Medium]
// https://leetcode.com/problems/course-schedule/
//
// First benchmark: 100_000 iterations, 2026-04-07
//   My solution avg: 11.439us
//   AI solution avg: 10.956us
//   Ratio: ~tied
//
// There are numCourses courses (0 to numCourses-1). prerequisites[i] = [a, b]
// means you must take course b before course a.
// Return true if you can finish all courses (i.e., no cycle).
//
// Constraints: 1 <= numCourses <= 2000, 0 <= prerequisites.length <= 5000
//
// Run:  cargo run --example lc_0207_course_schedule
// Test: cargo test --example lc_0207_course_schedule
// Bench: cargo test --example lc_0207_course_schedule bench -- --nocapture

/// My solution — todo.
///
/// Time: O(?).
/// Space: O(?).
mod my_solution {
    pub struct Solution;

    impl Solution {
        /// Return true if all courses can be finished.
        pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            // DFS-based cycle detection
            let n = num_courses as usize;
            let mut adj = vec![vec![]; n];
            for e in &prerequisites {
                adj[e[1] as usize].push(e[0] as usize);
            }
            // 0 = unvisited, 1 = visiting, 2 = visited
            let mut state = vec![0u8; n];
            fn has_cycle(node: usize, adj: &[Vec<usize>], state: &mut [u8]) -> bool {
                state[node] = 1;
                for &next in &adj[node] {
                    if state[next] == 1 {
                        return true;
                    }
                    if state[next] == 0 && has_cycle(next, adj, state) {
                        return true;
                    }
                }
                state[node] = 2;
                false
            }
            for i in 0..n {
                if state[i] == 0 && has_cycle(i, &adj, &mut state) {
                    return false;
                }
            }
            true
        }
    }
}

/// AI solution — topological sort (Kahn's algorithm, BFS).
///
/// Build adjacency list and in-degree array. Start with 0-in-degree nodes.
/// Process each, decrement in-degree of neighbors. If all nodes processed,
/// no cycle exists.
///
/// Time: O(V + E).
/// Space: O(V + E).
mod ai_solution {
    use std::collections::VecDeque;

    pub struct Solution;

    impl Solution {
        /// Kahn's algorithm: BFS-based topological sort.
        pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            let n = num_courses as usize;
            let mut adj = vec![vec![]; n];
            let mut in_degree = vec![0usize; n];

            for edge in &prerequisites {
                let (course, prereq) = (edge[0] as usize, edge[1] as usize);
                adj[prereq].push(course);
                in_degree[course] += 1;
            }

            // Start with all courses that have no prerequisites
            let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
            let mut processed = 0;

            while let Some(node) = queue.pop_front() {
                processed += 1;
                for &neighbor in &adj[node] {
                    in_degree[neighbor] -= 1;
                    if in_degree[neighbor] == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
            processed == n
        }
    }
}

fn main() {
    let my = my_solution::Solution::can_finish(2, vec![vec![1, 0]]);
    let ai = ai_solution::Solution::can_finish(2, vec![vec![1, 0]]);
    println!("can finish: my={my}, ai={ai}");
}

#[cfg(test)]
mod tests {
    macro_rules! test_cs {
        ($mod_name:ident, $solution:path) => {
            mod $mod_name {
                #[test]
                fn test_no_cycle() {
                    assert!(<$solution>::can_finish(2, vec![vec![1, 0]]));
                }

                #[test]
                fn test_cycle() {
                    assert!(!<$solution>::can_finish(2, vec![vec![1, 0], vec![0, 1]],));
                }

                #[test]
                fn test_no_prereqs() {
                    assert!(<$solution>::can_finish(3, vec![]));
                }

                #[test]
                fn test_chain() {
                    assert!(<$solution>::can_finish(
                        4,
                        vec![vec![1, 0], vec![2, 1], vec![3, 2]],
                    ));
                }

                #[test]
                fn test_diamond() {
                    assert!(<$solution>::can_finish(
                        4,
                        vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]],
                    ));
                }
            }
        };
    }

    test_cs!(my, crate::my_solution::Solution);
    test_cs!(ai, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        let iterations = 100_000;
        let prereqs: Vec<Vec<i32>> = (0..99).map(|i| vec![i + 1, i]).collect();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::can_finish(
                std::hint::black_box(100),
                std::hint::black_box(prereqs.clone()),
            ));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::can_finish(
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
