# 14 · `room_occupancy`

## The problem

A booking report wants four facts about its rooms: adult and child totals, the largest
adult count in one room, and a flat list of child ages. The starter uses a small loop
for each answer. Replacing those loops with `sum`, `max`, and `flat_map` looks like a
mechanical cleanup. One accumulator puts the brakes on that approach because its
initial value affects inputs the tests never cover.

## Starter code

> **EXERCISE**
>
> Review the four functions together. Look for the operation each loop performs, then
> check whether the accumulator's initial value affects the result.

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
> Keep each public signature and the order returned by `child_ages`.

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

Four short loops would be fine here. Their repetition is still useful: it lets us put
familiar aggregate names next to the handwritten versions. `adults_total` and
`children_total` are sums. `max_adults_in_a_room` asks for a maximum. `child_ages`
turns a sequence of sequences into one sequence.

`children_total` is easy to misread. It counts entries in each child vector; it does
not add the ages stored there. The implementation gets that right. A name such as
`child_count` would make the distinction easier to spot, although renaming a public
function would affect callers and sits outside this refactor.

`max_adults_in_a_room` needs more care. Its zero initializer serves as the first
candidate and as the answer when there are no rooms. It also wins against every
negative value. Negative occupancy makes little domain sense, but the public `i32`
field allows it. Before changing the loop, we need to know whether zero is a promised
result or an accident of the implementation.

## A possible solution

```rust
{{#include ../../../solutions/14_room_occupancy.rs}}
```

The two totals translate almost word for word. We borrow the rooms, map each room to
the number we want, and sum those numbers. For children, that number is
`children.len()`. Since `Vec::len` is constant time, the function visits rooms rather
than individual children. The `as i32` conversion remains and deserves a separate
look.

`Iterator::max` returns `Option<i32>` because an iterator may be empty. The fallback
keeps the starter's result for an empty input. It does not preserve the
all-negative case. Given `[-3, -1]`, the starter returns `0` and the solution returns
`-1`.

I would accept the canonical version only if negative occupancy is invalid and that
rule is enforced or documented nearby. Otherwise, `fold(0, i32::max)` is the honest
refactor because it preserves the starter's behaviour. The tests leave both readings
open, which is why green tests cannot settle this review.

`child_ages` has a cleaner translation. `flat_map` gives each room an iterator over its
ages and presents the inner iterators as one sequence. `copied` changes each `&i32`
into an `i32`; `collect` builds the result. Room order and the order inside each room
stay intact.

## Alternative approaches

Free functions work well while these are isolated calculations. Once the same room
slice travels through a larger API, a `Rooms` newtype can put the operations next to
the data they summarize:

```rust
{{#include ../../../solutions/14_room_occupancy_alt.rs:rooms}}
```

Callers now write `rooms.adults_total()` instead of passing the same slice to several
helpers. The storage choice is private, so the type could later validate construction
or cache a summary.

None of that happens automatically. This constructor still accepts negative counts
and ages, and the methods retain the `usize as i32` conversion. The newtype supplies a
boundary. Its constructors and field visibility decide whether the boundary enforces
any rules.

The alternative is a separate runnable example:

```sh
cargo test --example 14_room_occupancy_alt
```

## Edge cases and production concerns

> **EDGE CASE**
>
> The exercise keeps its types small so we can focus on aggregation. Production code
> needs decisions the current interface avoids.

- `adults_total` can overflow `i32`.
- Converting `children.len()` with `as i32` can truncate a value larger than
  `i32::MAX`.
- Public `i32` fields permit negative counts and ages.
- Returning zero for an empty maximum makes “no rooms” indistinguishable from a room
  with zero adults.
- `child_ages` always allocates and copies every age, even if a caller only needs the
  first match.

Changing counts to unsigned integers would reject negative values, but it would not
solve overflow or define the empty maximum. Returning `Option<i32>` from the maximum
would preserve the distinction between no rooms and a real value, at the cost of
making every caller handle absence.

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
| Flexibility | wins | Filters and transformations can be added to the iterator pipelines without introducing more accumulator state. |
| Error reporting | neutral | Neither version reports invalid values, overflow, or a narrowing conversion. |

Iterator vocabulary matters to this score. A long chain can be harder to debug than a
loop. Here the chains stay short, and each adapter names one step we already identified
in the handwritten code.

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
