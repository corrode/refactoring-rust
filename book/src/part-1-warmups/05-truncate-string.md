# `truncate_string`

This function limits a string to a byte budget.
Does it meet its documented behaviour for all inputs, or do the tests miss something?
Add tests for the missing cases, then refactor the function and fix any behaviour that violates the contract.
Keep the signature unchanged.
Run the tests with `cargo test --example 05_truncate_string`.

## Code to refactor

```rust,editable
{{#include ../../../examples/05_truncate_string.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/05_truncate_string.rs:tests}}
</script>

## Discussion

<details class="discussion-question">
<summary>What does <code>str::len</code> count?</summary>

Bytes, not Unicode scalar values or user-perceived characters.
`"café"` occupies five bytes even though it contains four Unicode scalar values.

</details>

<details class="discussion-question">
<summary>Which byte indices are valid boundaries in <code>"café"</code>?</summary>

The valid boundaries are 0, 1, 2, 3, and 5.
`c`, `a`, and `f` each use one byte; `é` uses two.
Index 4 lands inside the encoding of `é`.

</details>

<details class="discussion-question">
<summary>Why does the existing Unicode test miss the dangerous case?</summary>

Its limit of five keeps the whole string.
That proves the function accepts the complete non-ASCII string, but doesn't test what happens when the limit falls inside a multi-byte character.
Add a test with a limit inside the encoding to expose the bug.

> Try this test in `examples/05_truncate_string.rs`:
>
> ```rust,ignore,noplayground
> #[test]
> fn backs_up_to_a_character_boundary() {
>     assert_eq!(truncate("café", 4), "caf");
> }
> ```
>
> Run the focused test before changing the implementation.
> The panic tells you which precondition the current code violates:
>
> ```sh
> cargo test --example 05_truncate_string backs_up_to_a_character_boundary
> ```

</details>

<details class="discussion-question">
<summary>What precondition does <code>String::truncate</code> place on its argument?</summary>

If you pass a byte length shorter than the string, it must fall on a character boundary or `String::truncate` panics.
A length at or above the current length leaves the string unchanged.

</details>

<details class="discussion-question">
<summary>If four bytes would split <code>é</code>, should the function panic, return an error, or use the longest valid prefix below the limit?</summary>

Return `"caf"`, the longest valid prefix within the byte budget.
There's no need to panic or return an error: the shorter prefix meets the contract.

</details>

<details class="discussion-question">
<summary>Does <code>max_bytes</code> count bytes or Unicode scalar values?</summary>

The limit counts bytes.
Taking the first four Unicode scalar values would return `"café"`, which uses five bytes and exceeds a four-byte budget.
Name the unit with `max_bytes`, but also document what happens when the limit splits a character.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/05_truncate_string.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/05_truncate_string.rs:tests}}
</script>

Find a valid boundary before copying the prefix:

1. `s.len().min(max_bytes)` clamps the candidate index to the input length.
2. `floor_char_boundary` finds the greatest valid UTF-8 boundary at or below that candidate.
3. Slicing through the boundary preserves valid UTF-8.
4. `to_string` copies the prefix into an owned `String`, allocating if it is nonempty.

For `"café"` with a limit of four, the candidate is 4 and the preceding boundary is 3, so the result is `"caf"`.
A limit beyond the input length returns the complete string.
Zero and empty input both return an empty string.

Can any valid `&str` and `usize` fail under this contract?
No, even a limit inside a multi-byte encoding has a valid prefix at the preceding boundary.

`floor_char_boundary` requires Rust 1.91, when it became stable.
If your project supports an older compiler, use the manual alternative below.

### Alternative approaches

The solution file also includes a helper for a different unit, character count:

```rust,editable
pub fn truncate_chars(s: &str, num_chars: usize) -> String {
    s.chars().take(num_chars).collect()
}
```

<script type="text/plain" class="playground-tests">
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_unicode_scalar_values_not_bytes() {
        assert_eq!(truncate_chars("café", 4), "café");
        assert_eq!(truncate_chars("café", 3), "caf");
        assert_eq!(truncate_chars("🦀rust", 1), "🦀");
    }

    #[test]
    fn empty_zero_and_oversized_limits() {
        assert_eq!(truncate_chars("", 3), "");
        assert_eq!(truncate_chars("hello", 0), "");
        assert_eq!(truncate_chars("hi", 100), "hi");
    }
}
</script>

Use this helper when your limit counts Unicode scalar values, not bytes.
It does not enforce a byte budget.
Four scalar values from `"café"` occupy five bytes.
A Rust `char` can also be only part of a user-perceived symbol, so a grapheme limit needs yet another contract.

If the minimum supported Rust version is older than 1.91, you can search for the boundary with older stable APIs:

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

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/05_truncate_string.rs:tests}}
</script>

The loop backs up until it finds a character boundary, preserving the same byte-cap policy.
You write more code, but you can see the boundary search and use older stable Rust releases.

### Edge cases and production concerns

> **EDGE CASE**
>
> A UTF-8 character boundary keeps the text valid.
> It does not guarantee that the result ends at a complete grapheme cluster.

Emoji with modifiers, flags, and letters followed by combining marks can span several Unicode scalar values.
`floor_char_boundary` may split such a sequence while still returning valid UTF-8.
If the result goes straight into a user interface, use a grapheme-aware library and a function name that states the unit.

Can you always fill the byte budget exactly?
No, there is no valid four-byte prefix of `"café"`.
If your caller requires exactly four bytes, represent failure in the return type or operate on bytes rather than promise valid text.

A nonempty result requires an allocation because the function returns an owned `String`.
An empty result does not.
Returning `&str` would avoid copying and allocating, but the result could not outlive the input.
Make that API change only when callers benefit from it.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | `floor_char_boundary` | Manual boundary search | `truncate_chars` (scalar cap) |
|---|---|---|---|---|
| Type-safety | Boundary rule absent from types | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Boundary checked at runtime | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Boundary checked at runtime | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Limit unit still untyped |
| Readability | Short; boundary precondition hidden | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Named policy; nested expression needs familiarity | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Explicit policy; more control flow | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Direct scalar count, if adapters familiar |
| API design | Byte cap; owned result | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same signature and byte unit | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same signature and byte unit | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> Scalar cap cannot enforce byte budget |
| Performance | Copies entire input | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Copies less when prefix shorter | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Copies less when prefix shorter | N/A |
| Flexibility | Byte cap; older stable APIs | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> Requires Rust 1.91 | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Byte cap; older stable APIs | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> Replaces byte budget with scalar count |
| Error reporting | Panics inside multi-byte encoding | N/A | N/A | N/A |

Both byte-cap solutions fix the panic when a limit splits a UTF-8 character, even though neither changes the types or signature.
Neither needs an error because the contract asks for “at most” the byte limit.
An exact byte-length contract would need a failure case.
Copying only the prefix reduces copied bytes when truncating, not necessarily elapsed time; nonempty results still allocate.

`truncate_chars` is valid for a different contract, not a substitute for a byte cap: four scalar values from `"café"` occupy five bytes.
Its negative scores concern the byte-budget requirement, and its performance is not scored against a different unit.
None of these variants guarantees complete grapheme clusters.

</details>

## Takeaways For Our Own Codebase

> - Trace your byte budgets back to their source: a protocol, database, or external API.
> - Check whether each limit counts bytes, Unicode scalar values, or grapheme clusters, and name the unit in the API.
> - Test limits inside multi-byte characters, not just inputs that contain them.
> - Do callers need an owned `String`, or would a borrowed prefix work?
> - Check your minimum supported Rust version before choosing `floor_char_boundary`: it requires Rust 1.91.
> - If an exact byte length is impossible, should you return `Option` or `Result`, or change the API to return raw bytes?

## References

- [UTF-8 byte boundaries](../appendix/concept-index.md#utf-8-byte-boundaries): string slicing and truncation use byte indices, but only some indices preserve valid UTF-8.
- [`str::floor_char_boundary`](../appendix/concept-index.md#floor-char-boundary): find the greatest valid boundary no larger than a byte limit.
- [Characterization tests](../appendix/concept-index.md#characterization-tests): add a focused test that exposes an untested contract before changing the code.
- [Unicode scalar values](../appendix/concept-index.md#unicode-scalar-values): `chars()` counts scalar values rather than bytes or grapheme clusters.
- [Panicking preconditions](../appendix/concept-index.md#panicking-preconditions): a safe standard-library operation can still panic when its documented precondition is violated.
