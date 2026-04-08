// @lc app=leetcode id=366 lang=rust
// #[Medium] - Find Leaves of Binary Tree
// Iterations: 100000 | Date: 2025-07-10
// my_solution  | avg: 0.193us
// ai_solution  | avg: 0.752us
// Performance: My 3.90x faster

/// # 366. Find Leaves of Binary Tree
///
/// Collect leaf nodes, remove them, repeat until the tree is empty.
/// Return a list of lists where each inner list contains the leaves at each round.
///
/// Constraints:
/// - Number of nodes in [1, 100]
/// - -100 <= Node.val <= 100

/// ## My Solution — DFS returning height O(n)
///
/// The "height" of a node (leaves = 0, parent of leaf = 1, ...) determines
/// which group it belongs to. Single DFS collecting by height.
///
/// - Time: O(n)
/// - Space: O(n)
mod my_solution {
    pub fn find_leaves(tree: &[Option<i32>]) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        dfs(tree, 0, &mut ans);
        ans
    }

    /// Returns the height of node at index `i` (0-based, leaves = 0).
    fn dfs(tree: &[Option<i32>], i: usize, ans: &mut Vec<Vec<i32>>) -> i32 {
        if i >= tree.len() || tree[i].is_none() {
            return -1; // null sentinel
        }
        let left_h = dfs(tree, 2 * i + 1, ans);
        let right_h = dfs(tree, 2 * i + 2, ans);
        let h = left_h.max(right_h) + 1;
        if ans.len() == h as usize {
            ans.push(Vec::new());
        }
        ans[h as usize].push(tree[i].unwrap());
        h
    }
}

/// ## AI Solution — Iterative leaf removal via adjacency
///
/// Build adjacency with degree tracking. Iteratively peel leaves (degree <= 1)
/// like topological sort (BFS from leaves inward).
///
/// - Time: O(n)
/// - Space: O(n)
mod ai_solution {
    use std::collections::VecDeque;

    pub fn find_leaves(tree: &[Option<i32>]) -> Vec<Vec<i32>> {
        // Collect all valid node indices
        let n = tree.len();
        let mut val = vec![0i32; n];
        let mut exists = vec![false; n];
        let mut degree = vec![0u32; n]; // number of existing neighbors (parent + children)
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];

        for i in 0..n {
            if let Some(v) = tree[i] {
                exists[i] = true;
                val[i] = v;
            }
        }

        // Build edges (parent-child)
        for i in 0..n {
            if !exists[i] {
                continue;
            }
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            if left < n && exists[left] {
                degree[i] += 1;
                degree[left] += 1;
                children[i].push(left);
            }
            if right < n && exists[right] {
                degree[i] += 1;
                degree[right] += 1;
                children[i].push(right);
            }
        }

        // Root has no parent edge, so leaves have degree 0 (no children, root)
        // or degree 1 (only parent edge). For root: leaf if no children => degree 0.
        // Actually: degree counts only child edges. Let's track parent separately.
        // Simpler: just use the DFS height approach but iteratively via parent array.

        // Rebuild: use parent tracking
        let mut parent = vec![usize::MAX; n];
        let mut child_count = vec![0u32; n];
        for i in 0..n {
            if !exists[i] {
                continue;
            }
            for &c in [2 * i + 1, 2 * i + 2].iter() {
                if c < n && exists[c] {
                    parent[c] = i;
                    child_count[i] += 1;
                }
            }
        }

        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut removed = vec![false; n];

        // Initial leaves: nodes with child_count == 0
        for i in 0..n {
            if exists[i] && child_count[i] == 0 {
                queue.push_back(i);
            }
        }

        while !queue.is_empty() {
            let mut layer = Vec::new();
            let size = queue.len();
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                removed[node] = true;
                layer.push(val[node]);
                if parent[node] != usize::MAX {
                    child_count[parent[node]] -= 1;
                    if child_count[parent[node]] == 0 && !removed[parent[node]] {
                        queue.push_back(parent[node]);
                    }
                }
            }
            ans.push(layer);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(&[Option<i32>]) -> Vec<Vec<i32>>) {
        // [1,2,3,4,5] => leaves: [4,5,3], then [2], then [1]
        let tree = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
        let result = f(&tree);
        assert_eq!(result.len(), 3);
        // Order within each layer may vary; sort for comparison
        let r: Vec<Vec<i32>> = result
            .into_iter()
            .map(|mut v| {
                v.sort();
                v
            })
            .collect();
        assert_eq!(r, vec![vec![3, 4, 5], vec![2], vec![1]]);

        let tree2 = vec![Some(1)];
        assert_eq!(f(&tree2), vec![vec![1]]);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::find_leaves);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::find_leaves);
    }
}

fn main() {
    let tree = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::find_leaves(std::hint::black_box(&tree)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::find_leaves(std::hint::black_box(&tree)));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::find_leaves(std::hint::black_box(&tree)));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::find_leaves(std::hint::black_box(&tree)));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 366 - Find Leaves of Binary Tree");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
