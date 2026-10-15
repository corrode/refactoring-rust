# 03 · `even_numbers`

## The problem

`evens` returns the even numbers from zero up to, but not including, `max`, in ascending
order. It returns an empty sequence when `max` is zero. The starter checks every integer
and builds a complete `Vec`, even when the caller only needs the first few values. This
exercise changes it to return a lazy iterator.

## Starter code

> **EXERCISE**
>
> Preserve the sequence while changing how callers receive it. For a nonzero bound,
> the sequence must still start at zero and exclude `max`. Return the values lazily
> instead of building a complete collection inside `evens`.

```rust,editable
{{#include ../../../examples/03_even_numbers.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/03_even_numbers.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/03_even_numbers.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/03_even_numbers.rs`. You may change the
> function signature in this exercise.
>
> ```sh
> cargo test --example 03_even_numbers
> ```
>
> While you work, account for the callers. What would they need to change if `evens` no
> longer returned a `Vec<u32>`?

## Review

> **REVIEW**
>
> - Does `evens` need to allocate and fill a complete `Vec` before returning?
> - What work happens if you only want the first three values?
> - Can a range replace the counter and loop condition?
> - Can the traversal generate only even values instead of testing every integer?
> - Which parts of the contract do the tests establish: start, bound, order, or storage
>   type?
> - What do callers gain and lose when the function returns an iterator?

The tests check the values and their order: `evens(10)` produces `[0, 2, 4, 6, 8]`,
`evens(0)` is empty, and `evens(7)` includes 6. The starter tests compare the result
directly with a `Vec`, so they need to change when the function returns an iterator.
The expected values stay the same.

The loop is a half-open range written out by hand: start a counter at zero, keep going
while `n < max`, and increment it after each pass. It then discards every odd candidate.
Because zero is the first value, a stride of two expresses the same sequence without a
parity check.

Changing the return type affects callers. A `Vec` provides owned storage, indexing,
and repeated traversal, but `evens` must generate the complete result before returning.
An iterator lets the caller choose how many values to consume and whether to collect
them.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/03_even_numbers.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/03_even_numbers.rs:tests}}
</script>

`0..max` has the required bounds and is empty when `max` is zero. `step_by(2)` yields
every other value, starting with zero. This removes the manual counter and parity check.

The return type, `impl Iterator<Item = u32>`, hides the concrete
`StepBy<Range<u32>>` type. The implementation can use different adapters later as long
as it preserves the promised sequence and public trait guarantees.

Calling `evens` creates the iterator without generating the full sequence. `take(3)`
limits a later traversal to at most three values; it does not consume them by itself.
`sum` consumes the sequence without building a temporary collection. A caller that
needs owned storage can use `collect::<Vec<_>>()`.

The solution tests call `collect` before comparing a complete sequence and use `count`
for the empty case. Callers that rely on a `Vec` for indexing, storage, or repeated
traversal also need to collect the iterator or change how they use the result.

The solution adds a `stays_lazy` test for the property the shared tests cannot establish.
It uses `u32::MAX` as the bound and consumes only three values. This is not an allocation
measurement. It shows that a caller can stop after a small prefix instead of constructing
the full result.

</details>

## Alternative approaches

If callers rely on an owned collection, you can remove the manual loop without changing
the API:

```rust,editable
pub fn evens(max: u32) -> Vec<u32> {
    (0..max).step_by(2).collect()
}
```

This is the safer refactor when the `Vec` is part of the contract. It improves the
implementation and leaves callers alone. It remains eager, so it does not satisfy this
exercise's lazy-result requirement.

A lazy filter is also correct:

```rust,editable
pub fn evens(max: u32) -> impl Iterator<Item = u32> {
    (0..max).filter(|n| n % 2 == 0)
}
```

`filter` is useful when a predicate determines which values to keep. Here the values
are evenly spaced, so `step_by(2)` states the rule directly without a parity test for
each candidate.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical lazy iterator with the eager `Vec` returned by
> the starter. Fully collecting either version produces the same values.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept any `u32` bound and rely on their implementation to produce the promised sequence. |
| Readability | wins | The range states the bounds and `step_by(2)` states the progression without mutable bookkeeping. |
| API design | wins | The return type promises a sequence and leaves collection policy to the caller, which matches the stated lazy requirement. |
| Performance | wins | The solution works on demand and allocates no result collection unless the caller chooses one. |
| Flexibility | wins | Callers can stop early or compose the result with `take`, `map`, `sum`, and `collect`. |
| Error reporting | N/A | Every `u32` bound defines a valid finite sequence. |

Returning an iterator meets this exercise's lazy-result requirement. It does not
provide the indexing or repeated traversal that callers get from a `Vec`.

## Quiz

### 1. Should a review always require replacing a returned `Vec` with an iterator?

*Answer:* No. Return an iterator when callers benefit from incremental consumption,
composition, or avoiding mandatory allocation. Keep a `Vec` when callers need owned
storage, indexing, or repeated traversal.

### 2. Does replacing the loop with `filter` complete this refactor?

*Answer:* It depends on the return type. Returning the filter makes the API lazy.
Collecting it inside `evens` remains eager. For this sequence, `step_by(2)` expresses the
fixed progression more directly.

### 3. What does the solution-only `stays_lazy` test add?

*Answer:* It checks that a caller can request a small prefix from a sequence with an
enormous upper bound. The shared tests establish the values after full consumption. This
test adds evidence that consumption can stop early.

## Follow-up questions

> **FOLLOW-UP**
>
> - Do the real call sites need an owned collection, or do they consume each value only
>   once?
> - Which callers read better with `take`, `map`, `sum`, or direct iteration?
> - Which callers need an explicit `collect`, and is moving that choice outward useful
>   for them?
> - Is a helper named `evens` clearer than writing `(0..max).step_by(2)` at each call
>   site?
> - Would you make the API change in the same pull request as the loop cleanup?

## Concepts recap

> **CONCEPT**
>
> - [Half-open ranges](../appendix/concept-index.md#half-open-ranges): `0..max`
>   excludes the upper bound and starts at zero unless the range is empty.
> - [Lazy iterators](../appendix/concept-index.md#lazy-iterators): produce values as
>   consumers request them instead of storing the full result first.
> - [Return-position `impl Trait`](../appendix/concept-index.md#return-position-impl-trait):
>   promise iterator behaviour while keeping the concrete adapter type private.
> - [`Iterator::step_by`](../appendix/concept-index.md#iterator-step-by): traverse a
>   sequence at a fixed stride.
> - [Iterator consumers](../appendix/concept-index.md#iterator-consumers): `sum` and
>   `collect` consume an iterator; adapters such as `take` limit later consumption.
