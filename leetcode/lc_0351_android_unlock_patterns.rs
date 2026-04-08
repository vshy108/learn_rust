// @lc app=leetcode id=351 lang=rust
// #[Medium] - Android Unlock Patterns
// Iterations: 1000 | Date: 2025-07-09
// my_solution  | avg: 1561.459us
// ai_solution  | avg: 4433.134us
// Performance: My 2.84x faster

/// # 351. Android Unlock Patterns
///
/// Count valid unlock patterns on a 3x3 grid using between m and n keys.
/// A pattern is valid if every jump over an unvisited middle dot is avoided.
///
/// Constraints:
/// - 1 <= m, n <= 9

/// ## My Solution — DFS backtracking with symmetry
///
/// Exploit symmetry: corners (1,3,7,9) are equivalent, edges (2,4,6,8) are
/// equivalent, and center (5) is unique. So total = dfs(1)*4 + dfs(2)*4 + dfs(5).
///
/// - Time: O(n!) worst case, pruned by constraints
/// - Space: O(n)
mod my_solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        // cross[i][j] = the dot that must be visited before jumping from i to j
        let mut cross = [[0u8; 10]; 10];
        cross[1][3] = 2;
        cross[3][1] = 2;
        cross[1][7] = 4;
        cross[7][1] = 4;
        cross[1][9] = 5;
        cross[9][1] = 5;
        cross[2][8] = 5;
        cross[8][2] = 5;
        cross[3][7] = 5;
        cross[7][3] = 5;
        cross[3][9] = 6;
        cross[9][3] = 6;
        cross[4][6] = 5;
        cross[6][4] = 5;
        cross[7][9] = 8;
        cross[9][7] = 8;

        let mut vis = [false; 10];

        fn dfs(
            i: usize,
            cnt: i32,
            m: i32,
            n: i32,
            vis: &mut [bool; 10],
            cross: &[[u8; 10]; 10],
        ) -> i32 {
            if cnt > n {
                return 0;
            }
            vis[i] = true;
            let mut ans = if cnt >= m { 1 } else { 0 };
            for j in 1..=9 {
                let x = cross[i][j] as usize;
                // Can visit j if not visited and (no crossing dot OR crossing dot already visited)
                if !vis[j] && (x == 0 || vis[x]) {
                    ans += dfs(j, cnt + 1, m, n, vis, cross);
                }
            }
            vis[i] = false;
            ans
        }

        // Use symmetry: 4 corners * dfs(1) + 4 edges * dfs(2) + 1 center * dfs(5)
        dfs(1, 1, m, n, &mut vis, &cross) * 4
            + dfs(2, 1, m, n, &mut vis, &cross) * 4
            + dfs(5, 1, m, n, &mut vis, &cross)
    }
}

/// ## AI Solution — DFS without symmetry optimization
///
/// Iterate all 9 starting positions individually. Simpler but ~4.5x more work.
///
/// - Time: O(9 * n!)
/// - Space: O(n)
mod ai_solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut cross = [[0u8; 10]; 10];
        cross[1][3] = 2;
        cross[3][1] = 2;
        cross[1][7] = 4;
        cross[7][1] = 4;
        cross[1][9] = 5;
        cross[9][1] = 5;
        cross[2][8] = 5;
        cross[8][2] = 5;
        cross[3][7] = 5;
        cross[7][3] = 5;
        cross[3][9] = 6;
        cross[9][3] = 6;
        cross[4][6] = 5;
        cross[6][4] = 5;
        cross[7][9] = 8;
        cross[9][7] = 8;

        let mut vis = [false; 10];
        let mut total = 0;

        for start in 1..=9 {
            total += dfs(start, 1, m, n, &mut vis, &cross);
        }
        total
    }

    fn dfs(
        i: usize,
        cnt: i32,
        m: i32,
        n: i32,
        vis: &mut [bool; 10],
        cross: &[[u8; 10]; 10],
    ) -> i32 {
        if cnt > n {
            return 0;
        }
        vis[i] = true;
        let mut ans = if cnt >= m { 1 } else { 0 };
        for j in 1..=9 {
            let x = cross[i][j] as usize;
            if !vis[j] && (x == 0 || vis[x]) {
                ans += dfs(j, cnt + 1, m, n, vis, cross);
            }
        }
        vis[i] = false;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_tests(f: fn(i32, i32) -> i32) {
        assert_eq!(f(1, 1), 9);
        assert_eq!(f(1, 2), 65);
        assert_eq!(f(2, 2), 56); // 65 - 9
        assert_eq!(f(1, 9), 389497);
    }

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::number_of_patterns);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::number_of_patterns);
    }
}

fn main() {
    let (m, n) = (1, 9);
    let iterations = 1_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::number_of_patterns(
            std::hint::black_box(m),
            std::hint::black_box(n),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::number_of_patterns(
            std::hint::black_box(m),
            std::hint::black_box(n),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::number_of_patterns(
            std::hint::black_box(m),
            std::hint::black_box(n),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::number_of_patterns(
            std::hint::black_box(m),
            std::hint::black_box(n),
        ));
    }
    let ai_dur = start.elapsed();

    let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
    let ratio = if my_avg < ai_avg {
        format!("My {:.2}x faster", ai_avg / my_avg)
    } else {
        format!("AI {:.2}x faster", my_avg / ai_avg)
    };

    println!("LC 351 - Android Unlock Patterns");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
