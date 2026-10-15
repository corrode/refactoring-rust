# 01 · `starts_with_uppercase`

## Starter code

> **EXERCISE**
>
> Review the function against the tests, which define the current contract for empty input
> and for a non-ASCII uppercase letter.

```rust,editable
{{#include ../../../examples/01_starts_with_uppercase.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/01_starts_with_uppercase.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/01_starts_with_uppercase.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests. Then refactor `examples/01_starts_with_uppercase.rs` while
> preserving its behaviour:
>
> ```sh
> cargo test --example 01_starts_with_uppercase
> ```
>
> Choose a version that makes the borrowing requirement and empty-input result clear.

## Review

> **REVIEW**
>
> - Does answering this question require ownership of the string?
> - Is `chars().nth(0)` the clearest way to request the first character?
> - What makes the `unwrap` safe, and can the iterator's return type carry the empty
>   case instead?
> - Does comparing a boolean with `true` add information?
> - What result belongs to `""`?

The function only reads `s`; it neither stores nor changes it. Taking a `String`
transfers ownership, so a caller that needs the string afterward must clone it or change
its code. A borrowed `&str` gives the function the read access it needs without taking
ownership.

The early return for `s.len() == 0` makes the later `unwrap` safe: a nonempty UTF-8
string has at least one character. Using `chars().next()` and handling its
`Option<char>` avoids the `unwrap`. The result is `None` for an empty string and `Some`
for its first Unicode scalar value.

`first.is_uppercase()` already returns a boolean. Comparing it with `true` and then
returning `true` or `false` from an `if` is unnecessary.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/01_starts_with_uppercase.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_starts_with_uppercase.rs:tests}}
</script>

The borrowed signature accepts string literals, borrowed `String` values, and other
string slices without taking ownership. Inside the function, `next()` asks for the first
item rather than an item at an arbitrary position.

`is_some_and` then answers two questions in order:

1. Is there a first character?
2. If there is, is it uppercase?

For empty input, `next()` returns `None` and `is_some_and` returns `false`. Otherwise,
the predicate receives the first `char`. `Option` now carries both cases, so the code no
longer needs a separate branch or an `unwrap`.

The Unicode test checks that `"Über"` returns `true`. Its first byte is not the
character `Ü`, so inspecting a `char` rather than a byte preserves this behaviour.

</details>

## Alternative approaches

`str::starts_with` can take a character predicate, so this version expresses the same
contract even more directly:

```rust,editable
pub fn starts_with_uppercase(s: &str) -> bool {
    s.starts_with(char::is_uppercase)
}
```

Both versions satisfy the contract. The `chars().next().is_some_and(...)` version
shows the intermediate `Option` and how it handles empty input. The `starts_with`
version is shorter, but its predicate form may be less familiar.

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

If `is_some_and` is unfamiliar to your team, an explicit `match` on `s.chars().next()`
handles the same cases without `unwrap`. Choose the version your team finds easier to
read.

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

*Answer:* No. Borrowing removes unnecessary ownership, but the body need not be one
line. A direct `match` preserves the same contract without `is_some_and`. Choose based
on which version is clearer to the team.

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
