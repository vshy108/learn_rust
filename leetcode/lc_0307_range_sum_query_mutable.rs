// LeetCode 307: Range Sum Query - Mutable [Medium]
// https://leetcode.com/problems/range-sum-query-mutable/
// Given an integer array nums, support update(index, val) and sumRange(left, right).
//
// Benchmark: 100_000 iterations
// My solution (Binary Indexed Tree): avg 0.100us
// AI solution (Segment Tree):        avg 0.129us
// Winner: My solution (1.29x faster)

// ------ My Solution: Binary Indexed Tree (Fenwick Tree) ------
// Time: O(n) init, O(log n) update, O(log n) query
// Space: O(n)
/// Classic BIT where `c[x]` stores partial sums. Update adds a delta,
/// query computes prefix sum. Range sum = query(right+1) - query(left).
mod my_solution {
    pub struct NumArray {
        n: usize,
        c: Vec<i32>,
        nums: Vec<i32>,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let n = nums.len();
            let mut c = vec![0i32; n + 1];
            // Build BIT by adding each element
            for i in 0..n {
                let mut x = i + 1;
                let v = nums[i];
                while x <= n {
                    c[x] += v;
                    x += x & x.wrapping_neg(); // x += x & (-x)
                }
            }
            Self { n, c, nums }
        }

        pub fn update(&mut self, index: i32, val: i32) {
            let idx = index as usize;
            let delta = val - self.nums[idx];
            self.nums[idx] = val;
            let mut x = idx + 1;
            while x <= self.n {
                self.c[x] += delta;
                x += x & x.wrapping_neg();
            }
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.prefix_sum(right as usize + 1) - self.prefix_sum(left as usize)
        }

        /// Returns prefix sum of elements [0, x) using 1-indexed BIT.
        fn prefix_sum(&self, mut x: usize) -> i32 {
            let mut s = 0;
            while x > 0 {
                s += self.c[x];
                x -= x & x.wrapping_neg();
            }
            s
        }
    }
}

// ------ AI Solution: Segment Tree ------
// Time: O(n) init, O(log n) update, O(log n) query
// Space: O(4n) — segment tree array
/// Array-based segment tree. Each node stores sum of its range.
/// Point update and range query in O(log n).
mod ai_solution {
    pub struct NumArray {
        n: usize,
        tree: Vec<i32>,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let n = nums.len();
            let mut tree = vec![0i32; 4 * n];
            if n > 0 {
                Self::build(&nums, &mut tree, 1, 0, n - 1);
            }
            Self { n, tree }
        }

        fn build(nums: &[i32], tree: &mut [i32], node: usize, start: usize, end: usize) {
            if start == end {
                tree[node] = nums[start];
                return;
            }
            let mid = (start + end) / 2;
            Self::build(nums, tree, 2 * node, start, mid);
            Self::build(nums, tree, 2 * node + 1, mid + 1, end);
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }

        pub fn update(&mut self, index: i32, val: i32) {
            Self::update_helper(&mut self.tree, 1, 0, self.n - 1, index as usize, val);
        }

        fn update_helper(
            tree: &mut [i32],
            node: usize,
            start: usize,
            end: usize,
            idx: usize,
            val: i32,
        ) {
            if start == end {
                tree[node] = val;
                return;
            }
            let mid = (start + end) / 2;
            if idx <= mid {
                Self::update_helper(tree, 2 * node, start, mid, idx, val);
            } else {
                Self::update_helper(tree, 2 * node + 1, mid + 1, end, idx, val);
            }
            tree[node] = tree[2 * node] + tree[2 * node + 1];
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            Self::query(&self.tree, 1, 0, self.n - 1, left as usize, right as usize)
        }

        fn query(
            tree: &[i32],
            node: usize,
            start: usize,
            end: usize,
            l: usize,
            r: usize,
        ) -> i32 {
            if l > end || r < start {
                return 0;
            }
            if l <= start && end <= r {
                return tree[node];
            }
            let mid = (start + end) / 2;
            Self::query(tree, 2 * node, start, mid, l, r)
                + Self::query(tree, 2 * node + 1, mid + 1, end, l, r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::NumArray as AiArray;
    use super::my_solution::NumArray as MyArray;

    #[test]
    fn test_example() {
        let mut my = MyArray::new(vec![1, 3, 5]);
        let mut ai = AiArray::new(vec![1, 3, 5]);
        assert_eq!(my.sum_range(0, 2), 9);
        assert_eq!(ai.sum_range(0, 2), 9);
        my.update(1, 2);
        ai.update(1, 2);
        assert_eq!(my.sum_range(0, 2), 8);
        assert_eq!(ai.sum_range(0, 2), 8);
    }

    #[test]
    fn test_single_element() {
        let mut my = MyArray::new(vec![5]);
        let mut ai = AiArray::new(vec![5]);
        assert_eq!(my.sum_range(0, 0), 5);
        assert_eq!(ai.sum_range(0, 0), 5);
        my.update(0, 10);
        ai.update(0, 10);
        assert_eq!(my.sum_range(0, 0), 10);
        assert_eq!(ai.sum_range(0, 0), 10);
    }

    #[test]
    fn test_multiple_updates() {
        let mut my = MyArray::new(vec![1, 2, 3, 4, 5]);
        let mut ai = AiArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(my.sum_range(0, 4), 15);
        assert_eq!(ai.sum_range(0, 4), 15);
        my.update(2, 10);
        ai.update(2, 10);
        assert_eq!(my.sum_range(0, 4), 22);
        assert_eq!(ai.sum_range(0, 4), 22);
        assert_eq!(my.sum_range(2, 4), 19);
        assert_eq!(ai.sum_range(2, 4), 19);
    }

    #[test]
    fn test_negative_values() {
        let mut my = MyArray::new(vec![-1, -2, -3]);
        let mut ai = AiArray::new(vec![-1, -2, -3]);
        assert_eq!(my.sum_range(0, 2), -6);
        assert_eq!(ai.sum_range(0, 2), -6);
        my.update(1, 5);
        ai.update(1, 5);
        assert_eq!(my.sum_range(0, 2), 1);
        assert_eq!(ai.sum_range(0, 2), 1);
    }

    #[test]
    fn test_partial_range() {
        let my = MyArray::new(vec![1, 2, 3, 4, 5]);
        let ai = AiArray::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(my.sum_range(1, 3), 9);
        assert_eq!(ai.sum_range(1, 3), 9);
        assert_eq!(my.sum_range(2, 2), 3);
        assert_eq!(ai.sum_range(2, 2), 3);
    }

    #[test]
    fn test_update_same_value() {
        let mut my = MyArray::new(vec![1, 2, 3]);
        let mut ai = AiArray::new(vec![1, 2, 3]);
        my.update(1, 2); // no change
        ai.update(1, 2);
        assert_eq!(my.sum_range(0, 2), 6);
        assert_eq!(ai.sum_range(0, 2), 6);
    }

    #[test]
    fn test_sequential_updates() {
        let mut my = MyArray::new(vec![0, 0, 0, 0]);
        let mut ai = AiArray::new(vec![0, 0, 0, 0]);
        for i in 0..4 {
            my.update(i, (i + 1) as i32);
            ai.update(i, (i + 1) as i32);
        }
        assert_eq!(my.sum_range(0, 3), 10);
        assert_eq!(ai.sum_range(0, 3), 10);
    }

    #[test]
    fn test_two_elements() {
        let my = MyArray::new(vec![3, 7]);
        let ai = AiArray::new(vec![3, 7]);
        assert_eq!(my.sum_range(0, 1), 10);
        assert_eq!(ai.sum_range(0, 1), 10);
        assert_eq!(my.sum_range(0, 0), 3);
        assert_eq!(ai.sum_range(0, 0), 3);
    }
}

fn main() {
    println!("LeetCode 307: Range Sum Query - Mutable");

    let mut my = my_solution::NumArray::new(vec![1, 3, 5]);
    let mut ai = ai_solution::NumArray::new(vec![1, 3, 5]);
    println!("sumRange(0,2): My={}, AI={}", my.sum_range(0, 2), ai.sum_range(0, 2));
    my.update(1, 2);
    ai.update(1, 2);
    println!("After update(1,2), sumRange(0,2): My={}, AI={}", my.sum_range(0, 2), ai.sum_range(0, 2));

    // Benchmark
    let iterations = 100_000;
    let nums = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut arr = my_solution::NumArray::new(std::hint::black_box(nums.clone()));
        std::hint::black_box(arr.sum_range(0, 9));
        arr.update(5, 100);
        std::hint::black_box(arr.sum_range(3, 7));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut arr = ai_solution::NumArray::new(std::hint::black_box(nums.clone()));
        std::hint::black_box(arr.sum_range(0, 9));
        arr.update(5, 100);
        std::hint::black_box(arr.sum_range(3, 7));
    }
    let ai_time = start.elapsed();

    let my_avg = my_time.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_time.as_nanos() as f64 / iterations as f64 / 1000.0;

    println!("\nBenchmark ({iterations} iterations):");
    println!("  My solution:  {my_avg:.3}us avg");
    println!("  AI solution:  {ai_avg:.3}us avg");
    if my_avg < ai_avg {
        println!("  Winner: My solution ({:.2}x faster)", ai_avg / my_avg);
    } else {
        println!("  Winner: AI solution ({:.2}x faster)", my_avg / ai_avg);
    }
}
