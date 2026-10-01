# 04 · `truncate_string`

## The problem

A service needs to keep a text value within a byte budget. The starter copies the
input into a `String` and truncates it to the requested length. Its tests pass,
including one with non-ASCII text. The missing case is a limit inside a multi-byte
UTF-8 encoding, where `String::truncate` panics instead of returning a shorter string.

## Starter code

> **EXERCISE**
>
> Review the implementation and the tests. The contract is “at most `max_bytes`
> bytes,” so a valid result may be shorter than the requested limit.

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
> Run the focused test and inspect the panic before changing the implementation:
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

`"café"` occupies five bytes: one each for `c`, `a`, and `f`, followed by two for `é`.
Its valid boundaries are 0, 1, 2, 3, and 5. Index 4 lies inside the encoding of `é`.
`String::truncate` accepts a byte length, but it panics unless that length is a valid
character boundary.

The existing Unicode test uses a limit of five. That proves the function accepts the
complete non-ASCII string, but it never asks the implementation to cut inside a
multi-byte value. Unicode input alone is not enough; the limit has to reach the
boundary condition.

The parameter name gives us the intended unit. Under an “at most four bytes” contract,
`"caf"` is the longest valid prefix. Under a “first four Unicode scalar values”
contract, the result would be `"café"`, which occupies five bytes. Those are different
APIs.

## A possible solution

```rust
{{#include ../../../solutions/04_truncate_string.rs}}
```

The byte-cap implementation works from the inside out:

1. `s.len().min(max_bytes)` clamps the candidate index to the input length.
2. `floor_char_boundary` finds the greatest valid UTF-8 boundary at or below that
   candidate.
3. Slicing through the boundary preserves valid UTF-8.
4. `to_string` allocates and copies the returned prefix.

For `"café"` with a limit of four, the candidate is 4 and the preceding boundary is 3,
so the result is `"caf"`. A limit beyond the input length returns the complete string.
Zero and empty input both produce an empty result.

Every valid `&str` and `usize` now has a result. A non-boundary limit is ordinary input
under this contract, so the function has no reason to panic or return an error.

`floor_char_boundary` has been stable since Rust 1.91. Projects with an older minimum
supported Rust version need the manual alternative below.

## Alternative approaches

The solution file also includes a character-count interpretation:

```rust
pub fn truncate_chars(s: &str, num_chars: usize) -> String {
    s.chars().take(num_chars).collect()
}
```

This is correct when the limit counts Unicode scalar values. It does not enforce a
byte budget: four scalar values from `"café"` occupy five bytes. A Rust `char` can also
be smaller than a complete user-perceived symbol, so character and grapheme limits
need separate contracts.

For Rust versions before 1.91, we can search for the boundary explicitly:

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

The loop implements the same byte-cap policy. It is longer, but it uses older stable
APIs and makes the boundary search visible.

## Edge cases and production concerns

> **EDGE CASE**
>
> A UTF-8 character boundary prevents invalid text. It does not guarantee that the
> result ends at a complete grapheme cluster.

Emoji with modifiers, flags, and letters followed by combining marks can contain
several Unicode scalar values. `floor_char_boundary` may split those sequences while
still returning valid UTF-8. If the output appears directly in a user interface, a
grapheme-aware library and a clearly named limit may be more appropriate.

A byte cap cannot always be filled exactly. No valid prefix of `"café"` occupies four
bytes. If a caller requires exactly four bytes, the return type should represent
failure or the API should operate on bytes rather than promise valid text.

The function always allocates because it returns `String`. Returning a borrowed `&str`
would avoid that allocation, but it would tie the result's lifetime to the input and
change the interface for every caller.

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

The panic removal is a correctness fix that the six axes do not capture neatly. That
is a useful limit of the rubric: a scorecard supports review judgement, but it does
not replace a failing characterization test.

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
