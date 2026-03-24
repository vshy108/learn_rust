// Demonstrates deref coercions in Rust.
//
// Deref coercion automatically converts a reference to a type that
// implements Deref into a reference to its Target type.
//
// Coercion rules:
// - `&T` -> `&U` when `T: Deref<Target = U>` (immutable)
// - `&mut T` -> `&mut U` when `T: DerefMut<Target = U>` (mutable)
// - `&mut T` -> `&U` when `T: Deref<Target = U>` (mut to immut, OK)
// - `&T` -> `&mut U` is NEVER allowed (immut to mut)
//
// Run: cargo run --example s26_deref_coercions

fn main() {
    println!("=== Deref Coercions ===");

    // --- 1. String -> str ---
    let s = String::from("hello");
    greet(&s); // &String -> &str

    // --- 2. Vec -> slice ---
    let v = vec![1, 2, 3];
    print_sum(&v); // &Vec<i32> -> &[i32]

    // --- 3. Box -> inner ---
    let b = Box::new(42);
    print_num(&b); // &Box<i32> -> &i32

    // --- 4. Multi-level coercion ---
    let b = Box::new(String::from("chained"));
    greet(&b); // &Box<String> -> &String -> &str

    // --- 5. Mutable deref coercion ---
    let mut v = vec![1, 2, 3];
    push_value(&mut v); // &mut Vec<i32> -> can call slice methods too
    println!("after push: {v:?}");

    // --- 6. &mut T can coerce to &T ---
    let mut s = String::from("mutable");
    greet(&s); // &String -> &str (coercion)
    let _ = &mut s; // &mut String can also coerce to &str

    // --- 7. Array references ---
    let arr = [1, 2, 3, 4, 5];
    print_sum(&arr); // &[i32; 5] -> &[i32]
}

/// Accepts a &str — String and Box<String> coerce to this.
fn greet(name: &str) {
    println!("  hello, {name}!");
}

/// Accepts a slice — Vec and arrays coerce to this.
fn print_sum(nums: &[i32]) {
    println!("  sum: {}", nums.iter().sum::<i32>());
}

/// Accepts &i32 — Box<i32> coerces to this.
fn print_num(n: &i32) {
    println!("  num: {n}");
}

/// Accepts &mut Vec — demonstrates mutable coercion.
fn push_value(v: &mut Vec<i32>) {
    v.push(99);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_string_to_str_coercion() {
        fn takes_str(s: &str) -> usize {
            s.len()
        }
        let s = String::from("test");
        assert_eq!(takes_str(&s), 4);
    }

    #[test]
    fn test_vec_to_slice_coercion() {
        fn takes_slice(s: &[i32]) -> i32 {
            s.iter().sum()
        }
        let v = vec![1, 2, 3];
        assert_eq!(takes_slice(&v), 6);
    }

    #[test]
    fn test_box_deref_coercion() {
        fn takes_ref(n: &i32) -> i32 {
            *n * 2
        }
        let b = Box::new(21);
        assert_eq!(takes_ref(&b), 42);
    }
}
