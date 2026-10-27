# `person_info`

This function formats a person's name and an age-based label.
Can you make the match easier to read?
Keep the function signature, the `Person` fields, and the exact output strings unchanged.

## Code to refactor

```rust,editable
{{#include ../../../examples/04_person_info.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/04_person_info.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 04_person_info
```

</div>

## Discussion

<details class="discussion-question">
<summary>Can a pattern test <code>age</code> and bind <code>name</code> at the same time?</summary>

A struct pattern can match one field against a range and bind another field for use in the arm.
`Person { name, age: 0..=19 }` does both: it binds `name` only for people whose age matches the inclusive range.
The `name` field uses shorthand for `name: name`.

</details>

<details class="discussion-question">
<summary>Does matching <code>&Person</code> move its <code>String</code>?</summary>

No.
Match ergonomics let a `Person` pattern match the borrowed `&Person`, binding `name` as `&String` rather than moving the field.
The caller keeps ownership of the person and can use it again afterward.
The original `format!("{} ...", person.name)` also borrows the name for formatting, so neither version clones it.
Both still construct a new output `String`.

</details>

<details class="discussion-question">
<summary>What does <code>..</code> mean in the last struct pattern?</summary>

`Person { name, .. }` binds the name and ignores the remaining fields, which here means `age`.
It does not itself describe an age range; earlier arms determine which ages reach it.
The first two patterns name every field without `..`, so adding a field to `Person` would require updating those patterns or explicitly ignoring the new field.

</details>

<details class="discussion-question">
<summary>Should we correct the age labels while changing the patterns?</summary>

Not in this refactor.
The supplied rules call everyone from zero through nineteen a teenager and everyone forty or older middle-aged.
Those are the example's existing labels, not a realistic demographic classification.
Changing the ranges or labels is a separate policy change; the boundary tests deliberately preserve them here.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/04_person_info.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/04_person_info.rs:tests}}
</script>

Matching the whole person brings the age condition and the name binding into the same pattern.
Each arm then uses its local `name` binding in `format!("{name} ...")` instead of reaching through `person.name`.
The inclusive ranges, punctuation, and fallback label stay the same.

Matching only `person.age` remains reasonable when it makes the decision easier to follow.
Destructuring is useful when naming the fields used by an arm clarifies that arm; it is not a performance optimization here.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Struct patterns |
|---|---|---|
| Type-safety | Exhaustive age match; fixed labels | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same age domain and labels |
| Readability | Matches age; repeats `person.name` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Binds the name beside its condition; longer patterns |
| API design | Borrows `Person`; returns `String` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same fields, signature, and output |
| Performance | Borrows the name; formats an owned string | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same formatting work; no name clone |
| Flexibility | Adding fields leaves the match unchanged | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> New fields require updating the first two patterns |
| Error reporting | Every `u8` age gets a description | N/A |

The field-update requirement makes an explicit choice visible: list every field, or use `..` when extra fields should not affect the operation.

</details>

## Takeaways For Our Own Codebase

> - Can a pattern bind the fields an arm needs while retaining a borrow of the original value?
> - Which labels and ranges are existing policy rather than incidental syntax to clean up?
> - Should adding a field force a review of the match, or should `..` explicitly ignore it?

## References

- [Struct patterns](../appendix/concept-index.md#struct-patterns): test fields and bind the values an arm needs.
- [Range patterns](../appendix/concept-index.md#range-patterns): match inclusive bounds with `..=`.
- [Match ergonomics](../appendix/concept-index.md#match-ergonomics): destructure borrowed values without moving their fields.
