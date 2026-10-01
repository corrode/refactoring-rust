# 02 · `better_match`

## The problem

A `match` expression can repeat an enum constructor even when only the value inside it
changes. This function is correct, but the first two arms make you scan past `Some`
eight times to find the numbers that define each category. Rust patterns let us put
the alternatives where the values differ.

## Starter code

> **EXERCISE**
>
> Review the repeated alternatives in this `match`. Keep every tested result, including
> the distinction between `None` and a number that isn't listed.

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
> Stop when each arm puts the values that differ first and leaves the shared enum
> constructor in the background.

## Review

> **REVIEW**
>
> - Which parts of the first two arms change, and which parts repeat?
> - Can you put an or-pattern inside an enum variant pattern?
> - If you move `|` inside `Some`, do the arms still cover the same values?
> - Which values can reach the final `_` after the earlier arms run?
> - Do the labels describe mathematical categories, or only the listed values?

In the first arm, every alternative has the same outer constructor. Only the contained
integer changes. Rust applies or-patterns recursively, so `Some(2 | 3)` covers exactly
the same values as `Some(2) | Some(3)`. The second arm can use the same refactor.

Now inspect the fallback. `_` can match any value, including `None`. In this function,
it only receives unlisted `Some` values because an earlier arm already handles `None`.
Writing `Some(_)` would describe that remaining case more precisely. It would also make
the exhaustiveness check fail if someone accidentally removed the `None` arm.

The labels invite another question: are these mathematical categories? They aren't.
The function recognizes two fixed sets rather than calculating whether a number is
prime or square. The `Some(11)` test proves the distinction. Although 11 is prime, this
function returns `"something else"`.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/02_better_match.rs}}
```

`Some(2 | 3 | 5 | 7)` applies one constructor pattern to four possible payloads. The
second arm does the same for its four values. `None` and the fallback stay unchanged,
so the match still partitions the input into four cases:

1. `Some` containing one of the listed prime values;
2. `Some` containing one of the listed square values;
3. `None`;
4. every other `Some(u32)`.

The two literal sets don't overlap. Moving the `|` operators inward therefore preserves
both the matched values and the priority of the arms.

This is a readability refactor. The function's design stays the same. The nested form
puts the changing information together and removes repeated constructor syntax. If your
team hasn't used nested or-patterns before, the new form may take a moment to parse. A
review should explain why the patterns are equivalent rather than treating fewer tokens
as proof that the code is clearer.

</details>

## Edge cases and production concerns

> **EDGE CASE**
>
> The category names promise more than the implementation provides. Both versions are
> fixed lookup tables.

Other primes such as 13 and squares such as 16 fall through to `"something else"`. If
the requirement changes to mathematical classification, you'll need algorithms, wider
tests, and a rule for values that belong to more than one category. Adding more literals
would only postpone that design decision.

Match arms run in source order. If a future edit puts the same literal in both sets, the
earlier category wins. The contract should define that priority rather than leaving
source order to decide it by accident.

Returning `&'static str` works for display. If downstream code branches on these labels,
an enum would prevent typos and make category changes visible to the compiler. That is
an API redesign, so it doesn't belong in this warm-up refactor.

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

Nested or-patterns are standard Rust, but your team's familiarity still matters. A
useful review comment explains why the patterns are equivalent. I wouldn't make the
nested form a hard requirement if the team can maintain the expanded version more
comfortably.

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
> - When does removing repeated constructor syntax help you read a pattern, and when
>   does nesting make the pattern harder to scan?
> - Would `Some(_)` communicate the fallback better than `_` in your codebase?
> - Does `describe` tell you that this function recognizes fixed categories?
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
