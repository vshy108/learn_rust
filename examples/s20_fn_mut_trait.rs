// Demonstrates defining a method that accepts an FnMut closure.
//
// FnMut closures can be called multiple times and may mutate captures.
// Use when your method calls the closure repeatedly.
//
// Run: cargo run --example s20_fn_mut_trait

/// Applies a transformation N times, accumulating results.
fn repeat_apply<F>(n: usize, mut f: F) -> Vec<i32>
where
    F: FnMut(usize) -> i32,
{
    (0..n).map(|i| f(i)).collect()
}

/// A custom struct that processes items with a callback.
struct Processor<T> {
    items: Vec<T>,
}

impl<T> Processor<T> {
    fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    /// Process each item with a mutable closure.
    fn process_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        for item in &self.items {
            f(item);
        }
    }

    /// Transform items using a mutable closure.
    fn transform<U, F>(&self, mut f: F) -> Vec<U>
    where
        F: FnMut(&T) -> U,
    {
        self.items.iter().map(|item| f(item)).collect()
    }
}

fn main() {
    // --- 1. repeat_apply with mutable state ---
    let mut running_sum = 0;
    let results = repeat_apply(5, |i| {
        running_sum += i as i32 + 1;
        running_sum
    });
    println!("cumulative sums: {results:?}");

    // --- 2. Processor with counter ---
    let proc = Processor::new(vec!["alpha", "beta", "gamma"]);
    let mut index = 0;
    proc.process_each(|item| {
        println!("  [{index}] {item}");
        index += 1;
    });

    // --- 3. Transform with index ---
    let proc = Processor::new(vec![10, 20, 30]);
    let mut idx = 0;
    let labeled: Vec<String> = proc.transform(|&val| {
        let label = format!("item{idx}={val}");
        idx += 1;
        label
    });
    println!("labeled: {labeled:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_apply() {
        let results = repeat_apply(3, |i| (i * 2) as i32);
        assert_eq!(results, vec![0, 2, 4]);
    }

    #[test]
    fn test_processor_count() {
        let p = Processor::new(vec![1, 2, 3]);
        let mut count = 0;
        p.process_each(|_| count += 1);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_processor_transform() {
        let p = Processor::new(vec![1, 2, 3]);
        let doubled: Vec<i32> = p.transform(|&x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);
    }
}
