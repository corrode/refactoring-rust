# `parse_srt_timestamp`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Parse timestamp text such as `00:01:23,456` into a `Duration`.
- Keep `parse_srt_timestamp(s: &str) -> Option<Duration>` for the first solution.
- Establish the accepted input format before considering a different public interface.

## Code to refactor

- Starter and its tests: `examples/21_parse_srt_timestamp.rs`.
- Workshop prompts and speaker notes: `slides/pages/21_parse_srt_timestamp.md`.
- Collect colon-separated fields into `Vec<&str>` and require three pieces.
- Parse hours and minutes, then collect the last field's comma-separated pieces.
- Require two final pieces and parse seconds and milliseconds.
- Multiply each component by its millisecond weight, add the results, and construct a `Duration`.
- Note the separate format checks, numeric conversions, and arithmetic operations during the walkthrough.

## Discussion

<details class="discussion-question">
<summary>Does returning <code>Option</code> mean malformed input cannot panic?</summary>

- The separator-count checks return `None`, but each numeric parse uses `unwrap`.
- `ab:cd:ef,gh` reaches a numeric panic rather than returning `None`.
- Hours and minutes are parsed before the comma-count check, so validation order also affects the failure path.
- Even successfully parsed `u64` components can overflow during multiplication or addition.
- Ordinary arithmetic panics with overflow checks enabled and wraps when those checks are disabled.

</details>

<details class="discussion-question">
<summary>How strict is the timestamp format today?</summary>

- The starter accepts variable field widths, not only the displayed two-digit fields and three-digit fraction.
- Minutes and seconds may exceed 59, and the millisecond field may exceed 999.
- `1:2:3,4` treats the last field as four milliseconds, not four tenths of a second.
- Integer parsing permits a leading `+`, but rejects negative values, surrounding whitespace, and non-ASCII digits.
- Separator matching is literal ASCII `:` and `,`; this is not a general Unicode-number parser.
- Enforcing strict SRT field widths or ranges would change the accepted domain rather than merely refactor it.

</details>

<details class="discussion-question">
<summary>Can the fields be borrowed without collecting them?</summary>

- Each split only needs the next delimiter and the remaining text.
- `split_once` returns two slices into the original UTF-8 string, with no field-vector allocation.
- Chaining two colon splits and one comma split expresses the field order directly.
- Remaining extra separators become part of a numeric field and fail numeric parsing.
- Avoid claiming that this classifies every extra delimiter as a format error in the later `Result` alternative.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/21_parse_srt_timestamp.rs`, retaining `Option<Duration>`.
- Propagate missing separators with `?`, then turn numeric errors into `None` through `.ok()?`.
- Parse all four components before computing their weighted total.
- Use `checked_mul` for the three scaled components and `checked_add` for every addition.
- Preserve successful values within the existing permissive component grammar.
- Returning `None` instead of panicking or wrapping is a deliberate error-handling correction, not behavioral equivalence on invalid or overflowing input.
- The bound is `u64::MAX` total milliseconds, not the maximum duration representable by `Duration` itself.
- `Duration::from_millis` can represent that entire checked range without further arithmetic overflow.
- Parsing performs O(n) work for n input bytes with constant auxiliary storage and no intermediate heap collections.

### Alternative: return error details

- Source: `solutions/21_parse_srt_timestamp_alt.rs`, returning `Result<Duration, ParseError>`.
- `ok_or(BadFormat)` labels missing delimiters; `From<ParseIntError>` supports propagation as `BadNumber`.
- Extra delimiters inside a numeric field are `BadNumber`, unlike a missing required delimiter.
- Callers must replace `Some`/`None` handling with `Ok`/`Err` handling and adapt assertions to the new error type.
- The alternative retains unchecked weighted arithmetic, so it can still panic or wrap on otherwise parseable input.
- `BadNumber` retains the integer error but not the component name or input span.
- The enum derives `Debug` only; it does not yet implement `Display` or `std::error::Error`.
- A future `FromStr` implementation needs a local wrapper such as `SrtTimestamp`, since both `FromStr` and `Duration` are foreign types.
- Keep strict format validation and such a wrapper separate from the signature-preserving first solution.

<!-- TODO(prose): Decide whether to retain the Result alternative as a deliberately incomplete error-design example or request checked arithmetic plus an overflow error before publication.
Its source comment claims every failure is reported, but the unchecked arithmetic contradicts that claim; the corresponding slide already warns about overflow. -->

### Edge cases and test coverage

- Current starter tests cover zero, `00:01:23,456`, and the wrong fractional separator.
- Current main-solution tests additionally cover missing/extra separators, empty fields, invalid numbers, negative input, leading whitespace, and a component exceeding `u64`.
- Main tests preserve variable widths, out-of-range components, and leading plus signs.
- Main overflow tests cover each weighted multiplication, an overflowing addition, and `u64::MAX` milliseconds as an accepted boundary.
- Current alternative tests cover only the original three scenarios, using successful results and `BadFormat` assertions.
- Proposed alternative tests: inspect `BadNumber` for each component and document the current overflow behavior separately from any proposed fix.
- Proposed main tests: trailing whitespace, non-ASCII digits, and a total exactly at the boundary assembled from multiple components.
- Do not silently replace permissive acceptance tests with stricter SRT expectations.

### Trade-offs

<!-- Final comparison matrices are deferred while this chapter is a bullet outline. -->

- **Type-safety:** **Original:** `Option` alongside unchecked parses and arithmetic; main checks every conversion and total, but neither result type proves strict SRT validity.
- **Readability:** **Original:** vector lengths and numeric indexing; main follows delimiter order, while the checked arithmetic makes the overflow policy explicit at some visual cost.
- **API design:** **Original:** `&str -> Option<Duration>`; main preserves it, while the `Result` alternative requires caller changes and a future wrapper would add another migration.
- **Performance:** **Original:** two temporary vectors of borrowed fields; main and alternative avoid those allocations and remain O(n), with no benchmark-based ranking.
- **Flexibility:** **Original:** permissive components; main and alternative retain that grammar, while stricter widths, ranges, or a wider duration range require separate decisions.
- **Error reporting:** **Original:** either `None` or a panic; main consistently returns `None` for invalid/overflowing values, while the alternative distinguishes some causes but still misses arithmetic overflow.

</details>

## Takeaways For Our Own Codebase

> - Audit arithmetic after parsing, not just the conversion from text to numbers.
> - Write down the accepted grammar before deciding that a parser should be stricter.
> - Compare error coverage independently of whether a signature says `Option` or `Result`.

## References

- [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once): borrowed delimiter-based parsing.
- [`u64::checked_mul`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul) and [`checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add): arithmetic failure as `Option`.
- [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html): parsing into a local domain type with an associated error.
- [`Duration::from_millis`](https://doc.rust-lang.org/std/time/struct.Duration.html#method.from_millis): distinguish millisecond representation from duration capacity.
- [Fibonacci overflow boundaries](22-iterators.md#solutions): computing an unnecessary next value can fail before the requested result is returned.
