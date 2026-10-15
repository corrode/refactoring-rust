# 15 · `room_occupancy`

## The problem

The maximum loop starts at zero, so it returns zero even when every adult count
is negative. Decide whether to preserve that behavior or reject negative counts
before replacing the loop.

## Starter code

> **EXERCISE**
>
> Refactor one function at a time and run its tests after each change.

```rust,editable
{{#include ../../../examples/15_room_occupancy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/15_room_occupancy.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/15_room_occupancy.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/15_room_occupancy.rs`:
>
> ```sh
> cargo test --example 15_room_occupancy
> ```
>
> Keep every public signature and preserve the order returned by `child_ages`.

## Review

> **REVIEW**
>
> - Which loops compute a sum, which one computes a maximum, and which one joins nested
>   sequences?
> - Does `children_total` add ages or count entries?
> - What does `max_adults_in_a_room` return for an empty room list?
> - What happens when every room has a negative adult count?
> - Why does `children.len()` need a conversion before you can add it to an `i32`?
> - Does `child_ages` preserve both room order and the order within each room?

Each loop performs a common operation. `adults_total` and `children_total` calculate
sums. `max_adults_in_a_room` finds a maximum, with zero as its initial value.
`child_ages` combines the child-age lists from all rooms into one list.

`children_total` counts entries in each child vector; it does not add the ages stored
there. `child_count` would make that distinction clearer, but this refactor must keep
the public signatures. Renaming the function would affect callers.

The maximum loop starts at zero, so it returns zero for both an empty room list and a
list containing only negative counts. Negative occupancy is not a valid room count,
but the public `i32` field allows it. Before replacing the loop, decide whether to
preserve this result or change the function's contract.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/15_room_occupancy.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/15_room_occupancy.rs:tests}}
</script>

The two totals use `map` to select a number from each borrowed room, then `sum` to add
those numbers. For children, the number is `children.len()`. `Vec::len` takes constant
time, so `children_total` visits rooms rather than individual children. The `as i32`
conversion remains, along with its risk of truncation, discussed below.

`Iterator::max` returns `Option<i32>` because the iterator may be empty. The fallback
keeps the starter's answer for an empty input. It does not preserve every result. For
`[-3, -1]`, the starter returns `0`, while the solution returns `-1`.

Use the canonical version only if the contract rules out negative occupancy. Enforce
that rule or document the requirement for callers. To preserve the starter's behaviour
for every `i32` value, use `fold(0, i32::max)` instead. Both versions pass the current
tests, which do not cover all-negative input.

In `child_ages`, `flat_map` iterates over each room's ages in turn, producing one
sequence. `copied` turns each `&i32` into an `i32`, and `collect` builds the result.
Room order and the order within each room remain unchanged.

</details>

## Alternative approaches

Free functions work well for independent calculations. If several parts of an API use
the same room collection, a `Rooms` newtype can group the data and its operations:

```rust,editable
{{#include ../../../solutions/15_room_occupancy_alt.rs:rooms}}
```

Callers can write `rooms.adults_total()` instead of passing the same slice to several
helpers. Private storage also allows the type to validate data during construction or
cache a summary later.

The wrapper does not reject invalid values on its own. This constructor still accepts
negative counts and ages, and its methods retain the `usize as i32` conversion. To
enforce valid data, the type needs constructors that check values and private fields
that prevent callers from bypassing those checks.

Run the alternative as a separate example:

```sh
cargo test --example 15_room_occupancy_alt
```

## Edge cases and production concerns

> **EDGE CASE**
>
> The exercise focuses on aggregation. Production code also needs explicit rules for
> numeric ranges, invalid values, and empty input.

- `adults_total` can overflow `i32`.
- Converting `children.len()` with `as i32` can truncate a value larger than
  `i32::MAX`.
- Public `i32` fields permit negative counts and ages.
- Returning zero for an empty maximum makes “no rooms” indistinguishable from a room
  with zero adults.
- `child_ages` always allocates and copies every age, even when a caller only needs the
  first match.

Unsigned integers rule out negative values, but they do not prevent overflow or define
the maximum of an empty collection. Returning `Option<i32>` from the maximum keeps
“no rooms” distinct from a real value. Callers must then handle the absence of a maximum.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical iterator implementation with the starter loops.
> The `Rooms` newtype is a separate API design.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | The types and signatures are unchanged, so negative values and narrowing conversions remain possible. |
| Readability | wins | `sum`, `max`, and `flat_map` name the aggregate operations and remove mutable bookkeeping. |
| API design | neutral | The four free-function signatures stay in place, including the zero default and eager `Vec<i32>` result. |
| Performance | neutral | Both versions do the same asymptotic work and allocate only the returned age vector. |
| Flexibility | wins | Filters and transformations can join the iterator pipelines without adding accumulator state. |
| Error reporting | neutral | Neither version reports invalid values, overflow, or a narrowing conversion. |

Iterator familiarity affects the readability score. A long chain can be harder to debug
than a loop, especially on a team that rarely uses iterator methods. These chains are
short, and each method names an operation already present in the loops.

## Quiz

### 1. Does `max().unwrap_or(0)` preserve the starter's behaviour?

*Answer:* It preserves the empty-slice result, but changes the result for an all-negative
input. Approve it only after establishing that negative adult counts are invalid. Use
`fold(0, i32::max)` when you need to retain the starter's full behaviour.

### 2. Should `children_total` flatten every child vector and call `count()`?

*Answer:* No. Summing `children.len()` visits each room and reads a constant-time
length. Flattening and counting visits every child. The current approach produces the
same result with less work.

### 3. Should `child_ages` return `Vec<i32>` or an iterator?

*Answer:* Keep `Vec<i32>` when callers need an owned snapshot or repeated traversal.
Return `impl Iterator<Item = i32> + '_` when callers usually consume the ages once and
would benefit from lazy filtering or early termination. That decision changes the API.

### 4. Does `Rooms(Vec<RoomOccupancy>)` enforce valid occupancy data?

*Answer:* No. It groups the operations and provides a place for validation. It enforces
rules only when construction is controlled and invalid values are rejected.

## Follow-up questions

> **FOLLOW-UP**
>
> - Should an empty collection have a maximum of zero, or should the function return
>   `Option<i32>`?
> - Which integer types fit the domain rules for adult counts and child ages?
> - If callers usually need all four results, should one pass produce a `RoomSummary`?
> - Is the order returned by `child_ages` part of the public contract?
> - Which collection helpers in your codebase belong on a missing domain type?

## Concepts recap

> **CONCEPT**
>
> - [`map` and `sum`](../appendix/concept-index.md#map-and-sum): select a value from
>   each item, then add the selected values.
> - [`Iterator::max`](../appendix/concept-index.md#iterator-max): return `Option`
>   because an iterator may be empty.
> - [`flat_map`](../appendix/concept-index.md#flat-map): turn nested iterators into one
>   sequence while preserving traversal order.
> - [`copied`](../appendix/concept-index.md#iterator-copied): convert borrowed `Copy`
>   values into owned values.
> - [Accumulator identity values](../appendix/concept-index.md#accumulator-identity-values):
>   an initial value can affect the result for both empty and non-empty input.
> - [Newtypes](../appendix/concept-index.md#newtype): give a domain value its own
>   methods and a place to enforce validation rules.
