# Cheatsheet (Concept index)

> **Draft outline.** Bullet notes for the appendix; explanations still to be written.

- A working selection from chapter References and discussions, not a claim that every concept in the final book is indexed.
- Follow the chapter links for the actual contract, implementation, tests, and qualifications rather than treating an idiom as a universal recommendation.
- The headings retain the concept anchors already used by chapter links; related entries connect the same idea across different exercises.

## Borrowing and ownership

### Borrowed string slices

- `&str` reads existing UTF-8 text without taking ownership or copying it; changing a `String` parameter to a borrow is still an API migration in [01: is_yes](../part-1-warmups/01-is-yes.md) and [02: starts_with_uppercase](../part-1-warmups/02-starts-with-uppercase.md).

### Borrowed slices

- `&[T]` accepts a read-only sequence without requiring an owned vector, but does not make an owned output free to construct: [09: parse_ints](../part-2-option-result/09-parse-ints.md) and [15: spell_check](../part-3-aggregations/15-spell-check.md).

### Optional borrows

- `Option<&T>` describes an optional view without requiring the caller to store an `Option<T>`; compare it with `&Option<T>` in [14: excluded_path](../part-3-aggregations/14-excluded-path.md).

### Borrow before owning

- Check a borrowed view before constructing the owned result, avoiding work for rejected values without changing acceptance rules: [07: nesting](../part-2-option-result/07-nesting.md).

### Ownership and into iter

- Consuming a vector with `into_iter` yields its elements by value, unlike iterating over shared references; see [08: optional_values](../part-2-option-result/08-optional-values.md) and the removed string clones in [23: transformer](../part-4-strings-parsing/23-transformer.md).

### Iterator copied

- `copied()` turns `&T` into `T` when `T: Copy`, not arbitrary owned clones; [16: room_occupancy](../part-3-aggregations/16-room-occupancy.md) uses it when collecting nested age values.

## Patterns and control flow

### Or-patterns

- `|` joins alternative patterns with the same action, without broadening them into case-insensitive comparison or a numeric range: [01: is_yes](../part-1-warmups/01-is-yes.md) and [03: better_match](../part-1-warmups/03-better-match.md).

### Nested patterns

- Alternatives can sit inside a shared enum constructor instead of repeating it in every arm; [03: better_match](../part-1-warmups/03-better-match.md) keeps the original labels and fallback.

### Wildcard patterns

- `_` accepts all values left by preceding arms, which can be deliberate fallback behavior or can hide newly added cases: [03: better_match](../part-1-warmups/03-better-match.md).

### Exhaustive matching

- A match must cover the input type, but a wildcard can satisfy coverage without naming every policy case; compare [03: better_match](../part-1-warmups/03-better-match.md), [28: http_response_router](../part-5-domain-modeling/28-http-response-router.md), and [32: validation_feedback](../part-5-domain-modeling/32-validation-feedback.md).

### Let else

- `let ... else` binds the successful pattern and requires the other branch to diverge, often by returning early; [14: excluded_path](../part-3-aggregations/14-excluded-path.md) separates absence from the prefix scan.

## Optional values and errors

### Option map

- `Option::map` transforms the value inside `Some` and leaves `None` alone; it is not a rejection predicate: [07: nesting](../part-2-option-result/07-nesting.md).

### Option filter

- `Option::filter` keeps or rejects a present value through a predicate without changing its type; [07: nesting](../part-2-option-result/07-nesting.md) rejects an empty trimmed view before allocating uppercase text.

### Option combinator pipelines

- A sequence of transformations and rejections can replace nested branches, but merged `None` cases still lose their distinct reasons: [07: nesting](../part-2-option-result/07-nesting.md).

### Option is some and

- `Option::is_some_and` asks whether a value exists and satisfies a predicate, returning `false` on absence: [02: starts_with_uppercase](../part-1-warmups/02-starts-with-uppercase.md) and [06: path](../part-2-option-result/06-path.md).

### Option map or

- `Option::map_or` transforms `Some` or returns a supplied default, whose expression is evaluated eagerly; [06: path](../part-2-option-result/06-path.md) uses the inexpensive default `false`.

### Option versus Result

- `Option` represents absence without a reason, while `Result` can preserve a failure category; [12: parse_port](../part-2-option-result/12-parse-port.md) contrasts the existing absence result with a typed-error alternative.

### Error propagation

- Returning an error preserves the caller's opportunity to act, but does not automatically add context or handle the failure: [11: error_handling](../part-2-option-result/11-error-handling.md).

### Question mark operator

- `?` continues with a success value or returns early through a compatible failure path; it is propagation rather than recovery: [11: error_handling](../part-2-option-result/11-error-handling.md).

### Error conversion combinators

- `map_err` transforms an existing error, while `ok_or` turns absence into an error; [12: parse_port](../part-2-option-result/12-parse-port.md) distinguishes zero from parsing failures without preserving every parser detail.

### IO result

- `io::Result<T>` is the standard `Result` with `io::Error`, not a new error-handling mechanism; [11: error_handling](../part-2-option-result/11-error-handling.md) preserves filesystem, read, and text-decoding failures.

### Collecting results

- Collecting `Result<T, E>` items into `Result<Vec<T>, E>` stops at the first error and does not return the partial vector: [09: parse_ints](../part-2-option-result/09-parse-ints.md).

### Collect-all validation

- Running every validator and retaining all errors is a different contract from fail-fast collection; [09: parse_ints](../part-2-option-result/09-parse-ints.md) preserves validator order in the report.

### Return-type inference

- A declared return type can determine both the parsed element type and `collect`'s target, but cannot decide whether the chosen failure policy is appropriate: [09: parse_ints](../part-2-option-result/09-parse-ints.md).

### Standard-library convenience functions

- A function such as `fs::read_to_string` can replace an entire open-and-read sequence, while hiding intermediate steps needed for limits or streaming: [11: error_handling](../part-2-option-result/11-error-handling.md).

## Iteration and aggregation

### Iterator next

- `next()` requests one item and returns `Option`, rather than requiring a complete traversal; compare the first-character check in [02: starts_with_uppercase](../part-1-warmups/02-starts-with-uppercase.md) with stateful generation in [22: iterators](../part-4-strings-parsing/22-iterators.md).

### Lazy iterators

- Lazy production lets consumers avoid an unused suffix, but collecting still allocates and generating the next item may do lookahead work: [04: even_numbers](../part-1-warmups/04-even-numbers.md) and [22: iterators](../part-4-strings-parsing/22-iterators.md).

### Half-open ranges

- `0..max` excludes `max` and is empty when its end is not greater than zero, which matters for the sequence contract in [04: even_numbers](../part-1-warmups/04-even-numbers.md).

### Iterator step by

- `step_by(2)` expresses traversal at a fixed stride rather than testing every candidate's parity; [04: even_numbers](../part-1-warmups/04-even-numbers.md) separates this implementation choice from returning a lazy API.

### Return-position impl Trait

- `impl Iterator<Item = T>` exposes iteration while hiding the concrete adapter type, but no longer promises vector indexing or repeatable traversal: [04: even_numbers](../part-1-warmups/04-even-numbers.md).

### Iterator consumers

- Consumers such as `sum` and `collect` drive evaluation, while adapters such as `take` limit what a later consumer requests: [04: even_numbers](../part-1-warmups/04-even-numbers.md) and [10: floor](../part-2-option-result/10-floor.md).

### Option as an iterator

- `Some` contributes one item and `None` contributes none, letting optional values participate in iterator composition without a manual presence branch: [08: optional_values](../part-2-option-result/08-optional-values.md).

### Iterator flatten

- `flatten` removes one level of nested iteration, including the zero-or-one items in each `Option`; it does not recursively flatten arbitrary nesting: [08: optional_values](../part-2-option-result/08-optional-values.md).

### Filter map

- `filter_map` transforms each item into `Option` and keeps only `Some`; retaining errors through `.err()` in [09: parse_ints](../part-2-option-result/09-parse-ints.md) differs from silently skipping invalid lines in [36: env_file_parser](../part-6-capstone/36-env-file-parser.md).

### Flat map

- `flat_map` produces an inner iterator for each item and traverses those iterators in sequence; [16: room_occupancy](../part-3-aggregations/16-room-occupancy.md) flattens nested collections without discarding their traversal order.

### Iterator sum

- Numeric `sum` starts from an additive identity, while summing `Result` items also stops at the first error; compare [08: optional_values](../part-2-option-result/08-optional-values.md) and [10: floor](../part-2-option-result/10-floor.md).

### Map and sum

- Select the quantity to aggregate before adding it; [16: room_occupancy](../part-3-aggregations/16-room-occupancy.md) sums fields, while [17: text_stats](../part-3-aggregations/17-text-stats.md) needs occurrence-weighted lengths rather than one length per distinct word.

### Iterator max

- `max` returns `None` for an empty iterator, and a fallback after reduction is not an initial candidate inside it; [16: room_occupancy](../part-3-aggregations/16-room-occupancy.md) exposes this distinction with negative counts.
- `max_by_key` retains the last tied maximum in traversal order, which is not an input-order guarantee for a hash map: [18: mode](../part-3-aggregations/18-mode.md).

### Accumulator identity values

- Zero is an additive identity but not an identity for maximum over arbitrary signed integers; [16: room_occupancy](../part-3-aggregations/16-room-occupancy.md) contrasts a zero-seeded fold with an empty-only fallback.
- A legitimate empty string must not be confused with “no accumulator yet,” as the pipeline in [23: transformer](../part-4-strings-parsing/23-transformer.md) demonstrates.

### Empty collections

- An empty collection can represent “no configured items” when absence carries no extra meaning, but missing configuration may still need a distinct state elsewhere: [14: excluded_path](../part-3-aggregations/14-excluded-path.md).

### Short-circuiting

- Stop only when the current item determines the whole result; fail-fast parsing in [09: parse_ints](../part-2-option-result/09-parse-ints.md) and the first invalid instruction in [10: floor](../part-2-option-result/10-floor.md) differ from collecting every validation error.

### Iterator any

- `any` stops at the first match and returns `false` for an empty iterator; [14: excluded_path](../part-3-aggregations/14-excluded-path.md) still performs a prefix scan rather than an exact hash lookup.

### Adjacent deduplication

- Removing equal neighbors preserves repeats separated by other values, unlike global uniqueness or sorting before deduplication: [13: dedup_runs](../part-3-aggregations/13-dedup-runs.md).

### Frequency maps

- Keys identify distinct values and mapped counts represent occurrences; [17: text_stats](../part-3-aggregations/17-text-stats.md) sums those counts, while [18: mode](../part-3-aggregations/18-mode.md) selects by them.

### Stateful iteration

- A state pair can avoid recursive recomputation, but arithmetic done before yielding can overflow before the requested item is returned: [22: iterators](../part-4-strings-parsing/22-iterators.md).

## Text, paths, and numeric boundaries

### Unicode scalar values

- Rust `char` values are Unicode scalar values, not bytes or grapheme clusters; see [02: starts_with_uppercase](../part-1-warmups/02-starts-with-uppercase.md), [05: truncate_string](../part-1-warmups/05-truncate-string.md), and [17: text_stats](../part-3-aggregations/17-text-stats.md).

### UTF-8 byte boundaries

- String slicing uses byte offsets and requires valid UTF-8 boundaries, even in safe Rust; [05: truncate_string](../part-1-warmups/05-truncate-string.md) and [27: iban_prefix_check](../part-5-domain-modeling/27-iban-prefix-check.md) separate byte limits from character checks.

### Floor char boundary

- `str::floor_char_boundary` finds a valid boundary no greater than a byte limit, not a grapheme boundary or character-count limit; [05: truncate_string](../part-1-warmups/05-truncate-string.md) also discusses older-toolchain alternatives.

### Unicode case conversion

- Unicode casing may expand text and differs from ASCII-only conversion, case folding, and normalization; compare [07: nesting](../part-2-option-result/07-nesting.md), [15: spell_check](../part-3-aggregations/15-spell-check.md), and [24: fun_strings_ext](../part-4-strings-parsing/24-fun-strings-ext.md).

### Whitespace and tokenization

- Replacing an ASCII-space loop with Unicode-aware trimming or splitting can change accepted separators and payload text: [20: trim_log_line](../part-4-strings-parsing/20-trim-log-line.md) and [36: env_file_parser](../part-6-capstone/36-env-file-parser.md).

### OsStr and non-UTF-8 paths

- Platform paths need not be UTF-8, so converting an `OsStr` to text can fail even when a native path operation is meaningful: [06: path](../part-2-option-result/06-path.md).

### Path extension

- `Path::extension` examines the last component's suffix without querying the filesystem; [06: path](../part-2-option-result/06-path.md) uses it as a lexical predicate, not a file-existence check.

### Lexical path matching

- Component-based prefix matching does not resolve symlinks or establish filesystem containment: [06: path](../part-2-option-result/06-path.md) and [14: excluded_path](../part-3-aggregations/14-excluded-path.md).

### Path parameter types

- `&str`, `&Path`, and `AsRef<Path>` accept different callers, independently of whether file contents must be UTF-8: [11: error_handling](../part-2-option-result/11-error-handling.md).

### Checked arithmetic

- Checked operations turn integer overflow into an explicit value, but merely rewriting a loop as `sum` does not select that policy: [08: optional_values](../part-2-option-result/08-optional-values.md) and [21: parse_srt_timestamp](../part-4-strings-parsing/21-parse-srt-timestamp.md).

### Parse into the destination type

- Parsing directly into `u16` enforces its numeric range but does not reject zero without a separate rule: [12: parse_port](../part-2-option-result/12-parse-port.md), contrasted with zero acceptance in [29: config_loader](../part-5-domain-modeling/29-config-loader.md).

### Nonzero integers

- `NonZeroU16` carries a nonzero invariant after successful construction, but does not prove that a port is available or appropriate for every network API: [12: parse_port](../part-2-option-result/12-parse-port.md).

## Domain types and boundaries

### Newtype

- A wrapper creates a distinct type and a place for domain operations, but does not validate its contents by existing: [14: excluded_path](../part-3-aggregations/14-excluded-path.md), [16: room_occupancy](../part-3-aggregations/16-room-occupancy.md), and [26: rooms](../part-5-domain-modeling/26-rooms.md).

### Type invariants

- State precisely what construction proves and which mutation paths preserve it; nonzero ports in [12: parse_port](../part-2-option-result/12-parse-port.md) differ from the limited prefix checks in [27: iban_prefix_check](../part-5-domain-modeling/27-iban-prefix-check.md).

### Typed IDs

- Separate ID types prevent category mix-ups, unlike aliases for `usize`, but do not establish bounds, owner identity, or liveness: [31: typed_ids](../part-5-domain-modeling/31-typed-ids.md) and [35: friends](../part-5-domain-modeling/35-friends.md).

### Mutually exclusive policies

- An enum can remove contradictory combinations of configuration fields without validating every numeric payload: [30: buffer_policy](../part-5-domain-modeling/30-buffer-policy.md).

### Typed feedback

- Closed enum categories separate machine-readable decisions from displayed text and expose missing match arms, but require a separate parser for external strings: [32: validation_feedback](../part-5-domain-modeling/32-validation-feedback.md).

### Redacted debugging

- Custom `Debug` and explicit secret access reduce accidental disclosure through formatting, not deliberate exposure or all memory copies: [33: secret_token](../part-5-domain-modeling/33-secret-token.md).

### Typestate

- Different types make operations available only after required transitions, while runtime data validity remains a separate concern: [34: config_typestate](../part-5-domain-modeling/34-config-typestate.md) and [29: config_loader](../part-5-domain-modeling/29-config-loader.md).

### Extension traits

- A local trait can add methods to `str`, changing discovery and call syntax without removing intermediate allocations or preserving a changed casing policy: [24: fun_strings_ext](../part-4-strings-parsing/24-fun-strings-ext.md).

### Identity and ownership

- Relationships need not own their targets, and equal names need not identify the same person; [35: friends](../part-5-domain-modeling/35-friends.md) contrasts shared handles with IDs resolved through one owner.

### Parser states

- Explicit states can distinguish quoted content, separators, comments, and end-of-input, but changing the grammar is not a mechanical refactor: [36: env_file_parser](../part-6-capstone/36-env-file-parser.md).

### Command and storage models

- Parsing commands, storing typed values, and formatting replies are separate boundaries; [37: mini_redis](../part-6-capstone/37-mini-redis.md) changes all three in its alternative despite retaining the handler signature.

## Tests and implementation limits

### Characterization tests

- Record a behavior before changing it, then distinguish a compatibility assertion from a test for a deliberate bug fix: [05: truncate_string](../part-1-warmups/05-truncate-string.md) and [17: text_stats](../part-3-aggregations/17-text-stats.md).

### Panicking preconditions

- Safe APIs can panic on violated preconditions without causing memory unsafety; string boundaries in [05: truncate_string](../part-1-warmups/05-truncate-string.md) and empty input in [18: mode](../part-3-aggregations/18-mode.md) need explicit contracts.

### Recursion and hidden costs

- Shorter recursive code can change stack depth, allocation, arithmetic order, or destruction behavior; compare [19: list_sum](../part-3-aggregations/19-list-sum.md) and [25: quicksort](../part-4-strings-parsing/25-quicksort.md).

## Editorial follow-up

- TODO(prose): Reconcile new chapter References with this selected index after the remaining drafts settle, adding definitions where a link currently points only to a related example.
- Preserve existing heading anchors when reorganizing entries, and check incoming links before removing or merging a concept.
- Use [Further reading](further-reading.md) for external sources and [Trade-off matrix](trade-off-matrix.md) for comparison groups rather than duplicating their full contents here.
