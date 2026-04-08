// LeetCode 303: Range Sum Query - Immutable [Easy]
// https://leetcode.com/problems/range-sum-query-immutable/
// Given an integer array nums, implement NumArray with sum_range(left, right)
// returning the sum of elements between indices left and right inclusive.
//
// Benchmark: 100_000 iterations
// My solution (prefix sum array): avg 0.070us
// AI solution (inclusive prefix):  avg 0.023us
// Winner: AI solution (2.97x faster)

// ------ My Solution: Prefix Sum with n+1 array ------
// Time: O(n) init, O(1) query
// Space: O(n) — prefix sum array
/// Builds prefix sum `s` where `s[i] = nums[0] + ... + nums[i-1]`.
/// Query: `s[right+1] - s[left]`.
mod my_solution {
    pub struct NumArray {
        s: Vec<i32>,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            let n = nums.len();
            let mut s = vec![0; n + 1];
            for i in 0..n {
                s[i + 1] = s[i] + nums[i];
            }
            Self { s }
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            self.s[(right + 1) as usize] - self.s[left as usize]
        }
    }
}

// ------ AI Solution: Prefix Sum with in-place cumulation ------
// Time: O(n) init, O(1) query
// Space: O(n) — stores cumulative sums in the original array
/// Transforms `nums` in-place so `nums[i]` holds the cumulative sum from
/// index 0 to i. Query uses `nums[right] - nums[left - 1]` (with bounds check).
mod ai_solution {
    pub struct NumArray {
        prefix: Vec<i32>,
    }

    impl NumArray {
        pub fn new(nums: Vec<i32>) -> Self {
            // Build inclusive prefix sums: prefix[i] = nums[0] + ... + nums[i]
            let mut prefix = nums;
            for i in 1..prefix.len() {
                prefix[i] += prefix[i - 1];
            }
            Self { prefix }
        }

        pub fn sum_range(&self, left: i32, right: i32) -> i32 {
            let r = self.prefix[right as usize];
            if left == 0 {
                r
            } else {
                r - self.prefix[(left - 1) as usize]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::NumArray as AiNumArray;
    use super::my_solution::NumArray as MyNumArray;

    #[test]
    fn test_example() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let my = MyNumArray::new(nums.clone());
        let ai = AiNumArray::new(nums);
        assert_eq!(my.sum_range(0, 2), 1);
        assert_eq!(ai.sum_range(0, 2), 1);
        assert_eq!(my.sum_range(2, 5), -1);
        assert_eq!(ai.sum_range(2, 5), -1);
        assert_eq!(my.sum_range(0, 5), -3);
        assert_eq!(ai.sum_range(0, 5), -3);
    }

    #[test]
    fn test_single_element() {
        let my = MyNumArray::new(vec![5]);
        let ai = AiNumArray::new(vec![5]);
        assert_eq!(my.sum_range(0, 0), 5);
        assert_eq!(ai.sum_range(0, 0), 5);
    }

    #[test]
    fn test_two_elements() {
        let my = MyNumArray::new(vec![3, 7]);
        let ai = AiNumArray::new(vec![3, 7]);
        assert_eq!(my.sum_range(0, 0), 3);
        assert_eq!(ai.sum_range(0, 0), 3);
        assert_eq!(my.sum_range(1, 1), 7);
        assert_eq!(ai.sum_range(1, 1), 7);
        assert_eq!(my.sum_range(0, 1), 10);
        assert_eq!(ai.sum_range(0, 1), 10);
    }

    #[test]
    fn test_all_negative() {
        let my = MyNumArray::new(vec![-1, -2, -3]);
        let ai = AiNumArray::new(vec![-1, -2, -3]);
        assert_eq!(my.sum_range(0, 2), -6);
        assert_eq!(ai.sum_range(0, 2), -6);
        assert_eq!(my.sum_range(1, 2), -5);
        assert_eq!(ai.sum_range(1, 2), -5);
    }

    #[test]
    fn test_full_range() {
        let nums = vec![1, 2, 3, 4, 5];
        let my = MyNumArray::new(nums.clone());
        let ai = AiNumArray::new(nums);
        assert_eq!(my.sum_range(0, 4), 15);
        assert_eq!(ai.sum_range(0, 4), 15);
    }

    #[test]
    fn test_same_index() {
        let nums = vec![10, 20, 30];
        let my = MyNumArray::new(nums.clone());
        let ai = AiNumArray::new(nums);
        assert_eq!(my.sum_range(1, 1), 20);
        assert_eq!(ai.sum_range(1, 1), 20);
    }

    #[test]
    fn test_zeros() {
        let my = MyNumArray::new(vec![0, 0, 0]);
        let ai = AiNumArray::new(vec![0, 0, 0]);
        assert_eq!(my.sum_range(0, 2), 0);
        assert_eq!(ai.sum_range(0, 2), 0);
    }

    #[test]
    fn test_multiple_queries() {
        let nums = vec![1, -1, 2, -2, 3];
        let my = MyNumArray::new(nums.clone());
        let ai = AiNumArray::new(nums);
        assert_eq!(my.sum_range(0, 1), 0);
        assert_eq!(ai.sum_range(0, 1), 0);
        assert_eq!(my.sum_range(2, 3), 0);
        assert_eq!(ai.sum_range(2, 3), 0);
        assert_eq!(my.sum_range(0, 4), 3);
        assert_eq!(ai.sum_range(0, 4), 3);
    }
}

fn main() {
    println!("LeetCode 303: Range Sum Query - Immutable");

    let nums = vec![-2, 0, 3, -5, 2, -1];
    let my = my_solution::NumArray::new(nums.clone());
    let ai = ai_solution::NumArray::new(nums);
    println!(
        "sum_range(0,2): My={}, AI={}",
        my.sum_range(0, 2),
        ai.sum_range(0, 2)
    );
    println!(
        "sum_range(2,5): My={}, AI={}",
        my.sum_range(2, 5),
        ai.sum_range(2, 5)
    );

    // Benchmark
    let iterations = 100_000;
    let test_nums = vec![-2, 0, 3, -5, 2, -1];

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let arr = my_solution::NumArray::new(std::hint::black_box(test_nums.clone()));
        std::hint::black_box(arr.sum_range(0, 2));
        std::hint::black_box(arr.sum_range(2, 5));
        std::hint::black_box(arr.sum_range(0, 5));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let arr = ai_solution::NumArray::new(std::hint::black_box(test_nums.clone()));
        std::hint::black_box(arr.sum_range(0, 2));
        std::hint::black_box(arr.sum_range(2, 5));
        std::hint::black_box(arr.sum_range(0, 5));
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
