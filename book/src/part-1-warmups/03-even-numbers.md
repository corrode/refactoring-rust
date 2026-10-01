# 03 · `even_numbers`

## The problem

A helper returns every even integer below an upper bound. The starter computes the
right values, but it also decides that every caller must wait for a complete `Vec`.
That work is wasted when a caller only wants a prefix, a sum, or another iterator
operation. The loop also visits every integer and tests a property that the traversal
can encode directly.

## Starter code

> **EXERCISE**
>
> Review the implementation and its return type separately. Preserve the sequence,
> including zero and the exclusive upper bound, then decide whether the function
> should construct the complete result before returning.

```rust
{{#include ../../../examples/03_even_numbers.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/03_even_numbers.rs`. The exercise
> allows the signature to change.
>
> ```sh
> cargo test --example 03_even_numbers
> ```
>
> Check what existing callers would need to change if the return type stops being
> `Vec<u32>`.

## Review

> **REVIEW**
>
> - Does `evens` need to allocate and fill a complete `Vec` before returning?
> - What happens when a caller only wants the first three values?
> - Can a range replace the manual counter and loop condition?
> - Can the traversal produce only even values instead of testing every integer?
> - Which parts of the contract are established by the tests: start, bound, order, or
>   storage type?
> - What do callers gain and lose if the function returns an iterator?

The tests establish an ascending sequence that starts at zero and excludes `max`.
`evens(0)` is empty, and an odd bound includes the preceding even value. They do not
require the function to return a particular collection type.

The loop spells out a half-open range by hand: start at zero, continue while `n < max`,
and increment after each iteration. It then discards every odd value. Since the first
candidate is zero, stepping by two can produce the desired sequence without a parity
check.

The return type is the larger decision. A `Vec` gives callers owned storage, indexing,
and repeated traversal. It also commits the function to generating and allocating the
complete result. An iterator lets the caller decide how much work to perform and
whether to collect the values at all.

## A possible solution

```rust
{{#include ../../../solutions/03_even_numbers.rs}}
```

`0..max` preserves every tested boundary: it starts at zero, excludes `max`, and is
empty when `max` is zero. `step_by(2)` turns that range into the required arithmetic
progression. No mutable counter or parity branch remains.

The return type promises an `Iterator<Item = u32>` while keeping the concrete
`StepBy<Range<u32>>` type private. We can change the internal adapters later as long
as they still produce owned `u32` values.

Evaluation now happens when a caller consumes the iterator. `take(3)` requests three
values. `sum` can reduce the sequence without a temporary collection.
`collect::<Vec<_>>()` remains available when a caller needs owned storage.

That API change is visible. The solution tests call `collect` before comparing a full
sequence and use `count` for the empty case. Existing callers that index, store, or
traverse the old `Vec` more than once would also need changes.

The solution adds a `stays_lazy` test with `u32::MAX` as the bound and consumes only
three values. The test does not measure allocation. It does establish that a caller
can stop after a small prefix instead of constructing the full result.

## Alternative approaches

We can remove the manual loop while keeping the eager API:

```rust
pub fn evens(max: u32) -> Vec<u32> {
    (0..max).step_by(2).collect()
}
```

This is the safer refactor when callers rely on an owned collection. It improves the
implementation without changing the interface, but it does not meet the exercise's
request for a lazy result.

A lazy filter is also valid:

```rust
pub fn evens(max: u32) -> impl Iterator<Item = u32> {
    (0..max).filter(|n| n % 2 == 0)
}
```

`filter` reads naturally when selection depends on an irregular predicate. Here the
accepted values form a fixed progression, so `step_by(2)` states the rule more
directly and never visits the odd candidates.

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

An iterator is not a universal upgrade over `Vec`. Owned storage is a useful contract
when callers need it. The scores favour the iterator here because laziness is part of
the exercise and callers can still opt into collection.

## Quiz

### 1. Should a review always require replacing a returned `Vec` with an iterator?

*Answer:* No. Return an iterator when callers benefit from incremental consumption,
composition, or avoiding mandatory allocation. Keep a `Vec` when callers need owned
storage, indexing, or repeated traversal.

### 2. Does replacing the loop with `filter` complete this refactor?

*Answer:* It depends on the return type. Returning the filter makes the API lazy;
collecting it inside `evens` remains eager. For this sequence, `step_by(2)` states the
fixed progression more directly.

### 3. What does the solution-only `stays_lazy` test add?

*Answer:* It checks that a caller can request a small prefix from a sequence with an
enormous upper bound. The shared tests establish values after full consumption; this
one adds evidence for early termination.

## Follow-up questions

> **FOLLOW-UP**
>
> - Do the real call sites need an owned collection, or do they consume each value
>   once?
> - Which callers become simpler with `take`, `map`, `sum`, or direct iteration?
> - Which callers need an explicit `collect`, and is moving that choice outward useful
>   for them?
> - Is a helper named `evens` clearer than writing `(0..max).step_by(2)` at its call
>   sites?
> - Would you make the API change in the same pull request as the loop cleanup?

## Concepts recap

> **CONCEPT**
>
> - [Half-open ranges](../appendix/concept-index.md#half-open-ranges): `0..max`
>   includes zero and excludes the upper bound.
> - [Lazy iterators](../appendix/concept-index.md#lazy-iterators): produce values as
>   consumers request them instead of storing the full result first.
> - [Return-position `impl Trait`](../appendix/concept-index.md#return-position-impl-trait):
>   promise iterator behaviour while keeping the concrete adapter type private.
> - [`Iterator::step_by`](../appendix/concept-index.md#iterator-step-by): traverse a
>   sequence at a fixed stride.
> - [Iterator consumers](../appendix/concept-index.md#iterator-consumers): choose
>   `take`, `sum`, `collect`, or another operation according to the caller's needs.
