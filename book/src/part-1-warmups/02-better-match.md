# 02 · `better_match`

## The problem

The first two arms of this `match` repeat `Some` eight times. That repetition is not a
correctness bug, but it hides the part you need to compare: the numbers assigned to each
category. Rust patterns can keep the shared enum constructor in one place and put the
alternatives where the values differ.

## Starter code

> **EXERCISE**
>
> Review the repeated alternatives in this `match`. Preserve every tested result,
> especially the difference between `None` and a number that isn't listed.

```rust,editable
{{#include ../../../examples/02_better_match.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/02_better_match.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/02_better_match.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests. Then refactor `examples/02_better_match.rs` without changing
> its behaviour:
>
> ```sh
> cargo test --example 02_better_match
> ```
>
> Stop when each arm foregrounds the values that differ and leaves the shared enum
> constructor as context.

## Review

> **REVIEW**
>
> - What changes across the first two arms, and what stays the same?
> - Can an enum variant pattern contain an or-pattern?
> - After moving `|` inside `Some`, do the arms still accept exactly the same values?
> - Which inputs can reach the final `_` once the earlier arms have run?
> - Are the labels mathematical categories, or names for the listed values?

Every alternative in the first arm has the same outer constructor. Only the integer
inside it changes. Or-patterns work recursively, which means `Some(2 | 3)` accepts
exactly the values accepted by `Some(2) | Some(3)`. The same reasoning applies to the
second arm.

The fallback deserves a separate look. `_` can match anything, including `None`, but
`None` never reaches it here because an earlier arm handles that case. In this match,
only unlisted `Some` values remain. Writing `Some(_)` would state that more precisely,
and the compiler would reject the match if a later edit accidentally removed the
`None` arm.

There is a more important limitation behind the labels. These are not mathematical
classifications. The function recognizes two fixed sets; it does not calculate whether
a number is prime or square. The `Some(11)` test makes that contract visible: 11 is
prime, but this function still returns `"something else"`.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/02_better_match.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/02_better_match.rs:tests}}
</script>

`Some(2 | 3 | 5 | 7)` applies one constructor pattern to four possible payloads. The
next arm does the same for its four values. Since `None` and the fallback are unchanged,
the match still divides every input into the same four cases:

1. `Some` containing one of the listed prime values;
2. `Some` containing one of the listed square values;
3. `None`;
4. every other `Some(u32)`.

The two sets of literals do not overlap. Moving the `|` operators inward therefore
preserves both which values match and which arm has priority.

Nothing about the function's design changes. This is a readability refactor: the nested
patterns collect the varying information and remove repeated constructor syntax. The
objection is fair, though. If nested or-patterns are new to your team, the compact
version may take longer to parse at first. A useful review explains the equivalence. A
smaller token count alone does not prove that code is easier to read.

</details>

## Edge cases and production concerns

> **EDGE CASE**
>
> The names suggest broader categories than the implementation provides. Both versions
> are fixed lookup tables.

Primes such as 13 and squares such as 16 still fall through to `"something else"`. If
the requirement becomes mathematical classification, the implementation needs
algorithms, wider tests, and a rule for numbers that belong to both categories. A longer
list of literals would defer that design decision, not resolve it.

Match arms are evaluated in source order. If a later change places one literal in both
sets, the earlier category wins. That priority belongs in the contract rather than
emerging accidentally from the order of the arms.

A `&'static str` is adequate for display. If downstream code starts branching on these
labels, an enum would rule out typos and make category changes visible to the compiler.
That would redesign the API, so I would not fold it into this warm-up refactor.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare nested or-patterns with the repeated `Some` patterns in the
> starter.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions match the same `Option<u32>` states and represent the same fixed sets of literals. |
| Readability | wins | Each arm writes `Some` once and keeps payloads with the same result together. |
| API design | neutral | The signature, broad `describe` name, string return type, and fallback do not change. |
| Performance | neutral | Both versions perform one fixed pattern match and allocate nothing. |
| Flexibility | neutral | Changing either category still requires a source edit in both versions. |
| Error reporting | N/A | Every `Option<u32>` maps to a label, so the function has no failure case. |

Nested or-patterns are ordinary Rust, but familiarity still affects readability. A good
review comment shows why the two forms match the same inputs. I wouldn't require the
nested version when a team can maintain the expanded form more comfortably.

## Quiz

### 1. Should code review require the nested form?

*Answer:* Require preserved behaviour and clear grouping. The nested form is worth
suggesting because it removes repeated context, but the starter is already correct.
What the team knows belongs in the readability decision.

### 2. Should the fallback stay `_`, or change to `Some(_)`?

*Answer:* They behave identically here because an earlier arm handles `None`.
`Some(_)` names the intended remaining case more precisely, and removing the `None` arm
by accident would then produce a compiler error.

### 3. Product asks to recognize every prime and square. Is another or-pattern the right
change?

*Answer:* No. As the `Some(11)` test demonstrates, this function recognizes a hard-coded
set. General classification calls for arithmetic rules, broader tests, and a decision
about values that belong to both categories.

## Follow-up questions

> **FOLLOW-UP**
>
> - When does removing a repeated constructor make a pattern easier to read, and when
>   does nesting make it harder to scan?
> - In your codebase, would `Some(_)` communicate the fallback more clearly than `_`?
> - Does the name `describe` reveal that this function recognizes fixed categories?
> - When should a small fixed set stay in a pattern, and when should it become data or
>   an algorithm?

## Concepts recap

> **CONCEPT**
>
> - [Or-patterns](../appendix/concept-index.md#or-patterns): use `|` when several
>   patterns should run the same match arm.
> - [Nested patterns](../appendix/concept-index.md#nested-patterns): place alternatives
>   inside their shared enum constructor.
> - [Wildcard patterns](../appendix/concept-index.md#wildcard-patterns): `_` accepts
>   every value not consumed by an earlier arm.
> - [Exhaustive matching](../appendix/concept-index.md#exhaustive-matching): account for
>   every possible input with explicit cases and an intentional fallback.
