# `optional_values`

This function adds up optional integers.
Can you express that more directly?
Keep the same totals, including for missing entries, empty input, and input with no present values.
Keep the function signature for your first solution.
Explore different input types as an alternative afterward.

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

Zero, as the tests require.
Neither an empty vector nor a vector containing only `None` contributes any integers, so both return `0`.

</details>

<details class="discussion-question">
<summary>Does this function consume the vector or borrow it?</summary>

It consumes the vector.
`sum_options` takes `Vec<Option<i32>>` by value, and the `for` loop takes its items by value.
`into_iter()` keeps that ownership behaviour.
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

You now have an iterator over the present integers in their original order.
`sum()` consumes that iterator and adds the integers.
The function's `i32` return type tells `sum` which result type to produce.

Do you need a branch for empty input?
No, `Sum` for `i32` starts at zero, the additive identity.
With no items to add, it returns `0`, so the `empty` and `all_none` tests still pass.

### Alternative approaches

You can also use `filter_map` with a closure that returns each option unchanged:

```rust,editable
pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    options.into_iter().filter_map(|option| option).sum()
}
```

The closure returns each option unchanged.
`filter_map` extracts the value from `Some` and drops `None`.
`flatten` does the same without an identity closure.

If callers need to keep the vector, accept a borrowed slice:

```rust,editable
pub fn sum_options(options: &[Option<i32>]) -> i32 {
    options.iter().flatten().copied().sum()
}
```

Here, `iter()` yields borrowed options and `flatten()` yields `&i32` values.
`copied()` copies those integers into the `i32` values that `sum` adds.
Callers can now keep their input, but you've changed the starter's ownership contract.

### Edge cases and production concerns

> **EDGE CASE**
>
> Replacing the loop with `sum` does not protect `i32` addition from overflow.

The total can still exceed the range of `i32`.
Depending on the build's overflow-check settings, addition can panic or wrap, just like `sum += value` in the starter.

If overflow is a valid runtime failure, fold with `checked_add` and return `Option<i32>` or a domain-specific `Result`.
If large totals are valid, use a wider integer type.
Check your callers and domain limits before making either change as part of a control-flow cleanup.

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

Callers must pass a reference to use the borrowed version.
That saves a clone only if they would otherwise clone the vector to keep the input.
Replacing the loop with an iterator chain doesn't guarantee a speed-up.

</details>

## Takeaways For Our Own Codebase

> - Do callers need the vector after computing the total?
>   If so, consider borrowing a slice rather than making them clone it.
> - Does your domain distinguish empty input from input containing only missing values?
>   Returning zero for both loses that distinction.
> - Check the largest and smallest totals your domain allows.
>   Can your integer type represent them, or do you need checked arithmetic and an error result?
> - Compare `flatten` with `filter_map` where the closure only returns its argument.
>   Keep the version your team finds easier to read.

## References

- [`Option` as an iterator](../appendix/concept-index.md#option-as-an-iterator): `Some` yields one item, while `None` yields no items.
- [`Iterator::flatten`](../appendix/concept-index.md#iterator-flatten): join nested iterators that each contain zero or one value into a single sequence.
- [`Iterator::sum`](../appendix/concept-index.md#iterator-sum): reduce numeric items from the type's additive identity.
- [Ownership and `into_iter`](../appendix/concept-index.md#ownership-and-into-iter): consume a collection to yield its items by value.
- [`filter_map`](../appendix/concept-index.md#filter-map): transform present values and discard absent results with one adapter.
- [Checked arithmetic](../appendix/concept-index.md#checked-arithmetic): turn integer overflow into a value instead of relying on overflow settings.
