// LeetCode 399: Evaluate Division [Medium]
// https://leetcode.com/problems/evaluate-division/
// Benchmark: 10000 iterations
//   My solution (BFS weighted graph):     avg 3.16us
//   AI solution (Union-Find with weights): avg 2.52us
//   -> AI solution is 1.26x faster
//
// Given equations like a/b=2.0, b/c=3.0, answer queries like a/c=?
// Build a weighted graph where edges carry division ratios, then BFS/DFS
// to compute transitive ratios. Return -1.0 for unknown variables.
//
// Examples:
//   equations=[["a","b"],["b","c"]], values=[2.0,3.0]
//   queries=[["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
//   -> [6.0, 0.5, -1.0, 1.0, -1.0]
//
// Constraints:
//   - 1 <= equations.length <= 20
//   - 1 <= queries.length <= 20
//   - Ai, Bi, Ci, Di are 1-5 char strings of lowercase letters/digits
//
// Run:  cargo run --example lc_0399_evaluate_division
// Test: cargo test --example lc_0399_evaluate_division
// Bench: cargo test --example lc_0399_evaluate_division bench -- --nocapture

/// My solution module — BFS on weighted graph.
///
/// Build an adjacency list where edge a->b has weight values[i] and
/// b->a has weight 1/values[i]. For each query (c,d), BFS from c to d,
/// multiplying weights along the path.
///
/// Time:  O(Q * (V + E)) where Q = queries, V = variables, E = equations
/// Space: O(V + E) for the graph
mod my_solution {
    use std::collections::{HashMap, HashSet, VecDeque};

    pub struct Solution;

    impl Solution {
        /// Evaluate each query as a division between two variables.
        ///
        /// # Arguments
        ///
        /// * `equations` - Pairs [Ai, Bi]
        /// * `values` - values[i] = Ai / Bi
        /// * `queries` - Pairs [Ci, Di] to evaluate
        ///
        /// # Returns
        ///
        /// Result for each query, or -1.0 if undetermined
        pub fn calc_equation(
            equations: Vec<Vec<String>>,
            values: Vec<f64>,
            queries: Vec<Vec<String>>,
        ) -> Vec<f64> {
            // Build adjacency list: variable -> [(neighbor, weight)]
            let mut graph: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
            for (eq, &val) in equations.iter().zip(values.iter()) {
                let a = eq[0].as_str();
                let b = eq[1].as_str();
                graph.entry(a).or_default().push((b, val));
                graph.entry(b).or_default().push((a, 1.0 / val));
            }

            queries
                .iter()
                .map(|q| {
                    let c = q[0].as_str();
                    let d = q[1].as_str();
                    Self::bfs(&graph, c, d)
                })
                .collect()
        }

        /// BFS from `src` to `dst`, returning the product of edge weights.
        fn bfs(graph: &HashMap<&str, Vec<(&str, f64)>>, src: &str, dst: &str) -> f64 {
            if !graph.contains_key(src) || !graph.contains_key(dst) {
                return -1.0;
            }
            if src == dst {
                return 1.0;
            }
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((src, 1.0));
            visited.insert(src);

            while let Some((node, product)) = queue.pop_front() {
                if let Some(neighbors) = graph.get(node) {
                    for &(next, weight) in neighbors {
                        if next == dst {
                            return product * weight;
                        }
                        if visited.insert(next) {
                            queue.push_back((next, product * weight));
                        }
                    }
                }
            }
            -1.0
        }
    }
}

/// AI solution module — Union-Find with weighted edges.
///
/// Use a weighted union-find where each node stores its ratio to its
/// root. For a/b = v, union a and b with the appropriate weight.
/// For a query c/d, if they share a root, return weight(c)/weight(d).
///
/// Time:  O((E + Q) * alpha(V)) ~ O(E + Q) with path compression
/// Space: O(V)
mod ai_solution {
    use std::collections::HashMap;

    pub struct Solution;

    impl Solution {
        /// Evaluate each query as a division between two variables.
        ///
        /// # Arguments
        ///
        /// * `equations` - Pairs [Ai, Bi]
        /// * `values` - values[i] = Ai / Bi
        /// * `queries` - Pairs [Ci, Di] to evaluate
        ///
        /// # Returns
        ///
        /// Result for each query, or -1.0 if undetermined
        pub fn calc_equation(
            equations: Vec<Vec<String>>,
            values: Vec<f64>,
            queries: Vec<Vec<String>>,
        ) -> Vec<f64> {
            // Map variable names to integer IDs
            let mut id_map: HashMap<String, usize> = HashMap::new();
            let mut next_id = 0usize;

            let get_id = |name: &str, map: &mut HashMap<String, usize>, nxt: &mut usize| -> usize {
                if let Some(&id) = map.get(name) {
                    id
                } else {
                    let id = *nxt;
                    map.insert(name.to_string(), id);
                    *nxt += 1;
                    id
                }
            };

            // Assign IDs first
            for eq in &equations {
                get_id(&eq[0], &mut id_map, &mut next_id);
                get_id(&eq[1], &mut id_map, &mut next_id);
            }

            let n = next_id;
            let mut parent: Vec<usize> = (0..n).collect();
            // weight[i] = ratio of i to parent[i], i.e. i / parent[i]
            let mut weight: Vec<f64> = vec![1.0; n];

            // Find with path compression, updating weights
            fn find(x: usize, parent: &mut [usize], weight: &mut [f64]) -> usize {
                if parent[x] != x {
                    let root = find(parent[x], parent, weight);
                    weight[x] *= weight[parent[x]];
                    parent[x] = root;
                }
                parent[x]
            }

            // Union a and b where a/b = val
            fn union(a: usize, b: usize, val: f64, parent: &mut [usize], weight: &mut [f64]) {
                let pa = find(a, parent, weight);
                let pb = find(b, parent, weight);
                if pa == pb {
                    return;
                }
                parent[pa] = pb;
                // pa/root_pa = 1 (after compression), a/root_pa = weight[a]
                // a/b = val, b/root_pb = weight[b]
                // root_pa / root_pb = (b * val) / a * (a / root_pa) / (b / root_pb)
                //                   = weight[b] * val / weight[a]
                weight[pa] = weight[b] * val / weight[a];
            }

            // Process equations
            for (eq, &val) in equations.iter().zip(values.iter()) {
                let a = id_map[&eq[0]];
                let b = id_map[&eq[1]];
                union(a, b, val, &mut parent, &mut weight);
            }

            // Process queries
            queries
                .iter()
                .map(|q| {
                    let c_id = id_map.get(&q[0]);
                    let d_id = id_map.get(&q[1]);
                    match (c_id, d_id) {
                        (Some(&c), Some(&d)) => {
                            let pc = find(c, &mut parent, &mut weight);
                            let pd = find(d, &mut parent, &mut weight);
                            if pc != pd {
                                -1.0
                            } else {
                                weight[c] / weight[d]
                            }
                        }
                        _ => -1.0,
                    }
                })
                .collect()
        }
    }
}

fn main() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];

    println!("--- My Solution ---");
    let result =
        my_solution::Solution::calc_equation(equations.clone(), values.clone(), queries.clone());
    println!("  {result:?}");

    println!("--- AI Solution ---");
    let result = ai_solution::Solution::calc_equation(equations, values, queries);
    println!("  {result:?}");
}

#[cfg(test)]
mod tests {
    fn approx_eq(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 1e-5)
    }

    macro_rules! test_eval {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use super::approx_eq;
                use $sol_path as Solution;

                fn s(v: &str) -> String {
                    v.to_string()
                }

                #[test]
                fn test_example1() {
                    let result = Solution::calc_equation(
                        vec![vec![s("a"), s("b")], vec![s("b"), s("c")]],
                        vec![2.0, 3.0],
                        vec![
                            vec![s("a"), s("c")],
                            vec![s("b"), s("a")],
                            vec![s("a"), s("e")],
                            vec![s("a"), s("a")],
                            vec![s("x"), s("x")],
                        ],
                    );
                    assert!(approx_eq(&result, &[6.0, 0.5, -1.0, 1.0, -1.0]));
                }

                #[test]
                fn test_example2() {
                    let result = Solution::calc_equation(
                        vec![
                            vec![s("a"), s("b")],
                            vec![s("b"), s("c")],
                            vec![s("bc"), s("cd")],
                        ],
                        vec![1.5, 2.5, 5.0],
                        vec![
                            vec![s("a"), s("c")],
                            vec![s("c"), s("b")],
                            vec![s("bc"), s("cd")],
                            vec![s("cd"), s("bc")],
                        ],
                    );
                    assert!(approx_eq(&result, &[3.75, 0.4, 5.0, 0.2]));
                }

                #[test]
                fn test_example3() {
                    let result = Solution::calc_equation(
                        vec![vec![s("a"), s("b")]],
                        vec![0.5],
                        vec![
                            vec![s("a"), s("b")],
                            vec![s("b"), s("a")],
                            vec![s("a"), s("c")],
                            vec![s("x"), s("y")],
                        ],
                    );
                    assert!(approx_eq(&result, &[0.5, 2.0, -1.0, -1.0]));
                }
            }
        };
    }

    test_eval!(my_tests, crate::my_solution::Solution);
    test_eval!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 500_000_u32;
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["c".to_string(), "d".to_string()],
            vec!["d".to_string(), "e".to_string()],
        ];
        let values = vec![2.0, 3.0, 4.0, 5.0];
        let queries = vec![
            vec!["a".to_string(), "e".to_string()],
            vec!["e".to_string(), "a".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "y".to_string()],
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::my_solution::Solution::calc_equation(
                std::hint::black_box(equations.clone()),
                std::hint::black_box(values.clone()),
                std::hint::black_box(queries.clone()),
            ));
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(crate::ai_solution::Solution::calc_equation(
                std::hint::black_box(equations.clone()),
                std::hint::black_box(values.clone()),
                std::hint::black_box(queries.clone()),
            ));
        }
        let ai_duration = start.elapsed();

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!(
            "My solution:  {:.2?} (avg {:.2?})",
            my_duration,
            my_duration / iterations
        );
        println!(
            "AI solution:  {:.2?} (avg {:.2?})",
            ai_duration,
            ai_duration / iterations
        );
        let ratio = my_duration.as_secs_f64() / ai_duration.as_secs_f64();
        if ratio > 1.0 {
            println!("-> AI solution is {ratio:.2}x faster");
        } else {
            println!("-> My solution is {:.2}x faster", 1.0 / ratio);
        }
    }
}
