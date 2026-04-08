// LeetCode 449: Serialize and Deserialize BST [Medium]
// https://leetcode.com/problems/serialize-and-deserialize-bst/
//
// Design an algorithm to serialize and deserialize a binary search tree.
// The encoded string should be as compact as possible.
//
// Examples:
//   root = [2,1,3] -> serialize -> deserialize -> [2,1,3]
//   root = [] -> serialize -> deserialize -> []
//
// Constraints:
//   0 <= number of nodes <= 10^4
//   0 <= Node.val <= 10^4
//   The input tree is guaranteed to be a BST.
//
// Run:  cargo run --example lc_0449_serialize_and_deserialize_bst
// Test: cargo test --example lc_0449_serialize_and_deserialize_bst
// Bench: cargo test --example lc_0449_serialize_and_deserialize_bst bench -- --nocapture
//
// First benchmark (500K iterations, 2026-04-08):
//   My solution: avg 503ns
//   AI solution: avg 672ns
//   Ratio: 0.75x (recursive faster than stack)

use std::cell::RefCell;
use std::rc::Rc;

/// Shared TreeNode definition.
#[derive(Debug, PartialEq, Eq)]
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

/// Collect tree values in pre-order for comparison.
fn preorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            result.push(n.val);
            dfs(&n.left, result);
            dfs(&n.right, result);
        }
    }
    dfs(root, &mut result);
    result
}

/// My solution — preorder traversal with BST bounds
/// (translated from doocs Python/Java).
///
/// Serialize: preorder traversal, space-separated.
/// Deserialize: rebuild from preorder using BST property (min/max bounds).
///
/// Time:  O(n)
/// Space: O(n)
#[allow(dead_code)]
mod my_solution {
    use super::*;

    pub struct Codec;
    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Serialize BST to a string via preorder traversal.
        pub fn serialize(&self, root: &Option<Rc<RefCell<TreeNode>>>) -> String {
            let mut nums = Vec::new();
            Self::ser_dfs(root, &mut nums);
            nums.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        }

        fn ser_dfs(node: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                nums.push(n.val);
                Self::ser_dfs(&n.left, nums);
                Self::ser_dfs(&n.right, nums);
            }
        }

        /// Deserialize string back to BST using BST bounds.
        pub fn deserialize(&self, data: &str) -> Option<Rc<RefCell<TreeNode>>> {
            if data.is_empty() {
                return None;
            }
            let nums: Vec<i32> = data.split(' ').map(|s| s.parse().unwrap()).collect();
            let mut i = 0;
            Self::de_dfs(&nums, &mut i, i32::MIN, i32::MAX)
        }

        fn de_dfs(nums: &[i32], i: &mut usize, mi: i32, mx: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if *i >= nums.len() || nums[*i] < mi || nums[*i] > mx {
                return None;
            }
            let val = nums[*i];
            *i += 1;
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = Self::de_dfs(nums, i, mi, val);
            node.borrow_mut().right = Self::de_dfs(nums, i, val, mx);
            Some(node)
        }
    }
}

/// AI solution — preorder traversal with iterative stack-based deserialize.
///
/// Serialize: same preorder approach.
/// Deserialize: use a stack to rebuild the BST iteratively.
///
/// Time:  O(n)
/// Space: O(n)
mod ai_solution {
    use super::*;

    pub struct Codec;
    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Serialize BST to string via preorder.
        pub fn serialize(&self, root: &Option<Rc<RefCell<TreeNode>>>) -> String {
            let mut nums = Vec::new();
            let mut stack = Vec::new();
            let mut cur = root.clone();
            // Iterative preorder using a stack
            while cur.is_some() || !stack.is_empty() {
                if let Some(node) = cur {
                    let n = node.borrow();
                    nums.push(n.val);
                    // Push right child for later, go left
                    if n.right.is_some() {
                        stack.push(n.right.clone());
                    }
                    cur = n.left.clone();
                } else {
                    cur = stack.pop().unwrap();
                }
            }
            nums.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        }

        /// Deserialize using monotonic stack approach.
        pub fn deserialize(&self, data: &str) -> Option<Rc<RefCell<TreeNode>>> {
            if data.is_empty() {
                return None;
            }
            let vals: Vec<i32> = data.split(' ').map(|s| s.parse().unwrap()).collect();

            let root = Rc::new(RefCell::new(TreeNode::new(vals[0])));
            let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(&root)];

            for &val in &vals[1..] {
                let node = Rc::new(RefCell::new(TreeNode::new(val)));
                if val < stack.last().unwrap().borrow().val {
                    // Goes to the left of the top of stack
                    let parent = Rc::clone(stack.last().unwrap());
                    parent.borrow_mut().left = Some(Rc::clone(&node));
                } else {
                    // Pop until we find the correct parent
                    let mut parent = Rc::clone(stack.last().unwrap());
                    while !stack.is_empty() && stack.last().unwrap().borrow().val < val {
                        parent = stack.pop().unwrap();
                    }
                    parent.borrow_mut().right = Some(Rc::clone(&node));
                }
                stack.push(node);
            }
            Some(root)
        }
    }
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(2)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let tree = Some(root);

    let my_codec = my_solution::Codec::new();
    let serialized = my_codec.serialize(&tree);
    let deserialized = my_codec.deserialize(&serialized);
    println!("my:  {} -> {:?}", serialized, preorder(&deserialized));

    let ai_codec = ai_solution::Codec::new();
    let serialized = ai_codec.serialize(&tree);
    let deserialized = ai_codec.deserialize(&serialized);
    println!("ai:  {} -> {:?}", serialized, preorder(&deserialized));
}

#[allow(unused_macros)]
macro_rules! solution_tests {
    ($module:ident) => {
        use super::*;
        use $module::Codec;

        #[test]
        fn test_roundtrip_basic() {
            let root = Rc::new(RefCell::new(TreeNode::new(2)));
            root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
            root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            let tree = Some(root);

            let codec = Codec::new();
            let data = codec.serialize(&tree);
            let rebuilt = codec.deserialize(&data);
            assert_eq!(preorder(&rebuilt), vec![2, 1, 3]);
        }

        #[test]
        fn test_roundtrip_empty() {
            let codec = Codec::new();
            let data = codec.serialize(&None);
            let rebuilt = codec.deserialize(&data);
            assert_eq!(preorder(&rebuilt), Vec::<i32>::new());
        }

        #[test]
        fn test_roundtrip_larger() {
            // BST: 5 -> L:3(L:2,R:4), R:7(L:6,R:8)
            let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
            let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
            let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
            n3.borrow_mut().left = Some(n2);
            n3.borrow_mut().right = Some(n4);
            let n6 = Rc::new(RefCell::new(TreeNode::new(6)));
            let n8 = Rc::new(RefCell::new(TreeNode::new(8)));
            let n7 = Rc::new(RefCell::new(TreeNode::new(7)));
            n7.borrow_mut().left = Some(n6);
            n7.borrow_mut().right = Some(n8);
            let root = Rc::new(RefCell::new(TreeNode::new(5)));
            root.borrow_mut().left = Some(n3);
            root.borrow_mut().right = Some(n7);
            let tree = Some(root);

            let codec = Codec::new();
            let data = codec.serialize(&tree);
            let rebuilt = codec.deserialize(&data);
            assert_eq!(preorder(&rebuilt), vec![5, 3, 2, 4, 7, 6, 8]);
        }
    };
}

#[cfg(test)]
mod tests_my {
    solution_tests!(my_solution);
}
#[cfg(test)]
mod tests_ai {
    solution_tests!(ai_solution);
}

#[cfg(test)]
fn bench_codec<F, G>(name: &str, ser: F, deser: G)
where
    F: Fn(&Option<Rc<RefCell<TreeNode>>>) -> String,
    G: Fn(&str) -> Option<Rc<RefCell<TreeNode>>>,
{
    let root = Rc::new(RefCell::new(TreeNode::new(2)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let tree = Some(root);

    let iterations = 500_000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let data = ser(std::hint::black_box(&tree));
        std::hint::black_box(deser(std::hint::black_box(&data)));
    }
    let elapsed = start.elapsed();
    println!(
        "{}: {} iterations in {:?} (avg {:?})",
        name,
        iterations,
        elapsed,
        elapsed / iterations
    );
}

#[test]
fn bench_solutions() {
    let my_codec = my_solution::Codec::new();
    bench_codec(
        "My solution",
        |tree| my_codec.serialize(tree),
        |data| my_codec.deserialize(data),
    );
    let ai_codec = ai_solution::Codec::new();
    bench_codec(
        "AI solution",
        |tree| ai_codec.serialize(tree),
        |data| ai_codec.deserialize(data),
    );
}
