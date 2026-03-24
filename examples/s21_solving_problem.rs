// Demonstrates solving a practical problem with iterators.
//
// Problem: Given exam scores, calculate statistics and letter grades.
//
// Run: cargo run --example s21_solving_problem

fn letter_grade(score: f64) -> &'static str {
    match score as u32 {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

fn main() {
    let students = vec![
        ("Alice", vec![95, 87, 92]),
        ("Bob", vec![78, 65, 82]),
        ("Charlie", vec![90, 95, 88]),
        ("Diana", vec![55, 62, 58]),
        ("Eve", vec![100, 98, 97]),
    ];

    // Calculate average for each student:
    let averages: Vec<(&str, f64)> = students
        .iter()
        .map(|(name, scores)| {
            let avg = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
            (*name, avg)
        })
        .collect();

    // Display grades:
    println!("Student Report Card:");
    for (name, avg) in &averages {
        println!("  {name:<10} avg: {avg:>5.1} grade: {}", letter_grade(*avg));
    }

    // Class statistics:
    let class_avg = averages.iter().map(|(_, a)| a).sum::<f64>() / averages.len() as f64;
    println!("\nClass average: {class_avg:.1}");

    // Who passed (>= 60)?
    let passed: Vec<&str> = averages
        .iter()
        .filter(|(_, avg)| *avg >= 60.0)
        .map(|(name, _)| *name)
        .collect();
    println!("Passed: {passed:?}");

    // Highest scorer:
    let top = averages
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .expect("no students");
    println!("Top student: {} ({:.1})", top.0, top.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_grade() {
        assert_eq!(letter_grade(95.0), "A");
        assert_eq!(letter_grade(85.0), "B");
        assert_eq!(letter_grade(75.0), "C");
        assert_eq!(letter_grade(65.0), "D");
        assert_eq!(letter_grade(55.0), "F");
    }
}
