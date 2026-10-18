# `better_match`

The next function assigns "labels" to numbers.

The (somewhat arbitrary) rules are:

- If the number is prime (2, 3, 5, or 7), return `"prime"`.
- If the number is a perfect square (0, 1, 4, or 9), return `"square"`.
- If the input is `None`, return `"nothing"`.
- For any other number, return `"something else"`.

It works, but is there a way to reduce the repetition?

## Code to refactor

```rust,editable
{{#include ../../../examples/02_better_match.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/02_better_match.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What changes across the match arms, and what stays the same?</summary>

Only the integer inside the constructor changes!
But the `Some` constructor and the returned label are the same for each arm.

</details>

<details class="discussion-question">
<summary>Can an enum variant contain an or-pattern?</summary>

Or-patterns work recursively, so yes!
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

`Some(2 | 3 | 5 | 7)` applies one constructor pattern to the four primes.
The next arm does the same for the square values. 
Since `None` and the fallback are unchanged, we still have four cases: 

1. `Some` containing one of the listed prime values;
2. `Some` containing one of the listed square values;
3. `None`;
4. every other `Some(u32)`.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Nested or-patterns |
|---|---|---|
| Type-safety | Exhaustive match; string labels | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same states and string labels |
| Readability | Repeats constructor for each literal | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Less repetition, if nesting familiar |
| API design | Fixed lookup; wildcard fallback | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same signature and fallback |
| Performance | Fixed match; no allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same matching work; no allocation |
| Flexibility | Categories require source edits | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Categories still require source edits |
| Error reporting | Every input maps to a label | N/A |

Note that keeping the expanded patterns is reasonable if your team finds them easier to scan.

</details>

## Takeaways For Our Own Codebase

> - How can you make pattern matching easier to read?
> - Can your fallbacks be more precise than `_`? E.g. maybe use `Some(_)` instead of `_` to indicate that the fallback only applies to `Some` values?
> - Always be on the lookout for better function names. In the example above, `describe` could be replaced with something more descriptive, like `label_number`.
> - How do you decide whether to use pattern matching or turn the logic into its own function?

## References

- [Or-patterns](../appendix/concept-index.md#or-patterns): use `|` when several patterns should run the same match arm.
- [Nested patterns](../appendix/concept-index.md#nested-patterns): place alternatives inside their shared enum constructor.
- [Wildcard patterns](../appendix/concept-index.md#wildcard-patterns): `_` accepts every value not consumed by an earlier arm.
- [Exhaustive matching](../appendix/concept-index.md#exhaustive-matching): account for every possible input with explicit cases and an intentional fallback.
