# `iban_prefix_check`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Review a checker for country codes, check-digit positions, ASCII content, and country-specific lengths.
- Keep support limited to DE, GB, FR, CH, NL, and AT, without adding checksum or bank-account verification.
- API design is open in this domain-modeling part, but any interface change needs an explicit caller and test migration.

## Code to refactor

- Starter: `examples/27_iban_prefix_check.rs`, including its five unit tests.
- Public entry point: `is_valid_iban(s: &str) -> bool`, borrowing the input without returning a parsed value.
- Current approach: collect `s.chars()` into `Vec<char>`, check its length, slice the country from the string, then inspect individual characters.
- Country lengths come from an `if` ladder, followed by a comparison against the collected character count.
- Workshop context: `slides/pages/27_iban_prefix_check.md`, including notes on byte access, country tables, and the excluded checksum calculation.

## Discussion

<details class="discussion-question">
<summary>Does counting characters make <code>&s[0..2]</code> safe?</summary>

- `chars.len() >= 4` establishes a count of Unicode scalar values, not a UTF-8 boundary at byte offset two.
- `"€000"` passes that guard, but slicing at byte two panics before the ASCII checks run.
- `&str` guarantees valid UTF-8, not that every byte offset is a character boundary.
- A validator for an ASCII format still receives arbitrary Unicode through this signature, so rejecting unsupported text must not depend on that text already being ASCII.

</details>

<details class="discussion-question">
<summary>Would one ASCII-alphanumeric check enforce all the prefix rules?</summary>

- No: letters are alphanumeric too, so positions two and three still need their own digit checks.
- Matching a supported country already establishes the first two uppercase letters, making the later alphabetic checks redundant in the main solution.
- The main solution nevertheless retains all four explicit checks and scans the whole input with `bytes().all(...)`, including those first four bytes again.
- Treat the slide note about replacing the loop as a simplification of the suffix scan, not a claim that the country, length, and positional checks disappear.

</details>

<details class="discussion-question">
<summary>What does a successful boolean result actually establish?</summary>

- Only the exercise's layout rules: a supported uppercase country, its expected length, two ASCII digits, and ASCII-alphanumeric remaining content.
- Lowercase letters in the body pass, while lowercase country codes and whitespace do not, because exact country matching and ASCII-alphanumeric checks impose different restrictions.
- No mod-97 checksum, country-specific bank/account layout, account existence, or ownership check happens here.
- The caller still holds an ordinary `&str`, and downstream functions cannot tell from its type whether this check ever ran.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution: checked slicing and byte-oriented rules

- Source: `solutions/27_iban_prefix_check.rs`, retaining `pub fn is_valid_iban(s: &str) -> bool` and the same five tests.
- `s.get(0..2)` returns `None` for a short string or a non-boundary endpoint, allowing the country match to reject either case without panicking.
- The match maps DE and GB to 22, FR to 27, CH to 21, NL to 18, and AT to 20, rejecting every other prefix.
- `s.len() == expected` establishes a byte length of at least 18 before indexing `b[0]` through `b[3]`, so all four byte accesses are in bounds.
- That indexing argument depends on the runtime length guard and the table's values, not on a new compile-time length guarantee.
- `as_bytes()` borrows the existing storage, the digit checks enforce positions two and three, and `bytes().all(...)` rejects every non-ASCII byte or punctuation byte.
- Byte counts and character counts agree for accepted ASCII strings, so the length change preserves the accepted language rather than broadening it to Unicode.
- Separate the changes: removing the character allocation and replacing the country ladder are refactors, while turning the UTF-8-boundary panic into `false` is a bug fix.

### Alternative designs, not additional checked-in solutions

- There is no `27_iban_prefix_check_alt.rs`; distinguish the following design options from the runnable main solution.
- A signature-preserving variation can omit the two alphabetic checks after the country match, but keep the check-digit and body restrictions explicit.
- A static table can suit a larger maintained country registry, while a newly constructed `HashMap` would add setup work for this six-entry lookup.
- Neither a table nor a `match` verifies that its country lengths agree with an external registry, so keep that maintenance responsibility visible.
- Proposed API redesign: a `PrefixCheckedIban<'a>` wrapper with private `&'a str` storage and `TryFrom<&'a str, Error = PrefixError>` construction.
- Such construction would validate at runtime, while downstream parameters of type `PrefixCheckedIban<'_>` would statically distinguish checked input from arbitrary text.
- Keep construction controlled outside the defining module and its descendants, expose no mutable storage, and name the type for the limited check rather than implying complete IBAN validation.
- A structured `PrefixError` could distinguish unsupported country, wrong length, and bad characters, but selecting and documenting error precedence would become part of the new contract.

### Caller migration and invariant boundaries

- Main solution: callers keep passing `&str` and matching a `bool`, with no new public types, ownership requirements, or visibility changes.
- The existing tests keep their calls and assertions unchanged, but those assertions do not cover the panic fixed by checked slicing.
- Proposed wrapper: construct through `TryFrom` at the input boundary, handle `PrefixError`, and pass the wrapper where downstream signatures previously accepted `&str`.
- Wrapper tests would assert construction success or an error category instead of a boolean, and include an outside-module construction check to test privacy.
- A borrowed wrapper cannot outlive its input, while an owned-string version would have a different allocation and ownership contract.
- Neither wrapper design would prove checksum correctness unless its constructor actually added that validation, which would be a separate behavior change.

### Edge cases, current coverage, and author decisions

- Current starter and main tests cover valid DE and NL inputs, a wrong DE length, an unknown country, and letters in the check-digit positions.
- `tests/domain_modeling.rs` contains no exercise-27 fixture, so it adds no Unicode or construction-boundary coverage here.
- Proposed regression: make `"€000"` return `false` without unwinding, documenting that the current starter would fail this new expectation.
- Proposed coverage: empty input, fewer than four characters, a multibyte prefix, and a multibyte body whose total byte length matches a supported country's length.
- Proposed coverage: all six country lengths and their immediate neighbors, plus uppercase versus lowercase country codes, lowercase body letters, spaces, and punctuation.
- Proposed coverage: changing only the numeric check-digit pair can still pass, making the absent checksum validation visible rather than calling these strings valid bank accounts.
- A panic-freedom property over arbitrary UTF-8 would exercise the public input domain more broadly than the five current examples.

<!-- TODO(prose): Decide whether the finished task should explicitly include finding a bug and gain an intentional starter regression test, since the current slides emphasize allocation and matching but omit the UTF-8 panic. -->
<!-- TODO(prose): Decide whether to retain the main solution's redundant alphabetic checks as visible documentation or propose a later code cleanup, without claiming the current implementation uses only one check. -->

### Trade-offs

- **Type-safety:** **Original** accepts arbitrary `&str` and returns `bool`; the main solution fixes a runtime panic without adding a checked-input type, while the proposed wrapper would record only its constructor's limited validation.
- **Readability:** **Original** separates character checks from a six-branch country ladder; the main solution brings country lengths into one match but needs its byte-index safety argument and redundant checks explained.
- **API design:** **Original** and main share the same borrowing predicate and potentially overbroad `is_valid_iban` name; the proposed wrapper makes the checked state explicit at the cost of new constructors, errors, and caller changes.
- **Performance:** **Original** decodes and collects the whole input before rejecting it; the main solution allocates nothing, rejects unknown prefixes or wrong byte lengths early, and scans at most 27 bytes with the current table.
- **Flexibility:** **Original** and main hard-code the same six countries and ASCII rules; a maintained table can ease registry updates, while normalization or full checksum validation would change the contract rather than merely the lookup mechanism.
- **Error reporting:** **Original** conflates rejection reasons and can panic on a malformed boundary; main consistently returns `false` for rejected text, but only the proposed error-returning design could explain why.

</details>

## Takeaways For Our Own Codebase

> - Check whether byte offsets are justified by input validation or merely by assumptions about a format.
> - Name validation APIs after the guarantees they actually establish, especially when checksums or domain-specific rules are absent.
> - Decide whether downstream code needs a boolean decision or a value whose type records a completed check.
> - Keep panic fixes and acceptance-policy changes visible when reviewing an allocation-focused refactor.

## References

- [`str::get`](https://doc.rust-lang.org/std/primitive.str.html#method.get): checked slicing with UTF-8-boundary handling.
- [`str::as_bytes`](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes): borrowed access to the encoded bytes, not an ASCII guarantee.
- [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html): fallible construction for the proposed checked-input wrapper.
- [Typed IDs](31-typed-ids.md): distinguish a new type from a mere name for the same primitive.
- [Configuration loading](29-config-loader.md): another boundary between raw text, runtime checks, and typed values.
