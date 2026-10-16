# `nesting`

This function trims optional text and returns it in uppercase when anything remains.
Refactor it without changing its signature or results for absent input, empty input, whitespace-only input, and non-empty text surrounded by whitespace.

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

Missing input and text that's empty after trimming both return `None`.
The latter includes an empty string and a string containing only whitespace.

</details>

<details class="discussion-question">
<summary>Can <code>Option::filter</code> replace the emptiness check?</summary>

Yes, `filter(|t| !t.is_empty())` keeps a present value when the predicate passes and returns `None` otherwise.
Combined with `map`, it expresses the same steps as the branches: trim a present value, keep it if it's non-empty, then uppercase it.

</details>

<details class="discussion-question">
<summary>When do we need an owned <code>String</code>?</summary>

Only when creating the uppercase result.
`to_uppercase` builds the `String` returned to the caller; trimming and checking for empty text can both use borrowed slices.

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

Read the chain as three stages, in order:

1.
   `map(str::trim)` turns `Some(&str)` into a trimmed `Some(&str)` and leaves `None` alone.
2.
   `filter(|t| !t.is_empty())` retains non-empty text and converts empty text to `None`.
3.
   `map(str::to_uppercase)` converts the remaining `&str` into an owned `String`.

These methods call their functions or closures only for `Some`.
Absent input remains `None` through the chain.
If `filter` rejects a value, `to_uppercase` is not called.

`filter` borrows the value inside the option to test its predicate.
The value is an `&str`, so the closure receives `&&str`.
Method-call auto-dereferencing lets `t.is_empty()` work without an explicit dereference.

`Option` has its own `map` and `filter` methods; it is not an `Iterator`.
It can also be converted into an iterator containing zero or one item.

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

Explicit branches are easier to extend with comments, logging, or distinct return values.
Named intermediate values can help when the combinators are unfamiliar.
The canonical chain fits this function because its three steps are short and have no side effects.

Keep `to_uppercase` after `filter` so the function checks the borrowed text before creating an owned result.

### Edge cases and production concerns

> **EDGE CASE**
>
> Unicode uppercasing can change both the character count and the byte count.
> It is not a fixed-length transformation that can happen in place.

For example, uppercasing `"straße"` produces `"STRASSE"`.
`str::to_uppercase` returns a new `String` because the output may require more storage than the input.

`trim` removes leading and trailing Unicode whitespace according to Rust's whitespace rules.
A protocol may define a narrower set, such as ASCII spaces only.
In that case, `trim` can accept input the protocol rejects.
Choose the whitespace rule required by the contract; this exercise keeps the starter's behavior.

Uppercasing is not locale-aware either.
Before using this result for identifiers, security checks, or language shown to users, decide whether Unicode's default case conversion is the normalization your domain intends.

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

> - Must callers distinguish missing input from text that becomes empty after trimming?
> - Which validation steps in your codebase can keep borrowing until the output is built?
> - How long can a combinator chain become before your team prefers named intermediate values?
> - Does your domain use Unicode whitespace, or does a protocol define a narrower rule?
> - Is Unicode default uppercasing appropriate for how the result will be compared or displayed?

## References

- [`Option::map`](../appendix/concept-index.md#option-map): transform a present value without changing `None`.
- [`Option::filter`](../appendix/concept-index.md#option-filter): retain a present value only when it passes a predicate.
- [Option combinator pipelines](../appendix/concept-index.md#option-combinator-pipelines): arrange transformations and rejections as a sequence rather than nested control flow.
- [Borrow before owning](../appendix/concept-index.md#borrow-before-owning): validate a borrowed view, then allocate an owned result only when the value survives.
- [Unicode case conversion](../appendix/concept-index.md#unicode-case-conversion): case mappings can expand text and do not provide locale-aware normalization.
