# LeetCode File Generation Prompts

## Prompt 1: Generate scaffold (before solving)

> Create a LeetCode problem file for **LeetCode {number}: {title} [{difficulty}]** following my existing file pattern in the `leetcode/` folder. Use the exact pattern from my other LC files (e.g. `lc_0004_median_of_two_sorted_arrays.rs`):
>
> 1. **File header**: problem number, title, difficulty, LeetCode URL, description, examples with expected output, constraints, and Run/Test/Bench commands.
> 2. **`my_solution` module**: `pub struct Solution` + `impl Solution` with the LeetCode function signature, doc comments (Arguments, Returns), and body as `todo!()`. Include `Time: O(?)` / `Space: O(?)` placeholders on the module doc.
> 3. **`ai_solution` module**: identical scaffold with `todo!()`.
> 4. **`fn main()`**: test cases printed for both solutions.
> 5. **`#[cfg(test)] mod tests`**: a `macro_rules!` test macro that generates tests for any Solution path, invoked for both `my_tests` and `ai_tests`. Include edge cases. For problems with multiple valid answers, use a helper that validates the result instead of exact `assert_eq!`.
> 6. **`bench_comparison` test**: 100k iterations, `black_box`, prints avg durations and ratio.
> 7. **Register the example in `Cargo.toml`** as `[[example]]`.
>
> Leave both solution bodies as `todo!()`. Do NOT fill in any solution logic.

## Prompt 2: Complete the file (after solving)

> I've implemented `my_solution` in `leetcode/lc_{number}_{name}.rs`. Now:
>
> 1. **Run the benchmark** (`cargo test --example lc_{number}_{name} bench -- --nocapture`) and record the results.
> 2. **Add benchmark results** to the file header in the format: `// --- Benchmark ({iterations} iterations, {date}) ---` with my solution time, AI solution time, ratio, and a `// Reason:` line explaining *why* the faster solution wins (e.g. fewer allocations, better cache locality, fewer passes, no hashing overhead).
> 3. **Write an optimized `ai_solution`** using a different/better algorithm or approach. Include full doc comments with Arguments, Returns, Complexity. Add `Time: O(...)` / `Space: O(...)` on the module doc comment.
> 4. **Update `my_solution` module doc** with actual `Time: O(...)` / `Space: O(...)` based on my implementation.
> 5. **Add a `// --- Rust concepts used ---` section** to the file header documenting key Rust concepts demonstrated in the file (ownership patterns, trait usage, slice operations, etc.).
> 6. **Re-run the benchmark** after adding the AI solution, and update the header with final numbers.
> 7. Ensure all tests pass (`cargo test --example lc_{number}_{name}`).
