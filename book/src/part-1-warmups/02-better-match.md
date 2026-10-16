# `better_match`

This function assigns labels to an optional number.
Refactor it without changing its signature or results, including the distinction between `None` and a number that isn't listed.
You can check your changes with `cargo test --example 02_better_match`.

## Code to refactor

```rust,editable
{{#include ../../../examples/02_better_match.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/02_better_match.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/02_better_match.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What changes across the first two arms, and what stays the same?</summary>

Within each arm, every alternative has the same outer `Some` constructor and returns the same label.
Only the integer inside the constructor changes.

</details>

<details class="discussion-question">
<summary>Can an enum variant pattern contain an or-pattern?</summary>

Yes, or-patterns work recursively.
You can put the alternatives inside the shared constructor, as in `Some(2 | 3)`.

</details>

<details class="discussion-question">
<summary>After moving <code>|</code> inside <code>Some</code>, do the arms still accept exactly the same values?</summary>

Yes, `Some(2 | 3)` accepts exactly the values accepted by `Some(2) | Some(3)`.
The same reasoning applies to both arms: moving the alternatives inside `Some` does not change which integers match.

</details>

<details class="discussion-question">
<summary>Which inputs can reach the final <code>_</code> once the earlier arms have run?</summary>

Only unlisted `Some` values remain.
`_` can match anything, including `None`, but `None` never reaches it here because an earlier arm handles that case.
Writing `Some(_)` would state the remaining case more precisely, and the compiler would reject the match if a later edit accidentally removed the `None` arm.

</details>

<details class="discussion-question">
<summary>Are the labels mathematical categories, or names for the listed values?</summary>

The labels apply only to the listed values.
The function does not calculate whether any given number is prime or square.
The `Some(11)` test confirms this: 11 is prime, but the function returns `"something else"`.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/02_better_match.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/02_better_match.rs:tests}}
</script>

`Some(2 | 3 | 5 | 7)` applies one constructor pattern to four possible payloads.
The next arm does the same for its four values.
Since `None` and the fallback are unchanged, the match still divides every input into the same four cases:

1. `Some` containing one of the listed prime values;
2. `Some` containing one of the listed square values;
3. `None`;
4. every other `Some(u32)`.

Each rewritten arm matches the same values as before, and the arm order is unchanged.
The signature and results stay the same; only the pattern syntax changes.

### Edge cases and production concerns

> **EDGE CASE**
>
> The names suggest broader categories than the implementation provides.
> Both versions are fixed lookup tables.

Primes such as 13 and squares such as 16 still return `"something else"`.
Recognizing all primes and squares would require arithmetic checks and broader tests, not just more literals.
These two mathematical categories do not overlap: no prime is a perfect square.

Match arms are evaluated in source order.
If a later change places one literal in both sets, the earlier category wins.
That priority belongs in the contract rather than emerging accidentally from the order of the arms.

A `&'static str` is adequate for display.
If downstream code starts branching on these labels, an enum would rule out typos and make category changes visible to the compiler.
That would be a separate API change, not part of this pattern refactor.

### Trade-offs

Compared with the original: **+** advantage, **=** comparable, **−** disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Nested or-patterns |
|---|---|---|
| Type-safety | Exhaustive match; string labels | = Same states and string labels |
| Readability | Repeats constructor for each literal | + Less repetition, if nesting familiar |
| API design | Fixed lookup; wildcard fallback | = Same signature and fallback |
| Performance | Fixed match; no allocation | = Same matching work; no allocation |
| Flexibility | Categories require source edits | = Categories still require source edits |
| Error reporting | Every input maps to a label | N/A |

Keeping the expanded patterns is reasonable if your team finds them easier to scan.
Neither implementation recognizes primes or squares beyond the listed values.
The suggested `Some(_)` fallback is a separate change: unlike `_`, it would let the compiler catch a missing `None` arm.

</details>

## Takeaways For Our Own Codebase

> - When does removing a repeated constructor make a pattern easier to read, and when does nesting make it harder to scan?
> - In your codebase, would `Some(_)` communicate the fallback more clearly than `_`?
> - Does the name `describe` reveal that this function recognizes fixed categories?
> - When should a small fixed set stay in a pattern, and when should it become data or an algorithm?

## References

- [Or-patterns](../appendix/concept-index.md#or-patterns): use `|` when several patterns should run the same match arm.
- [Nested patterns](../appendix/concept-index.md#nested-patterns): place alternatives inside their shared enum constructor.
- [Wildcard patterns](../appendix/concept-index.md#wildcard-patterns): `_` accepts every value not consumed by an earlier arm.
- [Exhaustive matching](../appendix/concept-index.md#exhaustive-matching): account for every possible input with explicit cases and an intentional fallback.
