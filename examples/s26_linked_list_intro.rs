// Intro to linked lists: why they need Box in Rust.
//
// A linked list node contains a value and a pointer to the next node.
// Because each node can point to another node, the type is recursive.
// Rust needs to know the size of every type at compile time, so we
// must use `Box` to give nodes a known size (one pointer).
//
// **Python analogy:** In Python, `node.next = other_node` just stores
// a reference (pointer). In Rust, we must be explicit with `Box`.
//
// Run: cargo run --example s26_linked_list_intro

fn main() {
    println!("=== Intro to Linked Lists ===");

    // --- Why Box is needed ---
    // This would NOT compile without Box:
    // enum List { Cons(i32, List), Nil }  // infinite size!
    //
    // With Box, each Cons is: i32 (4 bytes) + Box<List> (8 bytes) = known size

    // --- Simple linked list ---
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // Build: 1 -> 2 -> 3 -> Nil
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("{list:?}");

    // --- Traverse ---
    let mut current = &list;
    loop {
        match current {
            List::Cons(val, next) => {
                print!("{val} -> ");
                current = next;
            }
            List::Nil => {
                println!("Nil");
                break;
            }
        }
    }

    // --- Size comparison ---
    println!("List enum size: {} bytes", std::mem::size_of::<List>());
    println!("Box<List> size: {} bytes", std::mem::size_of::<Box<List>>());
}

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    /// Collect list values into a Vec for easy testing.
    fn collect(list: &List) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = list;
        loop {
            match current {
                List::Cons(val, next) => {
                    result.push(*val);
                    current = next;
                }
                List::Nil => break,
            }
        }
        result
    }

    #[test]
    fn test_linked_list() {
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        assert_eq!(collect(&list), vec![1, 2]);
    }

    #[test]
    fn test_empty_list() {
        let list = List::Nil;
        assert_eq!(collect(&list), Vec::<i32>::new());
    }
}
