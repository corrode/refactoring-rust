# 01 · `starts_with_uppercase`

## The problem

This function asks one small question: does the string begin with an uppercase
character? Before it can answer, though, it takes ownership of the string, checks for
empty input itself, pulls a character from an iterator, and routes a boolean through
another branch. Each step works. Together, they make the code harder to read and ask
more from callers than the predicate needs.

## Starter code

> **EXERCISE**
>
> Imagine this function appears in a pull request. Review it against the tests, which
> define the current contract for empty input and for a non-ASCII uppercase letter.

```rust
{{#rustdoc_include ../../../examples/01_starts_with_uppercase.rs:listing}}
```

> **TRY THIS**
>
> Run the starter tests. Then refactor `examples/01_starts_with_uppercase.rs` while
> preserving its behaviour:
>
> ```sh
> cargo test --example 01_starts_with_uppercase
> ```
>
> Stop at the version you would approve in a code review.

## Review

> **REVIEW**
>
> - Does answering this question require ownership of the string?
> - Is `chars().nth(0)` the clearest way to request the first character?
> - What makes the `unwrap` safe, and can the iterator's return type carry the empty
>   case instead?
> - Does comparing a boolean with `true` add information?
> - What result belongs to `""`?

The first source of friction is the signature. The function only reads `s`; it neither
stores nor changes it. Accepting a `String` still transfers ownership, which forces a
caller that needs the string afterward to clone it or reorganize its code. A borrowed
`&str` states the narrower requirement: read access to text.

The empty-string check and the `unwrap` are also two halves of the same operation.
`s.len() == 0` establishes that a character exists before `unwrap` depends on that fact
a few lines later. The reasoning is valid, but it is spread across a branch and a method
that can panic. `chars().next()` keeps the cases together in an `Option<char>`: `None`
for an empty string, `Some` for its first Unicode scalar value.

Finally, `first.is_uppercase()` already produces the result. Sending that boolean
through another `if` changes nothing, so returning it directly leaves less control flow
to inspect.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#rustdoc_include ../../../solutions/01_starts_with_uppercase.rs:listing}}
```

The borrowed signature accepts string literals, borrowed `String` values, and other
string slices without taking ownership. Inside the function, `next()` asks for the first
item rather than an item at an arbitrary position.

`is_some_and` then answers two questions in order:

1. Is there a first character?
2. If there is, is it uppercase?

For empty input, `next()` returns `None` and `is_some_and` returns `false`. Otherwise,
the predicate receives the first `char`. `Option` now carries both cases, so the code no
longer needs a separate branch or an `unwrap`.

The Unicode test guards a detail that a shorter refactor could still get wrong. Byte
indexing looks tempting here, but the first byte of `"Über"` is not the character `Ü`.
Walking through `char` values preserves the behaviour covered by the test.

</details>

## Alternative approaches

`str::starts_with` can take a character predicate, so this version expresses the same
contract even more directly:

```rust
pub fn starts_with_uppercase(s: &str) -> bool {
    s.starts_with(char::is_uppercase)
}
```

That reads well if your team knows the predicate form of `starts_with`. At this point in
the book, I prefer `chars().next().is_some_and(...)` because it keeps the intermediate
`Option` visible. You can see exactly why empty input becomes `false`. This is a teaching
preference, not a correctness claim. Both versions satisfy the contract.

## Trade-offs

> **TRADE-OFF**
>
> The scores compare the possible solution with the starter. Other valid
> implementations don't change that comparison.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept arbitrary string content and return a `bool`; the refactor adds no type-level invariant. |
| Readability | wins | The body follows the question in order: get the first character, then test it. Two branches and a repeated boolean disappear. |
| API design | wins | `&str` says that the function reads the input without taking ownership. |
| Performance | wins | A caller can borrow text instead of allocating or cloning a `String`; inspecting the first character remains constant time in both versions. |
| Flexibility | wins | The borrowed parameter accepts string slices and borrowed `String` values, whereas the starter requires an owned `String`. |
| Error reporting | N/A | Under this contract, empty input is an ordinary `false` result, so neither version has an error to report. |

The real cost is familiarity. `is_some_and` replaces a branch with a method that some
teams won't recognize yet. I think the trade is worthwhile here because the name states
the condition and the tests settle the empty-input case. If the chain slows your team
down, matching on `s.chars().next()` is an equally sound refactor.

## Quiz

### 1. Which signature best fits a read-only predicate?

Consider these options:

```rust,ignore
fn starts_with_uppercase(s: String) -> bool
fn starts_with_uppercase(s: &String) -> bool
fn starts_with_uppercase(s: &str) -> bool
```

*Answer:* `&str`. It accepts slices and borrowed `String` values without requiring
ownership or tying the function to one owned string container.

### 2. Should a review insist on the one-line solution?

*Answer:* Insist on fixing needless ownership when it creates work for real callers.
Treat the exact body as a recommendation. A direct `match` preserves the same contract
without `is_some_and`, and a team's familiarity changes what is easiest to read.

### 3. A teammate suggests `s.as_bytes()[0].is_ascii_uppercase()`. What would change?

*Answer:* It can panic on an empty string, and it replaces Unicode uppercase characters
with ASCII uppercase bytes as the contract. The existing tests catch both correctness
regressions. This is not a matter of style.

## Follow-up questions

> **FOLLOW-UP**
>
> - When is empty input a normal `false`, and when should it produce `Option<bool>` or
>   an error?
> - Which predicates in your codebase own values that they only inspect?
> - For small `Option` checks, does your team find an explicit `match` or a combinator
>   such as `is_some_and` easier to review? What makes the difference?

## Concepts recap

> **CONCEPT**
>
> - [`&str` and borrowed parameters](../appendix/concept-index.md#borrowed-string-slices):
>   take a view of text when the function only reads it.
> - [`Iterator::next`](../appendix/concept-index.md#iterator-next): request the first
>   item directly and receive `Option` when the iterator might be empty.
> - [`Option::is_some_and`](../appendix/concept-index.md#option-is-some-and): test both
>   presence and a condition without calling `unwrap`.
> - [Unicode scalar values](../appendix/concept-index.md#unicode-scalar-values): a Rust
>   string is UTF-8, so its first byte and its first `char` answer different questions.
