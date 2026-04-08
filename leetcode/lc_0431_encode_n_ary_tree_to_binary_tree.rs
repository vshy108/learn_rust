// LeetCode 431: Encode N-ary Tree to Binary Tree [Hard]
// https://leetcode.com/problems/encode-n-ary-tree-to-binary-tree/
//
// Design an algorithm to encode an N-ary tree into a binary tree and
// decode the binary tree back to the original N-ary tree.
//
// Encoding strategy (left-child right-sibling):
//   - The first child of an N-ary node becomes the left child of the
//     corresponding binary tree node.
//   - Subsequent siblings are chained via the right pointer.
//
// Examples:
//   [1,null,3,2,4,null,5,6] encodes to binary tree and decodes back
//   [] (empty) round-trips
//
// Constraints:
//   0 <= number of nodes <= 10^4
//   0 <= Node.val <= 10^4
//   Height of N-ary tree <= 1000
//
// Run:  cargo run --example lc_0431_encode_n_ary_tree_to_binary_tree
// Test: cargo test --example lc_0431_encode_n_ary_tree_to_binary_tree
// Bench: cargo test --example lc_0431_encode_n_ary_tree_to_binary_tree bench -- --nocapture
//
// First benchmark (200K iterations, 2026-04-08):
//   My solution: avg 69.241us
//   AI solution: avg 73.143us
//   Ratio: 1.06x (similar, recursive vs iterative)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// N-ary tree node.
#[derive(Debug, Clone, PartialEq)]
pub struct NaryNode {
    pub val: i32,
    pub children: Vec<Rc<RefCell<NaryNode>>>,
}

impl NaryNode {
    pub fn new(val: i32) -> Self {
        NaryNode {
            val,
            children: Vec::new(),
        }
    }
}

/// Binary tree node.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type NaryLink = Option<Rc<RefCell<NaryNode>>>;
type TreeLink = Option<Rc<RefCell<TreeNode>>>;

/// Time: O(n), Space: O(n) (recursion stack)
/// Recursive encode/decode using left-child right-sibling representation.
/// Encode: first child -> left, siblings chain via right.
/// Decode: left is first child, walk right for remaining siblings.
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Codec;

    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Encode an N-ary tree to a binary tree.
        ///
        /// # Arguments
        /// * `root` - Root of the N-ary tree
        ///
        /// # Returns
        /// Root of the encoded binary tree
        pub fn encode(&self, root: &NaryLink) -> TreeLink {
            let nary = match root {
                Some(n) => n.borrow(),
                None => return None,
            };

            let bt = Rc::new(RefCell::new(TreeNode::new(nary.val)));

            // First child becomes the left subtree of the binary node
            if !nary.children.is_empty() {
                bt.borrow_mut().left = self.encode(&Some(Rc::clone(&nary.children[0])));

                // Remaining siblings chain via right pointers
                // Walk along the leftmost spine's right children
                let mut cur = bt.borrow().left.clone().unwrap();
                for i in 1..nary.children.len() {
                    let sibling = self.encode(&Some(Rc::clone(&nary.children[i])));
                    cur.borrow_mut().right = sibling;
                    let next = cur.borrow().right.clone().unwrap();
                    cur = next;
                }
            }

            Some(bt)
        }

        /// Decode a binary tree back to an N-ary tree.
        ///
        /// # Arguments
        /// * `root` - Root of the encoded binary tree
        ///
        /// # Returns
        /// Root of the decoded N-ary tree
        pub fn decode(&self, root: &TreeLink) -> NaryLink {
            let bt = match root {
                Some(n) => n.borrow(),
                None => return None,
            };

            let nary = Rc::new(RefCell::new(NaryNode::new(bt.val)));

            // Left child is the first child; walk its right siblings
            let mut cur = bt.left.clone();
            while let Some(ref sibling) = cur {
                let child = self.decode(&Some(Rc::clone(sibling)));
                if let Some(c) = child {
                    nary.borrow_mut().children.push(c);
                }
                let next = sibling.borrow().right.clone();
                cur = next;
            }

            Some(nary)
        }
    }
}

/// Time: O(n), Space: O(n)
/// Iterative encode/decode using queues. Encode pairs each N-ary node
/// with its binary counterpart and processes children iteratively.
/// Decode uses a stack to walk the binary tree iteratively.
#[allow(dead_code)]
mod ai_solution {
    use super::*;

    pub struct Codec;

    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Encode an N-ary tree to a binary tree (iterative BFS).
        ///
        /// # Arguments
        /// * `root` - Root of the N-ary tree
        ///
        /// # Returns
        /// Root of the encoded binary tree
        pub fn encode(&self, root: &NaryLink) -> TreeLink {
            let nary_root = match root {
                Some(n) => Rc::clone(n),
                None => return None,
            };

            let bt_root = Rc::new(RefCell::new(TreeNode::new(nary_root.borrow().val)));

            // Queue of (nary_node, binary_node) pairs
            let mut queue: VecDeque<(Rc<RefCell<NaryNode>>, Rc<RefCell<TreeNode>>)> =
                VecDeque::new();
            queue.push_back((nary_root, Rc::clone(&bt_root)));

            while let Some((nary, bt)) = queue.pop_front() {
                let children = &nary.borrow().children;
                if children.is_empty() {
                    continue;
                }

                // Encode first child as left of bt
                let first_bt = Rc::new(RefCell::new(TreeNode::new(children[0].borrow().val)));
                bt.borrow_mut().left = Some(Rc::clone(&first_bt));
                queue.push_back((Rc::clone(&children[0]), Rc::clone(&first_bt)));

                // Chain remaining children via right pointers
                let mut prev_bt = first_bt;
                for child in children.iter().skip(1) {
                    let sib_bt = Rc::new(RefCell::new(TreeNode::new(child.borrow().val)));
                    prev_bt.borrow_mut().right = Some(Rc::clone(&sib_bt));
                    queue.push_back((Rc::clone(child), Rc::clone(&sib_bt)));
                    prev_bt = sib_bt;
                }
            }

            Some(bt_root)
        }

        /// Decode a binary tree back to an N-ary tree (iterative BFS).
        ///
        /// # Arguments
        /// * `root` - Root of the encoded binary tree
        ///
        /// # Returns
        /// Root of the decoded N-ary tree
        pub fn decode(&self, root: &TreeLink) -> NaryLink {
            let bt_root = match root {
                Some(n) => Rc::clone(n),
                None => return None,
            };

            let nary_root = Rc::new(RefCell::new(NaryNode::new(bt_root.borrow().val)));

            // Queue of (binary_node, nary_node) pairs
            let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, Rc<RefCell<NaryNode>>)> =
                VecDeque::new();
            queue.push_back((bt_root, Rc::clone(&nary_root)));

            while let Some((bt, nary)) = queue.pop_front() {
                // Walk the left child's right-sibling chain
                let mut cur = bt.borrow().left.clone();
                while let Some(ref bt_child) = cur {
                    let nary_child = Rc::new(RefCell::new(NaryNode::new(bt_child.borrow().val)));
                    nary.borrow_mut().children.push(Rc::clone(&nary_child));
                    queue.push_back((Rc::clone(bt_child), nary_child));
                    let next = bt_child.borrow().right.clone();
                    cur = next;
                }
            }

            Some(nary_root)
        }
    }
}

/// Build an N-ary tree from level-order representation with None separators.
fn build_nary_tree(vals: &[Option<i32>]) -> NaryLink {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(NaryNode::new(vals[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 2; // skip root value and first None separator
    while i < vals.len() && !queue.is_empty() {
        let parent = queue.pop_front().unwrap();
        while i < vals.len() {
            if vals[i].is_none() {
                i += 1;
                break;
            }
            let child = Rc::new(RefCell::new(NaryNode::new(vals[i].unwrap())));
            parent.borrow_mut().children.push(Rc::clone(&child));
            queue.push_back(child);
            i += 1;
        }
    }
    Some(root)
}

/// Compare two N-ary trees structurally.
fn nary_eq(a: &NaryLink, b: &NaryLink) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let a = a.borrow();
            let b = b.borrow();
            if a.val != b.val || a.children.len() != b.children.len() {
                return false;
            }
            a.children
                .iter()
                .zip(b.children.iter())
                .all(|(ca, cb)| nary_eq(&Some(Rc::clone(ca)), &Some(Rc::clone(cb))))
        }
        _ => false,
    }
}

/// Collect preorder values from an N-ary tree (for debug output).
fn nary_preorder(root: &NaryLink) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(node: &NaryLink, r: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            r.push(n.val);
            for child in &n.children {
                dfs(&Some(Rc::clone(child)), r);
            }
        }
    }
    dfs(root, &mut result);
    result
}

fn main() {
    println!("LeetCode 431: Encode N-ary Tree to Binary Tree");
    let tree = build_nary_tree(&[
        Some(1),
        None,
        Some(3),
        Some(2),
        Some(4),
        None,
        Some(5),
        Some(6),
    ]);
    println!("  original preorder: {:?}", nary_preorder(&tree));

    let c1 = my_solution::Codec::new();
    let encoded1 = c1.encode(&tree);
    let decoded1 = c1.decode(&encoded1);
    println!("  my round-trip: {}", nary_eq(&tree, &decoded1));

    let c2 = ai_solution::Codec::new();
    let encoded2 = c2.encode(&tree);
    let decoded2 = c2.decode(&encoded2);
    println!("  ai round-trip: {}", nary_eq(&tree, &decoded2));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: encode then decode, check structural equality.
    fn round_trip_test(
        vals: &[Option<i32>],
        encode: impl Fn(&NaryLink) -> TreeLink,
        decode: impl Fn(&TreeLink) -> NaryLink,
    ) {
        let original = build_nary_tree(vals);
        let encoded = encode(&original);
        let decoded = decode(&encoded);
        assert!(
            nary_eq(&original, &decoded),
            "round-trip failed: original={:?}, decoded={:?}",
            nary_preorder(&original),
            nary_preorder(&decoded),
        );
    }

    macro_rules! test_both {
        ($name:ident, $vals:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    let c = my_solution::Codec::new();
                    round_trip_test(&$vals, |r| c.encode(r), |r| c.decode(r));
                }
                #[test]
                fn ai() {
                    let c = ai_solution::Codec::new();
                    round_trip_test(&$vals, |r| c.encode(r), |r| c.decode(r));
                }
            }
        };
    }

    test_both!(empty_tree, []);
    test_both!(single_node, [Some(1)]);
    test_both!(
        example1,
        [
            Some(1),
            None,
            Some(3),
            Some(2),
            Some(4),
            None,
            Some(5),
            Some(6)
        ]
    );
    test_both!(two_children, [Some(10), None, Some(20), Some(30)]);
    test_both!(
        deep_chain,
        [Some(1), None, Some(2), None, Some(3), None, Some(4)]
    );
    test_both!(
        wide_root,
        [Some(1), None, Some(2), Some(3), Some(4), Some(5), Some(6)]
    );
    test_both!(
        complex_tree,
        [
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
            Some(7),
            None,
            Some(8)
        ]
    );

    /// Verify cross-codec compatibility: encode with my, decode with ai, and vice versa.
    #[test]
    fn cross_codec_compatibility() {
        let vals = [
            Some(1),
            None,
            Some(3),
            Some(2),
            Some(4),
            None,
            Some(5),
            Some(6),
        ];
        let original = build_nary_tree(&vals);
        let c1 = my_solution::Codec::new();
        let c2 = ai_solution::Codec::new();

        // Both use same left-child right-sibling encoding, so cross-decode works
        let enc1 = c1.encode(&original);
        let dec_by_ai = c2.decode(&enc1);
        assert!(
            nary_eq(&original, &dec_by_ai),
            "my encode -> ai decode failed"
        );

        let enc2 = c2.encode(&original);
        let dec_by_my = c1.decode(&enc2);
        assert!(
            nary_eq(&original, &dec_by_my),
            "ai encode -> my decode failed"
        );
    }
}

#[cfg(test)]
fn bench_codec<E, D>(name: &str, encode: E, decode: D)
where
    E: Fn(&NaryLink) -> TreeLink,
    D: Fn(&TreeLink) -> NaryLink,
{
    let iterations = 200_000;
    // Build a wide tree: root with 50 children, each with 10 grandchildren
    let root = Rc::new(RefCell::new(NaryNode::new(0)));
    for i in 1..=50 {
        let child = Rc::new(RefCell::new(NaryNode::new(i)));
        for j in 0..10 {
            child
                .borrow_mut()
                .children
                .push(Rc::new(RefCell::new(NaryNode::new(i * 100 + j))));
        }
        root.borrow_mut().children.push(child);
    }
    let tree: NaryLink = Some(root);

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let encoded = std::hint::black_box(encode(std::hint::black_box(&tree)));
        std::hint::black_box(decode(std::hint::black_box(&encoded)));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?})",
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    let c1 = my_solution::Codec::new();
    bench_codec("My solution", |r| c1.encode(r), |r| c1.decode(r));

    let c2 = ai_solution::Codec::new();
    bench_codec("AI solution", |r| c2.encode(r), |r| c2.decode(r));
}
