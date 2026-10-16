# `nesting`

This function trims optional text and returns it in uppercase when anything remains.
Refactor it while preserving the results for absent input, empty input,
whitespace-only input, and non-empty text surrounded by whitespace.

Run the local tests with `cargo test --example 06_nesting`.

## Code to refactor

```rust,editable
{{#include ../../../examples/06_nesting.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/06_nesting.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/06_nesting.rs:tests}}
</script>


## Discussion

> **DISCUSSION**
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


<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Solutions</h2></summary>

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

### Alternative approaches

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

### Edge cases and production concerns

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

</details>

## Trade-offs

Compared with the original: **+** advantage, **=** comparable, **−** disadvantage,
**N/A** not applicable. Readability depends on familiarity.

| Criterion | Original | `map` → `filter` → `map` | `let-else` + early returns |
|---|---|---|---|
| Type-safety | Optional input and output | = Same representable states | = Same representable states |
| Readability | Nested presence and emptiness checks | + Three named stages, no nesting | + Flat checks, named intermediate |
| API design | `Option<&str>` → `Option<String>` | = Same borrowed input, owned output | = Same borrowed input, owned output |
| Performance | Allocates uppercase result after validation | = Same trimming and uppercase allocation | = Same trimming and uppercase allocation |
| Flexibility | More checks can deepen nesting | + Add stages without nesting | + Add guards without nesting |
| Error reporting | Missing and empty become `None` | = Same indistinguishable failure reasons | = Same indistinguishable failure reasons |

Longer chains can hide intermediate types and failure points. Prefer named locals
and explicit branches when steps need comments or logging.

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

## Important Concepts

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
