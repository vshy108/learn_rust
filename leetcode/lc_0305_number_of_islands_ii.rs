// LeetCode 305: Number of Islands II [Hard]
// https://leetcode.com/problems/number-of-islands-ii/
// Given m x n grid of water, add land operations at positions[i] = [ri, ci].
// Return array where answer[i] = number of islands after the i-th operation.
//
// Benchmark: 100_000 iterations
// My solution (Union-Find with rank): avg 0.703us
// AI solution (Union-Find with size):  avg 0.693us
// Winner: AI solution (1.01x faster)

// ------ My Solution: Union-Find with path compression + union by rank ------
// Time: O(k * alpha(m*n)) per operation — nearly O(k)
// Space: O(m*n) — parent and rank arrays
/// Standard Union-Find. For each new land cell, increment island count,
/// then merge with any adjacent land cells (decrementing count per merge).
mod my_solution {
    pub struct Solution;

    impl Solution {
        pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
            let (m, n) = (m as usize, n as usize);
            let total = m * n;
            let mut parent: Vec<usize> = (0..total).collect();
            let mut rank = vec![0u32; total];
            let mut grid = vec![vec![false; n]; m];
            let mut cnt = 0i32;
            let mut ans = Vec::with_capacity(positions.len());

            // Find with path compression
            fn find(parent: &mut [usize], x: usize) -> usize {
                if parent[x] != x {
                    parent[x] = find(parent, parent[x]);
                }
                parent[x]
            }

            for pos in &positions {
                let (i, j) = (pos[0] as usize, pos[1] as usize);
                if grid[i][j] {
                    // Duplicate position — island count unchanged
                    ans.push(cnt);
                    continue;
                }
                grid[i][j] = true;
                cnt += 1;
                let id = i * n + j;

                // Check 4 neighbors
                for (di, dj) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        let (ni, nj) = (ni as usize, nj as usize);
                        if grid[ni][nj] {
                            let nid = ni * n + nj;
                            let pa = find(&mut parent, id);
                            let pb = find(&mut parent, nid);
                            if pa != pb {
                                // Union by rank
                                if rank[pa] < rank[pb] {
                                    parent[pa] = pb;
                                } else if rank[pa] > rank[pb] {
                                    parent[pb] = pa;
                                } else {
                                    parent[pb] = pa;
                                    rank[pa] += 1;
                                }
                                cnt -= 1;
                            }
                        }
                    }
                }
                ans.push(cnt);
            }
            ans
        }
    }
}

// ------ AI Solution: Union-Find with path compression + union by size ------
// Time: O(k * alpha(m*n)) per operation
// Space: O(m*n) — parent and size arrays
/// Same Union-Find approach but merges by subtree size instead of rank.
/// Smaller tree is attached under larger tree's root.
mod ai_solution {
    pub struct Solution;

    impl Solution {
        pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
            let (m, n) = (m as usize, n as usize);
            let total = m * n;
            let mut parent: Vec<usize> = (0..total).collect();
            let mut size = vec![1usize; total];
            let mut grid = vec![vec![false; n]; m];
            let mut cnt = 0i32;
            let mut ans = Vec::with_capacity(positions.len());

            fn find(parent: &mut [usize], x: usize) -> usize {
                if parent[x] != x {
                    parent[x] = find(parent, parent[x]);
                }
                parent[x]
            }

            for pos in &positions {
                let (i, j) = (pos[0] as usize, pos[1] as usize);
                if grid[i][j] {
                    ans.push(cnt);
                    continue;
                }
                grid[i][j] = true;
                cnt += 1;
                let id = i * n + j;

                for (di, dj) in [(-1i32, 0), (1, 0), (0, -1), (0, 1)] {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                        let (ni, nj) = (ni as usize, nj as usize);
                        if grid[ni][nj] {
                            let nid = ni * n + nj;
                            let pa = find(&mut parent, id);
                            let pb = find(&mut parent, nid);
                            if pa != pb {
                                // Union by size: attach smaller under larger
                                if size[pa] >= size[pb] {
                                    parent[pb] = pa;
                                    size[pa] += size[pb];
                                } else {
                                    parent[pa] = pb;
                                    size[pb] += size[pa];
                                }
                                cnt -= 1;
                            }
                        }
                    }
                }
                ans.push(cnt);
            }
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_solution::Solution as AiSolution;
    use super::my_solution::Solution as MySolution;

    #[test]
    fn test_example1() {
        let positions = vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![2, 1]];
        assert_eq!(MySolution::num_islands2(3, 3, positions.clone()), vec![1, 1, 2, 3]);
        assert_eq!(AiSolution::num_islands2(3, 3, positions), vec![1, 1, 2, 3]);
    }

    #[test]
    fn test_example2() {
        let positions = vec![vec![0, 0]];
        assert_eq!(MySolution::num_islands2(1, 1, positions.clone()), vec![1]);
        assert_eq!(AiSolution::num_islands2(1, 1, positions), vec![1]);
    }

    #[test]
    fn test_duplicate_position() {
        let positions = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(MySolution::num_islands2(2, 2, positions.clone()), vec![1, 1]);
        assert_eq!(AiSolution::num_islands2(2, 2, positions), vec![1, 1]);
    }

    #[test]
    fn test_merge_two_islands() {
        // Two separate islands then a bridge
        let positions = vec![vec![0, 0], vec![0, 2], vec![0, 1]];
        assert_eq!(MySolution::num_islands2(1, 3, positions.clone()), vec![1, 2, 1]);
        assert_eq!(AiSolution::num_islands2(1, 3, positions), vec![1, 2, 1]);
    }

    #[test]
    fn test_fill_row() {
        let positions = vec![vec![0, 0], vec![0, 1], vec![0, 2]];
        assert_eq!(MySolution::num_islands2(1, 3, positions.clone()), vec![1, 1, 1]);
        assert_eq!(AiSolution::num_islands2(1, 3, positions), vec![1, 1, 1]);
    }

    #[test]
    fn test_fill_col() {
        let positions = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
        assert_eq!(MySolution::num_islands2(3, 1, positions.clone()), vec![1, 1, 1]);
        assert_eq!(AiSolution::num_islands2(3, 1, positions), vec![1, 1, 1]);
    }

    #[test]
    fn test_corners() {
        let positions = vec![vec![0, 0], vec![0, 2], vec![2, 0], vec![2, 2]];
        assert_eq!(MySolution::num_islands2(3, 3, positions.clone()), vec![1, 2, 3, 4]);
        assert_eq!(AiSolution::num_islands2(3, 3, positions), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_merge_four() {
        // Four corners then center merges all
        let positions = vec![
            vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![1, 1],
        ];
        assert_eq!(MySolution::num_islands2(3, 3, positions.clone()), vec![1, 2, 3, 4, 1]);
        assert_eq!(AiSolution::num_islands2(3, 3, positions), vec![1, 2, 3, 4, 1]);
    }

    #[test]
    fn test_single_cell_grid() {
        let positions = vec![vec![0, 0]];
        assert_eq!(MySolution::num_islands2(1, 1, positions.clone()), vec![1]);
        assert_eq!(AiSolution::num_islands2(1, 1, positions), vec![1]);
    }

    #[test]
    fn test_l_shape_merge() {
        let positions = vec![vec![0, 0], vec![1, 0], vec![1, 1]];
        assert_eq!(MySolution::num_islands2(2, 2, positions.clone()), vec![1, 1, 1]);
        assert_eq!(AiSolution::num_islands2(2, 2, positions), vec![1, 1, 1]);
    }
}

fn main() {
    println!("LeetCode 305: Number of Islands II");

    let positions = vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![2, 1]];
    println!(
        "My: {:?}",
        my_solution::Solution::num_islands2(3, 3, positions.clone())
    );
    println!(
        "AI: {:?}",
        ai_solution::Solution::num_islands2(3, 3, positions.clone())
    );

    // Benchmark
    let iterations = 100_000;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::Solution::num_islands2(
            3,
            3,
            std::hint::black_box(positions.clone()),
        ));
    }
    let my_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::Solution::num_islands2(
            3,
            3,
            std::hint::black_box(positions.clone()),
        ));
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
