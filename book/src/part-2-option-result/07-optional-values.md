# 07 · `optional_values`

## The problem

A batch of measurements contains values that may be absent. The function must add every
present `i32` and ignore missing entries. The starter does that with a mutable total and
a `match`, but `Option` and `Iterator` already agree on a useful interpretation: a
`Some` contributes one value, while a `None` contributes no values.

## Starter code

> **EXERCISE**
>
> Review the loop as an aggregation over optional values. Preserve the treatment of
> `None`, empty input, and a vector containing no present values.

```rust
{{#include ../../../examples/07_optional_values.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/07_optional_values.rs` without changing
> its public signature or behaviour:
>
> ```sh
> cargo test --example 07_optional_values
> ```
>
> Try to express both operations in the function name: discard absent values, then sum
> the values that remain.

## Review

> **REVIEW**
>
> - What does each `Some` contribute to the result?
> - What should each `None` contribute?
> - What is the sum of an empty sequence?
> - Does the function consume the input vector, or only borrow it?
> - Which iterator adapter can turn a sequence of zero-or-one-value containers into a
>   sequence of values?
> - Does replacing the loop with `sum` change the risk of integer overflow?

The loop treats each `Option<i32>` as either one number or no number. That is also how
`Option<T>` behaves when converted into an iterator. `Some(value)` yields `value` once;
`None` yields nothing. Once those small iterators are joined, an ordinary sum can finish
the job.

The tests establish an important identity value. The sum of an empty sequence is zero.
That covers both an empty vector and a vector whose entries are all `None`, because both
produce zero values after the absent entries are removed.

The signature also establishes an ownership decision. `sum_options` receives a
`Vec<Option<i32>>` by value, and the `for` loop consumes it. A refactor using
`into_iter()` preserves that behaviour. This is not the same as proving that ownership
is necessary. If callers need the vector afterwards, a borrowed slice could be a better
interface, but that would be a separate API change.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/07_optional_values.rs}}
```

`into_iter()` takes ownership of the vector and yields each `Option<i32>` by value. That
matches the starter's ownership behaviour.

`flatten()` then uses the iterator implementation of each `Option`:

- `Some(value)` contributes one `i32`.
- `None` contributes zero `i32` values.

The resulting iterator contains only the present integers, in their original order.
`sum()` consumes that iterator and adds its items. The function's `i32` return type tells
`sum` which numeric result to produce.

No special branch is needed for empty input. `Sum` uses zero as the additive identity
for `i32`, so an iterator with no items returns `0`. This is why both `empty` and
`all_none` continue to pass.

</details>

## Alternative approaches

`filter_map` can express the same zero-or-one-item conversion with an identity closure:

```rust
pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    options.into_iter().filter_map(|option| option).sum()
}
```

The closure returns each `Option` unchanged. `filter_map` yields the value from `Some`
and discards `None`. This is correct, but `flatten` states that the items are already
nested containers and need no transformation. I prefer `flatten` here because the
closure adds no decision of its own.

If the function should leave the vector available to its caller, borrowing changes the
iterator types:

```rust
pub fn sum_options(options: &[Option<i32>]) -> i32 {
    options.iter().flatten().copied().sum()
}
```

Here `iter()` yields borrowed options, `flatten()` yields `&i32`, and `copied()` produces
the owned `i32` values expected by the sum. This interface is more permissive for
callers, but it is not behaviourally identical at the ownership boundary.

## Edge cases and production concerns

> **EDGE CASE**
>
> Replacing the loop with `sum` does not make `i32` addition safe from overflow.

A sufficiently large total can still exceed the range of `i32`. Depending on the build
and overflow-check settings, that can panic or wrap. The iterator version inherits the
same risk as `sum += value` in the starter.

If overflow is a valid runtime failure, use `checked_add` in a fold and return an
`Option<i32>` or a domain-specific `Result`. If larger totals are valid, choose a wider
integer type. Neither choice belongs in a control-flow cleanup without checking the
callers and domain limits.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `flatten().sum()` implementation with the starter
> loop. The borrowed-slice alternative changes the interface and needs its own review.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions use `Option<i32>` for absence and both can overflow while adding present values. |
| Readability | wins | `flatten` and `sum` name the two operations without mutable state or an empty `None` branch. |
| API design | neutral | The signature still consumes `Vec<Option<i32>>` and returns an unvalidated `i32` total. |
| Performance | neutral | Both versions make one pass, allocate nothing, and perform one addition per present value. |
| Flexibility | neutral | The public interface is unchanged; only the internal expression of the aggregation differs. |
| Error reporting | neutral | Neither version reports overflow or distinguishes empty input from an all-`None` input. |

The one-line body is not valuable because it is short. It is valuable when your team
reads `flatten` as “zero or one value from each option” and `sum` as the intended
reduction. If that vocabulary is unfamiliar, the starter loop remains correct and is a
reasonable stepping stone.

## Quiz

### 1. Should `None` be converted to `0` before summing?

*Answer:* It can be, but it needlessly creates an item for an absent value. Treating
`None` as zero elements gives the same total and matches the requirement to ignore
missing entries. This distinction matters if later operations do more than addition.

### 2. Should this refactor change the parameter to `&[Option<i32>]`?

*Answer:* Only after checking callers. Borrowing avoids consuming the vector and better
fits a read-only calculation, but it is an API change. The canonical solution keeps the
ownership contract separate from the iterator refactor.

### 3. Does `sum()` solve the overflow problem?

*Answer:* No. It performs ordinary `i32` addition and has the same overflow concern as
the starter. Use checked arithmetic or a wider type when the domain requires it.

## Follow-up questions

> **FOLLOW-UP**
>
> - Do callers still need their vector after computing the total?
> - Does the domain distinguish “no entries” from “entries exist, but every value is
>   missing”?
> - Should overflow be impossible by construction, reported as an error, or handled with
>   a wider accumulator?
> - Does your team find `flatten` clearer than an explicit `filter_map` for `Option`
>   values?

## Concepts recap

> **CONCEPT**
>
> - [`Option` as an iterator](../appendix/concept-index.md#option-as-an-iterator):
>   `Some` yields one item and `None` yields no items.
> - [`Iterator::flatten`](../appendix/concept-index.md#iterator-flatten): join nested
>   zero-or-one-item iterators into one sequence.
> - [`Iterator::sum`](../appendix/concept-index.md#iterator-sum): reduce numeric items
>   using the type's additive identity.
> - [Ownership and `into_iter`](../appendix/concept-index.md#ownership-and-into-iter):
>   consuming a collection yields its items by value.
> - [`filter_map`](../appendix/concept-index.md#filter-map): transform present values and
>   discard absent results in one adapter.
> - [Checked arithmetic](../appendix/concept-index.md#checked-arithmetic): represent
>   integer overflow as a value instead of leaving it to overflow settings.
