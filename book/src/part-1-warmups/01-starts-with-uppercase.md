# `starts_with_uppercase`

Here's a function that checks whether a string starts with an uppercase letter.
It works, but it could be prettier.
You can click the "Run" button to run the tests.
Try to refactor it while preserving its behaviour.
That means, don't change the function signature or return value.

If you like, you can also use your own development environment to run the tests and refactor the code.
To do so, clone the repository and run the tests with `cargo test --example 01_starts_with_uppercase`.

Once you're happy with your solution, take a look at the discussion and solution sections for some ideas on how to improve it further.

Good luck!

## Code to refactor


```rust,editable
{{#include ../../../examples/01_starts_with_uppercase.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/01_starts_with_uppercase.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/01_starts_with_uppercase.rs:tests}}
</script>

## Discussion

> **DISCUSSION**
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

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Solutions</h2></summary>

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

### Alternative approaches

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

</details>

## Trade-offs

Compared with the original: **+** advantage, **=** comparable, **−** disadvantage,
**N/A** not applicable. Readability depends on familiarity.

| Criterion | Original | `is_some_and` | `starts_with` predicate |
|---|---|---|---|
| Type-safety | Arbitrary text; boolean result | = No new type-level invariant | = No new type-level invariant |
| Readability | Explicit branches; redundant boolean checks | + Fewer branches, if combinators familiar | + Direct question, if predicate form familiar |
| API design | Takes ownership for read-only check | + Borrows read-only input | + Borrows read-only input |
| Performance | Ownership may require caller copies | + Avoids copies when borrowing suffices | + Avoids copies when borrowing suffices |
| Flexibility | Requires owned `String` | + Accepts slices and borrowed strings | + Accepts slices and borrowed strings |
| Error reporting | Empty input returns `false` | N/A | N/A |

All three inspect only the first Unicode scalar value. Borrowing avoids a clone when
callers need to keep their string, not a measured cost in the character check itself.
If `is_some_and` is unfamiliar, an explicit `match` on `s.chars().next()` remains an
option; the `starts_with` predicate form also needs familiarity. Empty input is an
ordinary `false`, not an error.

## Follow-up questions

> **FOLLOW-UP**
>
> - When is empty input a normal `false`, and when should it produce `Option<bool>` or
>   an error?
> - Which predicates in your codebase own values that they only inspect?
> - For small `Option` checks, does your team find an explicit `match` or a combinator
>   such as `is_some_and` easier to review? What makes the difference?

## Important Concepts

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
