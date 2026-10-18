# `nesting`

This function trims optional text and returns the remaining text in uppercase.
Can you make it easier to read without changing its signature or results?
Keep its behaviour for absent input, empty input, whitespace-only input, and non-empty text surrounded by whitespace.

Run the local tests with `cargo test --example 06_nesting`.

## Code to refactor

```rust,editable
{{#include ../../../examples/06_nesting.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/06_nesting.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What value and type do we have after each successful step?</summary>

The outer `if let` extracts an `&str` from `Some`.
Trimming produces another `&str`, and the emptiness check leaves that type unchanged.
Uppercasing produces a `String`, which the function wraps in `Some` to return `Option<String>`.

</details>

<details class="discussion-question">
<summary>Does <code>str::trim</code> allocate?</summary>

No, it returns a borrowed subslice of the original input.
You don't need an owned copy to check the trimmed text.

</details>

<details class="discussion-question">
<summary>Which inputs make the function return <code>None</code>?</summary>

The function returns `None` for missing input and for text that's empty after trimming.
That includes both an empty string and a string containing only whitespace.

</details>

<details class="discussion-question">
<summary>Can <code>Option::filter</code> replace the emptiness check?</summary>

Yes, `filter(|t| !t.is_empty())` keeps a present value when the predicate passes and returns `None` otherwise.
Combined with `map`, it expresses the same steps as the branches: trim a present value, keep it if it's non-empty, then uppercase it.

</details>

<details class="discussion-question">
<summary>When do we need an owned <code>String</code>?</summary>

Only when you create the uppercase result.
`to_uppercase` builds the `String` for the caller.
Until then, you can trim and check for empty text through borrowed slices.

</details>

<details class="discussion-question">
<summary>Would combinators hide a distinction the caller needs?</summary>

Not with this return type: the starter already returns `None` for both missing and empty text.
The combinator version preserves that behaviour.
If callers need to tell those cases apart, you'll need a different return type, not just different control flow.

</details>


<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/06_nesting.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/06_nesting.rs:tests}}
</script>

The chain trims, checks, and uppercases the text in that order:

1. `map(str::trim)` trims the `&str` inside `Some` and leaves `None` alone.
2. `filter(|t| !t.is_empty())` keeps non-empty text and converts empty text to `None`.
3. `map(str::to_uppercase)` converts the remaining `&str` into an owned `String`.

These methods call their functions or closures only for `Some`.
Absent input remains `None` through the chain.
If `filter` rejects a value, the final `map` skips `to_uppercase`.

`filter` borrows the value inside the option to test its predicate.
The value is an `&str`, so the closure receives `&&str`.
Method-call auto-dereferencing lets `t.is_empty()` work without an explicit dereference.

Does this make `Option` an `Iterator`?
No, `Option` has its own `map` and `filter` methods.
You can also convert it into an iterator containing zero or one item.

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

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/06_nesting.rs:tests}}
</script>

Use explicit branches when you need comments, logging, or distinct return values.
Named intermediate values also help if you find the combinators hard to follow.
The chain works well here because all three steps are short and have no side effects.

Keep `to_uppercase` after `filter` so the function checks the borrowed text before creating an owned result.

### Edge cases and production concerns

> **EDGE CASE**
>
> Unicode uppercasing can change both the character count and the byte count.
> It is not a fixed-length transformation that can happen in place.

For example, uppercasing `"straße"` produces `"STRASSE"`.
`str::to_uppercase` returns a new `String` because the output may require more storage than the input.

`trim` removes leading and trailing Unicode whitespace according to Rust's whitespace rules.
Does your protocol allow all of that whitespace?
Some protocols allow only ASCII spaces.
Using `trim` in that case can accept input the protocol rejects.
Use the whitespace rule your contract requires; this exercise keeps the starter's behaviour.

`to_uppercase` isn't locale-aware either.
Before you use its output for identifiers, security checks, or text shown to users, check whether your domain requires Unicode's default case conversion or a different normalization.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | `map` → `filter` → `map` | `let-else` + early returns |
|---|---|---|---|
| Type-safety | Optional input and output | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same representable states | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same representable states |
| Readability | Nested presence and emptiness checks | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Three named stages, no nesting | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Flat checks, named intermediate |
| API design | `Option<&str>` → `Option<String>` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same borrowed input, owned output | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same borrowed input, owned output |
| Performance | Allocates uppercase result after validation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same trimming and uppercase allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same trimming and uppercase allocation |
| Flexibility | More checks can deepen nesting | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Add stages without nesting | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Add guards without nesting |
| Error reporting | Missing and empty become `None` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same indistinguishable failure reasons | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same indistinguishable failure reasons |

Longer chains can hide intermediate types and failure points.
Prefer named locals and explicit branches when steps need comments or logging.

</details>

## Takeaways For Our Own Codebase

> - Do callers need to distinguish missing input from text that's empty after trimming?
>   If so, give those cases separate return values.
> - Check whether your validation code can borrow the input until it needs to build an owned result.
> - Can you follow the types and failure points in each combinator chain?
>   Use named intermediate values or branches where you can't.
> - Check your protocol's whitespace rules before using `trim`.
> - Does Unicode's default uppercasing match how you'll compare or display the result?

## References

- [`Option::map`](../appendix/concept-index.md#option-map): transform a present value without changing `None`.
- [`Option::filter`](../appendix/concept-index.md#option-filter): retain a present value only when it passes a predicate.
- [Option combinator pipelines](../appendix/concept-index.md#option-combinator-pipelines): arrange transformations and rejections as a sequence rather than nested control flow.
- [Borrow before owning](../appendix/concept-index.md#borrow-before-owning): validate a borrowed view, then allocate an owned result only when the value survives.
- [Unicode case conversion](../appendix/concept-index.md#unicode-case-conversion): case mappings can expand text and do not provide locale-aware normalization.
