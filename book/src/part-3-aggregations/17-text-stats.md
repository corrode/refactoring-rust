# `text_stats`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Compute the total word count, unique word count, and average word length for a string.
- Review the meaning of the results as well as the implementation.
- Keep `text_stats(text: &str) -> (usize, usize, f64)` and the public `count_words(text: &str) -> HashMap<String, usize>` helper in the first solution.

## Code to refactor

- Source and tests: `examples/17_text_stats.rs`.
- Starting approach: build a frequency map, then derive three tuple fields from separate traversals of that map.
- The helper uses `split_whitespace` and updates owned string keys through the entry API.
- Companion source: `slides/pages/17_text_stats.md`, including its review example and notes about deciding metric definitions before replacing methods.

## Discussion

<details class="discussion-question">
<summary>What does a key represent, and what does its count represent?</summary>

- A map key represents one distinct token, and its `usize` value records the token's number of occurrences.
- `values().count()` counts entries rather than summing their frequencies, so the starter's “total” is actually the distinct-token count.
- Filtering entries with `count == 1` counts words occurring exactly once, not all distinct words.
- The canonical contract uses “total” for all occurrences and “unique” for distinct keys, including keys occurring more than once.

</details>

<details class="discussion-question">
<summary>Why does the checked-in starter test miss all three metric errors?</summary>

- Every word in its sentence occurs once, making occurrence counts, distinct counts, and singleton counts coincide.
- Averaging lengths over distinct keys also happens to equal an occurrence-weighted average for that input.
- The slide's `"a a rust"` example separates the policies: the starter returns `(2, 1, 2.5)`, while the intended statistics are `(3, 2, 2.0)`.
- A repeated short word next to a longer word exposes the missing weight in the average as well as the count errors.

</details>

<details class="discussion-question">
<summary>What do “word” and “length” mean here?</summary>

- `split_whitespace` separates tokens using Unicode whitespace, ignoring leading, trailing, and repeated separators.
- Tokens remain case-sensitive and keep punctuation; `Rust`, `rust`, and `rust,` are different map keys.
- `chars().count()` measures Unicode scalar values, not UTF-8 bytes or user-perceived grapheme clusters.
- Precomposed `é` has one scalar value, while `e` followed by a combining acute accent has two, despite often displaying as the same grapheme.
- Neither the helper nor the main solution performs case conversion, Unicode normalization, or linguistic word segmentation.

</details>

<details class="discussion-question">
<summary>What average does the function return when there are no words?</summary>

- The starter divides floating-point zero by zero and returns `NaN`, rather than panicking on integer division.
- The canonical solution explicitly returns `(0, 0, 0.0)` for empty input, which also applies to whitespace-only input under this tokenizer.
- Zero is a selected API policy, not the mathematical mean of an empty sample; the existing tuple has no separate absence state.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/17_text_stats.rs`, preserving both public signatures and the tuple's field order.
- Sum `counts.values()` for total occurrences and use `counts.len()` for the number of distinct tokens.
- For each entry, multiply `word.chars().count()` by its frequency before summing the numerator of the average.
- Divide that scalar-value total by the occurrence total as `f64`, with an explicit zero-word branch returning `0.0`.
- Keep `count_words` unchanged; the counting helper already records the frequencies needed for these statistics.
- This is a correctness change, not a behavior-preserving rewrite of the starter's formulas, although existing callers need no type migration.
- Caller expectations and tests for repeated words or empty input must adopt the specified statistics instead of preserving the old values.
- TODO(prose): Make the bug-fix boundary explicit and explain why the main API keeps `0.0` for an absent average, with an optional-average redesign presented separately rather than silently changing the tuple.

### Alternative approaches

- No `_alt.rs` implementation exists for this exercise; these options expand on the slide notes rather than describe additional checked-in code.
- A named `TextStats` result would distinguish fields such as `total_words`, `unique_words`, and `average_word_length` without relying on tuple position.
  - Callers and tests would replace tuple destructuring or positional access with named fields; names alone would not enforce the arithmetic relationships.
- An `Option<f64>` average could preserve meaningful zero counts while representing an absent mean.
  - Returning `Option<TextStats>` would instead make the entire summary absent, while `Result` would make empty input a rejected operation.
  - Choose among these based on what callers need to distinguish, then update their empty-input branches and assertions explicitly.
- A borrowed-key counting helper returning `HashMap<&str, usize>` could avoid creating owned keys for this input.
  - That changes a public helper's return type and ties the map to the text's lifetime, so it is not a drop-in replacement for the existing owned-key API.
- A summary-only pass could count occurrences and scalar values while inserting borrowed tokens into a set for the distinct count.
  - This avoids storing frequencies that the summary no longer needs, but does not replace `count_words` for callers who need the frequency map itself.

### Edge cases and test coverage

- Existing starter coverage: only `all_words_occur_once`, asserting five total words, five unique words, and an average near `7.8`.
- Existing solution coverage: that same case, `repeated_words_affect_totals_and_average` using `"a a rust"`, and `empty_input_has_zero_average`.
- Suggested tests: whitespace-only input, tabs/newlines, and Unicode whitespace, all following `split_whitespace` semantics.
- Suggested tests: repeated identical words, mixed case, and punctuation, separating multiplicity from token equality.
- Suggested tests: non-ASCII and combining-character tokens, protecting scalar-value length from accidental substitution with byte length.
- Suggested helper tests: exact token frequencies in `count_words`, independently of the summary formulas.
- The average is floating-point and not an exact rational result; the existing nonempty tests use a tolerance, and sufficiently large integer totals can round during conversion to `f64`.
- The helper evaluates `word.to_string()` for every token before its entry lookup, including duplicates whose newly allocated key will not be retained.
- `counts.len()` is constant-time; traversals of a standard `HashMap` can visit empty buckets, so their cost depends on capacity rather than only the number of live entries.
- With expected hashing costs, total work includes scanning and hashing input bytes, scanning distinct token text for lengths, and traversing the map; fewer lines do not remove those costs.

### Trade-offs

<!-- Final comparison matrices are deferred until the prose is drafted. -->

- **Type-safety:** Original and main expose the same tuple of primitive types, so neither prevents swapping counts or computing an unweighted mean; named fields help interpretation, while an optional average adds an absence state.
- **Readability:** Original uses valid iterator methods for the wrong quantities; main aligns summation, map length, and weighted lengths with explicitly named metric definitions.
- **API design:** Original and main have identical signatures but different repeated-word and empty-input results; named statistics, optional averages, and borrowed-key maps require separate caller migrations.
- **Performance:** Original and main both build the same owned-key map; main adds frequency weighting while retaining per-token key creation, and a borrowed-key or summary-only design changes the work and lifetime constraints rather than proving a universal speed-up.
- **Flexibility:** Original's frequency map already contains information for several metrics; main uses it correctly, while a summary-only set gives up frequencies and a named result can accommodate more metrics without positional ambiguity.
- **Error reporting:** Original silently returns misleading statistics and `NaN` on no words; main returns defined values without an error channel, while an optional average or error-returning variant would expose a different empty-input policy.

</details>

## Takeaways For Our Own Codebase

> - Do our aggregation tests distinguish occurrences, distinct keys, and values appearing exactly once?
> - Are averages weighted by the observations callers mean, or only by distinct entries?
> - Have we named the unit of text length and decided how an absent average should appear in the API?

## References

- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry): updating token frequencies through a single entry operation.
- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) and [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars): the tokenizer and length unit used here.
- [`HashMap::iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter): entry traversal and its capacity-related performance caveat.
- [`room_occupancy`](16-room-occupancy.md): aggregate definitions and empty-input policies in another collection-based API.
- [`mode`](18-mode.md): another frequency-map consumer, selecting an entry rather than summing occurrences.
