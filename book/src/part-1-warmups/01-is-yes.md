# `is_yes`

This function checks whether a string means "yes."
Refactor it without changing its signature or which inputs it accepts.
The first solution keeps both; the alternatives explore changes to the API and behavior.

You can edit the code below and press **Run** to test your changes.
Each editable block has its own attached **Tests and local setup** editor.
Expand it to inspect the assertions, add a case, or change a caller.
Run sends the current implementation and its attached tests to the Rust Playground together.
If you try a different signature, update the callers yourself; the editor won't replace them for you.

## Code to refactor

```rust,editable
{{#include ../../../examples/01_is_yes.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/01_is_yes.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

Edit `examples/01_is_yes.rs` in the [workshop repository](https://github.com/corrode/refactoring-rust).
From the repository root:

```sh
cargo test --example 01_is_yes
```

</div>

## Discussion

<details class="discussion-question">
<summary>What do the four branches have in common?</summary>

Each branch compares the same input with a string literal and returns `true` on a match.
If none matches, the function returns `false`.
The question is whether the input belongs to a fixed set of spellings, not which branch supplied the answer.
A pattern can list those spellings together.

</details>

<details class="discussion-question">
<summary>Does checking the string require ownership?</summary>

The function only reads the input, so a borrowed `&str` would provide everything it needs.
But changing `String` to `&str` changes the API.
An existing call such as `is_yes(s)` needs to become `is_yes(&s)` when `s` is a `String`.
A caller that stores the function as `fn(String) -> bool` must also change.
Keep that migration separate from the first refactor.

</details>

<details class="discussion-question">
<summary>Does the original ignore case?</summary>

No.
It accepts exactly `"y"`, `"yes"`, `"Y"`, and `"YES"`.
It rejects the six mixed-case spellings `"Yes"`, `"yEs"`, `"yeS"`, `"YEs"`, `"YeS"`, and `"yES"`.
It also rejects whitespace around a word and longer strings that contain it.
Ignoring case might suit a confirmation prompt, but it changes this function's behavior.
The name alone doesn't settle that choice; the tests record the current rule.

</details>

<details class="discussion-question">
<summary>Where could an allocation happen?</summary>

The original comparisons don't allocate.
A caller may allocate a `String` from a literal or clone an existing string to retain it after the call.
Borrowing can avoid that caller-side work.

Converting the input with `to_ascii_lowercase` creates an owned copy and converts its ASCII uppercase letters.
For nonempty input, that adds a temporary allocation inside the function.
Borrowing the parameter doesn't prevent an implementation from allocating later.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Keep the signature and accepted spellings

```rust,editable
{{#include ../../../solutions/01_is_yes.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_is_yes.rs:tests}}
</script>

`matches!` returns whether a value matches a pattern.
The `|` separates alternative patterns, so any of these four literals produces `true`.
Every other string produces `false`.

`String::as_str` borrows the string's contents as a `&str` for the comparison.
It neither copies the contents nor allocates.
The function still takes ownership of the `String`, and callers still pass the same arguments.
Only the implementation changes.

### Alternative: borrow and keep exact matches

```rust,editable
{{#include ../../../solutions/01_is_yes_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_is_yes_alt.rs:tests}}
</script>

This version gives the same answer for every input string, but borrows instead of taking ownership.
You can pass a literal directly, borrow a `String` with `is_yes(&s)`, or pass a string slice.
The caller can keep using its string afterward without cloning it for this check.

The attached tests use the borrowed signature and check that the caller still owns its string.
When trying this change in the starter editor, update its owned calls and its `fn(String) -> bool` assertion too.
Keep the assertions about accepted spellings unchanged.

### Alternative: ignore ASCII case without a copy

```rust,editable
{{#include ../../../solutions/01_is_yes_alt_case_insensitive.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_is_yes_alt_case_insensitive.rs:tests}}
</script>

`eq_ignore_ascii_case` states the comparison rule directly and doesn't create a lowercase string.
The `||` returns `true` if either comparison succeeds and skips the second comparison when the first succeeds.

This alternative changes both the API and the accepted inputs.
It borrows, and it accepts mixed-case spellings such as `"Yes"`.
Its attached tests therefore expect `true` for all six mixed-case spellings that the original rejects.
Changing those assertions makes sense only if you intend to change the requirement, not just to make a refactor pass.

### Alternative: lowercase, then match

```rust,editable
{{#include ../../../solutions/01_is_yes_alt_lowercase.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/01_is_yes_alt_lowercase.rs:tests}}
</script>

This version creates an ASCII-lowercase copy and checks it against two patterns.
It accepts exactly the same inputs as the `eq_ignore_ascii_case` version and requires the same caller changes.
The patterns stay together, which can be convenient for a longer list of words.
But the function copies and converts the entire input just to check these two short words.
The temporary string adds no benefit to this check.

Neither case-insensitive version performs Unicode case folding, trims whitespace, or accepts a partial match.
All versions reject `" yes "`, `"yesterday"`, and `"yés"`.
ASCII case handling fits these ASCII tokens; it isn't a general rule for comparing words in other languages.

### Trade-offs

Compare each variant with the **Original** baseline using the [six trade-off axes](../appendix/trade-off-axes.md).
The scores describe specific consequences, not an overall winner.

<img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage; **N/A** means not applicable.

| Criterion | Original | Owned exact `matches!` | Borrowed exact `matches!` | `eq_ignore_ascii_case` | ASCII lowercase + `matches!` |
| --- | --- | --- | --- | --- | --- |
| Accepted inputs | `y`, `yes`, `Y`, `YES` | Same four | Same four | Also all six mixed-case spellings | Also all six mixed-case spellings |
| Type-safety | Any string; boolean result | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> No new invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> No new invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> No new invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> No new invariant |
| Readability | Four separate branches | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Groups exact spellings | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Groups exact spellings | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Names the case rule | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Two patterns after conversion |
| API design | Consumes `String` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same calls and ownership | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Borrows; migrate callers | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Borrows; migrate callers | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Borrows; migrate callers |
| Performance | No internal allocation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> No allocation; exact comparisons | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> No allocation; exact comparisons | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> No allocation; ASCII comparisons | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Copies and converts input |
| Flexibility | Can add any exact spelling | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Keeps case distinctions | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Keeps case distinctions | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Cannot distinguish ASCII case variants | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Cannot distinguish ASCII case variants |
| Error reporting | N/A: boolean predicate | N/A | N/A | N/A | N/A |

Readability scores assume familiarity with `matches!` and the string methods.
The API advantage of borrowing comes with a migration cost; it isn't a drop-in replacement for `String`.
The performance row compares work inside the function, while borrowing can also avoid allocations or clones at call sites.
These are observations about the code, not benchmark results or claims of a measured speed-up.
All five versions return `false` for unrecognized input rather than reporting an error.

Choose the accepted inputs before choosing the implementation.
For the existing contract, I'd keep the exact patterns and consider borrowing as a separate API change.
If the requirement changes to ignore ASCII case, I'd use `eq_ignore_ascii_case` to avoid the temporary copy.
Losing the ability to distinguish case variants is then intentional, not a defect.

</details>

## Takeaways For Our Own Codebase

> Which read-only checks take ownership of their inputs?
> Check whether callers clone values just to pass them in.
>
> Do tests distinguish exact spellings from case-insensitive matches?
> Add examples that capture the intended rule before replacing comparisons with normalization.
>
> Separate an implementation refactor from an API migration or a change in accepted inputs, so a reviewer can judge each decision on its own.

## References

- [`matches!`](https://doc.rust-lang.org/std/macro.matches.html) returns a boolean from a pattern match.
- [Or-patterns](https://doc.rust-lang.org/reference/patterns.html#or-patterns) combine alternatives with `|`.
- [`String::as_str`](https://doc.rust-lang.org/std/string/struct.String.html#method.as_str) borrows an owned string's contents without copying them.
- [References and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) explains how a caller keeps ownership while a function reads its data.
- [`str::eq_ignore_ascii_case`](https://doc.rust-lang.org/std/primitive.str.html#method.eq_ignore_ascii_case) compares without allocating a lowercase copy.
- [`str::to_ascii_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_ascii_lowercase) returns a new string and leaves non-ASCII characters unchanged.
