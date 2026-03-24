// Demonstrates the lines method for iterating over text lines.
//
// Run: cargo run --example s21_lines

fn main() {
    let poem = "\
Roses are red,
Violets are blue,
Rust is great,
And so are you.";

    // --- 1. Basic lines iteration ---
    println!("Lines:");
    for (i, line) in poem.lines().enumerate() {
        println!("  {}: {line}", i + 1);
    }

    // --- 2. Count lines ---
    println!("total lines: {}", poem.lines().count());

    // --- 3. Find a specific line ---
    let rust_line = poem.lines().find(|l| l.contains("Rust"));
    println!("rust line: {rust_line:?}");

    // --- 4. Process CSV-like data ---
    let data = "\
name,age,city
Alice,30,NYC
Bob,25,LA
Charlie,35,Chicago";

    let records: Vec<Vec<&str>> = data
        .lines()
        .skip(1) // skip header
        .map(|line| line.split(',').collect())
        .collect();
    for record in &records {
        println!(
            "  name={}, age={}, city={}",
            record[0], record[1], record[2]
        );
    }

    // --- 5. Filter non-empty lines ---
    let with_blanks = "hello\n\nworld\n\n\nrust";
    let non_empty: Vec<&str> = with_blanks.lines().filter(|l| !l.is_empty()).collect();
    println!("non-empty: {non_empty:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lines_count() {
        let text = "a\nb\nc";
        assert_eq!(text.lines().count(), 3);
    }

    #[test]
    fn test_lines_find() {
        let text = "hello\nworld\nrust";
        let found = text.lines().find(|l| l.contains("world"));
        assert_eq!(found, Some("world"));
    }

    #[test]
    fn test_csv_parse() {
        let csv = "a,b\n1,2";
        let rows: Vec<Vec<&str>> = csv.lines().map(|l| l.split(',').collect()).collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[1], vec!["1", "2"]);
    }
}
