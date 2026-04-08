# LeetCode File Generation Prompts

> **Status**: All 72 problems completed. These prompts document the workflow used to scaffold and complete each file.

## Cargo.toml Registration

Each LeetCode file must be registered as an `[[example]]` in the root `Cargo.toml`:

```toml
[[example]]
name = "lc_0001_two_sum"
path = "leetcode/lc_0001_two_sum.rs"
```

## Example File Header

Every completed file follows this header format (from `lc_0001_two_sum.rs`):

```rust
// LeetCode 1: Two Sum [Easy]
// https://leetcode.com/problems/two-sum/
//
// Given an array of integers `nums` and an integer `target`, return indices
// of the two numbers such that they add up to `target`.
//
// Run:  cargo run --example lc_0001_two_sum
// Test: cargo test --example lc_0001_two_sum
// Bench: cargo test --example lc_0001_two_sum bench -- --nocapture
//
// --- Benchmark (100k iterations, 2026-03-25) ---
//   My solution: 2.929us avg (293ms total)
//   AI solution: 1.904us avg (190ms total)
//   AI is 1.54x faster
//   Reason: AI does a single pass -- it inserts and looks up each number in one loop;
//   my solution builds the entire HashMap first, then iterates again.
```

## Prompt 1: Generate scaffold (before solving)

> Create a LeetCode problem file for **LeetCode {number}: {title} [{difficulty}]** following my existing file pattern in the `leetcode/` folder. Use the exact pattern from my other LC files (e.g. `lc_0004_median_of_two_sorted_arrays.rs`):
>
> 1. **File header**: problem number, title, difficulty, LeetCode URL, description, examples with expected output, constraints, and Run/Test/Bench commands.
> 2. **`my_solution` module**: `pub struct Solution` + `impl Solution` with the LeetCode function signature, doc comments (Arguments, Returns), and body as `todo!()`. Include `Time: O(?)` / `Space: O(?)` placeholders on the module doc.
> 3. **`ai_solution` module**: identical scaffold with `todo!()`.
> 4. **`fn main()`**: test cases printed for both solutions. **Must call both `my_solution` and `ai_solution`** (see "Avoiding dead_code warnings" below).
> 5. **`#[cfg(test)] mod tests`**: a `macro_rules!` test macro that generates tests for any Solution path, invoked for both `my_tests` and `ai_tests`. Include edge cases. For problems with multiple valid answers, use a helper that validates the result instead of exact `assert_eq!`.
> 6. **`bench_comparison` test**: 100k iterations, `black_box`, prints avg durations and ratio.
> 7. **Register the example in `Cargo.toml`** as `[[example]]`.
>
> Leave both solution bodies as `todo!()`. Do NOT fill in any solution logic.

## Avoiding dead_code Warnings

Rust's dead code analysis runs against non-test code only. Items referenced exclusively
inside `#[cfg(test)]` blocks are invisible to the compiler in normal builds, triggering
`dead_code` warnings. Follow these rules to stay warning-free:

### Rule 1: `fn main()` must exercise both solutions

`main()` must call **all public functions/methods from both `my_solution` and `ai_solution`**.
If only one solution is demoed, the other's `struct Solution` and associated functions will
warn as "never constructed" / "never used".

```rust
// GOOD — both solutions called
fn main() {
    println!("my: {}", my_solution::Solution::foo(42));
    println!("ai: {}", ai_solution::Solution::foo(42));
}

// BAD — ai_solution is dead code outside tests
fn main() {
    println!("{}", my_solution::Solution::foo(42));
}
```

For design problems (non-`Solution` structs like `HitCounter`, `PeekingIterator`), `main()`
must instantiate and call methods on **both** implementations.

### Rule 2: Test-only helpers need `#[cfg(test)]`

Helper functions/traits used **only** in tests (validation helpers, sorting helpers,
trait abstractions for polymorphic testing) must be gated with `#[cfg(test)]`:

```rust
// GOOD — gated because only tests use it
#[cfg(test)]
fn is_valid(result: &[i32]) -> bool { ... }

#[cfg(test)]
fn sorted_nested(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> { ... }

// For trait-based test helpers, gate the trait AND all impls
#[cfg(test)]
trait HitCounterApi { ... }

#[cfg(test)]
impl HitCounterApi for my_solution::HitCounter { ... }
```

### Rule 3: No unused struct fields

If a struct field is stored but never read, remove it. The compiler warns on fields
that are written during construction but never accessed afterward.

## Prompt 2: Complete the file (after solving)

> I've implemented `my_solution` in `leetcode/lc_{number}_{name}.rs`. Now:
>
> 1. **Run the benchmark** (`cargo test --example lc_{number}_{name} bench -- --nocapture`) and record the results.
> 2. **Add benchmark results** to the file header in the format: `// --- Benchmark ({iterations} iterations, {date}) ---` with my solution time, AI solution time, ratio, and a `// Reason:` line explaining *why* the faster solution wins (e.g. fewer allocations, better cache locality, fewer passes, no hashing overhead).
> 3. **Write an optimized `ai_solution`** using a different/better algorithm or approach. Include full doc comments with Arguments, Returns, Complexity. Add `Time: O(...)` / `Space: O(...)` on the module doc comment.
> 4. **Update `my_solution` module doc** with actual `Time: O(...)` / `Space: O(...)` based on my implementation.
> 5. **Add a `// --- Rust concepts used ---` section** to the file header documenting key Rust concepts demonstrated in the file (ownership patterns, trait usage, slice operations, etc.). **Only include concepts that have NOT appeared in any previous file's `Rust concepts used` section.** If a concept was already covered, omit it — do not repeat it. Add a `(Previously covered: ...)` note at the end referencing where each reused concept first appeared (e.g. `let mut x = x` -> lc_0263).
> 6. **Re-run the benchmark** after adding the AI solution, and update the header with final numbers.
> 7. Ensure all tests pass (`cargo test --example lc_{number}_{name}`).

## Premium LeetCode Problems

Premium (locked) problems are not accessible on leetcode.com without a subscription. Use the **doocs/leetcode** open-source repo on GitHub to get problem descriptions and reference solutions.

### doocs URL Patterns

- **Problem description (English)**:
  ```
  https://github.com/doocs/leetcode/blob/main/solution/{RANGE}/{NNNN}.{Title}/README_EN.md
  ```
  Example for LC 156:
  ```
  https://github.com/doocs/leetcode/blob/main/solution/0100-0199/0156.Binary%20Tree%20Upside%20Down/README_EN.md
  ```

- **Solution directory** (lists all available language files):
  ```
  https://github.com/doocs/leetcode/tree/main/solution/{RANGE}/{NNNN}.{Title}
  ```

- **Specific solution file** (e.g. Rust, Python, Java):
  ```
  https://github.com/doocs/leetcode/blob/main/solution/{RANGE}/{NNNN}.{Title}/Solution.rs
  https://github.com/doocs/leetcode/blob/main/solution/{RANGE}/{NNNN}.{Title}/Solution.py
  https://github.com/doocs/leetcode/blob/main/solution/{RANGE}/{NNNN}.{Title}/Solution.java
  ```

- **Range format**: `0000-0099`, `0100-0199`, `0200-0299`, etc.
- **Title format**: spaces encoded as `%20` in URLs, e.g. `Two%20Sum`

### Prompt 3: Premium problem — Rust-applicable (coding problem)

> Create a LeetCode problem file for **LeetCode {number}: {title} [{difficulty}] (Premium)**.
>
> This is a premium problem. Fetch the problem description from:
> `https://github.com/doocs/leetcode/blob/main/solution/{RANGE}/{NNNN}.{Title}/README_EN.md`
>
> Check the solution directory for available languages:
> `https://github.com/doocs/leetcode/tree/main/solution/{RANGE}/{NNNN}.{Title}`
>
> **Workflow**:
> 1. Fetch the `README_EN.md` for the problem description, examples, and constraints.
> 2. If `Solution.rs` exists in the doocs repo, use it as `my_solution` (converted/adapted to match our file pattern with `pub struct Solution` + `impl Solution`).
> 3. If no `Solution.rs` exists, translate the best available solution (prefer Python > Java > C++ > Go) into idiomatic Rust as `my_solution`. Add a comment: `// Translated from {language} solution in doocs/leetcode`.
> 4. Generate a **different** `ai_solution` using an alternative algorithm or approach. Do NOT duplicate the same logic.
> 5. Follow the same file structure as Prompt 1 (header, modules, main, tests, bench).
> 6. Register in `Cargo.toml`, run tests, run benchmark, update header with results.
>
> The doocs-sourced solution counts as "my solution" for benchmarking purposes.

### Prompt 4: Premium problem — NOT applicable for Rust (SQL/Shell)

> Create a placeholder file for **LeetCode {number}: {title} [{difficulty}] (Premium)**.
>
> This problem is **not applicable for Rust** because it is a {SQL|Shell} problem.
>
> **File format**:
> ```rust
> // LeetCode {number}: {title} [{difficulty}] (Premium)
> // https://leetcode.com/problems/{slug}/
> //
> // NOT APPLICABLE FOR RUST — this is a {SQL|Shell} problem.
> //
> // Problem description (from doocs/leetcode):
> // {brief 1-2 line description}
> //
> // Reference: https://github.com/doocs/leetcode/blob/main/solution/{RANGE}/{NNNN}.{Title}/README_EN.md
>
> fn main() {
>     println!("LeetCode {number}: {title}");
>     println!("This is a {SQL|Shell} problem — not applicable for Rust.");
>     println!("See README_EN.md link in file header for the problem and solution.");
> }
> ```
>
> Register in `Cargo.toml` as usual. No tests or benchmarks needed.

### Missing Premium Problems (1-242 range)

**Coding problems (8)** — create with Prompt 3:

| # | Title | Difficulty | doocs Rust? |
|---|-------|-----------|:-----------:|
| 156 | Binary Tree Upside Down | Medium | No |
| 157 | Read N Characters Given Read4 | Easy | No |
| 158 | Read N Characters Given read4 II - Call Multiple Times | Hard | No |
| 159 | Longest Substring with At Most Two Distinct Characters | Medium | No |
| 161 | One Edit Distance | Medium | No |
| 163 | Missing Ranges | Easy | No |
| 170 | Two Sum III - Data structure design | Easy | No |
| 186 | Reverse Words in a String II | Medium | No |

All have descriptions on doocs. None have Rust solutions — translate from Python/Java/C++/Go.

**SQL problems (12)** — create with Prompt 4:

| # | Title | Type |
|---|-------|------|
| 175 | Combine Two Tables | SQL |
| 176 | Second Highest Salary | SQL |
| 177 | Nth Highest Salary | SQL |
| 178 | Rank Scores | SQL |
| 180 | Consecutive Numbers | SQL |
| 181 | Employees Earning More Than Their Managers | SQL |
| 182 | Duplicate Emails | SQL |
| 183 | Customers Who Never Order | SQL |
| 184 | Department Highest Salary | SQL |
| 185 | Department Top Three Salaries | SQL |
| 196 | Delete Duplicate Emails | SQL |
| 197 | Rising Temperature | SQL |

**Shell problems (4)** — create with Prompt 4:

| # | Title | Type |
|---|-------|------|
| 192 | Word Frequency | Shell |
| 193 | Valid Phone Numbers | Shell |
| 194 | Transpose File | Shell |
| 195 | Tenth Line | Shell |
