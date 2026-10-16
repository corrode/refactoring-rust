# `room_occupancy`

These functions calculate adult and child totals, find the largest adult count in a room, and collect child ages.
Refactor one function at a time, keeping every public signature and the order returned by `child_ages`.
Check behaviour for empty input and the full range of values the field types allow before deciding to change any results.

Run the tests after each change with `cargo test --example 15_room_occupancy`.

## Code to refactor

```rust,editable
{{#include ../../../examples/15_room_occupancy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/15_room_occupancy.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/15_room_occupancy.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What does each loop calculate?</summary>

Each loop performs a common operation.
`adults_total` and `children_total` calculate sums.
`max_adults_in_a_room` finds a maximum, with zero as its initial value.
`child_ages` combines the child-age lists from all rooms into one list.

</details>

<details class="discussion-question">
<summary>Does <code>children_total</code> add ages or count children?</summary>

`children_total` counts entries in each child vector; it does not add the ages stored there.
`child_count` would make that distinction clearer, but this refactor must keep the public signatures.
Renaming the function would affect callers.

</details>

<details class="discussion-question">
<summary>What is the maximum for an empty room list?</summary>

The maximum loop starts at zero, so it returns zero for an empty room list.
That makes “no rooms” indistinguishable from a room with zero adults.

</details>

<details class="discussion-question">
<summary>What if every adult count is negative?</summary>

The loop also returns zero for a list containing only negative counts.
Negative occupancy is not a valid room count, but the public `i32` field allows it.
Before replacing the loop, decide whether to preserve this result or change the function's contract.

</details>

<details class="discussion-question">
<summary>Why convert <code>children.len()</code> to <code>i32</code>?</summary>

`children.len()` returns `usize`, while the total is an `i32`.
Rust does not implicitly convert between those integer types for addition.
The existing `as i32` conversion can truncate a value larger than `i32::MAX`; replacing the loop does not remove that risk.

</details>

<details class="discussion-question">
<summary>Does <code>child_ages</code> preserve order?</summary>

Yes, it visits rooms in order and keeps the order of ages within each room.
`flat_map` can combine the child-age lists in that same order.
The refactor must preserve both.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/15_room_occupancy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/15_room_occupancy.rs:tests}}
</script>

The two totals use `map` to select a number from each borrowed room, then `sum` to add those numbers.
For children, the number is `children.len()`.
`Vec::len` takes constant time, so `children_total` visits rooms rather than individual children.
The `as i32` conversion remains, along with its risk of truncation, discussed below.

`Iterator::max` returns `Option<i32>` because the iterator may be empty.
The fallback keeps the starter's answer for an empty input.
It does not preserve every result.
For `[-3, -1]`, the starter returns `0`, while the solution returns `-1`.

Use the canonical version only if the contract rules out negative occupancy.
Enforce that rule or document the requirement for callers.
To preserve the starter's behaviour for every `i32` value, use `fold(0, i32::max)` instead.
Both versions pass the current tests, which do not cover all-negative input.

In `child_ages`, `flat_map` iterates over each room's ages in turn, producing one sequence.
`copied` turns each `&i32` into an `i32`, and `collect` builds the result.
Room order and the order within each room remain unchanged.

### Alternative approaches

Free functions work well for independent calculations.
If several parts of an API use the same room collection, a `Rooms` newtype can group the data and its operations:

```rust,editable
{{#include ../../../solutions/15_room_occupancy_alt.rs:rooms}}
```

Callers can write `rooms.adults_total()` instead of passing the same slice to several helpers.
Private storage also allows the type to validate data during construction or cache a summary later.

The wrapper does not reject invalid values on its own.
This constructor still accepts negative counts and ages, and its methods retain the `usize as i32` conversion.
To enforce valid data, the type needs constructors that check values and private fields that prevent callers from bypassing those checks.

Run the alternative as a separate example:

```sh
cargo test --example 15_room_occupancy_alt
```

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

Unsigned integers rule out negative values, but they do not prevent overflow or define the maximum of an empty collection.
Returning `Option<i32>` from the maximum keeps “no rooms” distinct from a real value.
Callers must then handle the absence of a maximum.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Iterator aggregates (main) | `Rooms` methods |
|---|---|---|---|
| Type-safety | Signed counts, narrowing casts | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same types and casts | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No validation, same casts |
| Readability | Mutable aggregate bookkeeping | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names each aggregate operation | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Named aggregates grouped as methods |
| API design | Four borrowed-slice helpers | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same signatures, changed maximum semantics | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Groups operations, hides storage |
| Performance | Linear traversals, eager age vector | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same asymptotic work and copying | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same traversals; moves input vector |
| Flexibility | Accepts any room slice | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Composable filters and transformations | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> Requires owned vector, not slice |
| Error reporting | No explicit validation errors | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No added error reporting | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No added error reporting |

The readability gains assume iterator familiarity.
Both shown solutions return `-1` for adult counts `[-3, -1]`, where the original returns `0`: a contract change, not stronger type safety.
Use `fold(0, i32::max)` to preserve that behaviour.
`Rooms` provides a place for future validation, but currently accepts negative values.

</details>

## Takeaways For Our Own Codebase

> - Should an empty collection have a maximum of zero, or should the function return `Option<i32>`?
> - Which integer types fit the domain rules for adult counts and child ages?
> - If callers usually need all four results, should one pass produce a `RoomSummary`?
> - Is the order returned by `child_ages` part of the public contract?
> - Which collection helpers in your codebase belong on a missing domain type?

## References

- [`map` and `sum`](../appendix/concept-index.md#map-and-sum): select a value from each item, then add the selected values.
- [`Iterator::max`](../appendix/concept-index.md#iterator-max): return `Option` because an iterator may be empty.
- [`flat_map`](../appendix/concept-index.md#flat-map): turn nested iterators into one sequence while preserving traversal order.
- [`copied`](../appendix/concept-index.md#iterator-copied): convert borrowed `Copy` values into owned values.
- [Accumulator identity values](../appendix/concept-index.md#accumulator-identity-values): an initial value can affect the result for both empty and non-empty input.
- [Newtypes](../appendix/concept-index.md#newtype): give a domain value its own methods and a place to enforce validation rules.
