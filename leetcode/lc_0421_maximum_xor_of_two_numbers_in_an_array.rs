// Benchmark: 10,000 iterations (2025-04-08)
// My solution:  avg 3.41ms  | Binary Trie insert + XOR search
// AI solution:  avg 1.89ms  | HashSet bit-by-bit greedy
// -> AI solution is 1.80x faster
//
// LeetCode 421: Maximum XOR of Two Numbers in an Array [Medium]
// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
//
// Given an integer array `nums`, return the max result of nums[i] XOR nums[j].
//
// Examples:
//   [3,10,5,25,2,8] -> 28  (5 XOR 25 = 28)
//   [14,70,53,83,49,91,36,80,92,51,66,70] -> 127
//
// Constraints:
//   1 <= nums.length <= 2 * 10^5
//   0 <= nums[i] <= 2^31 - 1
//
// Run:  cargo run --example lc_0421_maximum_xor_of_two_numbers_in_an_array
// Test: cargo test --example lc_0421_maximum_xor_of_two_numbers_in_an_array
// Bench: cargo test --example lc_0421_maximum_xor_of_two_numbers_in_an_array bench -- --nocapture

/// My solution module — binary Trie.
///
/// Insert each number bit-by-bit (from MSB to LSB, 31 bits) into a trie.
/// For each number, greedily search the trie trying to go the opposite
/// bit at each level to maximize XOR.
///
/// Time:  O(n * 31)
/// Space: O(n * 31)
mod my_solution {
    pub struct Solution;

    /// A binary trie node with two children (0 and 1).
    struct TrieNode {
        children: [Option<Box<TrieNode>>; 2],
    }

    impl TrieNode {
        fn new() -> Self {
            Self { children: [None, None] }
        }
    }

    impl Solution {
        /// Find the maximum XOR of any two numbers in the array.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Maximum XOR value
        pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
            let mut root = TrieNode::new();
            let mut ans = 0;

            for &x in &nums {
                // Insert x into trie
                let mut node = &mut root;
                for i in (0..=30).rev() {
                    let bit = ((x >> i) & 1) as usize;
                    if node.children[bit].is_none() {
                        node.children[bit] = Some(Box::new(TrieNode::new()));
                    }
                    node = node.children[bit].as_mut()
                        .expect("just inserted");
                }

                // Search for max XOR partner
                let mut node = &root;
                let mut xor_val = 0;
                for i in (0..=30).rev() {
                    let bit = ((x >> i) & 1) as usize;
                    let want = bit ^ 1; // opposite bit maximizes XOR
                    if node.children[want].is_some() {
                        xor_val |= 1 << i;
                        node = node.children[want].as_ref()
                            .expect("checked");
                    } else {
                        node = node.children[bit].as_ref()
                            .expect("at least one path exists");
                    }
                }

                ans = ans.max(xor_val);
            }

            ans
        }
    }
}

/// AI solution module — HashSet bit-by-bit greedy.
///
/// Build the answer bit by bit from MSB to LSB. At each bit position,
/// try to set that bit in the answer. Collect all prefixes of that
/// length into a HashSet. Check if any pair of prefixes XOR to the
/// candidate answer. If so, keep that bit set.
///
/// Time:  O(n * 31)
/// Space: O(n)
mod ai_solution {
    use std::collections::HashSet;

    pub struct Solution;

    impl Solution {
        /// Find the maximum XOR of any two numbers in the array.
        ///
        /// # Arguments
        ///
        /// * `nums` - Integer array
        ///
        /// # Returns
        ///
        /// Maximum XOR value
        pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
            let mut ans = 0;

            for i in (0..=30).rev() {
                // Try to set bit i in the answer
                let candidate = ans | (1 << i);

                // Collect prefixes of all numbers down to bit i
                let prefixes: HashSet<i32> = nums
                    .iter()
                    .map(|&x| x >> i)
                    .collect();

                // Check if any pair XORs to `candidate >> i`
                let target = candidate >> i;
                let found = prefixes
                    .iter()
                    .any(|&p| prefixes.contains(&(p ^ target)));

                if found {
                    ans = candidate;
                }
            }

            ans
        }
    }
}

fn main() {
    let cases = [
        (vec![3, 10, 5, 25, 2, 8], 28),
        (vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70], 127),
    ];

    println!("--- My Solution ---");
    for (nums, expected) in &cases {
        let result = my_solution::Solution::find_maximum_xor(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }

    println!("--- AI Solution ---");
    for (nums, expected) in &cases {
        let result = ai_solution::Solution::find_maximum_xor(nums.clone());
        println!("  {nums:?} -> {result} (expected {expected})");
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_xor {
        ($mod_name:ident, $sol_path:path) => {
            mod $mod_name {
                use $sol_path as Solution;

                #[test]
                fn test_example1() {
                    assert_eq!(
                        Solution::find_maximum_xor(
                            vec![3, 10, 5, 25, 2, 8],
                        ),
                        28,
                    );
                }

                #[test]
                fn test_example2() {
                    assert_eq!(
                        Solution::find_maximum_xor(
                            vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51,
                                 66, 70],
                        ),
                        127,
                    );
                }

                #[test]
                fn test_single() {
                    // XOR of a number with itself is 0
                    assert_eq!(
                        Solution::find_maximum_xor(vec![5]),
                        0,
                    );
                }

                #[test]
                fn test_two_elements() {
                    assert_eq!(
                        Solution::find_maximum_xor(vec![1, 2]),
                        3,
                    );
                }

                #[test]
                fn test_all_zeros() {
                    assert_eq!(
                        Solution::find_maximum_xor(vec![0, 0, 0]),
                        0,
                    );
                }

                #[test]
                fn test_powers_of_two() {
                    // 1 XOR 8 = 9, 2 XOR 8 = 10, 4 XOR 8 = 12
                    // max is 1 XOR 2 XOR 4 XOR 8? No, pairs only.
                    // 1^2=3, 1^4=5, 1^8=9, 2^4=6, 2^8=10, 4^8=12
                    assert_eq!(
                        Solution::find_maximum_xor(vec![1, 2, 4, 8]),
                        12,
                    );
                }
            }
        };
    }

    test_xor!(my_tests, crate::my_solution::Solution);
    test_xor!(ai_tests, crate::ai_solution::Solution);

    #[test]
    fn bench_comparison() {
        use std::time::Instant;

        let iterations = 10_000_u32;
        let nums: Vec<i32> = (0..10_000).map(|i| i * 37 % 100_000).collect();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::my_solution::Solution::find_maximum_xor(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let my_duration = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(
                crate::ai_solution::Solution::find_maximum_xor(
                    std::hint::black_box(nums.clone()),
                ),
            );
        }
        let ai_duration = start.elapsed();

        let my_avg = my_duration / iterations;
        let ai_avg = ai_duration / iterations;

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution:  {my_duration:.2?} (avg {my_avg:.2?})");
        println!("AI solution:  {ai_duration:.2?} (avg {ai_avg:.2?})");

        if my_avg <= ai_avg {
            let ratio = ai_avg.as_nanos() as f64
                / my_avg.as_nanos().max(1) as f64;
            println!("-> My solution is {ratio:.2}x faster");
        } else {
            let ratio = my_avg.as_nanos() as f64
                / ai_avg.as_nanos().max(1) as f64;
            println!("-> AI solution is {ratio:.2}x faster");
        }
    }
}
