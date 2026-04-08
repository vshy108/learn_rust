// LeetCode 428: Serialize and Deserialize N-ary Tree [Hard]
// https://leetcode.com/problems/serialize-and-deserialize-n-ary-tree/
//
// Design an algorithm to serialize and deserialize an N-ary tree. An N-ary
// tree is a rooted tree where each node has no more than N children. There
// is no restriction on how the serialization/deserialization algorithm works,
// as long as the tree can be serialized to a string and deserialized back
// to the original tree structure.
//
// Examples:
//   [1,null,3,2,4,null,5,6] round-trips to identical tree
//   [] (empty tree) round-trips
//
// Constraints:
//   0 <= number of nodes <= 10^4
//   0 <= Node.val <= 10^4
//   Height of N-ary tree <= 1000
//
// Run:  cargo run --example lc_0428_serialize_and_deserialize_n_ary_tree
// Test: cargo test --example lc_0428_serialize_and_deserialize_n_ary_tree
// Bench: cargo test --example lc_0428_serialize_and_deserialize_n_ary_tree bench -- --nocapture
//
// First benchmark (200K iterations, 2026-04-08):
//   My solution: avg 119.976us
//   AI solution: avg 80.328us
//   Ratio: 1.49x (AI bracket format more compact)

use std::cell::RefCell;
use std::rc::Rc;

/// N-ary tree node: each node has a value and a list of children.
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

type NaryLink = Option<Rc<RefCell<NaryNode>>>;

/// Time: O(n), Space: O(n)
/// Preorder DFS with child count. Serialize as "val,num_children,child1..."
/// separated by spaces. Deserialize by reading val + count, then
/// recursively reading that many children.
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Codec;

    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Serialize an N-ary tree to a string.
        ///
        /// # Arguments
        /// * `root` - Root of the N-ary tree
        ///
        /// # Returns
        /// String representation of the tree
        pub fn serialize(&self, root: &NaryLink) -> String {
            let mut parts = Vec::new();
            Self::ser_dfs(root, &mut parts);
            parts.join(" ")
        }

        /// Recursive preorder serialization.
        /// Writes "val num_children" then recurses on each child.
        fn ser_dfs(node: &NaryLink, parts: &mut Vec<String>) {
            if let Some(n) = node {
                let n = n.borrow();
                parts.push(n.val.to_string());
                parts.push(n.children.len().to_string());
                for child in &n.children {
                    Self::ser_dfs(&Some(Rc::clone(child)), parts);
                }
            }
        }

        /// Deserialize a string back to an N-ary tree.
        ///
        /// # Arguments
        /// * `data` - String previously produced by serialize
        ///
        /// # Returns
        /// Root of the reconstructed tree
        pub fn deserialize(&self, data: &str) -> NaryLink {
            if data.is_empty() {
                return None;
            }
            let tokens: Vec<&str> = data.split_whitespace().collect();
            let mut idx = 0;
            Self::deser_dfs(&tokens, &mut idx)
        }

        /// Recursive deserialization: read val + child_count, then
        /// recursively reconstruct that many children.
        fn deser_dfs(tokens: &[&str], idx: &mut usize) -> NaryLink {
            if *idx >= tokens.len() {
                return None;
            }
            let val: i32 = tokens[*idx].parse().expect("valid integer");
            *idx += 1;
            let count: usize = tokens[*idx].parse().expect("valid count");
            *idx += 1;

            let node = Rc::new(RefCell::new(NaryNode::new(val)));
            for _ in 0..count {
                if let Some(child) = Self::deser_dfs(tokens, idx) {
                    node.borrow_mut().children.push(child);
                }
            }
            Some(node)
        }
    }
}

/// Time: O(n), Space: O(n)
/// Bracket-based format. Serialize as "val[child1 child2 ...]" where
/// each child is recursively serialized. Leaf nodes have no brackets.
/// Deserialize by parsing the bracket structure.
#[allow(dead_code)]
mod ai_solution {
    use super::*;

    pub struct Codec;

    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Serialize an N-ary tree to a bracket-based string.
        ///
        /// # Arguments
        /// * `root` - Root of the N-ary tree
        ///
        /// # Returns
        /// String representation like "1[3[5 6] 2 4]"
        pub fn serialize(&self, root: &NaryLink) -> String {
            match root {
                None => String::new(),
                Some(n) => {
                    let n = n.borrow();
                    let mut s = n.val.to_string();
                    if !n.children.is_empty() {
                        s.push('[');
                        for (i, child) in n.children.iter().enumerate() {
                            if i > 0 {
                                s.push(' ');
                            }
                            s.push_str(&self.serialize(&Some(Rc::clone(child))));
                        }
                        s.push(']');
                    }
                    s
                }
            }
        }

        /// Deserialize a bracket-based string back to an N-ary tree.
        ///
        /// # Arguments
        /// * `data` - String previously produced by serialize
        ///
        /// # Returns
        /// Root of the reconstructed tree
        pub fn deserialize(&self, data: &str) -> NaryLink {
            if data.is_empty() {
                return None;
            }
            let bytes = data.as_bytes();
            let mut idx = 0;
            Self::parse(bytes, &mut idx)
        }

        /// Parse a node starting at `idx`. Reads the integer value,
        /// then optionally parses "[child1 child2 ...]".
        fn parse(bytes: &[u8], idx: &mut usize) -> NaryLink {
            if *idx >= bytes.len() {
                return None;
            }

            // Parse integer value (may be negative)
            let mut val: i32 = 0;
            let negative = *idx < bytes.len() && bytes[*idx] == b'-';
            if negative {
                *idx += 1;
            }
            while *idx < bytes.len() && bytes[*idx].is_ascii_digit() {
                val = val * 10 + (bytes[*idx] - b'0') as i32;
                *idx += 1;
            }
            if negative {
                val = -val;
            }

            let node = Rc::new(RefCell::new(NaryNode::new(val)));

            // Check for children block '['
            if *idx < bytes.len() && bytes[*idx] == b'[' {
                *idx += 1; // consume '['
                while *idx < bytes.len() && bytes[*idx] != b']' {
                    // Skip space separators
                    if bytes[*idx] == b' ' {
                        *idx += 1;
                        continue;
                    }
                    if let Some(child) = Self::parse(bytes, idx) {
                        node.borrow_mut().children.push(child);
                    }
                }
                *idx += 1; // consume ']'
            }

            Some(node)
        }
    }
}

/// Build an N-ary tree from a level-order representation.
/// Format: values separated by None markers to indicate end of children list.
/// e.g. [Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)]
fn build_nary_tree(vals: &[Option<i32>]) -> NaryLink {
    if vals.is_empty() || vals[0].is_none() {
        return None;
    }
    let root = Rc::new(RefCell::new(NaryNode::new(vals[0].unwrap())));
    // vals[1] should be None (separator after root)
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 2; // skip root value and first None separator
    while i < vals.len() && !queue.is_empty() {
        let parent = queue.pop_front().unwrap();
        // Read children until None or end of array
        while i < vals.len() {
            if vals[i].is_none() {
                i += 1; // consume the None separator
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

/// Collect values from N-ary tree in preorder (val, then children L-to-R).
#[allow(dead_code)]
fn nary_preorder(root: &NaryLink) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(node: &NaryLink, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            result.push(n.val);
            for child in &n.children {
                dfs(&Some(Rc::clone(child)), result);
            }
        }
    }
    dfs(root, &mut result);
    result
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

fn main() {
    println!("LeetCode 428: Serialize and Deserialize N-ary Tree");
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
    let c1 = my_solution::Codec::new();
    let s1 = c1.serialize(&tree);
    let d1 = c1.deserialize(&s1);
    println!(
        "  my: serialized=\"{s1}\", round-trip={}",
        nary_eq(&tree, &d1)
    );

    let c2 = ai_solution::Codec::new();
    let s2 = c2.serialize(&tree);
    let d2 = c2.deserialize(&s2);
    println!(
        "  ai: serialized=\"{s2}\", round-trip={}",
        nary_eq(&tree, &d2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build tree, serialize, deserialize, check structural equality.
    fn round_trip_test(
        vals: &[Option<i32>],
        serialize: impl Fn(&NaryLink) -> String,
        deserialize: impl Fn(&str) -> NaryLink,
    ) {
        let original = build_nary_tree(vals);
        let serialized = serialize(&original);
        let restored = deserialize(&serialized);
        assert!(
            nary_eq(&original, &restored),
            "round-trip failed: original preorder={:?}, restored preorder={:?}, \
             serialized=\"{serialized}\"",
            nary_preorder(&original),
            nary_preorder(&restored),
        );
    }

    macro_rules! test_both {
        ($name:ident, $vals:expr) => {
            mod $name {
                use super::*;
                #[test]
                fn my() {
                    let c = my_solution::Codec::new();
                    round_trip_test(&$vals, |root| c.serialize(root), |data| c.deserialize(data));
                }
                #[test]
                fn ai() {
                    let c = ai_solution::Codec::new();
                    round_trip_test(&$vals, |root| c.serialize(root), |data| c.deserialize(data));
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
        deep_single_chain,
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
}

#[cfg(test)]
fn bench_codec<S, D>(name: &str, serialize: S, deserialize: D)
where
    S: Fn(&NaryLink) -> String,
    D: Fn(&str) -> NaryLink,
{
    let iterations = 200_000;
    // Build a wide tree: root with 100 children, each with 5 grandchildren
    let root = Rc::new(RefCell::new(NaryNode::new(0)));
    for i in 1..=100 {
        let child = Rc::new(RefCell::new(NaryNode::new(i)));
        for j in 0..5 {
            child
                .borrow_mut()
                .children
                .push(Rc::new(RefCell::new(NaryNode::new(i * 10 + j))));
        }
        root.borrow_mut().children.push(child);
    }
    let tree: NaryLink = Some(root);
    let serialized = serialize(&tree);

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let s = std::hint::black_box(serialize(std::hint::black_box(&tree)));
        std::hint::black_box(deserialize(std::hint::black_box(&s)));
    }
    let elapsed = start.elapsed();
    println!(
        "{name}: {iterations} iterations in {elapsed:?} (avg {:?}) [len={}]",
        elapsed / iterations,
        serialized.len()
    );
}

#[test]
fn bench_solutions() {
    let c1 = my_solution::Codec::new();
    bench_codec(
        "My solution",
        |root| c1.serialize(root),
        |data| c1.deserialize(data),
    );

    let c2 = ai_solution::Codec::new();
    bench_codec(
        "AI solution",
        |root| c2.serialize(root),
        |data| c2.deserialize(data),
    );
}
