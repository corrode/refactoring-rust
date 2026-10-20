# `room_occupancy`

These functions calculate adult and child totals, find the largest adult count in a room, and collect child ages.
Refactor one function at a time, keeping every public signature and the order returned by `child_ages`.
Check what each function returns for empty input and for the full range of values its field types allow.
Decide explicitly whether any result should change.

## Code to refactor

```rust,editable
{{#include ../../../examples/16_room_occupancy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/16_room_occupancy.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

After each change, run from the repository root:

```sh
cargo test --example 16_room_occupancy
```

</div>


## Discussion

<details class="discussion-question">
<summary>What does each loop calculate?</summary>

`adults_total` and `children_total` calculate sums.
`max_adults_in_a_room` finds a maximum, with zero as its initial value.
`child_ages` combines the child-age lists from all rooms into one list.

</details>

<details class="discussion-question">
<summary>Does <code>children_total</code> add ages or count children?</summary>

It counts children by counting entries in each child vector.
It does not add their ages.
`child_count` would make that distinction clearer, but this refactor must keep the public signatures.
Renaming the function would affect callers.

</details>

<details class="discussion-question">
<summary>What is the maximum for an empty room list?</summary>

Zero, because the loop starts at zero and has no entries to visit.
That makes “no rooms” indistinguishable from a room with zero adults.

</details>

<details class="discussion-question">
<summary>What if every adult count is negative?</summary>

It still returns zero: no negative count can replace the initial value.
Negative occupancy is not a valid room count, but callers can put negative values in the public `i32` field.
Before replacing the loop, decide whether to preserve this result or change the function's contract.

</details>

<details class="discussion-question">
<summary>Why convert <code>children.len()</code> to <code>i32</code>?</summary>

`children.len()` returns `usize`, while the total is an `i32`.
Rust does not implicitly convert between those integer types for addition.
The existing `as i32` conversion can truncate a value larger than `i32::MAX`.
Replacing the loop does not remove that risk.

</details>

<details class="discussion-question">
<summary>Does <code>child_ages</code> preserve order?</summary>

Yes, it visits rooms in order and keeps the order of ages within each room.
Keep both orders when you refactor.
`flat_map` can combine the child-age lists without changing either.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/16_room_occupancy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/16_room_occupancy.rs:tests}}
</script>

Use `map` to select a number from each borrowed room and `sum` to add the numbers.
For the child total, select `children.len()`, not the ages.
`Vec::len` takes constant time, so `children_total` visits rooms rather than individual children.
The `as i32` conversion remains, along with its risk of truncation, discussed below.

`Iterator::max` returns `Option<i32>` because the iterator may be empty.
The fallback preserves zero for empty input, but what about negative counts?
For `[-3, -1]`, the starter returns `0`, while the solution returns `-1`.

Use the canonical version only if the contract rules out negative occupancy.
Enforce that rule or document the requirement for callers.
To preserve the starter's behaviour for every `i32` value, use `fold(0, i32::max)` instead.
Both versions pass the current tests, which do not cover all-negative input.

In `child_ages`, `flat_map` visits each room's ages in turn to produce one sequence.
`copied` turns each `&i32` into an `i32`, and `collect` builds the result.
These operations preserve room order and the order of ages within each room.

You might think about grouping the four functions into a struct with methods and you're onto something.
We will revisit this idea [later](../part-5-domain-modeling/26-rooms.md). 🙃

### Edge cases and production concerns

> **EDGE CASE**
>
> The exercise focuses on aggregation.
> Production code also needs explicit rules for numeric ranges, invalid values, and empty input.

- `adults_total` can overflow `i32`.
- Converting `children.len()` with `as i32` can truncate a value larger than `i32::MAX`.
- Public `i32` fields permit negative counts and ages.
- Returning zero for an empty maximum makes “no rooms” indistinguishable from a room with zero adults.
- `child_ages` always allocates and copies every age, even when a caller only needs the first match.

Use unsigned integers to rule out negative values, but don't expect them to prevent overflow or define the maximum of an empty collection.
To distinguish “no rooms” from a real maximum, return `Option<i32>`.
Callers must then handle the absence of a maximum.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Iterator aggregates (main) |
|---|---|---|
| Type-safety | Signed counts, narrowing casts | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same types and casts |
| Readability | Mutable aggregate bookkeeping | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names each aggregate operation |
| API design | Four borrowed-slice helpers | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same signatures, changed maximum semantics |
| Performance | Linear traversals, eager age vector | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same asymptotic work and copying |
| Flexibility | Accepts any room slice | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Composable filters and transformations |
| Error reporting | No explicit validation errors | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No added error reporting |

The readability gains assume iterator familiarity.
The iterator solution returns `-1` for adult counts `[-3, -1]`, where the original returns `0`.
That changes the contract; it does not strengthen type safety.
Use `fold(0, i32::max)` to preserve the original behaviour.

</details>

## Takeaways For Our Own Codebase

> - Should an empty collection have a maximum of zero, or should the function return `Option<i32>`?
>   Test empty input and values below the loop's initial accumulator before replacing it.
> - Choose integer types that fit the allowed adult counts and child ages.
>   Check narrowing conversions and overflow separately.
> - If callers usually need all four results, could one pass produce a `RoomSummary`?
> - Does your API promise a particular order for collected values?
>   Keep tests for that order when replacing loops with iterators.
> - Which helpers repeatedly operate on the same collection?
>   Would a domain type give them a useful home?

## References

- [`map` and `sum`](../appendix/concept-index.md#map-and-sum): select a value from each item, then add the selected values.
- [`Iterator::max`](../appendix/concept-index.md#iterator-max): return `Option` because an iterator may be empty.
- [`flat_map`](../appendix/concept-index.md#flat-map): turn nested iterators into one sequence while preserving traversal order.
- [`copied`](../appendix/concept-index.md#iterator-copied): convert borrowed `Copy` values into owned values.
- [Accumulator identity values](../appendix/concept-index.md#accumulator-identity-values): an initial value can affect the result for both empty and non-empty input.
- [Newtypes](../appendix/concept-index.md#newtype): give a domain value its own methods and a place to enforce validation rules.
