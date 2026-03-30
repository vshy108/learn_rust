// Overview of smart pointers in Rust.
//
// Smart pointers own data and provide additional capabilities beyond references.
// They implement `Deref` (act like references) and `Drop` (cleanup on scope exit).
//
// Common smart pointers:
// - `Box<T>`: heap allocation, single ownership
// - `Rc<T>`: reference-counted shared ownership (single-thread only, NOT Send)
// - `Arc<T>`: atomic reference-counted (multi-thread, Send + Sync)
// - `Cell<T>` / `RefCell<T>`: interior mutability
//
// Box vs Rc vs Arc:
// ┌────────────┬──────────────┬──────────────────┬─────────────────────┐
// │ Pointer    │ Thread-safe? │ Multiple owners? │ Overhead            │
// ├────────────┼──────────────┼──────────────────┼─────────────────────┤
// │ Box<T>     │ Yes (1 owner)│ No               │ None (heap pointer) │
// │ Rc<T>      │ No           │ Yes              │ Non-atomic refcount │
// │ Arc<T>     │ Yes          │ Yes              │ Atomic refcount     │
// └────────────┴──────────────┴──────────────────┴─────────────────────┘
//
// Rc/Arc are read-only by default. For mutation combine with:
//   Rc<RefCell<T>>  — single-threaded shared mutable data
//   Arc<Mutex<T>>   — multi-threaded shared mutable data
//   Arc<RwLock<T>>  — multi-threaded, many readers OR one writer
//
// Run: cargo run --example s26_smart_pointers

fn main() {
    println!("=== Smart Pointers Overview ===");

    // --- 1. Box: heap allocation ---
    let boxed = Box::new(42);
    println!("Box value: {boxed}");
    // boxed is automatically freed when it goes out of scope (Drop)

    // --- 2. String is a smart pointer ---
    let s = String::from("hello");
    // String owns heap data, implements Deref<Target=str>
    print_str(&s); // Deref coercion: &String -> &str

    // --- 3. Vec is a smart pointer ---
    let v = vec![1, 2, 3];
    print_slice(&v); // Deref coercion: &Vec<i32> -> &[i32]

    // --- 4. Box for trait objects ---
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rect { w: 4.0, h: 5.0 }),
    ];
    for s in &shapes {
        println!("  area: {:.1}", s.area());
    }

    // --- 5. Rc: shared ownership, single-thread ---
    // Rc<T> = Reference Counted. Multiple owners of the same heap data.
    // Rc::clone() is cheap — increments a counter, NOT a deep copy.
    use std::rc::Rc;
    let shared_str: Rc<String> = Rc::new(String::from("shared data"));
    let clone1 = Rc::clone(&shared_str); // refcount: 2
    let clone2 = Rc::clone(&shared_str); // refcount: 3
    println!("Rc strong count: {}", Rc::strong_count(&shared_str)); // 3
    drop(clone2); // refcount: 2
    println!("After drop: {}", Rc::strong_count(&shared_str)); // 2
    println!("Rc value: {clone1}");
    // Rc is NOT Send — cannot be sent to another thread:
    // std::thread::spawn(move || println!("{shared_str}")); // COMPILE ERROR

    // --- 6. Arc: shared ownership, multi-thread ---
    // Arc<T> = Atomic Reference Counted. Same as Rc but thread-safe.
    // Use Arc when you need shared ownership across threads.
    use std::sync::Arc;
    let shared_num = Arc::new(42);
    let thread_copy = Arc::clone(&shared_num); // atomic refcount increment
    let handle = std::thread::spawn(move || {
        // thread_copy is Send + Sync — works across threads
        println!("Arc in thread: {thread_copy}");
    });
    handle.join().expect("thread panicked");
    println!("Arc in main: {shared_num}");

    // --- 7. Arc<Mutex<T>>: shared MUTABLE state across threads ---
    // Arc alone is read-only. Mutex adds interior mutability.
    use std::sync::Mutex;
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..3 {
        let counter = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            let mut val = counter.lock().expect("mutex poisoned");
            *val += 1;
        }));
    }
    for h in handles {
        h.join().expect("thread panicked");
    }
    println!("Arc<Mutex> counter: {}", counter.lock().expect("mutex poisoned"));
}

/// Print a string slice (demonstrates deref coercion from String).
fn print_str(s: &str) {
    println!("str: {s}");
}

/// Print a slice (demonstrates deref coercion from Vec).
fn print_slice(s: &[i32]) {
    println!("slice: {s:?}");
}

trait Shape {
    /// Calculate the area of this shape.
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rect {
    w: f64,
    h: f64,
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_box_deref() {
        let b = Box::new(42);
        assert_eq!(*b, 42); // Deref
    }

    #[test]
    fn test_trait_object_area() {
        let c: Box<dyn Shape> = Box::new(Circle { radius: 1.0 });
        let expected = std::f64::consts::PI;
        assert!((c.area() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_rc_shared_ownership() {
        let data = Rc::new(vec![1, 2, 3]);
        let clone = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        assert_eq!(*clone, vec![1, 2, 3]);
        drop(clone);
        assert_eq!(Rc::strong_count(&data), 1);
    }

    #[test]
    fn test_arc_across_threads() {
        let data = Arc::new(42);
        let thread_data = Arc::clone(&data);
        let handle = std::thread::spawn(move || *thread_data);
        assert_eq!(handle.join().expect("thread panicked"), 42);
    }

    #[test]
    fn test_arc_mutex_shared_mutation() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            handles.push(std::thread::spawn(move || {
                *counter.lock().expect("mutex poisoned") += 1;
            }));
        }
        for h in handles {
            h.join().expect("thread panicked");
        }
        assert_eq!(*counter.lock().expect("mutex poisoned"), 5);
    }
}
