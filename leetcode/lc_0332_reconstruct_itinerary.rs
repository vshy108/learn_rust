// @lc app=leetcode id=332 lang=rust
// #[Hard] - Reconstruct Itinerary
// Iterations: 100000 | Date: 2025-04-08
// my_solution  | avg: 1.343us
// ai_solution  | avg: 1.324us
// Performance: AI 1.01x faster

/// # 332. Reconstruct Itinerary
///
/// Given airline tickets `[from, to]`, reconstruct the itinerary starting
/// from "JFK". Return the itinerary with smallest lexical order.
/// All tickets must be used exactly once.
///
/// Constraints:
/// - 1 <= tickets.length <= 300
/// - tickets[i].length == 2
/// - from_i.length == 3, to_i.length == 3
/// - from_i != to_i

/// ## My Solution — Hierholzer's (iterative with explicit stack)
///
/// Build adjacency list sorted in reverse. Use iterative DFS with an
/// explicit stack; pop destinations greedily. Reverse the result at end.
///
/// - Time: O(E log E)
/// - Space: O(E)
mod my_solution {
    use std::collections::HashMap;

    pub fn find_itinerary(tickets: &[Vec<String>]) -> Vec<String> {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        // Sort tickets so destinations are in reverse lex order (pop = smallest)
        let mut sorted_tickets: Vec<(&str, &str)> = tickets
            .iter()
            .map(|t| (t[0].as_str(), t[1].as_str()))
            .collect();
        sorted_tickets.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        for &(from, to) in &sorted_tickets {
            graph.entry(from).or_default().push(to);
        }

        let mut stack = vec!["JFK"];
        let mut route: Vec<String> = Vec::new();
        while let Some(&airport) = stack.last() {
            if let Some(dests) = graph.get_mut(airport) {
                if !dests.is_empty() {
                    let next = dests.pop().expect("checked non-empty");
                    stack.push(next);
                    continue;
                }
            }
            // No more destinations: add to route and backtrack
            route.push(stack.pop().expect("stack non-empty").to_string());
        }
        route.reverse();
        route
    }
}

/// ## AI Solution — Hierholzer's (recursive DFS)
///
/// Same Eulerian path algorithm but expressed recursively.
///
/// - Time: O(E log E)
/// - Space: O(E)
mod ai_solution {
    use std::collections::HashMap;

    pub fn find_itinerary(tickets: &[Vec<String>]) -> Vec<String> {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut sorted_tickets: Vec<(&str, &str)> = tickets
            .iter()
            .map(|t| (t[0].as_str(), t[1].as_str()))
            .collect();
        // Sort in reverse so pop() gives smallest lexicographically
        sorted_tickets.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        for &(from, to) in &sorted_tickets {
            graph.entry(from).or_default().push(to);
        }

        let mut route: Vec<String> = Vec::new();
        dfs("JFK", &mut graph, &mut route);
        route.reverse();
        route
    }

    fn dfs<'a>(
        airport: &'a str,
        graph: &mut HashMap<&'a str, Vec<&'a str>>,
        route: &mut Vec<String>,
    ) {
        while let Some(next) = graph.get_mut(airport).and_then(|v| v.pop()) {
            dfs(next, graph, route);
        }
        route.push(airport.to_string());
    }
}

fn run_tests(f: fn(&[Vec<String>]) -> Vec<String>) {
    let make = |v: Vec<[&str; 2]>| -> Vec<Vec<String>> {
        v.into_iter()
            .map(|[a, b]| vec![a.to_string(), b.to_string()])
            .collect()
    };
    // Example 1
    let tickets = make(vec![
        ["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"],
    ]);
    assert_eq!(
        f(&tickets),
        vec!["JFK", "MUC", "LHR", "SFO", "SJC"]
    );
    // Example 2
    let tickets = make(vec![
        ["JFK", "SFO"], ["JFK", "ATL"], ["SFO", "ATL"],
        ["ATL", "JFK"], ["ATL", "SFO"],
    ]);
    assert_eq!(
        f(&tickets),
        vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_solution() {
        run_tests(my_solution::find_itinerary);
    }

    #[test]
    fn test_ai_solution() {
        run_tests(ai_solution::find_itinerary);
    }
}

fn main() {
    let tickets: Vec<Vec<String>> = vec![
        vec!["MUC".into(), "LHR".into()],
        vec!["JFK".into(), "MUC".into()],
        vec!["SFO".into(), "SJC".into()],
        vec!["LHR".into(), "SFO".into()],
    ];
    let iterations = 100_000;

    for _ in 0..1000 {
        std::hint::black_box(my_solution::find_itinerary(
            std::hint::black_box(&tickets),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(my_solution::find_itinerary(
            std::hint::black_box(&tickets),
        ));
    }
    let my_dur = start.elapsed();

    for _ in 0..1000 {
        std::hint::black_box(ai_solution::find_itinerary(
            std::hint::black_box(&tickets),
        ));
    }
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ai_solution::find_itinerary(
            std::hint::black_box(&tickets),
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

    println!("LC 332 - Reconstruct Itinerary");
    println!("my_solution  | avg: {:.3}us", my_avg);
    println!("ai_solution  | avg: {:.3}us", ai_avg);
    println!("Performance: {ratio}");
}
