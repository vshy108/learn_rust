#![allow(dead_code, unused_imports, unused_parens)]
// Demonstrates the Box smart pointer.
//
// `Box<T>` allocates data on the heap. Useful for:
// 1. Types whose size can't be known at compile time
// 2. Large data you don't want to copy on the stack
// 3. Owning a value that implements a specific trait (trait objects)
//
// Run: cargo run --example s26_box

fn main() {
    println!("=== The Box Smart Pointer ===");

    // --- 1. Basic Box ---
    let b = Box::new(5);
    println!("boxed: {b}"); // auto-derefs for Display

    // --- 2. Box with large data ---
    // Without Box, this array would be on the stack (80,000 bytes!)
    let _big: Box<[i32; 20_000]> = Box::new([0; 20_000]);
    println!(
        "big array on heap, stack cost: {} bytes",
        size_of::<Box<[i32; 20_000]>>()
    );

    // --- 3. Recursive types require Box ---
    // Without Box, Cons would have infinite size
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("list: {list:?}");
    println!("List size: {} bytes", size_of::<List>());

    // --- 4. Box owns the data ---
    let s = Box::new(String::from("hello"));
    let s2 = s; // ownership moves, s is no longer valid
    println!("moved: {s2}");

    // --- 5. Box auto-freed on scope exit ---
    {
        let _temp = Box::new(vec![1, 2, 3]);
        // _temp and its Vec are freed here
    }
    println!("temp box was dropped");
}

/// Helper to demonstrate `size_of` — returns the size of a type.
fn size_of<T>() -> usize {
    std::mem::size_of::<T>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_box_deref() {
        let b = Box::new(42);
        assert_eq!(*b, 42);
    }

    #[test]
    fn test_box_size() {
        // Box<T> is always pointer-sized (8 bytes on 64-bit)
        assert_eq!(
            std::mem::size_of::<Box<i32>>(),
            std::mem::size_of::<usize>()
        );
    }

    #[test]
    fn test_recursive_type() {
        #[derive(Debug, PartialEq)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        let list = List::Cons(1, Box::new(List::Nil));
        assert!(matches!(list, List::Cons(1, _)));
    }
}
