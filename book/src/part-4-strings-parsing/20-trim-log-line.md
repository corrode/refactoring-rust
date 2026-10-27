# `trim_log_line`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Clean up a log message by removing a leading severity tag and normalizing whitespace.
- Keep `trim_log_line(line: &str) -> String` for the first solution.
- Account for untagged messages as well as `[INFO]`, `[WARN]`, and `[ERROR]` messages.

## Code to refactor

- Starter and its tests: `examples/20_trim_log_line.rs`.
- Workshop prompts and speaker notes: `slides/pages/20_trim_log_line.md`.
- Walk characters into a new string, skipping leading whitespace until the first other character.
- Pop selected trailing characters, then check the three tags in array order.
- Slice off a matching tag and at most one following ASCII space.
- Replace tabs and newlines, then use `prev_space` to collapse adjacent ASCII spaces.
- Track the intermediate owned strings separately from the final output buffer.

## Discussion

<details class="discussion-question">
<summary>Does every stage mean the same thing by whitespace?</summary>

- Leading whitespace uses `char::is_whitespace`, which follows Unicode whitespace classification.
- Trailing cleanup recognizes only ASCII space, tab, and newline.
- Internal replacement and collapse also handle only those three characters.
- A carriage return after message text survives, and an internal non-breaking space remains unchanged.
- Distinguish the documented cleanup task from the behavior currently implemented.

</details>

<details class="discussion-question">
<summary>What happens to whitespace immediately after a tag?</summary>

- Tag removal happens after the initial trim-like passes.
- `[INFO]  hello` leaves one leading space in the starter's result.
- `[INFO]\thello` also leaves a leading space after the tab replacement.
- Matching is case-sensitive and requires no separator: `[INFO]hello` is recognized too.
- Only one leading tag is removed; `[INFO][WARN] hello` keeps `[WARN]` as message text.

</details>

<details class="discussion-question">
<summary>Are the byte slices unsafe for Unicode text?</summary>

- `tag.len()` counts bytes, but an exact `starts_with(tag)` match proves that boundary belongs to the matched ASCII prefix.
- The later slice at byte one is guarded by an ASCII-space prefix check.
- These particular slices do not cut a UTF-8 character; arbitrary byte offsets would be a different problem.
- The useful improvement is removing duplicated prefix-checking and slicing logic, not repairing an existing UTF-8 panic.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/20_trim_log_line.rs`, with the original signature and owned result.
- `trim` returns a borrowed substring without copying message bytes.
- `find_map` tries each tag's `strip_prefix` result and stops at the first `Some`.
- `unwrap_or(trimmed)` keeps untagged text; it is a fallback, not a panic-producing unwrap.
- `split_whitespace` yields nonempty borrowed words in input order.
- Collect word references into `Vec<&str>`, then join with one ASCII space into the output `String`.
- No intermediate owned message string before the join, but the word-reference vector still needs storage.
- The fixed three-tag search and whitespace passes take O(n) time for n input bytes, with O(n + w) storage for output and w word references.
- Explain the changed behavior: trailing Unicode whitespace, internal Unicode whitespace, and whitespace after a removed tag are normalized more broadly.
- This preserves the API, but it is not a behavior-preserving refactor over all `&str` inputs.

<!-- TODO(prose): Decide whether to present the broader normalization as intentional bug fixes or require an exact-compatibility variant before it.
The speaker notes in slides/pages/20_trim_log_line.md equate the first two loops with trim, but their trailing-whitespace rules differ; the implemented tag search is find_map, not the fold mentioned in those notes. -->

### Alternative approaches

- Possible same-signature variation, not a separate repository solution: append borrowed words into one output buffer with a separator between words.
- That avoids collecting a word-reference vector but restores explicit separator state.
- Preserve the canonical whitespace and tag rules when evaluating that allocation change.
- Exact compatibility with the starter requires retaining its narrower trailing/internal whitespace rules and its post-tag spacing behavior.
- A borrowed `&str` return cannot represent arbitrary collapsed text; a conditional-borrowing API would be a separate caller-facing design.

### Edge cases and test coverage

- Current starter and solution suites contain the same four tests: INFO removal, internal spaces/tabs, outer ASCII spaces, and untagged text.
- Those tests do not establish equivalence across the full Unicode input domain.
- Proposed regression cases: both other severity tags, adjacent tag/message text, multiple spaces after a tag, and a tab after a tag.
- Proposed contract cases: lowercase tags, unknown tags, embedded tags, and repeated leading tags.
- Proposed whitespace cases: empty input, whitespace-only input, tag-only input, CRLF endings, and internal non-breaking spaces.
- Record different expected results for starter and canonical normalization where behavior deliberately changes.
- Unicode whitespace is not every invisible character; a zero-width space is not removed by these standard whitespace methods.
- Preserve non-whitespace message bytes and their order; neither version performs Unicode normalization or parses a complete log format.

### Trade-offs

<!-- Final comparison matrices are deferred while this chapter is a bullet outline. -->

- **Type-safety:** **Original:** arbitrary `&str` input and `String` output; main keeps those types without proving that the input is a valid log record.
- **Readability:** **Original:** several flags, buffers, and loops; main names trim, prefix removal, and word joining, while the streaming variation needs separator bookkeeping.
- **API design:** **Original:** borrowed input and owned output; main preserves the signature, but its wider normalization must be documented as a behavior change.
- **Performance:** **Original:** multiple linear scans and owned intermediate buffers; main is still O(n), trading those buffers for borrowed word references and a final join, without a measured speed claim.
- **Flexibility:** **Original:** a fixed array of recognized tags; main retains that policy, while its Unicode whitespace handling may be too broad for byte-sensitive log payloads.
- **Error reporting:** **Original:** unknown tags remain text and no error is reported; main does the same, so callers still cannot use this helper as a validator.

</details>

## Takeaways For Our Own Codebase

> - Inventory which whitespace rules our cleanup functions actually use before substituting a standard helper.
> - Separate output changes from ownership and allocation changes in the review.
> - Add tests for separators next to removed prefixes, not just text in the middle.

## References

- [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim) and [`split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace): Unicode whitespace and borrowed substrings.
- [`str::strip_prefix`](https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix): match and remainder in one operation.
- [`Iterator::find_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find_map): stop at the first successful optional result.
- [Unicode casing in `fun_strings_ext`](24-fun-strings-ext.md#solutions): another case where a shorter implementation can change text semantics.
