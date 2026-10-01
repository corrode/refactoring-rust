# 06 · `nesting`

## The problem

An optional text value passes through three decisions: trim surrounding whitespace,
reject an empty result, then uppercase what remains. The starter spreads that sequence
across nested `if` expressions and repeated early returns. Its behaviour is correct, but
the branches make it harder to see that the value stays borrowed until the final step
creates an owned result.

## Starter code

> **EXERCISE**
>
> Read the branches as a sequence of data transformations. Preserve all four tested
> cases: absent input, empty input, whitespace-only input, and non-empty text surrounded
> by whitespace.

```rust,editable
{{#include ../../../examples/06_nesting.rs:listing}}
```

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

Start with the values. The input is `None` or `Some(&str)`. When text is present, `trim`
returns another `&str` borrowed from the same input. The function rejects that slice if
it is empty. Only a non-empty slice reaches `to_uppercase`, which creates the `String`
returned to the caller.

The outer `if let` asks whether a value is present. The inner `if` applies a predicate to
that value. Both failure paths return `None`. `Option` combinators express the same sequence:
transform a present value, retain it when a predicate passes, then transform it once
more.

Does `trim` allocate? No. It returns a subslice, so validation can continue with borrowed
text. `to_uppercase` returns a `String`, which is where allocation enters. Leaving that
step until last means absent, empty, and whitespace-only inputs never allocate an owned
result.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/06_nesting.rs:listing}}
```

Read the chain as three stages, in order:

1. `map(str::trim)` turns `Some(&str)` into a trimmed `Some(&str)` and leaves `None`
   alone.
2. `filter(|t| !t.is_empty())` retains non-empty text and converts empty text to `None`.
3. `map(str::to_uppercase)` converts the remaining `&str` into an owned `String`.

A combinator runs only while the option is `Some`. Absent input skips the entire chain.
Once `filter` rejects a value, the allocating final `map` does not run.

There is one type detail worth noticing. `filter` borrows the value inside the option to
test its predicate. That value is already an `&str`, so the closure receives a reference
to a reference. Method-call auto-dereferencing is why `t.is_empty()` works without
spelling out the extra layer.

This looks like an iterator pipeline, but `Option` is not an `Iterator`. It has its own
`map` and `filter` methods, and it can also be converted into an iterator containing zero
or one item. The useful connection is the vocabulary: each stage transforms or rejects
a value without introducing another nested branch.

</details>

## Alternative approaches

A combinator chain is not the only clear answer. You can remove the nesting while keeping
each decision explicit:

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

I would choose this form when the failure paths need comments, logging, or distinct
return values. Named intermediate values can also be easier to review while a team is
learning the combinators. The canonical chain works well here because every stage is
short, pure, and appears in the same order as the contract.

Do not move `to_uppercase` ahead of `filter`. That would allocate before the function
knows whether it will discard the trimmed input. It would also give up the useful
progression from borrowed validation to owned output.

## Edge cases and production concerns

> **EDGE CASE**
>
> Unicode uppercasing can change both the character count and the byte count. It is not a
> fixed-length transformation that can happen in place.

For example, uppercasing `"straße"` produces `"STRASSE"`. `str::to_uppercase` returns a
new `String` because the output may require more storage than the input. As in the
starter, the canonical solution allocates that string only after trimming and filtering
leave a value to convert.

`trim` removes leading and trailing Unicode whitespace according to Rust's whitespace
rules. A protocol may define a narrower set, such as ASCII spaces only. In that case,
`trim` can accept input the protocol rejects. That is a contract decision, not a reason
to broaden this exercise.

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

The readability win has a limit. A long chain of closures can obscure intermediate types
and make failure points harder to inspect. Here each operation has one job, and their
order mirrors the contract. Once that stops being true, use named locals and explicit
branches.

## Quiz

### 1. Should `to_uppercase` run before the empty check to keep all transformations together?

*Answer:* No. Trimming and the empty check work on borrowed `&str` values. Uppercasing
first would allocate a `String` that the function may immediately discard. Keep the
owned transformation after filtering unless the contract requires another order.

### 2. Should a review reject the explicit version because combinators are more idiomatic?

*Answer:* No. Explicit control flow is easier to extend when failure paths need logging,
different errors, or more involved decisions. The combinator chain is a good fit here
because three short stages follow the contract in order, not because explicit branches
are unsound.

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
