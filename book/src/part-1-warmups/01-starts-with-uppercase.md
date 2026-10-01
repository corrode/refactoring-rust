# 01 · `starts_with_uppercase`

## The problem

A small predicate can end up doing far more work than its question requires. This one
only needs to inspect the first character of a string. Instead, it takes ownership,
handles empty input by hand, extracts a character, and turns a boolean into the same
boolean through another branch. Every step works, but the machinery gets between you
and the question: does this string start with an uppercase character?

## Starter code

> **EXERCISE**
>
> Review this function as if you found it in a pull request. The tests define the
> current contract, including the expected results for empty input and a non-ASCII
> uppercase letter.

```rust
{{#include ../../../examples/01_starts_with_uppercase.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/01_starts_with_uppercase.rs` without
> changing the behaviour:
>
> ```sh
> cargo test --example 01_starts_with_uppercase
> ```
>
> Stop when you have a version you would approve in review.

## Review

> **REVIEW**
>
> - Does the function need to own the string?
> - Is `chars().nth(0)` the most direct way to ask for the first character?
> - Why is the `unwrap` safe, and can the iterator's return type represent the empty
>   case instead?
> - Does comparing a boolean with `true` tell you anything new?
> - What should the function return for `""`?

Start with the signature. The function reads `s`, but it never stores or modifies it.
Taking a `String` transfers ownership, so callers that still need the string must clone
it or rearrange their code. A borrowed `&str` asks for exactly what the function needs:
read access to some text.

Next, look at how the function handles an empty string. The `s.len() == 0` check proves
that a character exists before `unwrap` relies on that fact several lines later. The
proof is valid, but the code splits one operation across a branch and a panic-capable
method. `chars().next()` keeps the two cases together by returning `Option<char>`:
`None` for an empty string and `Some` for the first Unicode scalar value.

The final `if` returns the value of `first.is_uppercase()` through two more branches.
Returning that boolean expression directly gives the same result and leaves less code
to trace.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/01_starts_with_uppercase.rs}}
```

The new signature accepts string literals, borrowed `String` values, and other string
slices without taking ownership. In the body, `next()` says that we want the first item,
not an item at an arbitrary position.

`is_some_and` answers the two remaining questions in sequence:

1. Is there a first character?
2. If there is, is it uppercase?

When the input is empty, `next()` returns `None`, so `is_some_and` returns `false`.
Otherwise, it runs the predicate on the first `char`. We no longer need a separate
branch or an `unwrap` because `Option` already represents both cases.

The Unicode test protects a detail you could easily lose during this refactor. Byte
indexing may look like a shortcut, but the first byte of `"Über"` is not the character
`Ü`. Iterating over `char` values preserves the tested behaviour.

</details>

## Alternative approaches

`str::starts_with` also accepts a character predicate, which makes an even more direct
version possible:

```rust
pub fn starts_with_uppercase(s: &str) -> bool {
    s.starts_with(char::is_uppercase)
}
```

This version reads well once you know the predicate form of `starts_with`. The
`chars().next().is_some_and(...)` version leaves the intermediate `Option` visible,
which is useful this early in the book because you can see how empty input becomes
`false`. Both versions satisfy the contract.

## Trade-offs

> **TRADE-OFF**
>
> We score the possible solution relative to the starter. Other possible
> implementations do not affect the score.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept any string content and return a `bool`; the refactor does not encode a new invariant in a type. |
| Readability | wins | The body follows the question in evaluation order: take the first character, then test it. It removes two branches and a repeated boolean. |
| API design | wins | `&str` communicates that the function only reads the input and does not need ownership. |
| Performance | wins | Callers can pass borrowed text without allocating or cloning a `String`; character inspection remains constant time in both versions. |
| Flexibility | wins | The borrowed parameter accepts both string slices and borrowed `String` values, while the starter requires an owned `String`. |
| Error reporting | N/A | Empty input is a normal `false` result under the stated contract, so neither version has an error to report. |

The main cost is familiarity. `is_some_and` replaces a branch with a method some teams
won't know yet. I think that cost is small here: the name describes the condition, and
the tests pin down what happens for empty input. If your team finds the chain hard to
scan, a `match` on `s.chars().next()` is still a sound refactor.

## Quiz

### 1. Which signature best matches a read-only predicate?

Consider these options:

```rust,ignore
fn starts_with_uppercase(s: String) -> bool
fn starts_with_uppercase(s: &String) -> bool
fn starts_with_uppercase(s: &str) -> bool
```

*Answer:* `&str`. It accepts string slices and borrowed `String` values without asking
for ownership or a specific owned-string container.

### 2. Should a review require replacing the starter with the one-line solution?

*Answer:* Require the ownership fix if needless ownership would burden real callers.
Treat the exact body as a suggestion. A direct `match` can preserve the same contract
without `is_some_and`, and team familiarity affects readability.

### 3. A teammate proposes checking `s.as_bytes()[0].is_ascii_uppercase()`. What changes?

*Answer:* The code can panic on an empty string and changes the contract from Unicode
uppercase characters to ASCII uppercase bytes. The current tests expose both as
correctness regressions. This goes beyond style preference.

## Follow-up questions

> **FOLLOW-UP**
>
> - When should empty input produce `false`, and when should it return `Option<bool>` or
>   an error?
> - Which predicates in your codebase take ownership even though they only inspect
>   their input?
> - Does your team prefer explicit `match` expressions or combinators such as
>   `is_some_and` for small `Option` checks? What makes either version easier to review?

## Concepts recap

> **CONCEPT**
>
> - [`&str` and borrowed parameters](../appendix/concept-index.md#borrowed-string-slices):
>   accept a view of text when a function only needs to read it.
> - [`Iterator::next`](../appendix/concept-index.md#iterator-next): ask directly for the
>   first item and receive `Option` when the iterator may be empty.
> - [`Option::is_some_and`](../appendix/concept-index.md#option-is-some-and): combine
>   presence and a predicate without an `unwrap`.
> - [Unicode scalar values](../appendix/concept-index.md#unicode-scalar-values): Rust
>   strings are UTF-8, so the first byte and the first `char` are different questions.
