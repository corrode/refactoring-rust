# 08 · `parse_ints`

## The problem

Two functions process fallible work with different contracts. `parse_values` converts a
batch of strings to integers and stops at the first invalid value. `collect_errors` runs
a list of validators and must retain every error in validator order. The starter uses
loops for both. Their similar control flow can tempt you into applying the same
`Result` shortcut twice, but only one function is allowed to stop early.

## Starter code

> **EXERCISE**
>
> Review the two loops as separate error-handling policies. Preserve fail-fast parsing,
> collect-all validation, validator order, and the requirement that every validator
> runs.

```rust
{{#include ../../../examples/08_parse_ints.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/08_parse_ints.rs`:
>
> ```sh
> cargo test --example 08_parse_ints
> ```
>
> Let `parse_values` borrow its input. Before changing `collect_errors`, identify why a
> second fail-fast pipeline would violate its test.

## Review

> **REVIEW**
>
> - Does `parse_values` need ownership of each `String` to parse it?
> - What iterator item type appears after mapping `str::parse` over the input?
> - What does collecting an iterator of `Result<i32, ParseIntError>` produce?
> - If the second string is invalid, can the caller observe integers parsed before it?
> - Must validators after the first rejection still run?
> - Does `collect_errors` promise errors in validator order?
> - Why is `is_err()` followed by `err().expect(...)` redundant?

`parse_values` only reads each string. `String::parse` works through the string's `str`
contents and does not need to consume the allocation. Accepting `&[String]` therefore
states the narrower requirement and lets callers reuse their input after parsing.

Mapping `parse` over `values.iter()` produces an iterator whose items are
`Result<i32, ParseIntError>`. Rust has a specialised `FromIterator` implementation for
this case. Collecting those items into `Result<Vec<i32>, ParseIntError>` pushes every
`Ok` value into the vector and returns the first `Err` immediately.

That early return matches `parse_values`. It also means the caller receives no partial
output. A successfully parsed prefix may exist temporarily, but it is dropped when an
error occurs because the return type contains either the complete vector or one error.

`collect_errors` has the opposite requirement. The test puts a successful
`CountedAccept` validator between two rejecting validators. The expected call count
proves that validation continues after the first error, while the expected vector
proves that errors retain validator order. A short-circuiting `collect::<Result<_, _>>()`
or `?` would stop too soon.

The starter also inspects the same `Result` twice. After `is_err()` proves the variant,
`err()` converts it to `Option<String>`, and `expect` asserts the fact just checked.
Matching once would remove the redundancy. An iterator can go further and pass that
`Option` directly to `filter_map`.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/08_parse_ints.rs}}
```

The new `parse_values` signature accepts a borrowed slice. `values.iter()` visits each
`String` by reference, and method resolution lets `parse` read its `str` contents.
Neither the vector nor its strings are consumed.

The return type supplies the types omitted from the iterator chain. It tells `parse`
that each successful value must be `i32`, and it tells `collect` to build
`Result<Vec<i32>, ParseIntError>`. Collection stops on the first parse error. If every
parse succeeds, the `Ok` values are returned in input order. Empty input collects into
`Ok(vec![])`.

`collect_errors` deliberately does not collect `Result` values into a `Result`.
`filter_map` calls each validator, turns its result into `Option<String>` with
`Result::err`, and yields only the error strings. `Ok(())` becomes `None`; `Err(message)`
becomes `Some(message)`.

The final `collect` drains the complete iterator, so every validator runs. Iterators
preserve traversal order here, so the returned messages remain in validator order. The
pipeline also removes the `is_err()` check and the following `expect`: `filter_map`
already knows how to discard `None` and retain the value inside `Some`.

</details>

## Edge cases and production concerns

> **EDGE CASE**
>
> Fail-fast parsing and collect-all validation answer different questions. Choose the
> return type and iterator consumer that match the required failure policy. Similar loops
> can still need different consumers.

`parse_values` reports only a `ParseIntError`. It does not identify the offending input,
its index, or any successfully parsed prefix. That is enough when the caller only needs
to know whether the complete batch is valid. If the caller must highlight bad fields or
resume work, return an error carrying the index and source text, or design an explicit
partial-result type.

Parsing also follows Rust's integer syntax. Leading or trailing whitespace is rejected,
and values outside the `i32` range fail. Normalise input only if the domain says those
forms are acceptable.

`collect_errors` allocates every returned `String` because the trait requires owned
error messages. In a larger validation system, a structured error type can retain a
field name, code, and context without asking callers to parse human-readable text.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare both canonical iterator implementations with their starter
> loops. The two functions retain their distinct failure policies.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions use `Result` to distinguish successful parses from `ParseIntError` and return owned validation messages. |
| Readability | wins | Each pipeline names its policy: collect `Result` values to fail fast, or `filter_map(Result::err)` to retain every error. |
| API design | wins | `parse_values` now borrows the input slice, which communicates read-only access and leaves ownership with the caller. |
| Performance | wins | Parsing and validation remain single passes, while callers can retain their input without cloning it before `parse_values`. |
| Flexibility | wins | Borrowed input can be reused, and the iterator pipelines can accept further transformations without manual accumulator state. |
| Error reporting | neutral | Parsing still returns only the first `ParseIntError`, while validation still returns all string messages in order. |

The short `parse_values` body relies on a standard but non-trivial `collect`
implementation. I would keep it because the return type makes the policy visible and
fail-fast collection is common Rust vocabulary. The contrast with `collect_errors` is
the more important review point: fewer lines do not justify changing when work stops.

## Quiz

### 1. Should `parse_values` accept `Vec<String>` because it returns an owned `Vec<i32>`?

*Answer:* No. Ownership of the output does not require ownership of the input. Parsing
reads each string and creates new integers, so `&[String]` communicates the actual
requirement and lets the caller keep the strings.

### 2. Can `collect_errors` map `Validator::check` and use `collect::<Result<Vec<_>, _>>()`?

*Answer:* No. Collecting into `Result` stops at the first `Err`. The contract requires
all validators to run and all errors to appear in validator order, so
`filter_map(Result::err)` followed by `collect::<Vec<_>>()` is the right policy.

### 3. Should `parse_values` return the integers parsed before the first error?

*Answer:* Not under its current signature and contract. `Result<Vec<i32>, ParseIntError>`
represents a complete success or one failure, with no place for a partial vector. Add an
explicit partial-result type only when callers have a real use for incomplete output.

### 4. Is replacing `is_err()` and `err().expect(...)` only a style change?

*Answer:* It also removes a duplicated variant check and a panic-capable assertion. The
starter's `expect` is logically safe, but `filter_map` consumes the `Option` returned by
`err()` directly and states the collect-all policy more clearly.

## Follow-up questions

> **FOLLOW-UP**
>
> - Do your parsing APIs need the first error, every error, or a partial result?
> - Should a parse error include the input index or original text?
> - Could `parse_values` accept a more general input than `&[String]`, or would that make
>   the public interface harder to use?
> - Which validators in your codebase have side effects that make “every validator runs”
>   part of the contract?
> - Should validation errors remain strings, or does the caller need structured data?

## Concepts recap

> **CONCEPT**
>
> - [Borrowed slices](../appendix/concept-index.md#borrowed-slices): accept `&[T]` when a
>   function needs read access to a sequence without taking ownership.
> - [Collecting `Result` values](../appendix/concept-index.md#collecting-results): turn an
>   iterator of `Result<T, E>` into `Result<Vec<T>, E>` with fail-fast behaviour.
> - [Short-circuiting](../appendix/concept-index.md#short-circuiting): stop consuming an
>   iterator when the first failure determines the complete result.
> - [`filter_map`](../appendix/concept-index.md#filter-map): discard successful validator
>   results and retain each error in traversal order.
> - [Return-type inference](../appendix/concept-index.md#return-type-inference): let the
>   declared return type determine the item parsed and the collection built.
> - [Collect-all validation](../appendix/concept-index.md#collect-all-validation): run
>   every check and preserve every failure when callers need a complete report.
