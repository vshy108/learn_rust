# Learn Rust

> First commit: 2026-03-23

A comprehensive Rust learning project with **323 standalone examples** across **28 sections**, progressing from basics to smart pointers and web APIs, plus **72 LeetCode solutions** with benchmarks. Each topic is a self-contained file with `fn main()`, inline comments explaining Rust-specific nuances, and unit tests.

The topic list and section structure are based on the Udemy course [Learn to Code with Rust](https://www.udemy.com/course/learn-to-code-with-rust/). Thanks to the course author for the excellent curriculum.

## Repo Metadata

- Repo start date: 2026-04-24
- Related tech stack versions: Rust stable

## Why Rust?

Rust is the first language to prove that memory safety and zero-cost abstraction
are not opposites. The borrow checker rejects dangling pointers and data races
at *compile time* — not with a GC at runtime, not with sanitisers at test time.
You get C-level performance with the guarantee that your code cannot segfault or
have undefined behaviour from memory misuse. That is why AWS, Cloudflare, the
Linux kernel, and Windows are all adopting Rust for new systems code.

```rust
// The borrow checker: only one owner at a time — use-after-free is a compile error
let s1 = String::from("hello");
let s2 = s1;          // ownership moved: s1 is no longer valid
// println!("{s1}"); // ERROR: value borrowed after move

// Fearless concurrency: Arc<Mutex<T>> makes the sharing contract explicit
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || { *c.lock().unwrap() += 1; });

// Result<T, E>: errors are values — you cannot ignore them silently
fn parse(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()  // propagate with ? instead of try/catch
}
```

*vs C++*: C++ trusts you; Rust verifies you. Both produce the same machine code;
only Rust makes it impossible to ship memory bugs. *vs Go*: Go's GC trades
some latency and memory for simplicity; Rust's ownership model trades a steeper
learning curve for deterministic, GC-free resource management.

### Showcase coverage

| Must-have | Covered by |
|-----------|------------|
| Ownership / move semantics | `examples/s06_clone.rs`, `examples/s06_drop_function.rs` |
| Borrow checker (mutable borrow rules) | `examples/s20_capture_mut_refs.rs`, `examples/s19_elision_rules.rs` |
| `Result<T, E>` / `?` operator | `examples/s12_result_parse.rs`, `examples/s17_question_mark_option.rs` |
| `Arc<Mutex<T>>` (shared mutable state across threads) | `examples/s26_smart_pointers.rs` |
| Traits / generics | `examples/s18_copy_trait.rs`, `examples/s21_collect.rs` |
| Unsafe / raw pointers | `examples/s26_raw_pointers_unsafe.rs` |

## macOS Environment Setup

```sh
# Install Homebrew first if needed: https://brew.sh
brew install rustup-init
rustup-init -y
source "$HOME/.cargo/env"

# Install the toolchain pinned by rust-toolchain.toml and common checks.
rustup show
rustc --version
cargo --version
```

`cargo test --workspace` downloads crates on first run. Add `rustup component add clippy rustfmt` if you want lint/format checks locally.

## Quick Start

```bash
# Run any example
cargo run --example s01_println_macro

# Run a specific example's tests
cargo test --example s18_clone_trait

# Run all tests
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings

# Format
cargo fmt --all --check
```

## Project Structure

```
learn_rust/
├── Cargo.toml                       # Workspace root
├── examples/
│   ├── s01_*.rs  ..  s27_*.rs       # 323 standalone example files
├── leetcode/
│   ├── lc_0001_*.rs .. lc_3437_*.rs # 72 LeetCode solutions with benchmarks
├── rustfinity/
│   ├── rf_*.rs                      # 170 standalone challenge examples
│   ├── README.md                     # How to add and test challenges
│   ├── rf_procedural_macros/        # Proc-macro challenge crate
├── crates/
│   ├── s14_project_structure/       # Multi-file module demos
│   ├── s21_wordle/                  # Wordle game project
│   └── s28_axum_api/                # Full CRUD REST API with axum
├── docs/                            # Interview prep & reference docs
├── src/main.rs
├── PLAN.md                          # Full roadmap with all 340 topics + 72 LC
├── CHEATSHEET.md                    # Quick reference
└── README.md
```

**Convention**: one file per topic, named `s{NN}_{topic}.rs`. LeetCode files use
`lc_{NNNN}_{slug}.rs`. Rustfinity examples use `rf_{slug}.rs` and include the challenge
description, starter, and tests. Procedural macros use a separate crate because they cannot be
defined in a Cargo example.

## Sections

| # | Section | Examples | Key Topics |
|---|---------|----------|------------|
| 1 | Basics | 7 | `println!`, comments, interpolation, `cargo fmt/build/run` |
| 2 | Variables and Mutability | 8 | shadowing, scopes, constants, type aliases, compiler directives |
| 3 | Data Types | 19 | integers, floats, `as`, bool/char sizes, arrays, tuples, ranges, generics intro |
| 4 | Functions | 2 | parameters vs arguments, unit type `()` |
| 5 | Control Flow | 5 | `if let`, `while let`, `match`, recursion |
| 6 | Ownership | 18 | stack vs heap, `Copy`/`Clone`/`Drop`, `String`, references, borrowing |
| 7 | References and Borrowing | 5 | mutable/immutable refs, borrow rules, ownership with arrays/tuples |
| 8 | Slices | 6 | string slices, array slices, deref coercion |
| 9 | Structs | 17 | methods, `self`, associated fns, builder pattern, `Debug` |
| 10 | Enums | 10 | associated values, memory layout, `match`, `if let`, `let else` |
| 11 | Generics | 5 | turbofish, multi-generics, impl blocks, enums |
| 12 | Option and Result | 17 | `Option<T>`, `Result<T,E>`, `unwrap`, `?`, `while let`, `'static` |
| 13 | Vectors | 7 | create, add/remove, `get`, ownership, capacity |
| 14 | Project Structure | 16 + crate | modules, `pub`, `use`, `super`, `as`, doc comments, lib/bin crates |
| 15 | Strings | 5 | concatenation, `format!`, methods, user input |
| 16 | HashMaps | 8 | create, remove, `entry`, ownership, `HashSet` |
| 17 | Error Handling | 11 | `panic!`, file I/O, `?` operator, error propagation |
| 18 | Traits | 35 | defining/implementing, bounds, `dyn`, Display/Debug/Clone/Copy/Drop/PartialEq/Ord, associated types, async mock |
| 19 | Lifetimes | 13 | concrete lifetimes, NLL, elision rules, struct lifetimes, `'static` |
| 20 | Closures | 14 | `Fn`/`FnMut`/`FnOnce`, captures, `move`, `retain` |
| 21 | Iterators | 33 + crate | `map`, `filter`, `fold`, `collect`, `flat_map`, `enumerate`, `zip`, sorting, Wordle project |
| 22 | Testing | 21 | `assert_*!`, `#[should_panic]`, `Result` in tests, doc-tests, `rstest`, TDD |
| 23 | Randomness (`rand`) | 3 | `thread_rng`, `gen_range`, `shuffle` |
| 24 | Datetimes (`chrono`) | 8 | `NaiveDate`, `DateTime`, timezones, parsing, formatting |
| 25 | Regex (`regex`) | 13 | `find`, `captures`, character classes, anchors, `replace_all` |
| 26 | Smart Pointers I: Box | 16 | raw pointers, `Box`, linked list, BST, `Deref`/`Drop`, trait objects, `thiserror` |
| 27 | Smart Pointers II | 2 | `Cell`, `RefCell`, interior mutability |
| 28 | Web APIs with Axum | 1 crate | CRUD REST API, `axum`, `tower`, CORS, shared state |

## LeetCode Solutions

**72 problems** solved in Rust — each file includes both a hand-written solution and an AI-optimized solution with 100k-iteration benchmarks comparing the two.

```bash
# Run a solution
cargo run --example lc_0001_two_sum

# Run its tests
cargo test --example lc_0001_two_sum

# Run its benchmark
cargo test --example lc_0001_two_sum bench -- --nocapture
```

**Companion docs:**
- [RUBY_TO_RUST_PITFALLS.md](RUBY_TO_RUST_PITFALLS.md) — Rust idioms for LeetCode (coming from Ruby)
- [LEETCODE_PROMPTS.md](LEETCODE_PROMPTS.md) — Prompt templates used to scaffold each file

## Rustfinity Challenges

See [rustfinity/README.md](rustfinity/README.md) for the workflow to add a new challenge, register
it with Cargo, and run all or filtered tests.

Each standard challenge is independently runnable and testable as a Cargo example:

```bash
cargo test --example rf_fizz_buzz
cargo test --example rf_fizz_buzz should_return_fizz
```

The procedural-macro challenge has its own test crate:

```bash
cargo test --manifest-path rustfinity/rf_procedural_macros/Cargo.toml
```


<details>
<summary>Full problem list (72 problems: 28 Easy, 33 Medium, 11 Hard)</summary>

| # | Problem | Difficulty |
|---|---------|------------|
| 1 | Two Sum | Easy |
| 2 | Add Two Numbers | Medium |
| 3 | Longest Substring Without Repeating Characters | Medium |
| 4 | Median of Two Sorted Arrays | Hard |
| 5 | Longest Palindromic Substring | Medium |
| 6 | Zigzag Conversion | Medium |
| 7 | Reverse Integer | Medium |
| 8 | String to Integer (atoi) | Medium |
| 9 | Palindrome Number | Easy |
| 10 | Regular Expression Matching | Hard |
| 11 | Container With Most Water | Medium |
| 12 | Integer to Roman | Medium |
| 13 | Roman to Integer | Easy |
| 14 | Longest Common Prefix | Easy |
| 21 | Merge Two Sorted Lists | Easy |
| 26 | Remove Duplicates from Sorted Array | Easy |
| 27 | Remove Element | Easy |
| 28 | Find the Index of the First Occurrence in a String | Easy |
| 35 | Search Insert Position | Easy |
| 42 | Trapping Rain Water | Hard |
| 45 | Jump Game II | Medium |
| 55 | Jump Game | Medium |
| 58 | Length of Last Word | Easy |
| 76 | Minimum Window Substring | Hard |
| 79 | Word Search | Medium |
| 80 | Remove Duplicates from Sorted Array II | Medium |
| 88 | Merge Sorted Array | Easy |
| 102 | Binary Tree Level Order Traversal | Medium |
| 121 | Best Time to Buy and Sell Stock | Easy |
| 122 | Best Time to Buy and Sell Stock II | Medium |
| 134 | Gas Station | Medium |
| 135 | Candy | Hard |
| 142 | Linked List Cycle II | Medium |
| 151 | Reverse Words in a String | Medium |
| 169 | Majority Element | Easy |
| 189 | Rotate Array | Medium |
| 205 | Isomorphic Strings | Easy |
| 206 | Reverse Linked List | Easy |
| 219 | Contains Duplicate II | Easy |
| 222 | Count Complete Tree Nodes | Easy |
| 223 | Rectangle Area | Medium |
| 238 | Product of Array Except Self | Medium |
| 263 | Ugly Number | Easy |
| 274 | H-Index | Medium |
| 278 | First Bad Version | Easy |
| 279 | Perfect Squares | Medium |
| 295 | Find Median from Data Stream | Hard |
| 345 | Reverse Vowels of a String | Easy |
| 374 | Guess Number Higher or Lower | Easy |
| 380 | Insert Delete GetRandom O(1) | Medium |
| 392 | Is Subsequence | Easy |
| 409 | Longest Palindrome | Easy |
| 412 | Fizz Buzz | Easy |
| 446 | Arithmetic Slices II - Subsequence | Hard |
| 514 | Freedom Trail | Hard |
| 523 | Continuous Subarray Sum | Medium |
| 587 | Erect the Fence | Hard |
| 589 | N-ary Tree Preorder Traversal | Easy |
| 704 | Binary Search | Easy |
| 724 | Find Pivot Index | Easy |
| 835 | Image Overlap | Medium |
| 876 | Middle of the Linked List | Easy |
| 947 | Most Stones Removed with Same Row or Column | Medium |
| 1047 | Remove All Adjacent Duplicates In String | Easy |
| 1323 | Maximum 69 Number | Easy |
| 1342 | Number of Steps to Reduce a Number to Zero | Easy |
| 1480 | Running Sum of 1d Array | Easy |
| 1544 | Make The String Great | Easy |
| 1662 | Check If Two String Arrays are Equivalent | Easy |
| 1672 | Richest Customer Wealth | Easy |
| 2256 | Minimum Average Difference | Medium |
| 3437 | Permutations III | Medium |

</details>

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1 | Async runtime (full features) |
| `rand` | 0.9 | Random number generation |
| `chrono` | 0.4 | Date and time |
| `regex` | 1 | Regular expressions |
| `thiserror` | 2 | Custom error types |
| `mockall` | 0.13 | Mocking (dev) |
| `pretty_assertions` | 1 | Colored test diffs (dev) |
| `rstest` | 0.23 | Test fixtures (dev) |

**Axum API crate** (`crates/s28_axum_api/`):

| Crate | Version | Purpose |
|-------|---------|---------|
| `axum` | 0.8 | Web framework |
| `serde` / `serde_json` | 1 | JSON serialization |
| `tower-http` | 0.6 | CORS middleware |
| `tracing` / `tracing-subscriber` | 0.1 / 0.3 | Structured logging |
| `utoipa` | 5 | OpenAPI spec generation |

## Documentation

| Doc | Contents |
|-----|----------|
| [PLAN.md](PLAN.md) | Full roadmap — 340 topics + 72 LeetCode problems |
| [CHEATSHEET.md](CHEATSHEET.md) | Quick Rust reference with code examples |
| [RUBY_TO_RUST_PITFALLS.md](RUBY_TO_RUST_PITFALLS.md) | Ruby-to-Rust idiom translation for LeetCode |
| [LEETCODE_PROMPTS.md](LEETCODE_PROMPTS.md) | Prompt templates for LeetCode file scaffolding |
| [docs/RUST_API_DESIGN.md](docs/RUST_API_DESIGN.md) | REST API design patterns with axum |
| [docs/RUST_CRUD_SERVICE.md](docs/RUST_CRUD_SERVICE.md) | Full CRUD implementation with axum/sqlx |
| [docs/RUST_ERROR_HANDLING.md](docs/RUST_ERROR_HANDLING.md) | Error handling patterns (thiserror, anyhow, ?) |
| [docs/RUST_FULLSTACK_GENERAL.md](docs/RUST_FULLSTACK_GENERAL.md) | General backend Rust concepts |
| [docs/RUST_CHANGELOG.md](docs/RUST_CHANGELOG.md) | Curated Rust release notes (1.85 -- 1.94) |

## Requirements

- Rust edition 2024 (rustc 1.85+)
- `cargo`, `rustfmt`, `clippy`

## License

This is a personal learning project.
