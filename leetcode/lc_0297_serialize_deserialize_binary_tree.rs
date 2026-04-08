// LeetCode 297: Serialize and Deserialize Binary Tree [Hard]
// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
//
// First benchmark: 100_000 iterations, 2025-07-06
//   My solution avg: 2.051us
//   AI solution avg: 1.917us
//   Ratio: AI 1.07x faster
//
// Design an algorithm to serialize and deserialize a binary tree.
// Serialize: tree -> string. Deserialize: string -> same tree.
//
// Example 1: root = [1,2,3,null,null,4,5]
// Example 2: root = []
//
// Constraints:
//   - 0 <= nodes <= 10^4
//   - -1000 <= Node.val <= 1000
//
// Run:  cargo run --example lc_0297_serialize_deserialize_binary_tree
// Test: cargo test --example lc_0297_serialize_deserialize_binary_tree
// Bench: cargo test --example lc_0297_serialize_deserialize_binary_tree bench -- --nocapture

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

/// Build a tree from level-order for testing.
fn build_tree(vals: &[Option<i32>]) -> Node {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut i = 1;
    while i < vals.len() {
        let Some(node) = queue.pop_front() else { break };
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
        }
        if i < vals.len() {
            if let Some(v) = vals[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(v)));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
    }
    Some(root)
}

/// My solution — Level-order (BFS) serialization (doocs).
///
/// Serialize: BFS, writing val or "#" for null, comma-separated.
/// Deserialize: split by comma, BFS to reconstruct tree.
///
/// Time:  O(n) for both serialize and deserialize
/// Space: O(n) for queue and string
mod my_solution {
    use super::*;

    pub fn serialize(root: &Node) -> String {
        let Some(r) = root else { return String::new() };
        let mut queue = VecDeque::new();
        queue.push_back(Some(r.clone()));
        let mut parts: Vec<String> = Vec::new();
        while let Some(entry) = queue.pop_front() {
            match entry {
                Some(node) => {
                    let n = node.borrow();
                    parts.push(n.val.to_string());
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                }
                None => parts.push("#".to_string()),
            }
        }
        parts.join(",")
    }

    pub fn deserialize(data: &str) -> Node {
        if data.is_empty() {
            return None;
        }
        let vals: Vec<&str> = data.split(',').collect();
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].parse().unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while let Some(node) = queue.pop_front() {
            // Left child
            if i < vals.len() && vals[i] != "#" {
                let left = Rc::new(RefCell::new(TreeNode::new(vals[i].parse().unwrap())));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
            // Right child
            if i < vals.len() && vals[i] != "#" {
                let right = Rc::new(RefCell::new(TreeNode::new(vals[i].parse().unwrap())));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
        Some(root)
    }
}

/// AI solution — Preorder DFS serialization.
///
/// Serialize: preorder traversal, writing val or "#" for null.
/// Deserialize: consume tokens in preorder, recursively build tree.
///
/// Time:  O(n)
/// Space: O(n) (O(h) stack for balanced tree)
mod ai_solution {
    use super::*;

    pub fn serialize(root: &Node) -> String {
        let mut parts = Vec::new();
        fn dfs(node: &Node, parts: &mut Vec<String>) {
            match node {
                Some(n) => {
                    let n = n.borrow();
                    parts.push(n.val.to_string());
                    dfs(&n.left, parts);
                    dfs(&n.right, parts);
                }
                None => parts.push("#".to_string()),
            }
        }
        dfs(root, &mut parts);
        parts.join(",")
    }

    pub fn deserialize(data: &str) -> Node {
        if data.is_empty() {
            return None;
        }
        let tokens: Vec<&str> = data.split(',').collect();
        let mut idx = 0;
        fn build(tokens: &[&str], idx: &mut usize) -> Node {
            if *idx >= tokens.len() || tokens[*idx] == "#" {
                *idx += 1;
                return None;
            }
            let val: i32 = tokens[*idx].parse().unwrap();
            *idx += 1;
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = build(tokens, idx);
            node.borrow_mut().right = build(tokens, idx);
            Some(node)
        }
        build(&tokens, &mut idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: compare two trees structurally.
    fn trees_equal(a: &Node, b: &Node) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                a.val == b.val && trees_equal(&a.left, &b.left) && trees_equal(&a.right, &b.right)
            }
            _ => false,
        }
    }

    macro_rules! test_serde {
        ($name:ident, $vals:expr) => {
            #[test]
            fn $name() {
                let original = build_tree(&$vals);

                // my_solution round-trip
                let my_data = my_solution::serialize(&original);
                let my_tree = my_solution::deserialize(&my_data);
                assert!(
                    trees_equal(&original, &my_tree),
                    "my_solution round-trip failed"
                );

                // ai_solution round-trip
                let ai_data = ai_solution::serialize(&original);
                let ai_tree = ai_solution::deserialize(&ai_data);
                assert!(
                    trees_equal(&original, &ai_tree),
                    "ai_solution round-trip failed"
                );
            }
        };
    }

    // Example 1: [1,2,3,null,null,4,5]
    test_serde!(
        test_example_1,
        [Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]
    );

    // Example 2: empty tree
    test_serde!(test_empty, []);

    // Single node
    test_serde!(test_single, [Some(42)]);

    // Left-skewed
    test_serde!(test_left_skewed, [Some(1), Some(2), None, Some(3)]);

    // Right-skewed
    test_serde!(test_right_skewed, [Some(1), None, Some(2), None, Some(3)]);

    // Balanced
    test_serde!(
        test_balanced,
        [
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7)
        ]
    );

    // Negative values
    test_serde!(test_negatives, [Some(-1), Some(-2), Some(-3)]);

    // Single child nodes
    test_serde!(
        test_mixed_children,
        [Some(1), Some(2), Some(3), None, Some(4), Some(5), None]
    );

    // Two nodes
    test_serde!(test_two_nodes_left, [Some(1), Some(2)]);
    test_serde!(test_two_nodes_right, [Some(1), None, Some(2)]);
}

fn main() {
    println!("LeetCode 297: Serialize and Deserialize Binary Tree");
    let root = build_tree(&[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)]);
    let my_data = my_solution::serialize(&root);
    println!("my_solution serialized: {}", my_data);
    let my_root = my_solution::deserialize(&my_data);
    println!("my_solution deserialized ok: {}", my_root.is_some());
    let ai_data = ai_solution::serialize(&root);
    println!("ai_solution serialized: {}", ai_data);
    let ai_root = ai_solution::deserialize(&ai_data);
    println!("ai_solution deserialized ok: {}", ai_root.is_some());
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let root = build_tree(&[
            Some(4),
            Some(2),
            Some(6),
            Some(1),
            Some(3),
            Some(5),
            Some(7),
        ]);
        let iterations = 100_000;

        let my_data = my_solution::serialize(&root);
        let ai_data = ai_solution::serialize(&root);

        // Warm up
        for _ in 0..1_000 {
            std::hint::black_box(my_solution::serialize(&root));
            std::hint::black_box(my_solution::deserialize(&my_data));
            std::hint::black_box(ai_solution::serialize(&root));
            std::hint::black_box(ai_solution::deserialize(&ai_data));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let data = my_solution::serialize(&root);
            std::hint::black_box(my_solution::deserialize(&data));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let data = ai_solution::serialize(&root);
            std::hint::black_box(ai_solution::deserialize(&data));
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
