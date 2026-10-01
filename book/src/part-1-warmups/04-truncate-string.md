# 04 · `truncate_string`

## The problem

`truncate("café", 4)` panics. That is the awkward case hiding behind an otherwise green
test suite. The service wants text that fits within a byte budget, but byte 4 sits inside
the two-byte UTF-8 encoding of `é`. The starter copies the full input into a `String` and
passes that invalid boundary to `String::truncate`.

## Starter code

> **EXERCISE**
>
> Review the implementation and its tests against the contract. The result must use “at
> most `max_bytes` bytes,” which means it may be shorter than the requested limit.

```rust,editable
{{#include ../../../examples/04_truncate_string.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/04_truncate_string.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/04_truncate_string.rs:tests}}
</script>

> **TRY THIS**
>
> Start with the green starter suite:
>
> ```sh
> cargo test --example 04_truncate_string
> ```
>
> Then add this test to `examples/04_truncate_string.rs`:
>
> ```rust,ignore
> #[test]
> fn backs_up_to_a_character_boundary() {
>     assert_eq!(truncate("café", 4), "caf");
> }
> ```
>
> Run the focused test before changing the implementation. The panic tells you which
> precondition the current code violates:
>
> ```sh
> cargo test --example 04_truncate_string backs_up_to_a_character_boundary
> ```

## Review

> **REVIEW**
>
> - What does `str::len` count?
> - Which byte indices are valid boundaries in `"café"`?
> - Why does the existing Unicode test miss the dangerous case?
> - What precondition does `String::truncate` place on its argument?
> - If four bytes would split `é`, should the function panic, return an error, or use
>   the longest valid prefix below the limit?
> - Does `max_bytes` count bytes or Unicode scalar values?

`"café"` occupies five bytes. `c`, `a`, and `f` each use one byte; `é` uses two. The valid
boundaries are therefore 0, 1, 2, 3, and 5. Index 4 lands inside the encoding of `é`.
`String::truncate` takes a byte length, but it panics unless that length is also a valid
character boundary.

The existing Unicode test uses a limit of five. It proves that the function accepts the
complete non-ASCII string, but it never asks the implementation to cut through a
multi-byte value. Unicode input alone does not reach the bug. The limit has to land inside
the encoding.

The exercise contract settles another possible disagreement: the result must use at most
`max_bytes` bytes. Under that contract, `"caf"` is the longest valid prefix. A different
contract, “first four Unicode scalar values,” would return `"café"`, which takes five
bytes. Both policies can be useful, but they are different APIs. The parameter name
reinforces the byte-based contract; it does not define that contract by itself.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/04_truncate_string.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/04_truncate_string.rs:tests}}
</script>

The byte-cap implementation does four things:

1. `s.len().min(max_bytes)` clamps the candidate index to the input length.
2. `floor_char_boundary` finds the greatest valid UTF-8 boundary at or below that
   candidate.
3. Slicing through the boundary preserves valid UTF-8.
4. `to_string` allocates and copies the returned prefix.

For `"café"` with a limit of four, the candidate is 4 and the preceding boundary is 3, so
the result is `"caf"`. A limit beyond the input length returns the complete string. Zero
and empty input both return an empty string.

This gives every valid `&str` and `usize` a result. A limit inside a multi-byte encoding is
ordinary input under this contract. Backing up to the previous boundary satisfies the
byte cap, so a panic or error would make the API stricter than the requirement.

There is one compatibility limit: `floor_char_boundary` has been stable since Rust 1.91.
If the project supports an older compiler, use the manual alternative below.

</details>

## Alternative approaches

The solution file also includes a helper for a different unit, character count:

```rust,editable
pub fn truncate_chars(s: &str, num_chars: usize) -> String {
    s.chars().take(num_chars).collect()
}
```

This is correct when the limit counts Unicode scalar values. It does not enforce a byte
budget. Four scalar values from `"café"` occupy five bytes. A Rust `char` can also be only
part of a user-perceived symbol, so a grapheme limit needs yet another contract.

If the minimum supported Rust version is older than 1.91, you can search for the boundary
with older stable APIs:

```rust,editable
pub fn truncate(s: &str, max_bytes: usize) -> String {
    if max_bytes >= s.len() {
        return s.to_string();
    }

    let mut end = max_bytes;
    while !s.is_char_boundary(end) {
        end -= 1;
    }

    s[..end].to_string()
}
```

The loop implements the same byte-cap policy. It is longer, but the boundary search is
visible and it works with older stable Rust releases.

## Edge cases and production concerns

> **EDGE CASE**
>
> A UTF-8 character boundary keeps the text valid. It does not guarantee that the result
> ends at a complete grapheme cluster.

Emoji with modifiers, flags, and letters followed by combining marks can span several
Unicode scalar values. `floor_char_boundary` may split such a sequence while still
returning valid UTF-8. If the result goes straight into a user interface, use a
grapheme-aware library and a function name that states the unit.

A byte cap cannot always be filled exactly. No valid prefix of `"café"` occupies four
bytes. If the caller requires exactly four bytes, the return type should represent
failure, or the API should operate on bytes rather than promise valid text.

The function always allocates because it returns `String`. Returning a borrowed `&str`
would avoid that allocation, but the prefix would borrow from the input. Callers would
then have to accept the new lifetime and interface. Make that change only when the call
sites benefit from it.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the byte-cap implementation using `floor_char_boundary` with
> the starter. The character-count helper has a different contract.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept `&str` and `usize`; neither parameter type encodes the boundary rule. |
| Readability | wins | `floor_char_boundary` names the UTF-8 policy directly, though the nested expression needs a short walkthrough. |
| API design | neutral | The signature and owned return type stay the same, and `max_bytes` already states the intended unit. |
| Performance | wins | The starter copies the complete input before truncating it; the solution copies only the returned prefix. |
| Flexibility | neutral | The function still implements one byte-cap policy and returns an owned string. |
| Error reporting | N/A | Every input has a valid result under the “at most” contract. An exact-length contract would need a failure case. |

Removing the panic is a correctness fix, but the six axes do not express that result
neatly. That is a useful limit of the rubric. A scorecard can focus review judgement; it
cannot replace the characterization test that exposes the failure.

## Quiz

### 1. The starter already tests `"café"`. Is another Unicode test warranted?

*Answer:* Yes. The existing limit is five, both the full byte length and a valid boundary.
A limit of four reaches the untested position inside `é`.

### 2. What should `truncate("café", 4)` return?

*Answer:* Under an “at most four bytes” contract, `"caf"`. No valid UTF-8 prefix occupies
exactly four bytes. Under a four-scalar-value contract, `"café"`. The name `max_bytes`
supports the first interpretation.

### 3. Should a non-boundary limit return an error?

*Answer:* Not for the stated maximum-byte contract. Backing up gives the caller a valid
prefix within the limit. An error is appropriate when the caller requires an exact byte
length or needs to know that the boundary moved.

### 4. Can `s.chars().take(max_bytes).collect()` replace the canonical solution?

*Answer:* No. That expression treats the argument as a Unicode scalar count. A selected
scalar may occupy more than one byte, so the result can exceed the byte limit.

## Follow-up questions

> **FOLLOW-UP**
>
> - Where do byte budgets in your codebase come from: protocols, databases, or external
>   APIs?
> - Should those limits count bytes, Unicode scalar values, or grapheme clusters?
> - Do callers need an owned `String`, or would a borrowed prefix work?
> - Is Rust 1.91 an acceptable minimum compiler for the project?
> - When should an impossible exact byte length return `Option`, `Result`, or raw bytes?

## Concepts recap

> **CONCEPT**
>
> - [UTF-8 byte boundaries](../appendix/concept-index.md#utf-8-byte-boundaries): string
>   slicing and truncation use byte indices, but only some indices preserve valid
>   UTF-8.
> - [`str::floor_char_boundary`](../appendix/concept-index.md#floor-char-boundary): find
>   the greatest valid boundary no larger than a byte limit.
> - [Characterization tests](../appendix/concept-index.md#characterization-tests): add
>   a focused test that exposes an untested contract before changing the code.
> - [Unicode scalar values](../appendix/concept-index.md#unicode-scalar-values):
>   `chars()` counts scalar values rather than bytes or grapheme clusters.
> - [Panicking preconditions](../appendix/concept-index.md#panicking-preconditions): a
>   safe standard-library operation can still panic when its documented precondition
>   is violated.
