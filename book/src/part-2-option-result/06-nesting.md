# 06 · `nesting`

## The problem

`shout` trims an optional string, returns `None` if it is absent or empty after trimming,
and otherwise returns the text in uppercase. The starter nests the empty check inside
the presence check. `Option` methods can express these steps without nesting.

## Starter code

> **EXERCISE**
>
> Read the branches as a sequence of data transformations. Preserve all four tested
> cases: absent input, empty input, whitespace-only input, and non-empty text surrounded
> by whitespace.

```rust,editable
{{#include ../../../examples/06_nesting.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/06_nesting.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/06_nesting.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/06_nesting.rs` without changing its
> behaviour:
>
> ```sh
> cargo test --example 06_nesting
> ```
>
> Keep trimming and validation on borrowed text. Create the uppercase `String` only after
> the value survives the empty check.

## Review

> **REVIEW**
>
> - After each successful step, what value exists and what is its type?
> - Does `str::trim` allocate, or does it return a view into the original input?
> - Which two failure states become `None` in the starter?
> - Can `Option::filter` state the non-empty requirement?
> - Where does the function first need to own a `String`?
> - Would replacing the branches with combinators hide any useful distinction?

The input is `None` or `Some(&str)`. When text is present, `trim` returns a borrowed
subslice without allocating. The function rejects an empty slice. Otherwise,
`to_uppercase` creates the `String` returned to the caller.

The outer `if let` asks whether a value is present. The inner `if` applies a predicate to
that value. Both failure paths return `None`. `Option` combinators express the same sequence:
transform a present value, retain it when a predicate passes, then transform it once
more.


## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/06_nesting.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/06_nesting.rs:tests}}
</script>

Read the chain as three stages, in order:

1. `map(str::trim)` turns `Some(&str)` into a trimmed `Some(&str)` and leaves `None`
   alone.
2. `filter(|t| !t.is_empty())` retains non-empty text and converts empty text to `None`.
3. `map(str::to_uppercase)` converts the remaining `&str` into an owned `String`.

These methods call their functions or closures only for `Some`. Absent input remains
`None` through the chain. If `filter` rejects a value, `to_uppercase` is not called.

`filter` borrows the value inside the option to test its predicate. The value is an
`&str`, so the closure receives `&&str`. Method-call auto-dereferencing lets
`t.is_empty()` work without an explicit dereference.

`Option` has its own `map` and `filter` methods; it is not an `Iterator`. It can also
be converted into an iterator containing zero or one item.

</details>

## Alternative approaches

Early returns also remove the nesting:

```rust,editable
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

Explicit branches are easier to extend with comments, logging, or distinct return
values. Named intermediate values can help when the combinators are unfamiliar.
The canonical chain fits this function because its three steps are short and have no
side effects.

Keep `to_uppercase` after `filter` so the function checks the borrowed text before
creating an owned result.

## Edge cases and production concerns

> **EDGE CASE**
>
> Unicode uppercasing can change both the character count and the byte count. It is not a
> fixed-length transformation that can happen in place.

For example, uppercasing `"straße"` produces `"STRASSE"`. `str::to_uppercase` returns a
new `String` because the output may require more storage than the input.

`trim` removes leading and trailing Unicode whitespace according to Rust's whitespace
rules. A protocol may define a narrower set, such as ASCII spaces only. In that case,
`trim` can accept input the protocol rejects. Choose the whitespace rule required by
the contract; this exercise keeps the starter's behavior.

Uppercasing is not locale-aware either. Before using this result for identifiers,
security checks, or language shown to users, decide whether Unicode's default case
conversion is the normalization your domain intends.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `map`, `filter`, and `map` chain with the starter.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept `Option<&str>` and return `Option<String>`, with the same set of valid states. |
| Readability | wins | Trimming, rejection, and uppercasing appear in execution order, without nested branches or repeated returns. |
| API design | neutral | The signature still combines absent input and input that becomes empty after trimming into the same `None`. |
| Performance | neutral | Both versions trim without allocating and create one uppercase `String` only after a non-empty value survives. |
| Flexibility | wins | Another transformation or predicate can be added as a stage without another level of control-flow nesting. |
| Error reporting | neutral | Both versions erase the reason for `None`, so the caller cannot distinguish absent input from empty input. |

Longer chains can make intermediate types and failure points harder to inspect.
Use named locals and explicit branches when individual steps need more explanation.

## Quiz

### 1. Should `to_uppercase` run before the empty check to keep all transformations together?

*Answer:* No. Trimming and the empty check work on borrowed `&str` values. Uppercasing
first would create a `String` even for empty text, only to discard it. An empty `String`
need not allocate, but the conversion is unnecessary. Keep it after filtering unless
the contract requires another order.

### 2. Should a review reject the explicit version because combinators are more idiomatic?

*Answer:* No. Explicit control flow is easier to extend when failure paths need logging,
different errors, or more involved decisions. Both versions are correct. The combinator
chain fits this function because it expresses three short steps in execution order.

### 3. Should `shout` return `Result<String, Error>` instead of `Option<String>`?

*Answer:* Only when callers must respond differently to absent and empty input, or when
either state counts as a domain error. The tests require both cases to return the same
`None`, so `Option` matches the current contract.

### 4. Can callers assume the uppercase result has the same length as the trimmed input?

*Answer:* No. Unicode case conversion can expand text, including the conversion from `ß`
to `SS`. That is why `to_uppercase` returns an allocated `String`. The input length is not
a guarantee about the output length.

## Follow-up questions

> **FOLLOW-UP**
>
> - Must callers distinguish missing input from text that becomes empty after trimming?
> - Which validation steps in your codebase can keep borrowing until the output is built?
> - How long can a combinator chain become before your team prefers named intermediate
>   values?
> - Does your domain use Unicode whitespace, or does a protocol define a narrower rule?
> - Is Unicode default uppercasing appropriate for how the result will be compared or
>   displayed?

## Concepts recap

> **CONCEPT**
>
> - [`Option::map`](../appendix/concept-index.md#option-map): transform a present value
>   without changing `None`.
> - [`Option::filter`](../appendix/concept-index.md#option-filter): retain a present value
>   only when it passes a predicate.
> - [Option combinator pipelines](../appendix/concept-index.md#option-combinator-pipelines):
>   arrange transformations and rejections as a sequence rather than nested control flow.
> - [Borrow before owning](../appendix/concept-index.md#borrow-before-owning): validate a
>   borrowed view, then allocate an owned result only when the value survives.
> - [Unicode case conversion](../appendix/concept-index.md#unicode-case-conversion): case
>   mappings can expand text and do not provide locale-aware normalization.
