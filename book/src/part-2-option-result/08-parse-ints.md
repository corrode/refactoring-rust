# 08 · `parse_ints`

## The problem

These loops need different error policies. Parsing stops at the first failure;
validation must run every check and collect all errors.

## Code to refactor

> **EXERCISE**
>
> Refactor both loops. Keep validator errors in input order, and check whether either
> function needs to own its input.

```rust,editable
{{#include ../../../examples/08_parse_ints.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/08_parse_ints.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/08_parse_ints.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/08_parse_ints.rs`:
>
> ```sh
> cargo test --example 08_parse_ints
> ```
>
> Let `parse_values` borrow its input. Before you change `collect_errors`, explain why
> another fail-fast pipeline would break its test.

## Discussion

> **DISCUSSION**
>
> - Does parsing require `parse_values` to own each `String`?
> - After mapping `str::parse`, what is the iterator's item type?
> - What do you get when you collect `Result<i32, ParseIntError>` items?
> - If the second string is invalid, can the caller observe the integers parsed before it?
> - Do validators after the first rejection still have to run?
> - Must `collect_errors` retain validator order?
> - What work is repeated by `is_err()` followed by `err().expect(...)`?

`parse_values` only needs to read each string. The `parse` method works on the string's
`str` contents, so the function can take `&[String]` instead of owning the vector. This
lets the caller use the original strings afterward.

Mapping `parse` over
`values.iter()` produces `Result<i32, ParseIntError>` items. The standard library has a
specialised `FromIterator` implementation for this case. Collecting into
`Result<Vec<i32>, ParseIntError>` adds each `Ok` value to the vector, then returns the
first `Err` without consuming the rest of the iterator.

This preserves the fail-fast policy of `parse_values`. It also rules out partial output.
The function may collect some integers before parsing fails, but it drops that vector on
error. Its return type holds either the complete vector or one error, not both.

`collect_errors` has different requirements. Its test places a successful `CountedAccept`
validator between two rejecting validators. The expected call count shows that work must
continue after the first rejection. The expected vector shows that errors must stay in
validator order. `collect::<Result<_, _>>()` and `?` would both stop too early.

The starter also checks the same `Result` variant twice. `is_err()` checks for `Err`,
then `err()` turns the result into an `Option<String>`, and `expect` relies on the earlier
check. A single match avoids this repetition. In an iterator chain, `filter_map` can
consume that `Option` directly.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/08_parse_ints.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/08_parse_ints.rs:tests}}
</script>

The new `parse_values` takes a borrowed slice. `values.iter()` visits each `String` by
reference, and method resolution lets `parse` read the underlying `str`. Neither the
vector nor any of its strings is consumed.

The return type determines the types in the iterator chain. It tells `parse` that
successful values are `i32`, and it tells `collect` to build
`Result<Vec<i32>, ParseIntError>`. The first parse error ends collection. If every parse
succeeds, the function returns the integers in input order. Empty input becomes
`Ok(vec![])`.

`collect_errors` does not collect its `Result` values into another `Result`.
Instead, `filter_map` runs each validator and
passes its result to `Result::err`. `Ok(())` becomes `None`; `Err(message)` becomes
`Some(message)`. Only the messages continue through the pipeline.

The final `collect` consumes the entire iterator, so every validator runs. Traversal
order is preserved, which keeps the returned messages in validator order. This also
removes the repeated variant check and the `expect`: `filter_map` discards `None` and
keeps the value inside `Some`.

</details>

## Edge cases and production concerns

> **EDGE CASE**
>
> Choose the return type and iterator consumer to match the required error policy.
> Fail-fast parsing stops at the first error; collect-all validation runs every check.

`parse_values` returns only a `ParseIntError`. The caller does not learn which input
failed, what its index was, or how many values parsed successfully before it. That is
enough when you only need to know whether the whole batch is valid. If callers
need to mark a bad field or resume work, return an error with the index and source text,
or introduce an explicit partial-result type.

The parser also follows Rust's integer syntax. Leading and trailing whitespace are
rejected, as are values outside the `i32` range. Accepting whitespace or handling
out-of-range values differently would change the parsing policy, not just the implementation.

`collect_errors` allocates every returned `String` because the trait asks validators for
owned error messages. As the validation system grows, a structured error can carry a
field name, code, and context without making callers recover data from human-readable
text.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare both canonical iterator implementations with their starter
> loops. The two functions keep their distinct failure policies.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions use `Result` to distinguish successful parses from `ParseIntError` and return owned validation messages. |
| Readability | wins | Each pipeline states its policy: collect `Result` values to fail fast, or use `filter_map(Result::err)` to keep every error. |
| API design | wins | `parse_values` borrows the input slice, communicating read-only access while leaving ownership with the caller. |
| Performance | wins | Parsing and validation remain single passes, and callers can retain their input without cloning it before `parse_values`. |
| Flexibility | wins | The caller can reuse borrowed input, and further transformations do not require manual accumulator state. |
| Error reporting | neutral | Parsing still returns only the first `ParseIntError`, while validation still returns every string message in order. |

Understanding the shorter `parse_values` body requires knowing how `collect` handles
`Result` items. This is a common Rust pattern, and the return type states the fail-fast
policy. `collect_errors` needs a different iterator chain because it must continue after
an error. Shortening either function must not change when it stops.

## Quiz

### 1. Should `parse_values` accept `Vec<String>` because it returns an owned `Vec<i32>`?

*Answer:* No. Owning the output does not require owning the input. Parsing reads each
string and creates new integers. `&[String]` states that requirement and lets the caller
keep the strings.

### 2. Can `collect_errors` map `Validator::check` and use `collect::<Result<Vec<_>, _>>()`?

*Answer:* No. Collecting into `Result` stops at the first `Err`. The contract requires
every validator to run and every error to remain in validator order, so
`filter_map(Result::err)` followed by `collect::<Vec<_>>()` is the right policy.

### 3. Should `parse_values` return the integers parsed before the first error?

*Answer:* Not with its current signature and contract. `Result<Vec<i32>, ParseIntError>`
represents complete success or one failure. It has nowhere to put a partial vector. Add a
partial-result type only when callers have a real use for incomplete output.

### 4. Is replacing `is_err()` and `err().expect(...)` only a style change?

*Answer:* No. It removes a repeated variant check and a panic-capable assertion. The
starter's `expect` is logically safe, but `filter_map` consumes the `Option` from `err()`
directly and expresses the collect-all policy more clearly.

## Follow-up questions

> **FOLLOW-UP**
>
> - Do your parsing APIs need the first error, every error, or a partial result?
> - Should a parse error include the input index or original text?
> - Could `parse_values` accept something more general than `&[String]`, and would that
>   make its public interface harder to use?
> - Which validators in your codebase have side effects that make “every validator runs”
>   part of the contract?
> - Should validation errors remain strings, or do callers need structured data?

## Concepts recap

> **CONCEPT**
>
> - [Borrowed slices](../appendix/concept-index.md#borrowed-slices): use `&[T]` when a
>   function only needs to read a sequence and should not take ownership.
> - [Collecting `Result` values](../appendix/concept-index.md#collecting-results): collect
>   `Result<T, E>` items into `Result<Vec<T>, E>` with fail-fast behaviour.
> - [Short-circuiting](../appendix/concept-index.md#short-circuiting): stop consuming an
>   iterator once the first failure determines the complete result.
> - [`filter_map`](../appendix/concept-index.md#filter-map): discard successful validator
>   results while retaining each error in traversal order.
> - [Return-type inference](../appendix/concept-index.md#return-type-inference): let the
>   declared return type determine both the parsed item and the resulting collection.
> - [Collect-all validation](../appendix/concept-index.md#collect-all-validation): run
>   every check and preserve every failure when callers need a complete report.
