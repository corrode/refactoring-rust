# Trade-off matrix

> **Draft outline.** Bullet notes for the appendix; explanations still to be written.

- A plan for concrete comparison rows drawn from the chapters, not a completed scorecard or a new set of ratings.
- Each comparison starts with that exercise's **Original** implementation, then identifies the main solution and any distinct alternatives.
- Keep the chapter link and contract qualification beside each eventual row; a shorter implementation is not automatically a compatible replacement.
- Use separate groups for separate functions and repeat **Original** when splitting a comparison across tables.

## Columns and interpretation

- **Type-safety:** identify the invariant actually enforced, distinguishing compiler rejection, constructor validation, and runtime convention.
- **Readability:** name the bookkeeping removed or the concepts added, with familiarity assumptions rather than a universal score.
- **API design:** record ownership, public types, permitted inputs, result meaning, and the caller migration required.
- **Performance:** name allocations, copying, traversal, asymptotic work, and retained storage without inventing benchmark results.
- **Flexibility:** explain what becomes easier to extend and what deliberate restrictions remove.
- **Error reporting:** distinguish absence, runtime errors, panics, compile-time rejection, lost context, and changed messages.
- Preserve this six-axis order from [The trade-off axes](trade-off-axes.md#the-six-axes); accepted-input notes are contract context, not a seventh score.
- Final cells need a relative judgement and a concrete reason, with **N/A** reserved for criteria that do not apply rather than unknown or unmeasured results.
- Do not add scores across axes, rank unrelated exercises, or declare an overall winner.

## Part I: warm-ups

- [01: is_yes](../part-1-warmups/01-is-yes.md): compare Original with owned exact matching, borrowed exact matching, ASCII case-insensitive comparison, and allocated ASCII lowercasing.
  - Separate the original four accepted spellings from the broader ASCII-case contract before comparing allocation and ownership.
- [02: starts_with_uppercase](../part-1-warmups/02-starts-with-uppercase.md): compare Original with owned `is_some_and`, borrowed `is_some_and`, and the borrowed `starts_with` predicate.
  - All inspect the first Unicode scalar value, while only the borrowed alternatives change the caller's ownership obligations.
- [03: better_match](../part-1-warmups/03-better-match.md): compare Original with nested or-patterns, keeping the same labels and wildcard fallback rather than treating compact syntax as a new invariant.
- [04: even_numbers](../part-1-warmups/04-even-numbers.md): distinguish eager `step_by` collection from lazy `step_by` and lazy filtering, with indexing, repeated traversal, and partial consumption recorded explicitly.
- [05: truncate_string](../part-1-warmups/05-truncate-string.md): group Original, boundary-aware byte truncation, and manual boundary search together, but qualify the scalar-count alternative as a different unit.
  - Keep the panic fix, copied-prefix size, toolchain requirement, and absence of a grapheme guarantee visible.

## Part II: optional values and failures

- [06: path](../part-2-option-result/06-path.md): compare Original's nested conversion checks with `is_some_and`, `map_or`, and explicit matching, without turning lexical classification into a filesystem guarantee.
- [07: nesting](../part-2-option-result/07-nesting.md): compare Original with a `map`/`filter` pipeline and early-return control flow, preserving the merged `None` cases and allocation after validation.
- [08: optional_values](../part-2-option-result/08-optional-values.md): distinguish owned flattening, identity `filter_map`, and a borrowed-slice design, while leaving integer overflow as a separate policy.
- [09: parse_ints](../part-2-option-result/09-parse-ints.md): create separate row groups for fail-fast `parse_values` and collect-all `collect_errors`.
  - Compare owned and borrowed parsing APIs only within the parsing group; replacing collect-all validation with fail-fast collection would change the operation being scored.
- [10: floor](../part-2-option-result/10-floor.md): compare the loop, `map` plus `sum` over `Result`, and `try_fold`, preserving the first invalid instruction and zero for empty input.
  - These variants need no intermediate vector; `try_fold` exposes the running floor when later work needs state-dependent decisions.
- [11: error_handling](../part-2-option-result/11-error-handling.md): compare explicit propagation, open/read with `?`, `fs::read_to_string`, and a broader path parameter separately from file-content encoding and error context.
- [12: parse_port](../part-2-option-result/12-parse-port.md): compare Original and the signature-preserving `Option<u16>` parser before the `Result<NonZeroU16, InvalidPort>` redesign.
  - The alternative enforces an existing nonzero rule but changes caller handling and still groups syntax and range failures together.

## Part III: aggregation and collection contracts

- [13: dedup_runs](../part-3-aggregations/13-dedup-runs.md): compare Original's second output buffer with in-place `Vec::dedup`, noting that adjacent-run semantics stay the same but retained capacity can differ.
- [14: excluded_path](../part-3-aggregations/14-excluded-path.md): retain the chapter's separate optional-parameter and redesigned-interface comparisons.
  - Rows cover the main `let ... else`/`any` refactor, optional borrow, plain set borrow, `Excluder`, and extension trait; none changes the prefix scan into a hash membership lookup.
- [15: spell_check](../part-3-aggregations/15-spell-check.md): compare Original with per-call Unicode-lowercase set preparation, then qualify the reusable ASCII dictionary and borrowed output as a different contract.
  - Keep the proposed lazy byte-range API outside the implemented-variant ratings until its grammar, implementation, and tests are selected.
- [16: room_occupancy](../part-3-aggregations/16-room-occupancy.md): separate totals, maximum selection, and flattened age collection rather than giving four helpers one undifferentiated score.
  - The maximum changes for all-negative counts, because an empty-only fallback differs from a zero-seeded accumulator.
- [17: text_stats](../part-3-aggregations/17-text-stats.md): compare Original's formulas with occurrence totals, distinct counts, and a weighted average, labeling the corrected results and empty-input policy as behavioral changes.
- [18: mode](../part-3-aggregations/18-mode.md): compare Original's maximum bookkeeping with `max_by_key`, preserving the panic and counter limitations while allowing a different tied winner.
- [19: list_sum](../part-3-aggregations/19-list-sum.md): compare Original with `while let` and the recursive match, including changed arithmetic association, traversal stack, and destruction during unwinding.

## Part IV: text processing and algorithms

- [20: trim_log_line](../part-4-strings-parsing/20-trim-log-line.md): compare explicit loops with named trimming, prefix removal, and joining operations, while documenting the wider whitespace normalization in the main solution.
- [21: parse_srt_timestamp](../part-4-strings-parsing/21-parse-srt-timestamp.md): separate the checked main parser from the richer-error alternative, which does not yet check arithmetic overflow.
  - Neither the `Option` nor `Result` return type alone proves strict SRT field widths, component ranges, or use of the full `Duration` range.
- [22: iterators](../part-4-strings-parsing/22-iterators.md): separate scalar Fibonacci calculation, eager prefixes, and lazy iteration when comparing recursive recomputation with linear state updates.
  - The current iterator's lookahead can overflow before yielding a requested valid value, so laziness is not a blanket safety improvement.
- [23: transformer](../part-4-strings-parsing/23-transformer.md): use separate batch-command and trait-object-pipeline rows, distinguishing removed input clones from intermediate allocations and the corrected empty-string sentinel behavior.
- [24: fun_strings_ext](../part-4-strings-parsing/24-fun-strings-ext.md): compare free functions and folding without conflating them with extension-trait syntax or the alternative's narrower ASCII casing.
- [25: quicksort](../part-4-strings-parsing/25-quicksort.md): compare the copied-buffer in-place algorithm with recursive partition-and-concatenate, including skewed-input recursion and the main solution's possible quadratic retained heap storage.
  - Standard sorting is a production design option, not evidence that the displayed recursive refactor has better time or memory bounds.

## Part V: domain modeling

- [26: rooms](../part-5-domain-modeling/26-rooms.md): compare slice-based helpers with the owned `Rooms` API, separating named collection ownership from validation that the constructor does not perform.
- [27: iban_prefix_check](../part-5-domain-modeling/27-iban-prefix-check.md): compare Original with the allocation-free checked prefix/length implementation, without claiming full IBAN checksum validation or a checked wrapper already exists.
- [28: http_response_router](../part-5-domain-modeling/28-http-response-router.md): compare Original with private status classification, preserving the public types while recording removed logging and the retained decode-before-204 ordering.
- [29: config_loader](../part-5-domain-modeling/29-config-loader.md): separate the compatible getter cleanup from the fallible, privately stored `Settings` snapshot.
  - Defaults on invalid input, per-call observation, borrowed host access, and all-or-error construction are different policies, not interchangeable convenience details.
- [30: buffer_policy](../part-5-domain-modeling/30-buffer-policy.md): compare two configuration fields with mutually exclusive enum variants, keeping the existing assertion behavior separate from representable policy combinations.
- [31: typed_ids](../part-5-domain-modeling/31-typed-ids.md): compare aliases with separate newtypes, attributing compiler rejection only to category mix-ups rather than missing entries or IDs from another owner.
- [32: validation_feedback](../part-5-domain-modeling/32-validation-feedback.md): compare open string categories and allocated messages with enum categories and static messages, including the closed-set migration and remaining vector allocation.
- [33: secret_token](../part-5-domain-modeling/33-secret-token.md): compare ordinary string formatting with a restricted wrapper and redacted `Debug`, not with encryption, memory locking, or guaranteed erasure.
- [34: config_typestate](../part-5-domain-modeling/34-config-typestate.md): compare runtime missing-roots errors with consuming concrete-state transitions, recording both compiler rejection and the loss of the starter's replacement flexibility.
- [35: friends](../part-5-domain-modeling/35-friends.md): compare shared person handles with one owner and local IDs, including self-link behavior, equal-name identity, lifetime control, and unresolved cross-owner IDs.

## Part VI: larger refactors

- [36: env_file_parser](../part-6-capstone/36-env-file-parser.md): compare Original with the compatible line helper first, then treat the state-machine alternative as a different grammar and failure policy.
  - Its richer errors do not yet imply reliable line locations after multiline quoted input, and partial accepted data differs from all-or-error parsing.
- [37: mini_redis](../part-6-capstone/37-mini-redis.md): compare Original with the compatible dispatch cleanup before the single-map value/command/reply redesign.
  - Keep store-field compatibility, cross-type collisions, arity behavior, command casing, and reply formatting beside the API comparison even though the handler signature stays unchanged.

## Conditions before final ratings

- TODO(prose): Transfer reviewed chapter judgements only after each contract and alternative is settled, rather than converting draft bullets into unexplained icons.
- Keep unimplemented ideas labeled as proposals, even when their design trade-offs can already be discussed.
- Include fixes for intended behavior separately from compatibility changes, especially where the signature remains identical.
- Link each comparison back to its tests and chapter explanation; absence of a regression test is not evidence that two behaviors are equivalent.
- Consult the [Concept index](concept-index.md) for definitions and [Further reading](further-reading.md) for source provenance, neither of which substitutes for evaluating the local implementation.
