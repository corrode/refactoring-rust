# `parse_ints`

One function parses integers; the other collects validation errors.
Can you make both easier to read without changing how they handle errors?
Parsing must stop at the first failure.
Validation must run every check and collect all errors in input order.
Keep the original function signatures and results.
Consider changes to input ownership afterward as a separate alternative.

Run the local tests with `cargo test --example 09_parse_ints`.

## Code to refactor

```rust,editable
{{#include ../../../examples/09_parse_ints.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/09_parse_ints.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>Does <code>parse_values</code> need to own each <code>String</code>?</summary>

No, it only reads each string.
`parse` works on the string's `str` contents, so you can accept `&[String]` instead of taking ownership of the vector.
Callers can then keep using their strings afterward.
That changes the API, so keep it separate from the first refactor.

</details>

<details class="discussion-question">
<summary>What is the iterator's item type after parsing each string?</summary>

`Result<i32, ParseIntError>`, when you parse as `i32`.
Mapping `parse` over `values.into_iter()` produces either a parsed integer or a parse error for each string it visits.

</details>

<details class="discussion-question">
<summary>What happens when we collect those items into <code>Result&lt;Vec&lt;i32&gt;, ParseIntError&gt;</code>?</summary>

You get either all the parsed integers or the first error.
The standard library's specialised `FromIterator` implementation adds each `Ok` value to the vector and stops at the first `Err`.
It returns that error without consuming the rest of the iterator.
If every parse succeeds, it returns `Ok` with the complete vector.
That keeps `parse_values` fail-fast.

</details>

<details class="discussion-question">
<summary>If the second string is invalid, can the caller use the integer parsed before it?</summary>

No, the function drops the partially collected vector on error.
Its return type holds either the complete vector or one error, not both.

</details>

<details class="discussion-question">
<summary>Do validators after the first rejection still have to run?</summary>

Yes, `collect_errors` must run every validator, even after a rejection.
Its test places a successful `CountedAccept` validator between two rejecting validators.
The test checks the call count to confirm that validation continues after the first rejection.
Using `collect::<Result<_, _>>()` or propagating a rejection with `?` would stop too early.

</details>

<details class="discussion-question">
<summary>Does the order of errors from <code>collect_errors</code> matter?</summary>

Yes, the test's expected vector requires errors to stay in validator order.
Skipping successful results must not change the order of the remaining error messages.

</details>

<details class="discussion-question">
<summary>Do we need both <code>is_err()</code> and <code>err().expect(...)</code>?</summary>

No, one match is enough.
The starter uses `is_err()` to check for `Err`, then `err()` to convert the result into an `Option<String>`.
`expect` relies on the earlier check to extract the message.
You can match once and extract it directly.
In an iterator chain, `filter_map` can consume that `Option` directly.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/09_parse_ints.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/09_parse_ints.rs:tests}}
</script>

`parse_values` still takes ownership of `Vec<String>`.
`values.into_iter()` yields each owned `String`, and method resolution lets `parse` read the underlying `str`.
Both functions keep their original signatures, so callers need no changes.

The return type determines the types in the iterator chain.
It tells `parse` that successful values are `i32`, and it tells `collect` to build `Result<Vec<i32>, ParseIntError>`.
The first parse error ends collection.
If every parse succeeds, the function returns the integers in input order.
Empty input becomes `Ok(vec![])`.

Why not collect into a `Result` in `collect_errors` too?
That would stop at the first error, but this function needs every error.
Instead, the closure passed to `filter_map` runs each validator and calls `Result::err` on its result.
`Ok(())` becomes `None`; `Err(message)` becomes `Some(message)`.
`filter_map` keeps only the messages.

The final `collect` consumes the entire iterator, so every validator runs.
It collects the messages in validator order.
You no longer need the repeated variant check or `expect`: `filter_map` discards `None` and extracts the value inside `Some`.

### Alternative: borrow the input

If you can change the parsing API, accept a borrowed slice instead:

```rust,editable
{{#include ../../../solutions/09_parse_ints_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/09_parse_ints_alt.rs:tests}}
</script>

`values.iter()` visits each `String` by reference.
The caller keeps both the vector and its strings, but must now pass a reference to `parse_values`.
The error policy stays the same, and `collect_errors` needs no further changes.

### Edge cases and production concerns

> **EDGE CASE**
>
> Choose the return type and iterator consumer to match the required error policy.
> Fail-fast parsing stops at the first error; collect-all validation runs every check.

`parse_values` returns only a `ParseIntError`.
The caller does not learn which input failed, what its index was, or how many values parsed successfully before it.
Use that return type when you only need to know whether the whole batch is valid.
If callers need to mark a bad field or resume work, return an error with the index and source text, or introduce an explicit partial-result type.

The parser follows Rust's integer syntax.
It rejects leading and trailing whitespace and values outside the `i32` range.
Accepting whitespace or handling out-of-range values differently would change the parsing policy, not just the implementation.

`collect_errors` allocates every returned `String` because the trait asks validators for owned error messages.
If callers need a field name, error code, or context, return structured errors rather than making them extract that data from human-readable text.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

#### Parsing: `parse_values`

| Criterion | Original | Owned `map().collect()` (main) | Borrowed slice + `map().collect()` |
|---|---|---|---|
| Type-safety | Complete vector or `ParseIntError` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same all-or-error result | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same all-or-error result |
| Readability | Explicit push and early return | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names parsing and collection | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names parsing and collection |
| API design | Consumes `Vec<String>` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Preserves `Vec<String>` parameter | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Borrows `&[String]` |
| Performance | Single pass; allocates output vector | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Single pass; allocates output vector | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Avoids cloning when caller retains input |
| Flexibility | Requires owned vector | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Requires owned vector | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Accepts slices, arrays, borrowed vectors |
| Error reporting | First `ParseIntError`; no input index | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same error, still no index | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same error, still no index |

#### Validation: `collect_errors`

| Criterion | Original | `filter_map` with `.err()` + `collect` |
|---|---|---|
| Type-safety | `expect` guarded by `is_err` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same `Result` and `Option` types |
| Readability | Repeated variant checks, mutable accumulator | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names error selection and collection |
| API design | Borrows validators and input | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same borrowed signature |
| Performance | Every validator runs; stores error strings | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same calls and owned messages |
| Flexibility | Heterogeneous validators via trait objects | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same trait-object interface |
| Error reporting | All string errors, validator order | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same messages and order |

Only the borrowed alternative requires callers to pass a reference to `parse_values`.
Borrowing saves a clone only if they would otherwise clone the input to keep it.
Neither iterator chain guarantees a speed-up.
Don't replace the validation chain with `collect` over `Result`: it would stop at the first error.

</details>

## Takeaways For Our Own Codebase

> - Decide whether each parsing API should return the first error, every error, or a partial result before choosing an iterator consumer.
> - Can callers act on a parse error without the input index or original text?
>   Include that context when they need it.
> - Would accepting more than `&[String]` help your callers enough to justify a more general interface?
> - Check whether validators have side effects that require every validator to run.
>   Test that they still run after an earlier rejection.
> - Do callers parse your error strings to recover data?
>   Consider structured errors instead.

## References

- [Borrowed slices](../appendix/concept-index.md#borrowed-slices): use `&[T]` when a function only needs to read a sequence and should not take ownership.
- [Collecting `Result` values](../appendix/concept-index.md#collecting-results): collect `Result<T, E>` items into `Result<Vec<T>, E>` with fail-fast behaviour.
- [Short-circuiting](../appendix/concept-index.md#short-circuiting): stop consuming an iterator once the first failure determines the complete result.
- [`filter_map`](../appendix/concept-index.md#filter-map): discard successful validator results while retaining each error in traversal order.
- [Return-type inference](../appendix/concept-index.md#return-type-inference): let the declared return type determine both the parsed item and the resulting collection.
- [Collect-all validation](../appendix/concept-index.md#collect-all-validation): run every check and preserve every failure when callers need a complete report.
