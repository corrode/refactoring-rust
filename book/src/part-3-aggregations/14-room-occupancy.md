# 14 · `room_occupancy`

## The problem

A booking report needs four facts: the total number of adults, the total number of
children, the largest adult count in a room, and one flat list of child ages. The
starter has a small loop for each result. Those loops are easy to replace with `sum`,
`max`, and `flat_map`.

One replacement is not mechanically equivalent. The maximum starts at zero, and that
initial value decides the result for inputs the tests do not cover. The iterator cleanup
is useful, but only after we settle which behaviour belongs to the contract.

## Starter code

> **EXERCISE**
>
> Review the four functions together. Name the operation in each loop, then check where
> an accumulator's initial value can change the answer.

```rust,editable
{{#include ../../../examples/14_room_occupancy.rs:listing}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/14_room_occupancy.rs`:
>
> ```sh
> cargo test --example 14_room_occupancy
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

Four short loops are not a problem by themselves. Their repetition gives us a clean
comparison between handwritten accumulation and Rust's iterator vocabulary.
`adults_total` and `children_total` calculate sums. `max_adults_in_a_room` finds a
maximum. `child_ages` turns a sequence of sequences into one sequence.

`children_total` is easy to misread. It counts entries in each child vector; it does not
add the ages stored there. The implementation is correct. `child_count` would state the
intent more directly, but this refactor must keep the public signatures, and a rename
would affect callers.

The maximum is where the code pushes back. In the starter, zero is the first candidate
and the answer for an empty room list. It also wins against every negative value.
Negative occupancy makes little sense for the domain, but the public `i32` field permits
it. Before replacing the loop, decide whether zero is part of the contract or an
accident of the accumulator.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/14_room_occupancy.rs:listing}}
```

The two totals translate directly. Borrow the rooms, map each room to the number the
report needs, then sum those numbers. For children, that number is `children.len()`.
`Vec::len` takes constant time, so this function visits rooms rather than individual
children. The `as i32` conversion is still there. We will examine its risk below.

`Iterator::max` returns `Option<i32>` because the iterator may be empty. The fallback
keeps the starter's answer for an empty input. It does not preserve every result. For
`[-3, -1]`, the starter returns `0`, while the solution returns `-1`.

I would accept the canonical version only when negative occupancy is invalid and that
rule is enforced or documented nearby. Otherwise, `fold(0, i32::max)` is the honest
refactor because it retains the starter's behaviour. Both interpretations pass the
current tests. A green test run cannot make the domain decision for us.

`child_ages` has no such ambiguity. `flat_map` gives each room an iterator over its ages
and exposes those inner iterators as one sequence. `copied` turns each `&i32` into an
`i32`, and `collect` builds the result. Room order and the order within each room remain
unchanged.

</details>

## Alternative approaches

Free functions are a good fit for four isolated calculations. If the same room slice
keeps passing through a larger API, a `Rooms` newtype can keep those operations beside
the data they summarize:

```rust,editable
{{#include ../../../solutions/14_room_occupancy_alt.rs:rooms}}
```

Callers can write `rooms.adults_total()` instead of passing the same slice to several
helpers. Private storage also leaves room for controlled construction or a cached
summary later.

The wrapper does not reject invalid values on its own. This constructor still accepts
negative counts and ages, and its methods retain the `usize as i32` conversion. A
newtype gives you a boundary. The constructors and field visibility decide whether that
boundary rejects invalid data.

Run the alternative as a separate example:

```sh
cargo test --example 14_room_occupancy_alt
```

## Edge cases and production concerns

> **EDGE CASE**
>
> The exercise uses small types so the aggregation stays visible. Production code still
> needs explicit rules for numeric ranges, invalid values, and empty input.

- `adults_total` can overflow `i32`.
- Converting `children.len()` with `as i32` can truncate a value larger than
  `i32::MAX`.
- Public `i32` fields permit negative counts and ages.
- Returning zero for an empty maximum makes “no rooms” indistinguishable from a room
  with zero adults.
- `child_ages` always allocates and copies every age, even when a caller only needs the
  first match.

Would unsigned integers fix the model? They would rule out negative values, but they
would not prevent overflow or define the maximum of an empty collection. Returning
`Option<i32>` from the maximum keeps “no rooms” distinct from a real value. That choice
is visible at every call site because callers must handle absence.

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
than a loop, especially on a team that rarely uses adapters. These chains are short,
and each adapter names an operation already present in the handwritten code. I would
use them here.

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
>   an initial value can affect both empty input and valid-looking edge cases.
> - [Newtypes](../appendix/concept-index.md#newtype): give a domain value its own
>   methods and a boundary for future invariants.
