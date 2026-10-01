# 14 · `room_occupancy`

## The problem

A booking report needs four facts about its rooms: the total numbers of adults and
children, the largest adult count in one room, and a flat list of child ages. The
starter computes each result with a small loop. You can replace those loops with
`sum`, `max`, and `flat_map`, but one accumulator needs closer inspection. Its initial
value changes the result for inputs the tests never cover.

## Starter code

> **EXERCISE**
>
> Review the four functions together. Identify the operation performed by each loop,
> then check whether an accumulator's initial value can affect the answer.

```rust
{{#include ../../../examples/14_room_occupancy.rs}}
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
> - Which loops compute a sum, which computes a maximum, and which joins nested
>   sequences?
> - Does `children_total` sum ages or count entries?
> - What does `max_adults_in_a_room` return for an empty room list?
> - What happens if every room has a negative adult count?
> - Why does `children.len()` need a conversion before adding it to `i32`?
> - Does `child_ages` preserve both room order and the order within each room?

There is nothing wrong with four short loops here. Their repetition gives us a useful
comparison between handwritten accumulation and Rust's iterator vocabulary.
`adults_total` and `children_total` compute sums. `max_adults_in_a_room` finds a
maximum. `child_ages` turns a sequence of sequences into one sequence.

The name `children_total` can send you in the wrong direction. The function counts
entries in each child vector; it does not add the ages stored in that vector. The
implementation is correct. A name such as `child_count` would state the intent more
directly, but renaming a public function would affect callers and falls outside this
refactor.

The maximum deserves more scrutiny. In the starter, zero is both the first candidate
and the answer when there are no rooms. It also beats every negative value. Negative
occupancy makes little domain sense, yet the public `i32` field allows it. Before you
replace the loop, ask whether returning zero is part of the contract or an accident of
the implementation.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/14_room_occupancy.rs}}
```

The two totals translate almost word for word. We borrow the rooms, map each room to
the number we need, then sum those numbers. For children, the number is
`children.len()`. Because `Vec::len` takes constant time, the function visits rooms
rather than individual children. The `as i32` conversion remains, and we'll return to
its risks below.

`Iterator::max` returns `Option<i32>` because an iterator can be empty. The fallback
preserves the starter's answer for an empty input. Does it preserve every result? No.
Given `[-3, -1]`, the starter returns `0`, while the solution returns `-1`.

I would accept the canonical version only if negative occupancy is invalid and that
rule is enforced or documented nearby. Otherwise, `fold(0, i32::max)` is the honest
refactor because it preserves the starter's behaviour. The tests support either
interpretation, so a green test run cannot settle this review for us.

`child_ages` has a more direct translation. `flat_map` gives each room an iterator over
its ages and presents all those inner iterators as one sequence. `copied` turns each
`&i32` into an `i32`, then `collect` builds the result. Room order and the order within
each room remain intact.

</details>

## Alternative approaches

Free functions fit these isolated calculations well. If the same room slice starts
moving through a larger API, a `Rooms` newtype can keep the operations beside the data
they summarize:

```rust
{{#include ../../../solutions/14_room_occupancy_alt.rs:rooms}}
```

Callers can write `rooms.adults_total()` instead of passing the same slice to several
helpers. Because the storage is private, the type could later validate construction or
cache a summary.

The wrapper does not enforce those rules by itself. This constructor still accepts
negative counts and ages, and the methods keep the `usize as i32` conversion. A newtype
gives you a boundary. Its constructors and field visibility determine whether that
boundary rejects invalid data.

Run the alternative as a separate example:

```sh
cargo test --example 14_room_occupancy_alt
```

## Edge cases and production concerns

> **EDGE CASE**
>
> The exercise uses small types so we can focus on aggregation. Production code still
> needs explicit decisions about ranges, invalid values, and empty input.

- `adults_total` can overflow `i32`.
- Converting `children.len()` with `as i32` can truncate a value larger than
  `i32::MAX`.
- Public `i32` fields permit negative counts and ages.
- Returning zero for an empty maximum makes “no rooms” indistinguishable from a room
  with zero adults.
- `child_ages` always allocates and copies every age, even if a caller only needs the
  first match.

Would unsigned integers fix the model? They would reject negative values, but they
would not prevent overflow or define the result of an empty maximum. Returning
`Option<i32>` from the maximum would preserve the difference between no rooms and a
real value. The trade-off is visible at every call site, where callers must now handle
absence.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical iterator implementation with the starter loops.
> The `Rooms` newtype is a separate API design.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | The types and signatures stay the same, so negative values and narrowing conversions remain possible. |
| Readability | wins | `sum`, `max`, and `flat_map` name the aggregate operations and remove mutable bookkeeping. |
| API design | neutral | The four free-function signatures do not change, including the zero default and eager `Vec<i32>` result. |
| Performance | neutral | Both versions perform the same asymptotic work and allocate only the returned age vector. |
| Flexibility | wins | You can add filters and transformations to the iterator pipelines without introducing more accumulator state. |
| Error reporting | neutral | Neither version reports invalid values, overflow, or a narrowing conversion. |

Your team's familiarity with iterators affects the readability score. A long chain can
be harder to debug than a loop. These chains stay short, and each adapter names one
step we already found in the handwritten code.

## Quiz

### 1. Does `max().unwrap_or(0)` preserve the starter's behaviour?

*Answer:* It preserves the empty-slice result, but it changes the result for an
all-negative input. Approve it only after establishing that negative adult counts are
invalid, or use `fold(0, i32::max)` to retain the starter's full behaviour.

### 2. Should `children_total` flatten every child vector and call `count()`?

*Answer:* No. Summing `children.len()` visits each room and reads a constant-time
length. Flattening and counting visits every child. The current approach expresses the
same result with less work.

### 3. Should `child_ages` return `Vec<i32>` or an iterator?

*Answer:* Keep `Vec<i32>` when callers need an owned snapshot or repeated traversal.
Return `impl Iterator<Item = i32> + '_` when callers usually consume the ages once and
could benefit from lazy filtering or early termination. That choice changes the API.

### 4. Does `Rooms(Vec<RoomOccupancy>)` enforce valid occupancy data?

*Answer:* No. It groups the operations and provides a place for validation. It only
enforces rules when construction is controlled and invalid values are rejected.

## Follow-up questions

> **FOLLOW-UP**
>
> - Should an empty collection have a maximum of zero, or should the function return
>   `Option<i32>`?
> - Which integer types match the domain rules for adult counts and child ages?
> - If callers usually need all four results, should one pass produce a `RoomSummary`?
> - Is the order returned by `child_ages` part of the public contract?
> - Which collection helpers in your codebase are really methods on a missing domain
>   type?

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
