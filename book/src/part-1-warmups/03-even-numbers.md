# 03 · `even_numbers`

## The problem

A helper returns every even integer below an upper bound. The starter gets the values
right, but it also makes every caller wait for a complete `Vec`. If you only need a
prefix, a sum, or the next step in an iterator chain, most of that work is wasted. The
loop has another cost: it visits every integer, then tests a property we can encode in
the traversal itself.

## Starter code

> **EXERCISE**
>
> Review the implementation and its return type separately. Keep the sequence intact,
> including zero and the exclusive upper bound. Then decide whether `evens` should
> construct the complete result before it returns.

```rust
{{#include ../../../examples/03_even_numbers.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/03_even_numbers.rs`. You may change the
> function signature in this exercise.
>
> ```sh
> cargo test --example 03_even_numbers
> ```
>
> As you work, check what existing callers would need to change if the return type were
> no longer `Vec<u32>`.

## Review

> **REVIEW**
>
> - Does `evens` need to allocate and fill a complete `Vec` before returning?
> - What happens when you only want the first three values?
> - Can a range replace the manual counter and loop condition?
> - Can the traversal produce only even values instead of testing every integer?
> - Which parts of the contract do the tests establish: start, bound, order, or storage
>   type?
> - What do callers gain and lose if the function returns an iterator?

Start with the behavior the tests pin down. The sequence is ascending, begins at zero,
and excludes `max`. `evens(0)` is empty. An odd bound includes the even value just below
it. None of those tests require a particular collection type.

The loop writes a half-open range by hand: initialize a counter at zero, continue while
`n < max`, then increment after each pass. It also throws away every odd value. Because
zero is the first candidate, a stride of two can generate the required sequence without
a parity check.

The return type calls for more thought. A `Vec` gives you owned storage, indexing, and
repeated traversal. In return, `evens` must generate and allocate the entire result even
when you need only part of it. An iterator moves that choice to the caller. You decide
how much work to perform and whether to collect the values at all.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/03_even_numbers.rs}}
```

`0..max` keeps every tested boundary: it starts at zero, excludes `max`, and is empty
when `max` is zero. Adding `step_by(2)` turns the range into the required arithmetic
progression. The mutable counter and parity branch disappear.

The return type promises an `Iterator<Item = u32>` without exposing the concrete
`StepBy<Range<u32>>` type. That leaves us free to change the internal adapters later,
as long as they still produce owned `u32` values.

Work now happens as the caller consumes the iterator. `take(3)` requests three values.
`sum` reduces the sequence without a temporary collection. When you do need owned
storage, `collect::<Vec<_>>()` is still available.

This is a visible API change. The solution tests call `collect` before comparing a full
sequence and use `count` for the empty case. Existing callers that index, store, or
traverse the old `Vec` more than once would need changes too.

The solution also adds a `stays_lazy` test. It passes `u32::MAX` as the bound, then
consumes only three values. This test does not measure allocation. It establishes that
a caller can stop after a small prefix instead of constructing the full result.

</details>

## Alternative approaches

You can remove the manual loop and keep the eager API:

```rust
pub fn evens(max: u32) -> Vec<u32> {
    (0..max).step_by(2).collect()
}
```

This is the safer refactor when callers depend on an owned collection. It improves the
implementation without changing the interface, but it does not meet the exercise's
request for a lazy result.

A lazy filter is valid too:

```rust
pub fn evens(max: u32) -> impl Iterator<Item = u32> {
    (0..max).filter(|n| n % 2 == 0)
}
```

`filter` reads well when selection depends on an irregular predicate. Here, the accepted
values form a fixed progression. I prefer `step_by(2)` because it states that rule
directly and skips the odd candidates.

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

An iterator is not always an upgrade over `Vec`. Owned storage is a useful contract when
callers need it. The iterator wins these comparisons because laziness is part of this
exercise, and callers can still choose to collect the sequence.

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
> - Which callers become easier to read with `take`, `map`, `sum`, or direct iteration?
> - Which callers need an explicit `collect`, and does moving that choice outward help
>   them?
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
