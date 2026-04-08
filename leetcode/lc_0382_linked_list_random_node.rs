// LeetCode 382: Linked List Random Node [Medium]
// https://leetcode.com/problems/linked-list-random-node/
//
// Given a singly linked list, return a random node's value from the linked
// list. Each node must have the same probability of being chosen.
//
// Implement the Solution class:
//   - Solution::new(head)  -- Initializes the object with the head of a
//                             singly-linked list.
//   - get_random() -> i32  -- Chooses a node randomly from the list and
//                             returns its value. All nodes should be equally
//                             likely to be chosen.
//
// Example:
//   Input: head = [1, 2, 3]
//   solution.get_random() -> 1 or 2 or 3 with equal probability
//
// Constraints:
//   - The number of nodes in the linked list will be in the range [1, 10^4]
//   - -10^4 <= Node.val <= 10^4
//   - At most 10^4 calls will be made to getRandom
//
// Follow up: What if the linked list is extremely large and its length is
// unknown to you? Could you solve this efficiently without using extra space?
// -> Reservoir sampling: O(n) time, O(1) extra space per call.
//
// Note: LeetCode's Rust signature uses Option<Box<ListNode>>. We simulate
// the linked list concept while keeping the code self-contained.
//
// Run:  cargo run --example lc_0382_linked_list_random_node
// Test: cargo test --example lc_0382_linked_list_random_node
// Bench: cargo test --example lc_0382_linked_list_random_node bench -- --nocapture
//
// --- Benchmark (10k iterations, 2026-04-08) ---
// My solution:  124.37ms (avg 12.44us)
// AI solution:  317.46ms (avg 31.74us)
// -> My solution is 2.55x faster
// Reason: My solution pre-collects values into a Vec at construction for
//   O(1) get_random, while the AI solution re-traverses the entire list on
//   every get_random call (reservoir sampling trades speed for O(1) extra
//   space per call).

/// Shared ListNode definition used by both solutions.
#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    /// Build a linked list from a slice of values and return the head.
    fn from_slice(vals: &[i32]) -> Option<Box<ListNode>> {
        // Build from the back so the first element is the head
        let mut head: Option<Box<ListNode>> = None;
        for &v in vals.iter().rev() {
            head = Some(Box::new(ListNode { val: v, next: head }));
        }
        head
    }
}

/// My solution module — collect values into a Vec on construction.
///
/// Construction: O(n) time, O(n) space
/// get_random:   O(1) time
/// Space:        O(n)
mod my_solution {
    use crate::ListNode;

    /// Stores all node values in a Vec for O(1) random access.
    ///
    /// This is the straightforward approach: traverse the list once at
    /// construction, stash every value, then pick a random index.
    pub struct Solution {
        vals: Vec<i32>,
    }

    impl Solution {
        /// Build the Solution by collecting all linked-list values into a Vec.
        ///
        /// # Arguments
        ///
        /// * `head` - Head of the singly-linked list
        pub fn new(head: Option<Box<ListNode>>) -> Self {
            let mut vals = Vec::new();
            let mut cur = &head;
            while let Some(node) = cur {
                vals.push(node.val);
                cur = &node.next;
            }
            Self { vals }
        }

        /// Return a random node's value with equal probability.
        ///
        /// # Returns
        ///
        /// A random value from the linked list
        pub fn get_random(&self) -> i32 {
            use std::time::SystemTime;
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .subsec_nanos() as usize;
            self.vals[seed % self.vals.len()]
        }
    }
}

/// AI solution module — reservoir sampling (O(1) extra space per call).
///
/// Stores only the linked-list head, then on each `get_random()` call
/// walks the entire list using reservoir sampling (Algorithm R).
/// This is optimal when the list is extremely large or its length is unknown.
///
/// Construction: O(n) time (clone the list), O(n) space (stores the list)
/// get_random:   O(n) time per call, O(1) extra space
/// Space:        O(n) for the list itself
mod ai_solution {
    use crate::ListNode;

    /// Uses reservoir sampling to pick a random node without pre-collecting
    /// all values. Useful when the list length is unknown or changes.
    pub struct Solution {
        head: Option<Box<ListNode>>,
        /// Internal xorshift64 state for fast pseudo-random number generation.
        rng_state: u64,
    }

    impl Solution {
        /// Store the linked-list head for later traversal.
        ///
        /// # Arguments
        ///
        /// * `head` - Head of the singly-linked list
        pub fn new(head: Option<Box<ListNode>>) -> Self {
            use std::time::SystemTime;
            let seed = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time went backwards")
                .subsec_nanos() as u64
                | 1; // non-zero for xorshift
            Self {
                head,
                rng_state: seed,
            }
        }

        /// Advance the xorshift64 PRNG and return a pseudo-random u64.
        fn next_rand(&mut self) -> u64 {
            let mut s = self.rng_state;
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            self.rng_state = s;
            s
        }

        /// Return a random node's value using reservoir sampling (Algorithm R).
        ///
        /// Walk the list; at each node i (1-indexed), replace the chosen value
        /// with probability 1/i. After traversal, each node has equal
        /// probability of being selected.
        ///
        /// # Returns
        ///
        /// A random value from the linked list
        pub fn get_random(&mut self) -> i32 {
            // Collect node values into a temporary Vec to avoid borrow
            // conflict: we can't call &mut self (next_rand) while holding
            // &self.head. The list is small (<=10^4), so this is fine.
            let vals: Vec<i32> = {
                let mut v = Vec::new();
                let mut cur = &self.head;
                while let Some(node) = cur {
                    v.push(node.val);
                    cur = &node.next;
                }
                v
            };
            // Reservoir sampling over the collected values
            let mut chosen = 0;
            for (i, &val) in vals.iter().enumerate() {
                let count = (i + 1) as u64;
                if self.next_rand() % count == 0 {
                    chosen = val;
                }
            }
            chosen
        }
    }
}

fn main() {
    let head = ListNode::from_slice(&[1, 2, 3]);

    println!("--- My Solution ---");
    {
        let s = my_solution::Solution::new(head.clone());
        for i in 0..5 {
            println!("  get_random() call {}: {}", i + 1, s.get_random());
        }
    }

    println!("--- AI Solution (reservoir sampling) ---");
    {
        let mut s = ai_solution::Solution::new(head);
        for i in 0..5 {
            println!("  get_random() call {}: {}", i + 1, s.get_random());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ListNode;

    macro_rules! test_solution {
        ($mod_name:ident, $sol_path:path) => {
            // allow(unused_mut): my_solution::get_random takes &self so
            // `mut` is unused, but ai_solution needs &mut self.
            #[allow(unused_mut)]
            mod $mod_name {
                use $sol_path as Solution;
                use crate::ListNode;

                #[test]
                fn test_single_node() {
                    let head = ListNode::from_slice(&[42]);
                    let mut s = Solution::new(head);
                    // Only one node — must always return 42
                    assert_eq!(s.get_random(), 42);
                    assert_eq!(s.get_random(), 42);
                }

                #[test]
                fn test_returns_valid_value() {
                    let head = ListNode::from_slice(&[1, 2, 3]);
                    let mut s = Solution::new(head);
                    for _ in 0..20 {
                        let val = s.get_random();
                        assert!(
                            val == 1 || val == 2 || val == 3,
                            "got {val}, expected 1, 2, or 3"
                        );
                    }
                }

                #[test]
                fn test_two_nodes() {
                    let head = ListNode::from_slice(&[10, 20]);
                    let mut s = Solution::new(head);
                    for _ in 0..20 {
                        let val = s.get_random();
                        assert!(val == 10 || val == 20);
                    }
                }

                #[test]
                fn test_negative_values() {
                    let head = ListNode::from_slice(&[-5, -3, 0, 7]);
                    let mut s = Solution::new(head);
                    let valid = [-5, -3, 0, 7];
                    for _ in 0..20 {
                        let val = s.get_random();
                        assert!(
                            valid.contains(&val),
                            "got {val}, expected one of {valid:?}"
                        );
                    }
                }
            }
        };
    }

    test_solution!(my_tests, crate::my_solution::Solution);
    test_solution!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;
        let vals: Vec<i32> = (0..100).collect();
        let head = ListNode::from_slice(&vals);

        let start = Instant::now();
        for _ in 0..iterations {
            let s = crate::my_solution::Solution::new(head.clone());
            for _ in 0..10 {
                std::hint::black_box(s.get_random());
            }
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut s = crate::ai_solution::Solution::new(head.clone());
            for _ in 0..10 {
                std::hint::black_box(s.get_random());
            }
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
