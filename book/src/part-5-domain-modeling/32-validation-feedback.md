# `validation_feedback`

`ValidationReport::suggestions` turns recorded feedback into messages for display.
Review the report's public interface while preserving the exact messages, order, and duplicates for the three known categories.
You may change the API in this domain-modeling exercise; adapt callers and tests along with it.
The task is about representing feedback, not implementing validation rules, password security, or scoring.[^provenance]

## Code to refactor

```rust,editable
{{#include ../../../examples/32_validation_feedback.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/32_validation_feedback.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 32_validation_feedback
```

</div>

## Discussion

<details class="discussion-question">
<summary>What happens to feedback the method doesn't recognize?</summary>

The wildcard arm silently skips it.
A caller can supply any `String`, but only `"too short"`, `"missing uppercase"`, and `"missing digit"` produce suggestions.
A spelling mistake and a deliberately unsupported category have the same result: no message.
The existing tests cover known categories, not a promise to preserve silent dropping after an API redesign.

</details>

<details class="discussion-question">
<summary>Which strings identify a category, and which are for display?</summary>

`"missing digit"` selects a branch; `"Include a digit."` is the message that branch returns.
Those are different jobs.
Ask whether callers need arbitrary text to identify one of three categories, and whether changing display wording should change how the program makes decisions.

</details>

<details class="discussion-question">
<summary>What should happen when another category is added?</summary>

Adding a new string at a call site compiles even if `suggestions` has no corresponding branch.
The wildcard hides the missing case.
It would be useful for a change to the category set to identify every message-selection function that needs updating.

</details>

<details class="discussion-question">
<summary>Which behaviour should the tests keep checking?</summary>

Each known category must produce its exact message.
An empty report produces no suggestions, and repeated categories keep their positions rather than being sorted or deduplicated.
Calling `suggestions` must leave the report unchanged.
An API change can require different construction syntax without changing those expectations.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Store categories, render messages

```rust,editable
{{#include ../../../solutions/32_validation_feedback.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/32_validation_feedback.rs:tests}}
</script>

`Feedback` names the three supported categories: `TooShort`, `MissingUppercase`, and `MissingDigit`.
A report now stores `Vec<Feedback>`, so its entries cannot be misspelled category strings.
`Debug`, `Copy`, `Clone`, `PartialEq`, and `Eq` support inspection, reuse, and comparisons without giving categories a textual representation to parse.

The important change is the exhaustive `match` in `Feedback::suggestion`, not the shorter iterator expression.
There is no `_` arm.
Add a variant and the compiler reports a non-exhaustive match until you decide what message it should return.
A wildcard would give up that reminder.
The compiler checks coverage, not the correctness of the sentence in each arm; the exact-message tests still matter.

`iter()` borrows the categories, `map(Feedback::suggestion)` renders each one, and `collect()` builds the result in the same order.
Every entry produces one message, including duplicates.
An empty report still produces an empty vector, and repeated calls leave `feedback` unchanged.

### Migrate callers and return types

Replace string construction with enum variants:

```rust,ignore,noplayground
// Before, with the starter's ValidationReport:
let report = ValidationReport {
    feedback: vec!["missing digit".into(), "too short".into()],
};
let messages: Vec<String> = report.suggestions();

// After, with the solution's ValidationReport and Feedback:
let report = ValidationReport {
    feedback: vec![Feedback::MissingDigit, Feedback::TooShort],
};
let messages: Vec<&'static str> = report.suggestions();
```

These are alternative call sites, not one standalone program.
Update helper parameters and stored report values to carry `Feedback` too.
The solution's attached tests make the same change while retaining the known-message, order, duplicate, empty-report, and non-mutation checks.

The returned strings are literals with a `'static` lifetime, not borrows tied to the report.
The solution also tests that a message can outlive the report that produced it.
Consumers that only display text can use the references directly.
If a downstream API requires owned, mutable strings, convert at that boundary:

```rust,ignore,noplayground
// With the solution's report in scope:
let messages: Vec<String> = report
    .suggestions()
    .into_iter()
    .map(str::to_owned)
    .collect();
```

That conversion restores per-message string allocations.
Without it, the result vector still allocates storage for a nonempty collection of references, but the individual static messages need no allocation.
This is not a zero-allocation API, and fewer allocations alone are not a measured speed-up.

### Wording is not category identity

Code that makes decisions should inspect `Feedback`, not parse the returned sentence.
You can then change `"Include a digit."` without changing category-based logic, as long as consumers don't parse or otherwise depend on that exact text.
Display snapshots and exact-message tests will still need updating for an intentional wording change.
For this exercise, keep the original messages unchanged.

### Unknown input still needs a boundary

The new report cannot contain an unknown string, but external data can still contain one.
If categories arrive from a file or an API, parse them before constructing the report, for example through a fallible conversion returning `Result<Feedback, ParseFeedbackError>`.
Choose what to do with unsupported values there: reject them with a useful error, retain them separately, or apply an explicitly documented compatibility policy.
No such parser is implemented by this solution.

The old silent-drop behaviour is not preserved for arbitrary strings: the input type has changed.
A direct Rust caller now supplies a known variant; a string-based caller needs a parsing step and an error policy.
Do not hide that migration behind `filter_map` that quietly discards parse failures.
An enum makes the internal category set explicit; it does not make external input trustworthy.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Enum categories |
|---|---|---|
| Type-safety | Any string accepted | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Known variants; exhaustive message selection |
| Readability | Category spellings repeated in callers | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Named categories separate from display text |
| API design | String input and owned string output | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Explicit domain; callers and return types must migrate |
| Performance | Result vector and individual message allocations | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Static messages avoid individual allocations; vector remains |
| Flexibility | Arbitrary strings accepted, unknown ones dropped | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Closed set; new categories require code changes |
| Error reporting | Missing cases silently skipped | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Compiler flags missing match arms |

The error-reporting advantage concerns Rust category definitions and matches, not external parsing errors.
This is an intentional API redesign, not an interchangeable implementation of the original string contract.
For a fixed set of categories, I would accept the caller migration to get compiler feedback when that set changes.

</details>

## Takeaways For Our Own Codebase

> - Which strings are category identifiers rather than text for people?
> - Keep decisions on typed categories and render wording at the display boundary.
> - Check exhaustive matches when adding a category; check message tests when changing wording.
> - Locate external string inputs and choose an explicit unknown-value policy before migrating callers.
> - Preserve order and duplicates unless the domain contract says otherwise.

## References

- [Defining enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html): represent a value as one of a known set of variants.
- [Matches are exhaustive](https://doc.rust-lang.org/book/ch06-02-match.html#matches-are-exhaustive): require a branch for every possible variant.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map): transform each item without filtering it out.
- [String literals](https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-literal-expressions): why these messages have type `&'static str`.
- [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html): fallible conversion for a future external-input boundary.

[^provenance]: This fresh, instructor-written teaching fixture is inspired by a feedback-representation pattern in beginner-course submissions. It is not verbatim learner code and does not reproduce a particular submission.
