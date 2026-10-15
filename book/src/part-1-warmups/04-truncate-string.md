# 04 · `truncate_string`

## Code to refactor

> **EXERCISE**
>
> Check whether the function meets its documented behavior for all inputs. Add tests
> for cases you think are missing, then make any needed changes without changing the
> signature.

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
> Run the existing starter tests, which pass:
>
> ```sh
> cargo test --example 04_truncate_string
> ```
>
> Look for inputs the existing tests do not cover. Try writing a failing test before
> changing the implementation.

<details>
<summary>Hint: a test case to try</summary>

> Try this test in `examples/04_truncate_string.rs`:
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

</details>

## Discussion

> **DISCUSSION**
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
`String::truncate` takes a byte length. If the length is less than the current string
length, it must be a character boundary or the method panics. A length at or above the
current length leaves the string unchanged.

The existing Unicode test uses a limit of five. It proves that the function accepts the
complete non-ASCII string, but it never asks the implementation to cut through a
multi-byte character. To expose the bug, the test needs a limit inside the encoding.

The limit counts bytes, not characters. With a four-byte limit, `"caf"` is the longest
valid prefix. Taking the first four Unicode scalar values would instead return
`"café"`, which uses five bytes. The name `max_bytes` states the unit, but the contract
also needs to say what happens when the limit splits a character.

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Solutions</h2></summary>

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
4. `to_string` copies the prefix into an owned `String`, allocating if it is nonempty.

For `"café"` with a limit of four, the candidate is 4 and the preceding boundary is 3, so
the result is `"caf"`. A limit beyond the input length returns the complete string. Zero
and empty input both return an empty string.

Every valid `&str` and `usize` has a result under this contract. A limit inside a
multi-byte encoding does not need an error: the preceding boundary gives a valid prefix
within the byte limit.

There is one compatibility limit: `floor_char_boundary` has been stable since Rust 1.91.
If the project supports an older compiler, use the manual alternative below.

### Alternative approaches

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

</details>

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

A nonempty result requires an allocation because the function returns an owned
`String`. An empty result does not. Returning `&str` would avoid copying and allocating,
but the result could not outlive the input. Make that API change only when callers
benefit from it.

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

The main improvement is the correctness fix: a limit inside a UTF-8 character no longer
causes a panic. None of the six axes records that directly, so the failing test remains
important alongside these scores.

## Follow-up questions

> **FOLLOW-UP**
>
> - Where do byte budgets in your codebase come from: protocols, databases, or external
>   APIs?
> - Should those limits count bytes, Unicode scalar values, or grapheme clusters?
> - Do callers need an owned `String`, or would a borrowed prefix work?
> - Is Rust 1.91 an acceptable minimum compiler for the project?
> - When should an impossible exact byte length return `Option`, `Result`, or raw bytes?

## Important Concepts

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
