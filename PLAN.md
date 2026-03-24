# Rust Learning Roadmap

> Track progress by checking off topics as they are implemented.
> Run any example: `cargo run --example s01_println_macro`

## Workspace Layout

```
learn_rust/
├── Cargo.toml              # [workspace] members = [".", "crates/*"]
├── src/main.rs
├── examples/
│   ├── s01_*.rs             # Section 1 topics
│   ├── s02_*.rs             # Section 2 topics
│   ├── ...                  # Sections 3-13, 15-20, 22-27
│   └── s27_*.rs
├── crates/
│   ├── s14_project_structure/   # Section 14 multi-file topics
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── ... (modules, submodules)
│   │   └── examples/
│   └── s21_wordle/              # Section 21 Wordle project
│       ├── Cargo.toml
│       └── src/main.rs
```

## Conventions

- **File naming**: `examples/s{NN}_{topic}.rs` — Cargo auto-discovers these as runnable examples
- **No consolidation**: every topic = one file, independently runnable
- **Section 14** multi-file module topics → `crates/s14_project_structure/`
- **Section 21** Wordle project → `crates/s21_wordle/`

## Verification (per topic)

```sh
cargo build --example <name>           # compiles without warnings
cargo run --example <name>             # runs and produces expected output
cargo test --example <name>            # tests pass (if topic has tests)
cargo clippy --workspace -- -D warnings
cargo fmt --all --check
```

---

## Section 1: Basics

- [ ] 1. `s01_why_not_println_string_from` — Why not `println!(String::from(...))`
- [ ] 2. `s01_params_in_main` — Parameters in main function
- [ ] 3. `s01_multiple_main_functions` — Multiple main functions
- [ ] 4. `s01_println_macro` — println! macro deep dive
- [ ] 5. `s01_fmt_build_run` — fmt, build and run
- [ ] 6. `s01_comments` — Comment styles
- [ ] 7. `s01_interpolation_curly_braces` — Interpolation with curly braces

## Section 2: Variables and Mutability

- [ ] 8. `s02_variable` — Variable basics
- [ ] 9. `s02_rustc_explain` — rustc --explain
- [ ] 10. `s02_error_codes_index` — Rust error codes index
- [ ] 11. `s02_variable_shadowing` — Variable shadowing
- [ ] 12. `s02_scope_vs_block` — Scope vs block
- [ ] 13. `s02_constants` — Constants
- [ ] 14. `s02_type_aliases` — Type aliases
- [ ] 15. `s02_compiler_directives` — Compiler directives

## Section 3: Data Types

- [ ] 16. `s03_data_types` — Rust data types overview
- [ ] 17. `s03_usize_isize` — usize vs isize
- [ ] 18. `s03_visual_separator` — _ as visual separator
- [ ] 19. `s03_format_specifiers` — Format specifiers
- [ ] 20. `s03_as_keyword` — The `as` keyword
- [ ] 21. `s03_division` — Division operators and edge cases
- [ ] 22. `s03_float_division` — 2.0/3.0 float behavior
- [ ] 23. `s03_augmented_assignment` — Augmented assignment operators
- [ ] 24. `s03_is_positive_negative` — is_positive(), is_negative()
- [ ] 25. `s03_bool_size` — bool bytes and size
- [ ] 26. `s03_char_size` — char size
- [ ] 27. `s03_scalar_vs_compound` — Scalar vs compound types
- [ ] 28. `s03_array_declaration` — Array declaration
- [ ] 29. `s03_dbg_macro` — dbg! macro
- [ ] 30. `s03_tuple` — Tuple
- [ ] 31. `s03_range_for_in` — Range and for-in
- [ ] 32. `s03_generics_intro` — Generics intro
- [ ] 33. `s03_str` — str type
- [ ] 34. `s03_overflowing_ops` — Overflow-aware integer arithmetic

## Section 4: Functions

- [ ] 35. `s04_parameters_vs_arguments` — Parameters vs arguments
- [ ] 36. `s04_unit_empty_tuple` — Unit () empty tuple

## Section 5: Control Flow

- [ ] 36. `s05_control_flow` — Control flow overview
- [ ] 37. `s05_if_let` — if let
- [ ] 38. `s05_while_let` — while let
- [ ] 39. `s05_unreachable_match` — unreachable!() with _ in match
- [ ] 40. `s05_recursion` — Recursion

## Section 6: Ownership

- [ ] 41. `s06_i32_f64` — i32, f64 on stack
- [ ] 42. `s06_stack_vs_heap` — Stack vs heap
- [ ] 43. `s06_box_vec_ref` — Box, Vec, &
- [ ] 44. `s06_scope_and_ownership` — Scope and ownership
- [ ] 45. `s06_heap_or_stack` — Determine heap vs stack
- [ ] 46. `s06_copy_trait` — Copy trait
- [ ] 47. `s06_copy_drop_conflict` — Copy + Drop cannot coexist
- [ ] 48. `s06_string_new_vs_from` — String::new vs String::from
- [ ] 49. `s06_push_str` — push_str
- [ ] 50. `s06_drop_function` — drop function
- [ ] 51. `s06_clone` — clone()
- [ ] 52. `s06_references_borrowing` — References and borrowing
- [ ] 53. `s06_dereference_deref_trait` — Dereference and Deref trait
- [ ] 54. `s06_string_types` — String, &String, str, &str
- [ ] 55. `s06_copy_trait_refs` — Copy trait with references
- [ ] 56. `s06_ownership_fn_params` — Ownership and function parameters
- [ ] 57. `s06_mutable_params` — Mutable parameters
- [ ] 58. `s06_return_values` — Return values and ownership

## Section 7: References and Borrowing

- [ ] 59. `s07_immut_mut_ref_params` — Immutable and mutable reference parameters
- [ ] 60. `s07_multiple_immut_refs` — Multiple immutable references
- [ ] 61. `s07_mut_ref_restrictions` — Mutable reference restrictions
- [ ] 62. `s07_ownership_immut_mut_refs` — Ownership with immutable and mutable references
- [ ] 63. `s07_ownership_arrays_tuples` — Ownership with arrays and tuples

## Section 8: Slices

- [ ] 64. `s08_string_slice` — String slice from String
- [ ] 65. `s08_syntactic_shortcuts` — Syntactic shortcuts
- [ ] 66. `s08_string_slices_fn_params` — String slices as function parameters
- [ ] 67. `s08_array_slices` — Array slices
- [ ] 68. `s08_deref_coercion_array` — Deref coercion with array slices
- [ ] 69. `s08_mutable_array_slices` — Mutable array slices

## Section 9: Structs

- [ ] 70. `s09_struct` — Struct basics
- [ ] 71. `s09_create_instance` — Create struct instance
- [ ] 72. `s09_access_fields` — Access struct fields
- [ ] 73. `s09_overwrite_fields` — Overwrite struct fields
- [ ] 74. `s09_create_in_function` — Create structs in a function
- [ ] 75. `s09_update_syntax` — Struct update syntax
- [ ] 76. `s09_passing_structs` — Passing structs into a function
- [ ] 77. `s09_debug_trait` — Deriving Debug trait
- [ ] 78. `s09_methods` — Defining struct methods
- [ ] 79. `s09_self_mut` — self parameter as mutable struct instance
- [ ] 80. `s09_self_ref` — self as immutable/mutable references
- [ ] 81. `s09_methods_multi_params` — Methods with multiple parameters
- [ ] 82. `s09_calling_methods` — Calling methods from other methods
- [ ] 83. `s09_associated_functions` — Associated functions
- [ ] 84. `s09_multiple_impl_blocks` — Multiple impl blocks
- [ ] 85. `s09_builder_pattern` — Builder pattern
- [ ] 86. `s09_unit_like_structs` — Unit-like structs

## Section 10: Enums

- [ ] 87. `s10_enum_associated_values` — Enum with associated values
- [ ] 88. `s10_enum_memory` — Enum memory layout
- [ ] 89. `s10_struct_variants` — Struct variants
- [ ] 90. `s10_nesting_enums` — Nesting enums in enums
- [ ] 91. `s10_match_keyword` — The match keyword with enum
- [ ] 92. `s10_enum_methods` — Defining methods on enums
- [ ] 93. `s10_match_multiple` — match catching multiple values
- [ ] 94. `s10_match_exact` — match with exact value
- [ ] 95. `s10_if_let_construct` — The if let construct
- [ ] 96. `s10_let_else` — The let else construct

## Section 11: Generics

- [ ] 97. `s11_turbofish` — The turbofish operator
- [ ] 98. `s11_multiple_generics` — Multiple generics
- [ ] 99. `s11_generics_structs` — Generics in structs
- [ ] 100. `s11_generics_impl` — Generics and impl blocks
- [ ] 101. `s11_generics_enums` — Generics in enums

## Section 12: Option and Result Enums

- [ ] 102. `s12_option_enum` — The Option enum
- [ ] 103. `s12_option_get_array` — Option with array get method
- [ ] 104. `s12_unwrap_expect` — unwrap and expect methods
- [ ] 105. `s12_match_option` — match with Option
- [ ] 106. `s12_return_option` — Returning Option\<T\> from a function
- [ ] 107. `s12_top_level_option` — Top-level Option variants
- [ ] 108. `s12_prelude` — Rust's prelude
- [ ] 109. `s12_unwrap_or` — The unwrap_or method
- [ ] 110. `s12_build_option` — Building Option from scratch
- [ ] 111. `s12_result_enum` — The Result enum
- [ ] 112. `s12_result_parse` — Result with parse method
- [ ] 113. `s12_return_result` — Returning Result from a function
- [ ] 114. `s12_static_str` — &'static str vs &str
- [ ] 115. `s12_t_static` — T: 'static vs &'static T
- [ ] 116. `s12_result_methods` — Result methods
- [ ] 117. `s12_unwrap_result_nuances` — Nuances of unwrap on Result
- [ ] 118. `s12_while_let` — The while let construct

## Section 13: Vectors

- [ ] 119. `s13_create_vector` — Create a vector
- [ ] 120. `s13_add_remove` — Adding and removing elements
- [ ] 121. `s13_read_elements` — Reading vector elements
- [ ] 122. `s13_get_method` — The get method
- [ ] 123. `s13_ownership_vectors` — Ownership with vectors
- [ ] 124. `s13_write_elements` — Writing vector elements
- [ ] 125. `s13_capacity` — Vector capacity behind the scenes

## Section 14: Project Structure

> Single-file concept demos go in `examples/s14_*.rs`.
> Multi-file topics live in `crates/s14_project_structure/`.

- [ ] 126. `s14_packages_crates` — Packages and crates
- [ ] 127. `s14_modules` — Modules
- [ ] 128. `s14_pub_keyword` — The pub keyword
- [ ] 129. `s14_namespaces` — The benefit of namespaces
- [ ] 130. `crates/s14_project_structure/` — Module as file **(multi-file)**
- [ ] 131. `crates/s14_project_structure/` — Module as folder **(multi-file)**
- [ ] 132. `s14_module_ambiguity` — Module ambiguity
- [ ] 133. `s14_pub_enums_structs` — Public enums, structs, and fields
- [ ] 134. `crates/s14_project_structure/` — Submodules **(multi-file)**
- [ ] 135. `s14_crate_prefix` — The crate prefix
- [ ] 136. `s14_use_keyword` — The use keyword
- [ ] 137. `s14_self_keyword` — The self keyword
- [ ] 138. `s14_super_keyword` — The super keyword
- [ ] 139. `s14_as_alias` — Create aliases with as
- [ ] 140. `s14_pub_use` — Using pub use to export names
- [ ] 141. `s14_external_crates` — External crates
- [ ] 142. `s14_standard_library` — The standard library
- [ ] 143. `s14_glob_operator` — The glob operator
- [ ] 144. `crates/s14_project_structure/` — Create library crate **(multi-file)**
- [ ] 145. `crates/s14_project_structure/` — Multiple binary crates **(multi-file)**
- [ ] 146. `s14_doc_comments` — Documentation comments
- [ ] 147. `crates/s14_project_structure/` — Walking through sample module structure **(multi-file)**

## Section 15: Strings

- [ ] 148. `s15_strings` — Strings overview
- [ ] 149. `s15_concatenation` — Concatenation
- [ ] 150. `s15_format_macro` — The format! macro
- [ ] 151. `s15_string_methods` — Common string methods
- [ ] 152. `s15_user_input` — Collecting user input with read_line

## Section 16: HashMaps

- [ ] 153. `s16_create_hashmap` — Create a HashMap
- [ ] 154. `s16_remove` — The remove method
- [ ] 155. `s16_ownership` — HashMaps and ownership
- [ ] 156. `s16_access_value` — Access a value by key
- [ ] 157. `s16_overwrite` — Overwriting a value
- [ ] 158. `s16_entry` — The entry method
- [ ] 159. `s16_hashset` — The HashSet
- [ ] 160. `s16_hashset_operations` — HashSet operations

## Section 17: Error Handling

- [ ] 161. `s17_panic` — The panic! macro
- [ ] 162. `s17_process_exit` — The process module and exit function
- [ ] 163. `s17_eprintln` — Standard error (eprintln!)
- [ ] 164. `s17_open_file` — Opening a file
- [ ] 165. `s17_ask_user_input` — Asking the user for input
- [ ] 166. `s17_read_file` — Reading file contents
- [ ] 167. `s17_propagating_errors` — Propagating errors
- [ ] 168. `s17_error_type_redecl` — Error type redeclaration
- [ ] 169. `s17_try_operator` — The ? operator
- [ ] 170. `s17_read_to_string` — The read_to_string associated function
- [ ] 171. `s17_question_mark_option` — Using ? with Option

## Section 18: Traits

- [ ] 172. `s18_intro_traits` — Intro to traits
- [ ] 173. `s18_defining_trait` — Defining a trait
- [ ] 174. `s18_impl_trait_struct` — Implementing trait for struct
- [ ] 175. `s18_default_impl` — Default implementations
- [ ] 176. `s18_calling_from_another` — Calling trait method from another method
- [ ] 177. `s18_trait_fn_constraints` — Traits for function parameter constraints
- [ ] 178. `s18_trait_bound_syntax` — Trait bound syntax
- [ ] 179. `s18_multiple_trait_bounds` — Multiple trait bounds
- [ ] 180. `s18_where_clauses` — where clauses
- [ ] 181. `s18_trait_return_values` — Traits as function return values
- [ ] 182. `s18_conditional_methods` — Trait bounds to conditionally implement methods
- [ ] 183. `s18_trait_objects_preview` — A preview of trait objects
- [ ] 184. `s18_dyn_trait` — &dyn Trait
- [ ] 185. `s18_trait_in_scope` — Trait must be in scope
- [ ] 186. `s18_project_structure` — Moving to project structure
- [ ] 187. `s18_project_multi_modules` — Project structure with multiple modules
- [ ] 188. `s18_associated_constants` — Associated constants in a trait
- [ ] 189. `s18_getters` — Getters in traits
- [ ] 190. `s18_setters` — Setters in traits
- [ ] 191. `s18_supertraits` — Supertraits (trait inheritance)
- [ ] 192. `s18_traits_generics` — Traits with generics
- [ ] 193. `s18_display_struct` — Implementing Display on a struct
- [ ] 194. `s18_display_enum` — Implementing Display on an enum
- [ ] 195. `s18_debug_trait` — Implementing the Debug trait
- [ ] 196. `s18_formatter_methods` — Formatter methods
- [ ] 197. `s18_drop_trait` — Implementing the Drop trait
- [ ] 198. `s18_clone_trait` — Implementing the Clone trait
- [ ] 199. `s18_copy_trait` — Implementing the Copy trait
- [ ] 200. `s18_partial_eq_struct` — PartialEq for structs
- [ ] 201. `s18_equality_diff_types` — Equality for different types
- [ ] 202. `s18_partial_eq_enum` — PartialEq for enums
- [ ] 203. `s18_eq_trait` — Implementing the Eq trait
- [ ] 204. `s18_partial_ord` — Implementing the PartialOrd trait
- [ ] 205. `s18_associated_types` — Associated types
- [ ] 206. `s18_async_trait_mock` — Async traits and mocking with mockall

## Section 19: Lifetimes

- [ ] 206. `s19_concrete_lifetimes_values` — Concrete lifetimes for values
- [ ] 208. `s19_concrete_lifetimes_refs` — Concrete lifetimes for references
- [ ] 209. `s19_non_lexical_lifetimes` — Non-lexical lifetimes
- [ ] 209. `s19_invalid_lifetimes` — Invalid lifetimes
- [ ] 212. `s19_no_return_ref_owned` — Functions cannot return references to owned values
- [ ] 213. `s19_refs_fn_params` — References as function parameters
- [ ] 214. `s19_intro_generic_lifetimes` — Intro to generic lifetimes
- [ ] 215. `s19_lifetimes_referents` — Lifetimes and referents
- [ ] 214. `s19_elision_rules` — Lifetime elision rules
- [ ] 215. `s19_multiple_params` — Multiple parameters
- [ ] 220. `s19_lifetimes_structs` — Lifetimes in structs
- [ ] 221. `s19_multiple_lifetimes` — Multiple lifetimes
- [ ] 222. `s19_static_lifetime` — The static lifetime

## Section 20: Closures

- [ ] 223. `s20_nested_functions` — Nested functions
- [ ] 224. `s20_intro_closures` — Intro to closures
- [ ] 225. `s20_closure_shortcuts` — Closure shortcuts
- [ ] 226. `s20_fn_trait_hierarchy` — The Fn trait hierarchy
- [ ] 227. `s20_capture_immut_refs` — Closures capturing immutable references
- [ ] 228. `s20_capture_mut_refs` — Closures capturing mutable references
- [ ] 229. `s20_closures_ownership` — Closures with ownership
- [ ] 230. `s20_move_keyword` — The move keyword
- [ ] 231. `s20_unwrap_or_else` — The unwrap_or_else method
- [ ] 232. `s20_fn_once_trait` — Defining method accepting FnOnce closure
- [ ] 233. `s20_string_retain` — The String.retain method
- [ ] 234. `s20_fn_mut_trait` — Defining method accepting FnMut closure
- [ ] 235. `s20_fn_trait` — The Fn trait
- [ ] 236. `s20_fn_to_fn_trait` — Passing function to Fn trait parameter

## Section 21: Iterators

- [ ] 237. `s21_manual_iteration` — Manual iteration
- [ ] 238. `s21_iterator_into_iterator` — Iterator and IntoIterator traits
- [ ] 239. `s21_into_iterator_action` — IntoIterator in action
- [ ] 240. `s21_exhausting_iterator` — Exhausting the iterator
- [ ] 241. `s21_for_loop_iterator` — for loop with iterator
- [ ] 242. `s21_why_immutable` — Why iterator can be immutable
- [ ] 243. `s21_iter` — The iter method
- [ ] 244. `s21_iter_mut` — The iter_mut method
- [ ] 245. `s21_hashmap_iteration` — HashMap iteration
- [ ] 246. `s21_string_iteration` — String iteration
- [ ] 247. `s21_solving_problem` — Solving a problem with iteration
- [ ] 248. `s21_for_each` — The for_each method
- [ ] 247. `s21_map` — The map method
- [ ] 248. `s21_collect` — The collect method
- [ ] 252. `s21_filter_find` — The filter and find methods
- [ ] 253. `s21_any_all` — The any and all methods
- [ ] 254. `s21_cloned` — The cloned method
- [ ] 255. `s21_filter_map` — The filter_map method
- [ ] 256. `s21_flatten` — The flatten method
- [ ] 257. `s21_flat_map` — The flat_map method
- [ ] 258. `s21_enumerate` — The enumerate method
- [ ] 259. `s21_partition` — The partition method
- [ ] 260. `s21_zip` — The zip method
- [ ] 261. `s21_fold` — The fold method
- [ ] 262. `s21_reduce` — The reduce method
- [ ] 263. `s21_sum_product_max_min_count` — sum, product, max, min, count
- [ ] 264. `s21_last_nth_position` — last, nth, nth_back, position
- [ ] 265. `s21_take_rev_skip_step_by` — take, rev, skip, step_by
- [ ] 266. `s21_sort` — sort and sort_by_key
- [ ] 267. `s21_lines` — The lines method
- [ ] 268. `s21_cli_arguments` — Collecting command line arguments
- [ ] 269. `s21_read_directory` — Reading directory
- [ ] 270. `s21_from_iterator` — The FromIterator trait
- [ ] 271. `crates/s21_wordle/` — Project: Wordle **(dedicated workspace crate)**

## Section 22: Testing

- [ ] 272. `s22_intro_testing` — Intro to testing
- [ ] 273. `s22_assert_eq` — Writing tests and assert_eq!
- [ ] 274. `s22_tests_module_cfg` — tests module and cfg attribute
- [ ] 275. `s22_test_failures` — Test failures
- [ ] 276. `s22_assert_macro` — The assert! macro
- [ ] 277. `s22_assert_ne` — assert_ne!
- [ ] 278. `s22_pretty_assertions` — pretty_assertions crate
- [ ] 279. `s22_trait_requirements` — Trait requirements for testing types
- [ ] 280. `s22_custom_failure_messages` — Custom failure messages
- [ ] 281. `s22_should_panic` — The should_panic attribute
- [ ] 282. `s22_result_in_tests` — Using Result in tests
- [ ] 283. `s22_subset_tests` — Running a subset of tests
- [ ] 284. `s22_ignoring_tests` — Ignoring tests
- [ ] 285. `s22_show_output` — Showing printed output from passing tests
- [ ] 286. `s22_movie_theater` — Adding MovieTheater struct
- [ ] 285. `s22_dependency_injection` — Dependency injection
- [ ] 289. `s22_integration_tests` — Integration tests
- [ ] 290. `s22_doc_tests` — Documentation tests
- [ ] 291. `s22_test_options` — Test options
- [ ] 292. `s22_rstest_fixtures` — Fixtures with rstest crate
- [ ] 293. `s22_tdd` — Test-driven development

## Section 23: Randomness with rand Crate

> Adds `rand` dependency to `Cargo.toml`

- [ ] 294. `s23_intro_rand` — Intro to rand crate
- [ ] 295. `s23_thread_rng` — The ThreadRng struct
- [ ] 296. `s23_shuffle` — Randomizing vector elements with shuffle

## Section 24: Datetimes with chrono Crate

> Adds `chrono` dependency to `Cargo.toml`

- [ ] 297. `s24_naive_date` — The NaiveDate struct
- [ ] 298. `s24_time_delta` — The TimeDelta struct
- [ ] 299. `s24_add_sub_time` — Adding and subtracting time
- [ ] 300. `s24_naive_time_datetime` — NaiveTime and NaiveDateTime
- [ ] 301. `s24_datetime` — The DateTime struct
- [ ] 302. `s24_converting_timezones` — Converting timezones
- [ ] 303. `s24_parse_from_str` — The parse_from_str function
- [ ] 304. `s24_format` — The format method

## Section 25: Regular Expressions with regex Crate

> Adds `regex` dependency to `Cargo.toml`

- [ ] 305. `s25_intro_regex` — Intro to regex crate
- [ ] 306. `s25_find_find_iter` — find and find_iter methods
- [ ] 307. `s25_searching_digits` — Searching for digits
- [ ] 308. `s25_alphanumeric` — Alphanumeric characters
- [ ] 309. `s25_whitespace` — Whitespace
- [ ] 310. `s25_word_boundaries` — Word boundaries
- [ ] 311. `s25_dot_metacharacter` — The dot metacharacter
- [ ] 312. `s25_square_brackets` — Square brackets with characters
- [ ] 311. `s25_num_matches` — Number of matches
- [ ] 315. `s25_or_logic` — Or logic
- [ ] 316. `s25_anchors` — Anchors
- [ ] 317. `s25_capture_groups` — Capture groups
- [ ] 318. `s25_replace_all` — The replace_all method

## Section 26: Smart Pointers I: Box

> Adds `thiserror` dependency to `Cargo.toml`

- [ ] 319. `s26_raw_pointers` — Raw pointers
- [ ] 320. `s26_raw_pointers_unsafe` — Raw pointers and unsafe code
- [ ] 321. `s26_smart_pointers` — Smart pointers overview
- [ ] 322. `s26_box` — The Box smart pointer
- [ ] 323. `s26_linked_list_intro` — Intro to linked lists
- [ ] 324. `s26_linked_list_define` — Defining the LinkedList
- [ ] 325. `s26_linked_list_create` — Creating the LinkedList
- [ ] 322. `s26_box_vs_refs` — Box vs regular references
- [ ] 328. `s26_vectors_smart_pointers` — Vectors are smart pointers
- [ ] 329. `s26_bst_intro` — Intro to binary search trees
- [ ] 330. `s26_bst_create` — Creating a binary search tree
- [ ] 326. `s26_deref_derefmut` — Deref and DerefMut traits
- [ ] 333. `s26_drop_trait` — The Drop trait
- [ ] 334. `s26_deref_coercions` — Deref coercions
- [ ] 329. `s26_trait_objects` — Trait objects and multiple error handling
- [ ] 337. `s26_custom_error_types` — Custom error types

## Section 27: Smart Pointers II (IN DEVELOPMENT)

- [ ] 338. `s27_cell` — The Cell type and interior mutability
- [ ] 339. `s27_refcell` — The RefCell type
