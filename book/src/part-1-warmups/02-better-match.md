# 02 · `better_match`

## The problem

A `match` expression sometimes repeats an enum constructor even though only the value
inside it changes. This function is correct, but the first two arms make us scan past
`Some` eight times to find the numbers that define each category. Rust patterns can
move those alternatives to the level where the values differ.

## Starter code

> **EXERCISE**
>
> Review the repeated alternatives in this `match`. Preserve every tested result,
> including the distinction between `None` and an unlisted number.

```rust
{{#include ../../../examples/02_better_match.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/02_better_match.rs` without changing
> its behaviour:
>
> ```sh
> cargo test --example 02_better_match
> ```
>
> Stop when each arm emphasizes the values that differ rather than the enum
> constructor they share.

## Review

> **REVIEW**
>
> - Which parts of the first two arms vary, and which parts repeat?
> - Can an or-pattern appear inside an enum variant pattern?
> - Does moving `|` inside `Some` change which values an arm covers?
> - What values can reach the final `_` after the earlier arms have run?
> - Do the labels describe mathematical categories or only the listed values?

Each alternative in the first arm has the same outer constructor. Only the contained
integer changes. Rust applies or-patterns recursively, so `Some(2 | 3)` covers the same
values as `Some(2) | Some(3)`. The second arm has the same opportunity.

The fallback deserves a closer reading. `_` can match any value, including `None`.
Here it only receives unlisted `Some` values because the preceding arm already handles
`None`. Writing `Some(_)` would state that remaining case more precisely and make an
accidentally missing `None` arm fail the exhaustiveness check.

The labels are slightly misleading. This function does not calculate whether any
number is prime or square. It recognizes two fixed sets. The test for `Some(11)` makes
that clear: 11 is prime, but this function returns `"something else"`.

## A possible solution

```rust
{{#include ../../../solutions/02_better_match.rs}}
```

`Some(2 | 3 | 5 | 7)` applies one constructor pattern to four possible payloads. The
second arm does the same for its four values. `None` and the fallback remain unchanged,
so the match still partitions its input into four cases:

1. `Some` containing one of the listed prime values;
2. `Some` containing one of the listed square values;
3. `None`;
4. every other `Some(u32)`.

The two literal sets do not overlap. Moving the `|` operators inward therefore keeps
both the matched values and the arm priority intact.

This is a readability refactor rather than a redesign. The nested form puts the
changing information together and removes repeated constructor syntax. A team that
has not used nested or-patterns may need a moment to read it, which is a reason to
explain the change in review rather than presenting fewer tokens as proof of clarity.

## Edge cases and production concerns

> **EDGE CASE**
>
> The category names are broader than the implementation. Both versions are fixed
> lookup tables.

Other primes such as 13 and squares such as 16 fall through. If the requirement
changes to mathematical classification, we need algorithms, wider tests, and a rule
for values that belong to more than one category. Adding more literals would postpone
that design decision.

Match arms run in source order. If a future edit places the same literal in both sets,
the earlier category wins. The contract should define that priority. Source order
shouldn't decide it by accident.

Returning `&'static str` is enough for display. If downstream code branches on these
labels, an enum would prevent typos and make category changes visible to the compiler.
That would be an API redesign, so it does not belong in this warm-up refactor.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the nested or-patterns with the starter's repeated `Some`
> patterns.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions match the same `Option<u32>` states and encode the same fixed literal sets. |
| Readability | wins | Each arm states `Some` once and groups the payloads that lead to the same result. |
| API design | neutral | The signature, broad `describe` name, string return type, and fallback remain unchanged. |
| Performance | neutral | Both versions perform one fixed pattern match without allocation. |
| Flexibility | neutral | Both versions require a source edit when either category changes. |
| Error reporting | N/A | The function maps every `Option<u32>` to a label and has no failure case. |

Nested or-patterns are standard Rust, but familiarity still matters. A useful review
comment explains why the patterns are equivalent and leaves room for the team to
prefer the expanded form if that is what they can maintain comfortably.

## Quiz

### 1. Should a review require the nested form?

*Answer:* Require unchanged behaviour and clear grouping. The nested form is a good
suggestion because it removes repeated context, but the starter is already correct.
Team familiarity belongs in the readability judgement.

### 2. Should the fallback remain `_`, or become `Some(_)`?

*Answer:* Both behave the same in this match because `None` is handled first.
`Some(_)` communicates the intended remaining case more precisely and would expose an
accidentally removed `None` arm during compilation.

### 3. Product now asks for every prime and square to be recognized. Is another
or-pattern the right change?

*Answer:* No. The current function recognizes a hard-coded set, as the `Some(11)` test
shows. General classification needs arithmetic rules, broader tests, and a decision
about overlapping categories.

## Follow-up questions

> **FOLLOW-UP**
>
> - When does removing repeated constructor syntax help a pattern, and when does
>   nesting make it harder to scan?
> - Would `Some(_)` communicate the fallback better than `_` in your codebase?
> - Does `describe` communicate that this function recognizes fixed categories?
> - When should a small fixed set remain a pattern, and when should it become data or
>   an algorithm?

## Concepts recap

> **CONCEPT**
>
> - [Or-patterns](../appendix/concept-index.md#or-patterns): use `|` when several
>   patterns lead to the same match arm.
> - [Nested patterns](../appendix/concept-index.md#nested-patterns): match alternatives
>   inside a shared enum constructor.
> - [Wildcard patterns](../appendix/concept-index.md#wildcard-patterns): `_` accepts
>   every value not handled by an earlier arm.
> - [Exhaustive matching](../appendix/concept-index.md#exhaustive-matching): cover every
>   possible input with explicit cases and a deliberate fallback.
