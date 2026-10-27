# `spell_check`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Return input words not found in the dictionary under the existing case-insensitive comparison.
- Preserve the output's original spelling, input order, and repeated misspellings.
- Keep `spell_check(words: &Vec<String>, dict: &Vec<String>) -> Vec<String>` as the first solution's API.

## Code to refactor

- Source and tests: `examples/15_spell_check.rs`.
- Starting approach: nested loops, a `found` flag, lowercase-string comparisons, and a clone of each unmatched input word.
- Companion sources: `slides/pages/15_spell_check.md` and its presenter notes, plus `TODO.md` §6 on a lazy spelling API.

## Discussion

<details class="discussion-question">
<summary>What does “case-insensitive” mean in this implementation?</summary>

- The starter compares the results of `str::to_lowercase`, including Unicode lowercase mappings and context-sensitive final sigma handling.
- This is not full Unicode case folding, locale-specific comparison, or Unicode normalization.
- `ÉCOLE` and `école` match, while `straße` and `STRASSE` do not become equal after lowercasing.
- Replacing the operation with ASCII lowercasing changes behavior for non-ASCII words rather than merely changing its cost.

</details>

<details class="discussion-question">
<summary>How much work is hidden inside the nested loops?</summary>

- With `n` input words and `m` dictionary entries, the starter can perform `n × m` comparisons, although each successful match stops its inner loop early.
- Every attempted comparison constructs two lowercase strings, repeatedly converting both the input word and dictionary words.
- String lengths matter: conversion, allocation, and comparison are not constant-time operations on arbitrarily long words.

</details>

<details class="discussion-question">
<summary>Would using a set remove repeated misspellings or reorder the result?</summary>

- Only the dictionary becomes a set in the main solution; the input remains the sequence being traversed.
- Duplicate normalized dictionary entries do not affect membership, but repeated unknown input words remain repeated in the output.
- Hash-table iteration order does not determine output order because the result is collected from the input iterator, not the set.

</details>

<details class="discussion-question">
<summary>Can we remove the output clones without changing the API?</summary>

- The current result owns its strings and can outlive both borrowed arguments.
- Returning references avoids cloning string contents, but makes the result depend on the input strings' lifetimes.
- Slice arguments would accept more containers because the function only reads, but the main solution deliberately retains `&Vec<String>` and its targeted Clippy allowance.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/15_spell_check.rs`, preserving both arguments and the owned `Vec<String>` result.
- Lowercase every dictionary entry once per call and collect the resulting owned strings into a `HashSet<String>`.
- Iterate over input words, lowercase each for `contains`, retain nonmembers, then clone only the retained original strings.
- Expected table work becomes `O(m + n)` insertions/lookups, plus string conversion, hashing, equality checks, and output cloning; this is not a worst-case constant-time string lookup guarantee.
- The set owns distinct lowercase keys, while the original input strings supply the result's spelling and order.
- Dictionary preparation still happens on every call, including calls with no input words; the original does no dictionary comparison work in that case.

### Alternative: reusable ASCII dictionary and borrowed results

- Source: `solutions/15_spell_check_alt.rs`, also shown after the main solution in the slides.
- Its API is `spell_check<'a>(words: &[&'a str], dict: &HashSet<String>) -> Vec<&'a str>`.
- Callers prepare an ASCII-lowercased dictionary once and reuse it, moving construction cost out of repeated checks.
- `to_ascii_lowercase` leaves non-ASCII characters unchanged; this version neither implements the main solution's Unicode contract nor rejects non-ASCII input.
- `HashSet<String>` does not prove that its entries were lowercased; an unprepared dictionary can silently report known words as missing.
- The returned references borrow the underlying input strings, not the dictionary or the temporary slice containing those references.
- Migration: turn owned input strings into `&str` views, keep those strings alive, and update dictionary setup and assertions for the ASCII policy.
- Building a vector of views can allocate, the result vector still allocates when needed, and each lookup still constructs a lowercase string; borrowed output does not mean allocation-free execution.
- A reusable dictionary with Unicode lowercasing is a possible separate redesign if the original comparison policy must survive; it is not the checked-in alternative.

### Further alternative: lazy byte ranges

- `TODO.md` §6 points to the pinned `vimspell` API in References, not to an existing local lazy solution.
- That API takes a dictionary receiver and borrowed byte input, returning `impl Iterator<Item = Range<usize>>` instead of a vector of words.
- Its iterator stores the dictionary, input, and scan position, then advances until the next misspelling is requested.
- A caller can request only the first misspelling without materializing all results; this avoids a result vector, not every possible cost of spelling checks.
- Migration includes tokenizing a text buffer instead of accepting pre-tokenized words and interpreting ranges against that same buffer.
- Offsets count bytes, not characters; the pinned API documents valid UTF-8 boundaries for valid UTF-8 input, and unrelated byte-range APIs need their own boundary guarantees.
- TODO(prose): Decide whether this stays external reading or becomes a future runnable alternative; the current `_alt.rs` is eager and ASCII-based, and the `vimspell` spelling rules are not interchangeable with lowercase-set membership.

### Edge cases and test coverage

- Existing starter tests cover an ASCII misspelling among known words and an empty input vector.
- Main-solution tests additionally cover original spelling, order, repeated misses, duplicate dictionary entries, and an empty dictionary.
- Its Unicode test covers accented casing, `İ` matching `i` plus a combining dot, Greek final sigma, and `STRASSE` remaining unknown against `straße`.
- Existing alternative tests cover only the ASCII misspelling example and empty input, using a helper that prepares the dictionary correctly.
- Suggested alternative tests: empty dictionary, repeated misses, an unnormalized dictionary, and a non-ASCII pair that distinguishes it from the main solution.
- Suggested main-solution test: composed and decomposed accented spellings remain distinct, documenting that lowercasing does not normalize Unicode.
- The inputs are already strings to compare, not text to tokenize; punctuation and whitespace inside a supplied word are not stripped.

### Trade-offs

<!-- Final comparison matrices are deferred until the prose is drafted. -->

- **Type-safety:** Original and main accept ordinary strings with no spelling invariant; the ASCII alternative enforces borrowing lifetimes but not dictionary normalization or ASCII-only input.
- **Readability:** Original exposes the search and flag updates; main names membership with `contains`, while the alternative makes preparation and casing assumptions part of the caller's reading burden.
- **API design:** Original and main return independent owned strings; the checked-in alternative requires prepared dictionary storage and borrowed word views, while lazy ranges introduce a different input and result contract.
- **Performance:** Original can repeat normalization across `n × m` comparisons; main adds a per-call set to reduce repeated work, and the alternative reuses that set but still converts queries and collects results, so small or empty workloads need separate measurement.
- **Flexibility:** Original and main accept raw dictionaries each time; reusable storage suits repeated checks, while lazy results allow early stopping but require a deliberate policy for tokenization and spelling rules.
- **Error reporting:** Original and main return unmatched original strings without positions or reasons; the ASCII alternative adds no validation errors for a misprepared dictionary, while a range API can locate misses without explaining them.

</details>

## Takeaways For Our Own Codebase

> - Is normalization repeated inside a search, and can its preparation be reused without changing comparison semantics?
> - Would callers benefit from owned strings, borrowed words, or positions in the original text?
> - Which assumptions about prepared data belong in a constructor rather than an undocumented `HashSet<String>` convention?

## References

- [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase) and [`str::to_ascii_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_ascii_lowercase): different comparison policies.
- [`HashSet::contains`](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.contains): exact membership rather than a scan for a related string.
- [Pinned `vimspell` source](https://github.com/exrok/vimspell/blob/361b63f71228af3d00291d3c257cd487cf5b95b8/src/lib.rs#L824): the lazy byte-range API requested in `TODO.md`.
- [`excluded_path`](14-excluded-path.md): why a set does not accelerate every search when the actual operation is prefix matching rather than equality.
