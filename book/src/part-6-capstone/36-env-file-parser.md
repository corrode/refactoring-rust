# `env_file_parser`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Refactor a function that reads `.env` text and returns owned key/value pairs.
- The first solution must preserve `parse_env(content: &str) -> HashMap<String, String>` and the existing input-handling rules.
- Keep filesystem access and changes to the process environment outside this exercise.

## Code to refactor

- Source: `examples/36_env_file_parser.rs`, including its five unit tests.
- Current approach: iterate over `content.lines()`, trim each line, remove comments, collect separator-delimited parts, and insert entries into a map.
- Value processing trims whitespace and removes surrounding double quotes before insertion.
- The starter tests cover ordinary assignments, blank lines and comments, surrounding quotes, whitespace, and empty values.

## Discussion

<details class="discussion-question">
<summary>Which inputs does the starter accept, rather than a general dotenv parser?</summary>

- An assignment needs exactly one `=` before the first `#`, including when the value contains double quotes.
- `WITH_EQUALS=a=b=c` disappears from the result rather than storing `a=b=c`.
- Empty keys, empty values, and internal spaces in keys are accepted, so `=value`, `EMPTY=`, and `TWO WORDS=yes` are valid here.
- A familiar file extension does not establish a grammar beyond the rules this implementation actually applies.

</details>

<details class="discussion-question">
<summary>What do double quotes protect in the original parser?</summary>

- Nothing protects a `#` from comment removal, so `HASH="not # a comment"` stores the value `"not`.
- Only a pair of outer double quotes is removed, with whitespace inside that pair preserved.
- A lone double quote, unmatched quotes, single quotes, and text after a closing quote remain ordinary value text unless another rule rejects the line.
- Backslash escapes stay literal, and physical newlines split assignments even inside apparent quotes.
- The original byte slicing is safe for UTF-8 here because `find('#')` returns a character boundary and the removed quote delimiters are ASCII.

</details>

<details class="discussion-question">
<summary>What does the caller learn about a rejected line or duplicate key?</summary>

- A line without an assignment or with extra `=` characters is silently skipped, while valid neighboring lines remain available.
- The returned map cannot distinguish a missing assignment from a rejected assignment.
- The last accepted assignment wins for duplicate keys, including the empty key.
- A rejected duplicate does not erase an earlier value, and map iteration does not preserve file order.

</details>

<details class="discussion-question">
<summary>Would replacing the split with <code>split_once('=')</code> preserve behavior?</summary>

- Not by itself, because the remainder can contain more `=` characters that the original rejects.
- A separate check on the remainder preserves that rule while avoiding the temporary `Vec<&str>`.
- Comment removal must still happen first, so `KEY=value # ignored=1` remains an accepted assignment.
- Improving the grammar and reducing parsing machinery are different changes, even when both start with the same string method.

</details>

<details class="discussion-question">
<summary>When would explicit parser states justify a different API?</summary>

- Quoted comments, decoded escapes, and multiline values require the parser to remember context across characters and line breaks.
- Rejecting malformed assignments instead of skipping them requires an explicit failure policy, not just an internal enum.
- The alternative chooses fail-fast parsing with `Result`, which changes both what callers receive and which files they can load.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/36_env_file_parser.rs`, with the original map-returning signature and owned key/value types.
- `parse_env` collects the results of `content.lines().filter_map(parse_line)` into the map.
- Private `parse_line` returns `Option<(String, String)>`, using `None` for lines the original would skip.
- `split_once('#')` selects the comment-free prefix before `split_once('=')` locates the assignment.
- The `value.contains('=')` guard preserves rejection of additional separators, including separators inside quotes.
- The `strip_prefix` and `strip_suffix` chain removes both quotes or keeps the entire original trimmed value, rather than removing only an opening quote.
- `lines()` retains LF and CRLF handling, while `trim()` retains Unicode whitespace trimming for keys and values.
- Strings are allocated after acceptance and quote handling, avoiding the original split vector and its extra copy of a quoted value.
- No caller or test signature migration is needed, and duplicate replacement, silent skipping, and unordered output remain unchanged.
- This solution does not add escape interpretation, multiline quoting, key validation, or error reporting.

### Alternative: an explicit parser state machine

- Source: `solutions/36_env_file_parser_alt.rs`, the only checked-in alternative for this exercise.
- The public signature becomes `parse_env(content: &str) -> Result<HashMap<String, String>, ParseError>`.
- Public `ParseError` contains `line: usize` and `kind: ErrorKind`, with `MissingEquals`, `EmptyKey`, `UnterminatedQuote`, and `TrailingGarbage` variants.
- `LineStart`, `InKey`, and `AfterKey` separate leading whitespace, key accumulation, and whitespace before the separator.
- `BeforeValue`, `InBareValue`, `InQuotedValue`, `InQuotedEscape`, and `AfterQuotedValue` distinguish value syntax and escape handling.
- `InComment` skips characters until a newline, then returns to `LineStart`.
- `step` matches the current state and the next `char`, mutating key/value buffers, the line counter, the output map, and a separate `quoted` flag.
- The enum makes control flow explicit but does not encode the buffers or `quoted` flag inside each state, so their consistency still depends on the implementation.
- `commit` moves keys and quoted values with `mem::take`, but copies a trimmed bare value before clearing its reusable buffer.
- A first error returns immediately through `?`, without returning earlier entries or collecting later errors.

### Grammar and caller migration

- Callers must handle or propagate the new `Result` before reading the map, and function-pointer types must change with the return type.
- The alternative's test helper unwraps inputs expected to succeed, while error tests inspect `ParseError` rather than treating it as a missing key.
- Returning a default map on error would discard valid earlier entries and is not equivalent to the original skip-and-continue policy.
- Extra `=` characters are accepted in bare and double-quoted values, and quoted `#` characters remain part of the value.
- Bare values still end at `#`, skip leading whitespace, and trim trailing Unicode whitespace.
- Double quotes introduce quoted syntax only at the start of the value after leading whitespace, not in the middle of a bare value.
- After a closing double quote, only whitespace, a comment, a newline, or EOF is allowed, with other text producing `TrailingGarbage`.
- Quoted escapes decode `\n`, `\t`, and `\r`, while `\"` and `\\` retain the escaped character without the backslash.
- Unknown quoted escapes also discard the backslash, so `\q` becomes `q`, while bare backslashes have no escape role.
- Empty keys now produce `EmptyKey`, and whitespace within a key leads to `MissingEquals` unless the next non-whitespace character is `=`.
- Key punctuation is not generally validated, and `A#B=1` now uses the key `A#B` because `#` inside `InKey` is ordinary text.
- Empty values and last-assignment-wins duplicates remain supported when the complete input parses successfully.
- Single quotes are still ordinary characters, shell variable expansion is absent, and an `export KEY=value` prefix is not supported.

### EOF, multiline values, and diagnostic limits

- At EOF, `LineStart` and `InComment` finish without committing another entry.
- At EOF, `InKey` and `AfterKey` produce `MissingEquals`.
- At EOF, `BeforeValue`, `InBareValue`, and `AfterQuotedValue` commit an empty, bare, or completed quoted value respectively.
- At EOF, `InQuotedValue` and `InQuotedEscape` produce `UnterminatedQuote`, including a final dangling backslash inside quotes.
- Physical newlines inside quotes are preserved in values, rather than ending the assignment as they do in the original and main solution.
- Quoted CRLF is preserved as two characters, while CRLF outside quotes is handled through whitespace rules and the LF transition.
- `chars()` walks Unicode scalar values rather than bytes, and only physical LF transitions outside the two quoted states increment the line counter.
- Source trace: `M="first\nsecond"\nBROKEN`, with `\n` denoting physical line breaks, reports `MissingEquals` on line 2 rather than line 3.
- The reported number is neither a reliable physical error line after multiline content nor a saved opening-quote location.
- `ParseError` derives `Debug`, `PartialEq`, and `Eq`, but does not implement `Display` or `std::error::Error` or carry a column or source span.
- None of the three implementations enforces input-size limits or loads environment variables, and any future diagnostics should avoid disclosing secret values.

### Existing tests and author decisions

- **Existing starter tests:** the five ordinary cases establish basic behavior without claiming coverage of the full grammar.
- **Existing main-solution tests:** nine tests include Unicode whitespace, CRLF, empty and spaced keys, unmatched quotes, single quotes, extra separators, and comments inside quotes.
- **Existing main-solution tests:** duplicate handling, literal escapes, and independently parsed lines inside apparent multiline quotes record limitations as compatibility expectations.
- **Existing alternative tests:** fourteen tests cover ordinary assignments, extra separators, quoted hashes, decoded newline/tab escapes, inline comments, and all four error kinds.
- **Existing alternative tests:** `line_number_tracks_across_lines` checks line 3 after an ordinary assignment and blank line, not after a multiline quoted value.
- **Recommended coverage:** a complete EOF-state matrix, comments after quoted values, escaped quotes/backslashes, and the policy for unknown escapes.
- **Recommended coverage:** actual multiline values, line numbers after quoted or escaped physical newlines, quoted CRLF, Unicode input, and duplicate keys in the alternative.
- **Recommended coverage:** compare the original and main solution over generated short UTF-8 inputs, keeping alternative grammar expectations in a separate suite.
- No new tests accompany this outline, and a passing existing suite does not establish that the alternative's multiline diagnostics are correct.

<!-- TODO(prose): Decide how to frame the opening slide notes that call extra separators, quoted hashes, and the map-returning API bugs, while the main code and later slide notes deliberately preserve those rules. -->
<!-- TODO(prose): The slides advertise multiline support and the alternative promises line locations, but step does not count physical LF inside either quoted state; decide how prominently to teach this limitation without presenting it as fixed. -->
<!-- TODO(prose): Describe the alternative as a chosen dotenv dialect rather than complete dotenv compatibility, and decide which unsupported shell conventions deserve space in the finished chapter. -->

### Trade-offs

- **Type-safety:** Original returns an unrestricted string map; the main solution keeps that contract, while the alternative adds explicit error and control-state enums without enforcing conventional key syntax or buffer invariants in types.
- **Readability:** Original exposes each line operation in a loop; the main solution isolates one line's rules, while the alternative makes contextual syntax visible at the cost of nine states and shared mutable parser data.
- **API design:** Original and main return partial accepted data without a failure signal; the alternative requires caller migration and returns either a complete map or the first error, not the same contract with a different wrapper.
- **Performance:** Original collects separator parts and recopies quoted values; the main solution avoids that work, while the alternative builds character buffers and copies bare values on commit, with no measured speed comparison between grammars.
- **Flexibility:** Original and main remain line-oriented with fixed comment precedence; the alternative supports context-sensitive quoting and multiline values, but additional dialect rules require explicit transitions and EOF decisions.
- **Error reporting:** Original and main silently skip rejected lines; the alternative names four failures but stops at the first and has inaccurate locations after multiline quoted input, so richer errors are not yet consistently accurate errors.

</details>

## Takeaways For Our Own Codebase

> - Which parser changes preserve accepted input, and which silently introduce a new dialect?
> - Record comment precedence, duplicate handling, and malformed-line policy before replacing string operations.
> - Test EOF and physical newlines in every state that can consume them, including states that preserve them as data.
> - Decide whether callers need partial results, one error, or accumulated diagnostics before changing the return type.

## References

- [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) separates the first delimiter without deciding whether later delimiters are legal.
- [`Iterator::filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) collects successful per-line extractions while omitting `None`.
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) defines LF and CRLF splitting for the compatibility implementation.
- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) explains the alternative's Unicode scalar iteration.
- [`Result`](https://doc.rust-lang.org/std/result/) documents explicit success and failure rather than partial-data conventions.
- [`mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) moves owned buffers out while leaving default values behind.
- [The six trade-off axes](../appendix/trade-off-axes.md#the-six-axes) provide the comparison criteria for the finished tables.
- [`mini_redis` solutions](37-mini-redis.md#solutions) make the same distinction between compatibility work and a new contract.
