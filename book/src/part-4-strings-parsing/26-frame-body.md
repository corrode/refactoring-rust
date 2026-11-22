# `frame_body`

> **Draft outline.** The exercise and solution are runnable; the bullet notes are a reference for the chapter's prose.

## Code to refactor

```rust,editable
{{#include ../../../examples/26_frame_body.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/26_frame_body.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 26_frame_body
```

</div>

This function recognizes a complete buffer containing an opening `0xA5` byte, a kind byte in `1..=3`, any number of payload bytes, and a closing `0x5A` byte.
It returns the kind and a borrowed payload, or `None` if the buffer does not match that format.
Make the implementation easier to follow while preserving its signature, accepted inputs, and allocation-free behaviour.
The starter is correct; this exercise does not require a bug fix.

## Discussion

<details class="discussion-question">
<summary>What makes the starter's indices valid?</summary>

- The length check establishes room for the opening marker, kind, and closing marker before any indexing or subtraction.
- With at least three bytes, `last` is at least two, so `frame[2..last]` is a valid slice even when it is empty.
- The checks are correct, but understanding the final slice requires keeping those earlier conditions in mind.
  Look for a way to express the required layout together with the values extracted from it.

</details>

<details class="discussion-question">
<summary>Where must the closing marker appear?</summary>

- At the end of the supplied buffer, not at the first occurrence of `0x5A`.
  Both marker values are allowed inside the payload.
- `[0xA5, 2, 0x5A]` has an empty payload.
  `[0xA5, 2, 0x5A, 10, 0x5A]` has the payload `[0x5A, 10]`.
- These rules describe a buffer whose boundaries are already known.
  Searching for the first closing marker would change the accepted payloads.

</details>

<details class="discussion-question">
<summary>Does returning the payload copy its bytes?</summary>

- No, the starter already returns a slice into the input.
  The output contains a copied kind byte and a shared view of the payload, not a new payload buffer.
- The returned slice cannot outlive the input storage.
  Lifetime elision ties it to the function's only borrowed input.
- The payload need not be UTF-8 text.
  No conversion to a string or collection is needed in either implementation.

</details>

## Solution

Both versions recognize complete frames; compare how clearly they express the format and the borrowed payload contract.

<details class="solution" id="solutions">
<summary>Show solution</summary>

### Check the layout and bind its parts together

```rust,editable
{{#include ../../../solutions/26_frame_body.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/26_frame_body.rs:tests}}
</script>

- The literal patterns check the first and last bytes.
  The pattern also requires at least three bytes before the arm can run.
- `kind @ 1..=3` both checks the inclusive range and binds the matching byte.
  Unlike a match guard, this condition is part of the pattern itself.
- `body @ ..` binds the middle slice, including an empty middle.
  The trailing literal reserves the final byte; `..` does not search the payload for a delimiter.
- Matching a borrowed slice binds `kind` as `&u8` and `body` as `&[u8]`.
  `*kind` copies one byte into the result; the payload remains borrowed.
- The fallback returns `None` for every invalid length, marker, or kind.
  All validation still happens at runtime, and the returned kind still has type `u8`, not a new restricted type.

### Preserve the contract

- The signature and caller-facing behaviour stay unchanged.
  Both versions are safe, allocation-free Rust, so this refactor removes index bookkeeping rather than repairing memory unsafety or eliminating an allocation.
- Both implementations run the same eight tests: a valid frame, supported kinds with empty payloads, short inputs, wrong markers, unsupported kinds, arbitrary payload bytes, a borrowed payload, and whole-buffer interpretation.
- The borrowing test checks that the returned payload starts at the corresponding address inside the input, not just that its contents are equal.
  The marker-byte and empty-payload tests protect against changing the format while rewriting its checks.
- This function does not find frame boundaries in a byte stream, distinguish incomplete input from invalid input, or validate payload contents.
  Concatenating two valid frames can produce another accepted buffer because the internal markers become payload bytes.
  A streaming protocol needs a separate boundary rule, such as a length field or escaping.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.

| Criterion | Original | Slice pattern |
|---|---|---|
| Type-safety | Length guard makes checked indexing valid | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Length and values checked together at runtime |
| Readability | Separate checks and slice-boundary arithmetic | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> One pattern describes the required layout |
| API design | Borrowed input and payload; `Option` result | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same signature and lifetime relationship |
| Performance | No allocation or payload copying | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Still no allocation or payload copying |
| Flexibility | Fixed markers and kind range | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same format and arbitrary payload bytes |
| Error reporting | `None` for every rejected buffer | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same undifferentiated failure |

Readability depends on familiarity with `@` and rest patterns.
Neither version scans the payload, and fewer source lines do not establish a measured speed-up.
Keep the explicit checks if they communicate the format better to your team.

</details>

## Takeaways For Our Own Codebase

> - Which length checks and index calculations describe a fixed layout that a pattern could express directly?
> - Test an empty middle and marker bytes inside the payload, not only ordinary nonempty data.
> - Distinguish recognizing a complete buffer from discovering boundaries in a stream.

## References

- [Slice patterns](../appendix/concept-index.md#slice-patterns) and [at-bindings](../appendix/concept-index.md#at-bindings): check a layout while naming its parts.
- [Rust Reference: slice patterns](https://doc.rust-lang.org/reference/patterns.html#slice-patterns) and [rest patterns](https://doc.rust-lang.org/reference/patterns.html#rest-patterns): fixed elements around a variable-length middle.
- [Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html#lifetime-elision-in-functions): the returned payload borrows from the input.
- [person_info](../part-1-warmups/04-person-info.md): combine conditions and borrowed bindings in a struct pattern.
- [quicksort](27-quicksort.md): reuse a slice pattern to separate a pivot from the remaining elements.
