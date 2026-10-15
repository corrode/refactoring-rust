# 07 · `optional_values`

## Code to refactor

> **EXERCISE**
>
> Treat the loop as an aggregation over optional values. Preserve the existing results for
> missing entries, an empty input vector, and a vector with no present values.

```rust,editable
{{#include ../../../examples/07_optional_values.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/07_optional_values.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/07_optional_values.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/07_optional_values.rs` without changing
> its public signature or behaviour:
>
> ```sh
> cargo test --example 07_optional_values
> ```
>
> Try to name both operations in the expression: remove absent values, then sum what
> remains.

## Discussion

> **DISCUSSION**
>
> - How many values does each `Some` contribute?
> - How many values should a `None` contribute?
> - What should an empty sequence sum to?
> - Does this function consume the vector or borrow it?
> - Which iterator adapter joins a sequence of zero-or-one-value containers into a
>   sequence of values?
> - Does `sum` alter the integer-overflow risk of the original loop?

An `Option<T>` implements `IntoIterator`: `Some(value)` yields one value, while `None`
yields none. `flatten` uses this to produce an iterator over the present integers.

The tests require an empty sequence to sum to zero. After absent
entries are removed, both an empty vector and a vector containing only `None` produce an
empty sequence, so both return `0`.

`sum_options` takes `Vec<Option<i32>>` by value, and the `for` loop consumes it.
`into_iter()` preserves that ownership behavior. Borrowing a slice would let callers
keep the vector, but that is a separate API change.

<details class="solution" id="a-possible-solution">
<summary><h2><span aria-hidden="true">💡</span> A possible solution</h2></summary>

```rust,editable
{{#include ../../../solutions/07_optional_values.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/07_optional_values.rs:tests}}
</script>

`into_iter()` consumes the vector and yields each `Option<i32>` by value, just as the
starter loop does.

Next, `flatten()` uses each option's `IntoIterator` implementation:

- `Some(value)` contributes one `i32`.
- `None` contributes zero `i32` values.

What remains is an iterator over the present integers in their original order. `sum()`
consumes it and adds the items. The function's `i32` return type supplies the result type
that `sum` needs.

Empty input needs no special branch. For `i32`, `Sum` uses zero as the additive identity,
so an iterator with no items returns `0`. That is why the `empty` and `all_none` tests
still pass.

</details>

## Alternative approaches

`filter_map` can perform the same zero-or-one-value conversion with an identity closure:

```rust,editable
pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    options.into_iter().filter_map(|option| option).sum()
}
```

The closure returns each option unchanged. `filter_map` extracts the value from `Some`
and drops `None`. `flatten` does the same without an identity closure.

If callers should retain the vector, a borrowed interface changes the iterator types:

```rust,editable
pub fn sum_options(options: &[Option<i32>]) -> i32 {
    options.iter().flatten().copied().sum()
}
```

Now `iter()` yields borrowed options, `flatten()` yields `&i32`, and `copied()` turns them
into the owned `i32` values used by the sum. The borrowed interface gives callers more
freedom, but it is not the same ownership contract as the starter.

## Edge cases and production concerns

> **EDGE CASE**
>
> Replacing the loop with `sum` does not protect `i32` addition from overflow.

A large enough total still exceeds the range of `i32`. Depending on the build and its
overflow-check settings, the operation can panic or wrap. The iterator version inherits
the same risk as `sum += value` in the starter.

If overflow is a valid runtime failure, fold with `checked_add` and return `Option<i32>`
or a domain-specific `Result`. If large totals are valid, use a wider integer type.
Neither change belongs in a control-flow cleanup until you have checked the callers and
the domain limits.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `flatten().sum()` implementation with the starter
> loop. The borrowed-slice alternative changes the interface and needs a separate review.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions represent absence with `Option<i32>`, and both can overflow while adding present values. |
| Readability | wins | `flatten` and `sum` state the two operations without mutable state or an empty `None` branch. |
| API design | neutral | The function still consumes `Vec<Option<i32>>` and returns an `i32` total without validation. |
| Performance | neutral | Both versions make one pass, allocate nothing, and add once for each present value. |
| Flexibility | neutral | The public interface does not change; only the internal expression of the aggregation does. |
| Error reporting | neutral | Neither version reports overflow or distinguishes empty input from input containing only `None`. |

The iterator version is easier to read when your team knows `flatten` and `sum`.
The explicit loop remains a valid choice if those methods are unfamiliar.

## Quiz

### 1. Should `None` be converted to `0` before summing?

*Answer:* It can be, but doing so creates an item for a value that is absent. Treating
`None` as zero elements produces the same total and matches the requirement to ignore
missing values. The distinction matters when later operations do more
than addition.

### 2. Should this refactor change the parameter to `&[Option<i32>]`?

*Answer:* Only after inspecting the callers. Borrowing fits a read-only calculation and
leaves the vector available, but it changes the API. The canonical solution keeps that
ownership decision separate from the iterator cleanup.

### 3. Does `sum()` solve the overflow problem?

*Answer:* No. `sum()` uses ordinary `i32` addition, so it carries the same overflow risk
as the starter. Use checked arithmetic or a wider type when the domain requires either.

## Follow-up questions

> **FOLLOW-UP**
>
> - Do callers need the vector after the total has been computed?
> - Does your domain distinguish “there are no entries” from “entries exist, but all their
>   values are missing”?
> - Should the type make overflow impossible, should the function report it, or should a
>   wider accumulator handle it?
> - For optional values, does your team find `flatten` clearer than an explicit
>   `filter_map`?

## Concepts recap

> **CONCEPT**
>
> - [`Option` as an iterator](../appendix/concept-index.md#option-as-an-iterator): `Some`
>   yields one item, while `None` yields no items.
> - [`Iterator::flatten`](../appendix/concept-index.md#iterator-flatten): join nested
>   iterators that each contain zero or one value into a single sequence.
> - [`Iterator::sum`](../appendix/concept-index.md#iterator-sum): reduce numeric items from
>   the type's additive identity.
> - [Ownership and `into_iter`](../appendix/concept-index.md#ownership-and-into-iter):
>   consume a collection to yield its items by value.
> - [`filter_map`](../appendix/concept-index.md#filter-map): transform present values and
>   discard absent results with one adapter.
> - [Checked arithmetic](../appendix/concept-index.md#checked-arithmetic): turn integer
>   overflow into a value instead of relying on overflow settings.
