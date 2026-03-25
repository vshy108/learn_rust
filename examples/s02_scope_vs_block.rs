// Demonstrates scope and block expressions in Rust.
//
// A *block* is any code enclosed in `{}`. Blocks create a new *scope*:
// variables declared inside a block are not accessible outside it.
//
// Rust creates a new scope for EVERY `{}` block — including `if`, `for`,
// `match`, and even bare `{}` blocks.
//
// A key Rust feature: blocks are *expressions* — they return a value.
//
// Run: cargo run --example s02_scope_vs_block

fn main() {
    // --- 1. Bare blocks create scope ---
    let outer = 10;
    {
        let inner = 20;
        println!("inside block: outer={outer}, inner={inner}");
    }
    // `inner` is not accessible here:
    // println!("{inner}");  // error[E0425]: cannot find value `inner`
    println!("outside block: outer={outer}");

    // --- 2. Blocks as expressions ---
    // The last expression in a block (WITHOUT a semicolon) becomes the
    // block's value.
    let result = {
        let a = 3;
        let b = 7;
        a + b // no semicolon = this is the block's return value
    };
    println!("block expression result = {result}"); // 10

    // Gotcha: adding a semicolon makes the block return `()` (unit type).
    #[allow(clippy::let_unit_value)]
    let unit_result = {
        let _x = 5;
        // x + 1;  // semicolon makes this a statement, block returns ()
    };
    println!("unit result = {unit_result:?}"); // ()

    // --- 3. Variables are dropped at end of scope ---
    // When a variable goes out of scope, Rust automatically calls `drop()`
    // on it (if it implements Drop). The drop is deterministic — it happens
    // immediately when scope ends.
    {
        let s = String::from("scoped string");
        println!("s = {s}");
    } // `s` is dropped here — memory is freed immediately

    // --- 4. Nested scopes ---
    let x = 1;
    {
        let x = 2; // shadows outer x
        {
            let x = 3; // shadows the shadow
            println!("innermost x = {x}"); // 3
        }
        println!("middle x = {x}"); // 2
    }
    println!("outermost x = {x}"); // 1

    // --- 5. `if` and `match` blocks are expressions too ---
    let condition = true;
    let val = if condition { "yes" } else { "no" };
    println!("if expression = {val}");

    let number = 3;
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("match expression = {description}");

    // --- 6. Loop labels and scope ---
    // `loop`, `while`, and `for` also create scopes.
    // `loop` can return a value via `break <value>`.
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10; // returns 50
        }
    };
    println!("loop result = {loop_result}"); // 50

    // --- Gotcha: temporaries live until end of statement ---
    // Temporary values created in an expression live until the end
    // of the enclosing statement, not just the expression.
    let r = &String::from("temp"); // temp String lives for this statement
    println!("temp ref = {r}");
    // After this statement, the temporary is dropped.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_block_expression() {
        let val = {
            let a = 2;
            let b = 3;
            a * b
        };
        assert_eq!(val, 6);
    }

    #[test]
    fn test_block_with_semicolon_returns_unit() {
        let val = {
            42; // semicolon makes this a statement
        };
        assert_eq!(val, ());
    }

    #[test]
    fn test_if_expression() {
        let x = if true { 1 } else { 2 };
        assert_eq!(x, 1);
    }

    #[test]
    fn test_nested_scope_shadowing() {
        let x = 1;
        let y = {
            let x = x + 10; // shadows with 11
            x * 2 // returns 22
        };
        assert_eq!(x, 1); // outer x unchanged
        assert_eq!(y, 22);
    }

    #[test]
    fn test_loop_expression() {
        let mut i = 0;
        let result = loop {
            i += 1;
            if i == 3 {
                break i;
            }
        };
        assert_eq!(result, 3);
    }
}
