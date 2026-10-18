# `starts_with_uppercase`

Here's a function that checks whether a string starts with an uppercase letter.
It works, but it could be prettier.


A bunch of notes before we get started:

- You can click the "Run" button to run the tests attached to that code block.
  Expand **Tests and callers** to inspect or edit them.
  If you explore an API change, update those calls alongside the implementation; Run uses your edits and does not choose another test suite for you.
- Try to refactor it while preserving its behaviour.
  That means, don't change the function signature or return value.
- If you like, you can also use your own development environment to run the tests and refactor the code.
  To do so, clone the repository and run the tests with `cargo test --example 01_starts_with_uppercase`.
- Once you're happy with your solution, take a look at the discussion and solution sections for some ideas on how to improve it further.

That's it.
Good luck.

## Code to refactor


```rust,editable
{{#include ../../../examples/01_starts_with_uppercase.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/01_starts_with_uppercase.rs:tests}}
</script>

## Discussion

A few things to think about:

<details class="discussion-question">
<summary>Do we need to own the string to check its first character?</summary>

No, the function only reads the input; it does not change it.
Taking a `String` transfers ownership, so a caller that needs the string afterward must clone it or change its code.
A borrowed `&str` is enough for read access, though changing the parameter would go beyond this exercise's signature constraint.

</details>

<details class="discussion-question">
<summary>Is there another way to get the first character than <code>chars().nth(0)</code>?</summary>

`chars().next()` asks for the first character directly and returns an `Option<char>`.
The result is `None` for an empty string and `Some` for its first Unicode scalar value.

</details>

<details class="discussion-question">
<summary>Is the <code>unwrap</code> safe?
Do we need it?</summary>

The early return for `s.len() == 0` makes this `unwrap` safe: a nonempty UTF-8 string has at least one character.
Handling the `Option<char>` from `chars().next()` avoids the `unwrap` and the separate empty-input check.

</details>

<details class="discussion-question">
<summary>Does comparing a boolean with <code>true</code> add information?</summary>

No, `first.is_uppercase()` already returns a boolean.
Comparing it with `true` and then returning `true` or `false` from an `if` is unnecessary.
You can return the boolean directly.

</details>

<details class="discussion-question">
<summary>What should be the return value for an empty string?</summary>

`false`, as in the original function.
An empty string has no first character, so it cannot start with an uppercase letter.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/01_starts_with_uppercase.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_starts_with_uppercase.rs:tests}}
</script>

This version keeps the original `String` parameter and `bool` result, so callers need no changes.
Inside the function, `next()` asks for the first item rather than an item at an arbitrary position.

`is_some_and` then answers two questions in order:

1. Is there a first character?
2. If there is, is it uppercase?

For empty input, `next()` returns `None` and `is_some_and` returns `false`.
Otherwise, the predicate receives the first `char`.
`Option` now carries both cases, so the code no longer needs a separate branch or an `unwrap`.

The Unicode test checks that `"Über"` returns `true`.
Its first byte is not the character `Ü`, so inspecting a `char` rather than a byte preserves this behaviour.

### Alternative: borrow the input

If you can change the API, the same implementation can borrow its input:

```rust,editable
{{#include ../../../solutions/01_starts_with_uppercase_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_starts_with_uppercase_alt.rs:tests}}
</script>

The borrowed signature accepts string literals, borrowed `String` values, and other string slices without taking ownership.
Callers that previously passed a `String` must now borrow it.
This is a separate API change, not part of the signature-preserving refactor above.

`str::starts_with` can take a character predicate, so this version expresses the same contract even more directly:

```rust,editable
pub fn starts_with_uppercase(s: &str) -> bool {
    s.starts_with(char::is_uppercase)
}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_starts_with_uppercase_alt.rs:tests}}
</script>

Both borrowed versions preserve the results, but change the original signature.
The `chars().next().is_some_and(...)` version shows the intermediate `Option` and how it handles empty input.
The `starts_with` version is shorter, but its predicate form may be less familiar.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Owned `is_some_and` (main) | Borrowed `is_some_and` | Borrowed `starts_with` predicate |
|---|---|---|---|---|
| Type-safety | Arbitrary text; boolean result | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No new type-level invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No new type-level invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No new type-level invariant |
| Readability | Explicit branches; redundant boolean checks | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Fewer branches, if combinators familiar | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Fewer branches, if combinators familiar | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Direct question, if predicate form familiar |
| API design | Takes ownership for read-only check | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Preserves owned parameter | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Borrows read-only input | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Borrows read-only input |
| Performance | Ownership may require caller copies | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same ownership costs; timing unmeasured | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Avoids copies when borrowing suffices | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Avoids copies when borrowing suffices |
| Flexibility | Requires owned `String` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Requires owned `String` | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Accepts slices and borrowed strings | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Accepts slices and borrowed strings |
| Error reporting | Empty input returns `false` | N/A | N/A | N/A |

All versions inspect only the first Unicode scalar value.
The main solution keeps the ownership contract; the borrowed alternatives require call-site changes.
Borrowing avoids a clone when callers need to keep their string, not a measured cost in the character check itself.
If `is_some_and` is unfamiliar, an explicit `match` on `s.chars().next()` remains an option; the `starts_with` predicate form also needs familiarity.
Empty input is an ordinary `false`, not an error.

</details>

## Takeaways For Our Own Codebase

> - When is an empty string a valid input for our functions?
>   Alternatively, when should we return an error or `Option<bool>` instead?
> - Which predicates in our codebase own values that they only inspect?
>   Consider borrowed parameters as a separate API change after checking the callers.
> - Does your team prefer explicit `match` or combinators such as `is_some_and`?

## References 

- [`&str` and borrowed parameters](../appendix/concept-index.md#borrowed-string-slices): take a view of text when the function only reads it.
- [`Iterator::next`](../appendix/concept-index.md#iterator-next): request the first item directly and receive `Option` when the iterator might be empty.
- [`Option::is_some_and`](../appendix/concept-index.md#option-is-some-and): test both presence and a condition without calling `unwrap`.
- [Unicode scalar values](../appendix/concept-index.md#unicode-scalar-values): a Rust string is UTF-8, so its first byte and its first `char` answer different questions.
