// Intro to binary search trees (BST) using Box.
//
// A BST is a tree where each node's left children are smaller
// and right children are larger. Like linked lists, trees are
// recursive structures that need Box for heap allocation.
//
// Run: cargo run --example s26_bst_intro

/// A binary search tree node.
#[derive(Debug)]
struct BstNode {
    value: i32,
    left: Option<Box<BstNode>>,
    right: Option<Box<BstNode>>,
}

impl BstNode {
    /// Creates a new leaf node.
    fn new(value: i32) -> Self {
        BstNode {
            value,
            left: None,
            right: None,
        }
    }

    /// Inserts a value into the BST.
    fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(BstNode::new(value))),
            }
        } else if value > self.value {
            match &mut self.right {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(BstNode::new(value))),
            }
        }
        // Duplicates ignored
    }

    /// Searches for a value in the BST.
    fn contains(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            self.left.as_ref().is_some_and(|l| l.contains(value))
        } else {
            self.right.as_ref().is_some_and(|r| r.contains(value))
        }
    }

    /// In-order traversal (produces sorted output).
    fn in_order(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(left) = &self.left {
            result.extend(left.in_order());
        }
        result.push(self.value);
        if let Some(right) = &self.right {
            result.extend(right.in_order());
        }
        result
    }
}

fn main() {
    println!("=== Intro to BST ===");

    let mut root = BstNode::new(5);
    for val in [3, 7, 1, 4, 6, 8] {
        root.insert(val);
    }

    println!("tree: {root:#?}");
    println!("in-order: {:?}", root.in_order());
    println!("contains 4: {}", root.contains(4));
    println!("contains 9: {}", root.contains(9));

    // --- Size of a node ---
    println!("BstNode size: {} bytes", std::mem::size_of::<BstNode>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_order() {
        let mut root = BstNode::new(5);
        root.insert(3);
        root.insert(7);
        root.insert(1);
        assert_eq!(root.in_order(), vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_contains() {
        let mut root = BstNode::new(10);
        root.insert(5);
        root.insert(15);
        assert!(root.contains(10));
        assert!(root.contains(5));
        assert!(!root.contains(7));
    }

    #[test]
    fn test_duplicates_ignored() {
        let mut root = BstNode::new(5);
        root.insert(5);
        assert_eq!(root.in_order(), vec![5]);
    }
}
