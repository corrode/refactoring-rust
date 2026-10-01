# 01 · `starts_with_uppercase`

## The problem

Small predicate functions often collect more machinery than they need. This one only
has to inspect the first character of a string, yet its signature takes ownership and
its body manages the empty case, extracts a character, and converts a boolean back
into a boolean. Each step works. Together, they obscure the question the function is
meant to answer.

## Starter code

> **EXERCISE**
>
> Review this function as if it appeared in a pull request. The tests define the
> current contract, including the empty string and a non-ASCII uppercase letter.

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
> Stop here until you have a version you would approve.

## Review

> **REVIEW**
>
> - Does the function need to own the string?
> - Is `chars().nth(0)` the most direct way to ask for the first character?
> - What makes the `unwrap` safe, and can the type returned by the iterator express
>   the empty case instead?
> - Does comparing a boolean with `true` add information?
> - What should the function return for `""`?

The signature is the first concern. The function reads from `s` but never stores or
modifies it. Taking a `String` transfers ownership to the function, so a caller that
needs the string afterwards must clone it or change the surrounding code. A borrowed
`&str` asks for no more than the function needs.

The empty check and the `unwrap` form a pair. `s.len() == 0` proves that a character
exists, then `unwrap` relies on that proof several lines later. The proof is valid,
but we don't need to split the operation across a branch and a panic-capable method.
`chars().next()` returns an `Option<char>`: `None` for an empty string and `Some` for
the first Unicode scalar value.

The final `if` repeats the value of `first.is_uppercase()`. Returning the boolean
expression gives the same result without asking us to follow two branches.

## A possible solution

```rust
{{#include ../../../solutions/01_starts_with_uppercase.rs}}
```

The new signature accepts string literals, borrowed `String` values, and other string
slices without taking ownership. Inside the function, `next()` states that we want
the first item rather than an item at an arbitrary position.

`is_some_and` combines the two remaining questions:

1. Is there a first character?
2. If so, is it uppercase?

For an empty string, `next()` returns `None` and `is_some_and` returns `false`. For a
non-empty string, the predicate runs on the first `char`. The branch and the `unwrap`
disappear because `Option` already represents both cases.

The Unicode test matters here. Byte indexing would be a tempting shortcut, but the
first byte of `"Über"` is not the character `Ü`. Iterating over `char` values preserves
the tested behaviour.

## Alternative approaches

`str::starts_with` can take a character predicate, so the function can be written even
more directly:

```rust
pub fn starts_with_uppercase(s: &str) -> bool {
    s.starts_with(char::is_uppercase)
}
```

This reads well once the predicate form of `starts_with` is familiar. The
`chars().next().is_some_and(...)` version exposes the intermediate `Option`, which is
useful in this early chapter because the empty-input behaviour remains visible. Both
versions satisfy the contract.

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

The only possible stumble is familiarity. `is_some_and` compresses a branch into a
method that some teams will not know yet. That cost is small here because the method
name describes the condition and the tests pin down the empty case. If a team finds
the chain hard to scan, a `match` on `s.chars().next()` is still a sound refactor.

## Quiz

### 1. Which signature best matches a read-only predicate?

Consider these options:

```rust,ignore
fn starts_with_uppercase(s: String) -> bool
fn starts_with_uppercase(s: &String) -> bool
fn starts_with_uppercase(s: &str) -> bool
```

*Answer:* `&str`. It accepts string slices and borrowed `String` values while asking
for neither ownership nor a specific owned-string container.

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
> - When should an empty input be `false`, and when should it return `Option<bool>` or
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
