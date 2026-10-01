# 04 · `truncate_string`

## The problem

A service must keep a text value within a byte budget. The starter copies the input
into a `String`, then truncates it to the requested length. Every test passes, including
one with non-ASCII text. But one case is missing: when the limit falls inside a
multi-byte UTF-8 encoding, `String::truncate` panics instead of returning a shorter
string.

## Starter code

> **EXERCISE**
>
> Review the implementation and the tests. The contract says “at most `max_bytes`
> bytes,” so the result may be shorter than the requested limit.

```rust
{{#include ../../../examples/04_truncate_string.rs}}
```

> **TRY THIS**
>
> Run the green starter suite:
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
> Run the focused test and inspect the panic before you change the implementation:
>
> ```sh
> cargo test --example 04_truncate_string backs_up_to_a_character_boundary
> ```

## Review

> **REVIEW**
>
> - What unit does `str::len` return?
> - Which byte indices are valid boundaries in `"café"`?
> - Why does the existing Unicode test miss the dangerous case?
> - What precondition does `String::truncate` place on its argument?
> - If four bytes would split `é`, should the function panic, return an error, or use
>   the longest valid prefix below the limit?
> - Does `max_bytes` mean bytes or Unicode scalar values?

`"café"` occupies five bytes. `c`, `a`, and `f` use one byte each, while `é` uses two.
That gives us valid boundaries at 0, 1, 2, 3, and 5. Index 4 sits inside the encoding of
`é`. Although `String::truncate` accepts a byte length, it panics unless that length is
a valid character boundary.

Why doesn't the existing Unicode test catch this? Its limit is five. That proves the
function accepts the complete non-ASCII string, but it never asks the implementation
to cut through a multi-byte value. Unicode input alone is not enough. The limit must
reach the boundary condition.

The parameter name tells us which unit the API intends to count. With an “at most four
bytes” contract, `"caf"` is the longest valid prefix. With a “first four Unicode scalar
values” contract, the result is `"café"`, which occupies five bytes. Those are different
APIs, and choosing between them belongs in the contract rather than an implementation
detail.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/04_truncate_string.rs}}
```

Read the byte-cap implementation from the inside out:

1. `s.len().min(max_bytes)` clamps the candidate index to the input length.
2. `floor_char_boundary` finds the greatest valid UTF-8 boundary at or below that
   candidate.
3. Slicing through the boundary preserves valid UTF-8.
4. `to_string` allocates and copies the returned prefix.

For `"café"` with a limit of four, the candidate is 4 and the previous boundary is 3.
The result is `"caf"`. A limit beyond the input length returns the complete string. Zero
and empty input both produce an empty result.

Every valid `&str` and `usize` now has a result. Under this contract, a non-boundary
limit is ordinary input, so the function has no reason to panic or return an error.

`floor_char_boundary` has been stable since Rust 1.91. If your minimum supported Rust
version is older, use the manual alternative below.

</details>

## Alternative approaches

The solution file also includes an interpretation based on character count:

```rust
pub fn truncate_chars(s: &str, num_chars: usize) -> String {
    s.chars().take(num_chars).collect()
}
```

This version is correct when the limit counts Unicode scalar values. It does not enforce
a byte budget: four scalar values from `"café"` occupy five bytes. A Rust `char` may also
be smaller than a complete user-perceived symbol, so character limits and grapheme
limits need separate contracts.

Before Rust 1.91, you can search for the boundary yourself:

```rust
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

The loop follows the same byte-cap policy. It takes more code, but it uses older stable
APIs and puts the boundary search in plain view.

## Edge cases and production concerns

> **EDGE CASE**
>
> A UTF-8 character boundary prevents invalid text. It does not guarantee that the
> result ends at a complete grapheme cluster.

Emoji with modifiers, flags, and letters followed by combining marks can contain
several Unicode scalar values. `floor_char_boundary` may split those sequences while
still returning valid UTF-8. If you display the output directly in a user interface, a
grapheme-aware library and a name that states the unit may be a better fit.

A byte cap cannot always be filled exactly. No valid prefix of `"café"` occupies four
bytes. If you require exactly four bytes, the return type should represent failure, or
the API should operate on bytes instead of promising valid text.

The function always allocates because it returns `String`. A borrowed `&str` would avoid
that allocation, but the result would borrow from the input. Every caller would then
have to work with the new lifetime and interface.

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

Removing the panic is a correctness fix that the six axes do not capture neatly. I see
that as a useful limit of the rubric: a scorecard can support review judgement, but it
cannot replace a failing characterization test.

## Quiz

### 1. The starter already tests `"café"`. Is another Unicode test warranted?

*Answer:* Yes. The existing limit is five, which is both the full byte length and a
valid boundary. A limit of four reaches the untested case inside `é`.

### 2. What should `truncate("café", 4)` return?

*Answer:* Under an “at most four bytes” contract, `"caf"`. No valid UTF-8 prefix uses
exactly four bytes. Under a four-scalar-value contract, `"café"`. The name `max_bytes`
supports the first interpretation.

### 3. Should a non-boundary limit return an error?

*Answer:* Not for the stated maximum-byte contract. Backing up produces a valid prefix
within the limit. An error makes sense when the caller requires an exact byte length or
needs to know that the boundary moved.

### 4. Can `s.chars().take(max_bytes).collect()` replace the canonical solution?

*Answer:* No. That treats the argument as a Unicode scalar count. A selected scalar can
occupy more than one byte, so the result may exceed the byte limit.

## Follow-up questions

> **FOLLOW-UP**
>
> - Where do byte budgets in your codebase come from: protocols, databases, or external
>   APIs?
> - Should those limits count bytes, Unicode scalar values, or grapheme clusters?
> - Do callers need an owned `String`, or could a borrowed prefix work?
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
