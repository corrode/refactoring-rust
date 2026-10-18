# `rooms`

These four functions calculate totals, find the largest adult count, and collect child ages from a room collection.
Their iterator implementations are already concise.
Consider how to organize the related operations.
You may change the public API in this domain-modeling exercise, but keep the calculation results unchanged.

Run the starter tests with `cargo test --example 25_rooms`.

## Code to refactor

```rust,editable
{{#include ../../../examples/25_rooms.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/25_rooms.rs:tests}}
</script>

## Discussion

<details class="discussion-question">
<summary>What do the four functions have in common?</summary>

Each function reads the same kind of collection, a slice of `RoomOccupancy` values.
None needs to modify the rooms or take ownership of them.
The [aggregation chapter](../part-3-aggregations/16-room-occupancy.md) worked on the calculations inside these functions.
Here the question is where those calculations belong.

</details>

<details class="discussion-question">
<summary>Is there anything wrong with keeping free functions?</summary>

No.
Each function has a clear name and accepts any room slice.
A module can already group related functions without introducing another type.
A `Rooms` type becomes useful if you want the collection to have its own operations, rather than remain an argument passed to separate helpers.
That benefit has to justify the extra code.

</details>

<details class="discussion-question">
<summary>What changes for callers when operations become methods?</summary>

Callers need a value of the new type before they can call its methods.
Instead of passing a slice to `adults_total(&rooms)`, they call `rooms.adults_total()` on that value.
This is an API change even when every calculation returns the same result.
Here that change is part of the exercise: decide what interface the collection should offer.

</details>

<details class="discussion-question">
<summary>Does a collection type need to own its rooms?</summary>

Not for these calculations.
A wrapper around `&[RoomOccupancy]` can borrow data from a vector, an array, or another slice.
It cannot outlive that data.
An owned wrapper around `Vec<RoomOccupancy>` instead takes responsibility for keeping the collection alive.
Choose ownership based on who should hold the data, not on whether you want method syntax.

</details>

<details class="discussion-question">
<summary>Does naming the collection <code>Rooms</code> prove its contents are valid?</summary>

No.
A newtype distinguishes the collection from its underlying representation, but it only enforces the rules its construction and mutation paths uphold.
Wrapping these values without checking them still permits negative adult counts and negative child ages.
Private storage alone is not validation.

</details>

<details class="discussion-question">
<summary>Which behaviour must survive the move?</summary>

Empty input produces zero for the totals and maximum, and an empty vector of ages.
For nonempty input, `max_adults_in_a_room` returns the largest adult count, including a negative maximum if every count is negative.
This starter already uses `max().unwrap_or(0)`, unlike the original loop in the aggregation chapter.
Keep that distinction: reorganizing this code is not an invitation to change its numeric policy.
`child_ages` must also preserve room order and the order of ages within each room.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### An owned collection

```rust,editable
{{#include ../../../solutions/25_rooms.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/25_rooms.rs:tests}}
</script>

`pub struct Rooms(Vec<RoomOccupancy>)` gives the collection its own type and four methods.
The type is public, but its vector is private.
Each method reads the vector through `self.0` and performs the same iterator calculation as before.

Callers move a vector into `Rooms::new` and then write `rooms.adults_total()`, `rooms.children_total()`, `rooms.max_adults_in_a_room()`, or `rooms.child_ages()`.
The constructor takes ownership of the existing vector without cloning its elements.
The methods borrow `&self`, so calculating a total does not consume the collection.

This version changes the public API.
A caller can no longer pass an arbitrary slice to a helper; it must supply an owned vector when constructing `Rooms`.
If the caller needs to retain its original data, it must arrange separate ownership, copy the data as appropriate, or keep a borrowed API instead.
An owned `Rooms` can outlive the variable that supplied the vector because the vector has moved into it.


Private vector storage also limits what callers can do after construction.
They get the four methods, not automatic access to every `Vec` operation.
That is useful when you want a deliberately small collection interface, but restrictive when callers need arbitrary slicing or mutation.
Add operations when the domain needs them rather than exposing the vector just to make every old call compile.

If callers only need a temporary view, a borrowed `Rooms<'a>(&'a [RoomOccupancy])` is another option.
It can wrap any room slice without allocating or copying, but cannot outlive its source.
Method syntax does not require ownership.

### What the wrapper does not guarantee

The solution does not check occupancy values.
`Rooms::new` accepts the vector it receives; a successfully constructed `Rooms` is not proof that its contents describe a valid booking.
A validated collection would need checks on every construction path and on any later operation that could break its rules.
That would be a separate contract change.

The arithmetic is unchanged too.
Adult totals can overflow `i32`, and child totals retain the narrowing `usize as i32` conversion and possible addition overflow.
Returning zero for an empty maximum still does not distinguish “no rooms” from a room with zero adults.
Moving these calculations onto a type does not settle those policies.

Nor does it combine the traversals.
Asking for all four results still runs four separate calculations, and `child_ages` still builds an owned vector by copying the ages in order.
The collection does not cache a summary.
If repeated traversal is the problem, consider a summary calculation separately from this organizational refactor.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Owned `Rooms` |
|---|---|---|
| Type-safety | Signed values, no validation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Distinct type, no validity guarantee |
| Readability | Four short iterator functions | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Collection operations discoverable as methods |
| API design | Public slice-based functions | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Collection interface; caller migration required |
| Performance | Separate scans, eager age vector | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same scans; constructor moves vector |
| Flexibility | Accepts any room slice | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> Requires ownership, limits collection access |
| Error reporting | No validation errors | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Constructor accepts unchecked data |

The owned API's advantages assume callers benefit from holding a named collection with a restricted interface.
They are not a reason to require ownership for a calculation that only reads a slice.
The newtype adds a constructor and an `impl`; keep the free functions if a named collection does not earn that extra structure.

</details>

## Takeaways For Our Own Codebase

> - Which functions repeatedly take the same collection, and would methods make them easier to find?
>   Compare a named type with the cheaper option of keeping the functions in one module.
> - Who should own the data?
>   Don't require a vector where a borrowed slice meets the caller's needs.
> - Which collection operations should callers have access to, and which should remain private?
> - What does successful construction of a domain type actually guarantee?
>   Check every construction and mutation path before treating the type as proof of validity.
> - Keep API migration, validation, and aggregation changes separate so each can be reviewed on its own terms.

## References

- [Newtypes](../appendix/concept-index.md#newtype): give a collection a distinct type and its own operations without assuming validation.
- [Room occupancy](../part-3-aggregations/16-room-occupancy.md): the iterator calculations and the difference between an empty-input fallback and an initial accumulator.
- [Method syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html): associate operations with a type and borrow the receiver with `&self`.
- [References and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html): read data without taking ownership.
- [Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html): expose a collection's operations while keeping its storage private.
