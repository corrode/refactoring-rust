# 06 · `nesting`

## The problem

An optional text value needs three steps: trim surrounding whitespace, reject an empty
result, then convert the remaining text to uppercase. The starter implements that policy
with nested `if` expressions and repeated early returns. The behaviour is sound, but the
control flow hides a useful sequence of transformations from borrowed input to an owned
result.

## Starter code

> **EXERCISE**
>
> Review the branches as a data-processing sequence. Preserve all four tested cases:
> absent input, empty input, whitespace-only input, and non-empty text with surrounding
> whitespace.

```rust
{{#include ../../../examples/06_nesting.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/06_nesting.rs` without changing the
> behaviour:
>
> ```sh
> cargo test --example 06_nesting
> ```
>
> Keep the value borrowed through trimming and validation. Allocate the uppercase
> `String` only after the value has passed the empty check.

## Review

> **REVIEW**
>
> - What value and type exist after each successful step?
> - Does `str::trim` allocate or return a view into the input?
> - Which two cases become `None` in the starter?
> - Can `Option::filter` express the empty-string check?
> - At what point does the function first need an owned `String`?
> - Would explicit branches communicate anything that a combinator chain would lose?

Start with the states rather than the syntax. The input is either `None` or
`Some(&str)`. If text is present, `trim` returns another `&str` borrowed from the same
input. An empty trimmed slice is rejected. Only a non-empty slice reaches
`to_uppercase`, which creates the returned `String`.

The outer `if let` handles presence, while the inner `if` handles a predicate on the
present value. Both failure branches return `None`. This is the same structure that
`Option` combinators describe: transform a present value, keep it only when a predicate
passes, then transform it again.

Does trimming allocate? No. `str::trim` returns a subslice, so the value can stay
borrowed through validation. `str::to_uppercase` returns `String`, so that final step
allocates. Keeping it last means `None`, empty input, and whitespace-only input never pay
for an owned result.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/06_nesting.rs}}
```

Read the chain from top to bottom as three stages:

1. `map(str::trim)` transforms `Some(&str)` into a trimmed `Some(&str)`. For `None`, it
   does nothing.
2. `filter(|t| !t.is_empty())` keeps non-empty text and turns empty text into `None`.
3. `map(str::to_uppercase)` transforms the surviving `&str` into an owned `String`.

Each combinator runs only for `Some`. That means an absent input skips every operation,
and a value rejected by `filter` never reaches the allocating final `map`.

`filter` borrows the contained value while it tests the predicate. Here the contained
value is already an `&str`, so the closure receives another reference around it. Method
call auto-dereferencing lets `t.is_empty()` read naturally without spelling out that
extra layer.

The chain resembles an iterator pipeline, but `Option` is not itself an `Iterator`.
`Option` provides its own `map` and `filter` methods, and it can also be converted into
an iterator over zero or one item. The useful connection is the shared vocabulary: each
step transforms or rejects the value without opening another nested branch.

</details>

## Alternative approaches

The combinator chain is not the only readable version. You can keep the decisions
explicit while removing the nesting:

```rust
pub fn shout(s: Option<&str>) -> Option<String> {
    let Some(s) = s else {
        return None;
    };

    let trimmed = s.trim();
    if trimmed.is_empty() {
        return None;
    }

    Some(trimmed.to_uppercase())
}
```

I would keep this form when the failure paths need comments, logging, or different
return values. It is also reasonable when a team is still learning the combinators and
can review named intermediate values more confidently. The canonical chain earns its
place here because every step is short, pure, and follows the contract in order.

Avoid moving `to_uppercase` before `filter`. That would allocate before the function
knows whether the trimmed input should be rejected. It also changes the useful type
progression from borrowed validation followed by owned output.

## Edge cases and production concerns

> **EDGE CASE**
>
> Unicode uppercasing can change the number of characters and bytes. It is not an
> in-place, fixed-length transformation.

For example, uppercasing `"straße"` produces `"STRASSE"`. `str::to_uppercase` returns a
new `String` because the output may need more storage than the input. The canonical
solution performs that allocation only for text that remains after trimming and
filtering, just as the starter does.

`trim` removes leading and trailing Unicode whitespace according to Rust's whitespace
rules. If the input comes from a protocol that defines a narrower set, such as ASCII
spaces only, `trim` may accept more input than the protocol permits. That is a contract
decision rather than a reason to expand this exercise's implementation.

Uppercasing is also not locale-aware. If the result is used for identifiers, security
checks, or user-facing language rules, decide whether Unicode default case conversion
matches the domain before treating it as normalization.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `map`, `filter`, and `map` chain with the starter.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions use the same `Option<&str>` input and `Option<String>` output, with the same valid states. |
| Readability | wins | The chain presents trimming, rejection, and uppercasing in the order they happen without nested branches or repeated returns. |
| API design | neutral | The signature still treats absent input and empty-after-trimming input as the same `None` result. |
| Performance | neutral | Both versions trim without allocating and allocate one uppercase `String` only after a non-empty value survives. |
| Flexibility | wins | Additional transformations and predicates can be inserted as stages without adding another level of control-flow nesting. |
| Error reporting | neutral | Both versions discard the reason for `None`, so callers cannot distinguish absent from empty input. |

The readability win depends on keeping the chain short. A long series of closures can
make intermediate types and failure points harder to inspect. Here, each operation has
one job and the order matches the function's contract. If that stops being true, named
locals and explicit branches are a better choice.

## Quiz

### 1. Should `to_uppercase` run before the empty check to keep all transformations together?

*Answer:* No. Trimming and checking emptiness can operate on borrowed `&str` values.
Uppercasing first would allocate a `String` for input that may be discarded. Keep the
owned transformation after filtering unless the contract requires a different order.

### 2. Should a review reject the explicit version because combinators are more idiomatic?

*Answer:* No. The explicit version can be easier to extend when failure cases need
logging, distinct errors, or more involved decisions. Prefer the combinator chain here
because its three short stages follow the contract in order. Explicit control flow is
still a sound choice.

### 3. Should `shout` return `Result<String, Error>` instead of `Option<String>`?

*Answer:* Only if callers need to react differently to absent input and empty input, or
if either state is an error in the domain. The tests require the same `None` result for
both, so `Option` matches the current contract.

### 4. Can callers assume the uppercase result has the same length as the trimmed input?

*Answer:* No. Unicode case conversion can expand text, as when `ß` becomes `SS`.
`to_uppercase` therefore returns an allocated `String`, and callers should not use input
length as an output-size guarantee.

## Follow-up questions

> **FOLLOW-UP**
>
> - Do callers need to distinguish missing input from input that becomes empty after
>   trimming?
> - Which validation steps in your codebase can stay borrowed until the final output is
>   constructed?
> - At what chain length does your team prefer named intermediate values?
> - Does your domain want Unicode whitespace trimming or a narrower protocol rule?
> - Is Unicode default uppercasing suitable for the way the result will be compared or
>   displayed?

## Concepts recap

> **CONCEPT**
>
> - [`Option::map`](../appendix/concept-index.md#option-map): transform a present value
>   while leaving `None` unchanged.
> - [`Option::filter`](../appendix/concept-index.md#option-filter): keep a present value
>   only when it satisfies a predicate.
> - [Option combinator pipelines](../appendix/concept-index.md#option-combinator-pipelines):
>   express a sequence of transformations and rejections without nested control flow.
> - [Borrow before owning](../appendix/concept-index.md#borrow-before-owning): validate a
>   borrowed view first, then allocate the owned result only when needed.
> - [Unicode case conversion](../appendix/concept-index.md#unicode-case-conversion): case
>   mappings may expand text and are not locale-aware normalization.
