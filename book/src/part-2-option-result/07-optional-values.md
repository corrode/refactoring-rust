# `optional_values`

This function adds up optional integers.
Refactor it while preserving the results for missing entries, empty input, and input with no present values.
You may change how it accepts its input, provided callers still get the same totals.

Run the local tests with `cargo test --example 07_optional_values`.

## Code to refactor

```rust,editable
{{#include ../../../examples/07_optional_values.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/07_optional_values.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/07_optional_values.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>How many values should <code>Some</code> and <code>None</code> contribute?</summary>

`Some(value)` contributes one integer; `None` contributes none.
An `Option<T>` implements `IntoIterator` with exactly this behaviour.

</details>

<details class="discussion-question">
<summary>What should an empty sequence sum to?</summary>

The tests require an empty sequence to sum to zero.
After absent entries are removed, both an empty vector and a vector containing only `None` produce an empty sequence, so both return `0`.

</details>

<details class="discussion-question">
<summary>Does this function consume the vector or borrow it?</summary>

`sum_options` takes `Vec<Option<i32>>` by value, and the `for` loop consumes it.
`into_iter()` preserves that ownership behaviour.
Borrowing a slice would let callers keep the vector, but that's a separate API change.

</details>

<details class="discussion-question">
<summary>Which iterator adapter gives us just the integers that are present?</summary>

`flatten` uses each option's `IntoIterator` implementation to produce a single iterator over the present integers.
It preserves their order and skips absent values.

</details>

<details class="discussion-question">
<summary>Does using <code>sum</code> change the risk of integer overflow?</summary>

No, adding `i32` values can still overflow, just as it can in the original loop.
Depending on the build's overflow-check settings, the addition can panic or wrap.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/07_optional_values.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/07_optional_values.rs:tests}}
</script>

`into_iter()` consumes the vector and yields each `Option<i32>` by value, just as the starter loop does.

Next, `flatten()` uses each option's `IntoIterator` implementation:

- `Some(value)` contributes one `i32`.
- `None` contributes zero `i32` values.

What remains is an iterator over the present integers in their original order.
`sum()` consumes it and adds the items.
The function's `i32` return type supplies the result type that `sum` needs.

Empty input needs no special branch.
For `i32`, `Sum` uses zero as the additive identity, so an iterator with no items returns `0`.
That is why the `empty` and `all_none` tests still pass.

### Alternative approaches

`filter_map` can perform the same zero-or-one-value conversion with an identity closure:

```rust,editable
pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    options.into_iter().filter_map(|option| option).sum()
}
```

The closure returns each option unchanged.
`filter_map` extracts the value from `Some` and drops `None`.
`flatten` does the same without an identity closure.

If callers should retain the vector, a borrowed interface changes the iterator types:

```rust,editable
pub fn sum_options(options: &[Option<i32>]) -> i32 {
    options.iter().flatten().copied().sum()
}
```

Now `iter()` yields borrowed options, `flatten()` yields `&i32`, and `copied()` turns them into the owned `i32` values used by the sum.
The borrowed interface gives callers more freedom, but it is not the same ownership contract as the starter.

### Edge cases and production concerns

> **EDGE CASE**
>
> Replacing the loop with `sum` does not protect `i32` addition from overflow.

A large enough total still exceeds the range of `i32`.
Depending on the build and its overflow-check settings, the operation can panic or wrap.
The iterator version inherits the same risk as `sum += value` in the starter.

If overflow is a valid runtime failure, fold with `checked_add` and return `Option<i32>` or a domain-specific `Result`.
If large totals are valid, use a wider integer type.
Neither change belongs in a control-flow cleanup until you have checked the callers and the domain limits.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | `flatten().sum()` on owned vector | `filter_map` identity + `sum` | Borrowed slice + `flatten().copied().sum()` |
|---|---|---|---|---|
| Type-safety | Optional integers; overflow still possible | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same overflow risk | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same overflow risk | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same overflow risk |
| Readability | Mutable total, explicit absence branch | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names flattening and summation | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names filtering and summation | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names aggregation; explicit copying |
| API design | Consumes `Vec<Option<i32>>` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same consuming signature | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same consuming signature | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Borrows `&[Option<i32>]` |
| Performance | One pass; no allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> One pass; no allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> One pass; no allocation | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Avoids cloning when caller retains input |
| Flexibility | Requires owned vector | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Still requires owned vector | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Still requires owned vector | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Accepts slices, arrays, borrowed vectors |
| Error reporting | No overflow error; empty sums zero | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same unreported overflow and zero | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same unreported overflow and zero | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same unreported overflow and zero |

The borrowed version changes the ownership contract and requires callers to pass a reference.
Its clone saving applies only when callers would otherwise clone to keep the input; the iterator rewrite alone makes no speed guarantee.

</details>

## Takeaways For Our Own Codebase

> - Do callers need the vector after the total has been computed?
> - Does your domain distinguish “there are no entries” from “entries exist, but all their values are missing”?
> - Should the type make overflow impossible, should the function report it, or should a wider accumulator handle it?
> - For optional values, does your team find `flatten` clearer than an explicit `filter_map`?

## References

- [`Option` as an iterator](../appendix/concept-index.md#option-as-an-iterator): `Some` yields one item, while `None` yields no items.
- [`Iterator::flatten`](../appendix/concept-index.md#iterator-flatten): join nested iterators that each contain zero or one value into a single sequence.
- [`Iterator::sum`](../appendix/concept-index.md#iterator-sum): reduce numeric items from the type's additive identity.
- [Ownership and `into_iter`](../appendix/concept-index.md#ownership-and-into-iter): consume a collection to yield its items by value.
- [`filter_map`](../appendix/concept-index.md#filter-map): transform present values and discard absent results with one adapter.
- [Checked arithmetic](../appendix/concept-index.md#checked-arithmetic): turn integer overflow into a value instead of relying on overflow settings.
