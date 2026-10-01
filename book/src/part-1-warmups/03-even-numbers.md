# 03 · `even_numbers`

## The problem

You ask for the first three even numbers, but the helper builds every even number below
its upper bound before handing any of them back. The values are correct. The contract is
needlessly eager. This also makes the loop visit every integer and test for evenness,
even though the traversal itself can skip the odd values.

## Starter code

> **EXERCISE**
>
> Treat the sequence and the return type as separate decisions. The sequence must still
> include zero and exclude the upper bound. Then decide whether `evens` should have to
> construct the complete result before it returns.

```rust
{{#rustdoc_include ../../../examples/03_even_numbers.rs:listing}}
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

First, keep the behavior that the tests establish. The sequence is ascending, starts at
zero, and excludes `max`. `evens(0)` is empty. If the bound is odd, the sequence includes
the even value immediately below it. The tests say nothing about how those values must be
stored.

The loop is a half-open range written out by hand: start a counter at zero, keep going
while `n < max`, and increment it after each pass. It then discards every odd candidate.
Because zero is the first value, a stride of two expresses the same sequence without a
parity check.

Changing the traversal is the easy part. The return type is the consequential choice. A
`Vec` gives callers owned storage, indexing, and repeated traversal. It also forces
`evens` to generate and allocate the complete result, even when a caller needs only a
prefix. An iterator puts that decision at the call site: the caller chooses how many
values to consume and whether to collect them.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#rustdoc_include ../../../solutions/03_even_numbers.rs:listing}}
```

`0..max` already has the tested boundaries: it starts at zero, excludes `max`, and is
empty when `max` is zero. `step_by(2)` turns that range into the required arithmetic
progression. There is no counter to update and no parity branch to take.

The return type exposes the useful promise, `Iterator<Item = u32>`, while keeping the
concrete `StepBy<Range<u32>>` type private. The internal adapter chain may change later as
long as it still yields owned `u32` values.

Nothing happens until the caller consumes the iterator. `take(3)` asks for three values.
`sum` reduces the sequence without building a temporary collection. A caller that needs
owned storage can still use `collect::<Vec<_>>()`.

That flexibility is not free. This is a visible API change. The solution tests must call
`collect` before comparing a complete sequence, and they use `count` for the empty case.
Existing callers that index the result, store it, or traverse it more than once also need
to change.

The solution adds a `stays_lazy` test for the property the shared tests cannot establish.
It uses `u32::MAX` as the bound and consumes only three values. This is not an allocation
measurement. It shows that a caller can stop after a small prefix instead of constructing
the full result.

</details>

## Alternative approaches

If callers rely on an owned collection, you can remove the manual loop without changing
the API:

```rust
pub fn evens(max: u32) -> Vec<u32> {
    (0..max).step_by(2).collect()
}
```

This is the safer refactor when the `Vec` is part of the contract. It improves the
implementation and leaves callers alone. It remains eager, so it does not satisfy this
exercise's lazy-result requirement.

A lazy filter is also correct:

```rust
pub fn evens(max: u32) -> impl Iterator<Item = u32> {
    (0..max).filter(|n| n % 2 == 0)
}
```

`filter` is a good fit when selection depends on an irregular predicate. These accepted
values form a fixed progression instead. I'd use `step_by(2)` here because it states that
rule and never visits the odd candidates.

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

An iterator is not a better `Vec`. Owned storage is the right contract when callers need
it. The iterator wins here because laziness is an explicit requirement, and callers that
need storage can still collect the sequence.

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
>   includes zero and excludes the upper bound.
> - [Lazy iterators](../appendix/concept-index.md#lazy-iterators): produce values as
>   consumers request them instead of storing the full result first.
> - [Return-position `impl Trait`](../appendix/concept-index.md#return-position-impl-trait):
>   promise iterator behaviour while keeping the concrete adapter type private.
> - [`Iterator::step_by`](../appendix/concept-index.md#iterator-step-by): traverse a
>   sequence at a fixed stride.
> - [Iterator consumers](../appendix/concept-index.md#iterator-consumers): choose
>   `take`, `sum`, `collect`, or another operation according to the caller's needs.
