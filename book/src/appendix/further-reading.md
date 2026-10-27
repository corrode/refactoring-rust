# Further reading

> **Draft outline.** Bullet notes for the appendix; explanations still to be written.

- A selected, consolidated bibliography from current chapter citations, source comments, and attribution records, not a completed inventory of every API link.
- Distinguish language/API documentation, style guidance, teaching examples, historical implementations, and verified production changes.
- A citation supplies context or provenance, not evidence that a local variant is faster, production-ready, or behaviorally identical to its source.

## Language, ownership, and API design

- [The Rust Book: references and borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) explains reading data without transferring ownership.
  - Pair with [01: is_yes](../part-1-warmups/01-is-yes.md), [02: starts_with_uppercase](../part-1-warmups/02-starts-with-uppercase.md), and [26: rooms](../part-5-domain-modeling/26-rooms.md), where accepting a borrow and choosing an owned domain object solve different problems.
- [Rust Reference: or-patterns](https://doc.rust-lang.org/reference/patterns.html#or-patterns) and [slice patterns](https://doc.rust-lang.org/reference/patterns.html#slice-patterns) explain the syntax behind compact, explicit cases.
  - Use with [03: better_match](../part-1-warmups/03-better-match.md), [25: quicksort](../part-4-strings-parsing/25-quicksort.md), and command arity in [37: mini_redis](../part-6-capstone/37-mini-redis.md).
- [The Rust Book: enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) and [exhaustive matches](https://doc.rust-lang.org/book/ch06-02-match.html#matches-are-exhaustive) ground closed choices and compiler-checked dispatch.
  - Apply to [30: buffer_policy](../part-5-domain-modeling/30-buffer-policy.md) and [32: validation_feedback](../part-5-domain-modeling/32-validation-feedback.md), without assuming an enum validates every payload.
- [Rust by Example: newtypes](https://doc.rust-lang.org/rust-by-example/generics/new_types.html), [type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html), and [visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html) distinguish a new type from a new name and an interface from unrestricted storage.
  - Read alongside [31: typed_ids](../part-5-domain-modeling/31-typed-ids.md), [33: secret_token](../part-5-domain-modeling/33-secret-token.md), and [34: config_typestate](../part-5-domain-modeling/34-config-typestate.md).
- [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) and [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) describe standard fallible conversion interfaces.
  - Compare [12: parse_port](../part-2-option-result/12-parse-port.md), [21: parse_srt_timestamp](../part-4-strings-parsing/21-parse-srt-timestamp.md), and [29: config_loader](../part-5-domain-modeling/29-config-loader.md), noting whether a trait is implemented, only used as a bound, or merely proposed.

## Style guidance already used in the chapters

- Google Rust Style Guide: a grouped reference for API choices, not a rule that overrides an exercise's compatibility constraint.
  - [Flexible immutable references](https://google.github.io/styleguide/rustguide.html#accept-flexible-references-for-immutable-data) and [pattern matching rather than check-then-unwrap](https://google.github.io/styleguide/rustguide.html#prefer-pattern-matching-over-check-then-unwrap) accompany [02: starts_with_uppercase](../part-1-warmups/02-starts-with-uppercase.md).
  - [Iterators versus owned return collections](https://google.github.io/styleguide/rustguide.html#iterators-vs-owned-collections-in-return-types) and [iterators versus slice parameters](https://google.github.io/styleguide/rustguide.html#iterators-vs-slices-in-function-parameters) frame choices in [04: even_numbers](../part-1-warmups/04-even-numbers.md) and [14: excluded_path](../part-3-aggregations/14-excluded-path.md).
  - [Established standard-library traits](https://google.github.io/styleguide/rustguide.html#implement-known-standard-library-traits-instead-of-custom-methods) inform the parsing discussion in [12: parse_port](../part-2-option-result/12-parse-port.md).
  - [Enums instead of unclear booleans](https://google.github.io/styleguide/rustguide.html#prefer-enum-over-bool-in-apis) and [newtypes versus aliases](https://google.github.io/styleguide/rustguide.html#newtypes-versus-type-aliases) connect [30: buffer_policy](../part-5-domain-modeling/30-buffer-policy.md) and [31: typed_ids](../part-5-domain-modeling/31-typed-ids.md).

## Collection, text, and arithmetic contracts

- [`Vec::dedup`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup) and [`Itertools::dedup`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.dedup) cover adjacent-run removal in [13: dedup_runs](../part-3-aggregations/13-dedup-runs.md).
  - The latter is a library alternative mentioned in the chapter, not a dependency or local implementation already added to this repository.
- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry), [`HashMap::iter`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.iter), and [`Iterator::max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key) document counting, capacity-sensitive traversal, and tie selection.
  - Consolidate their repeated use in [17: text_stats](../part-3-aggregations/17-text-stats.md), [18: mode](../part-3-aggregations/18-mode.md), and [37: mini_redis](../part-6-capstone/37-mini-redis.md).
- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars), [`split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace), and [`to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase) specify text units and transformations.
  - Contrast Unicode conversion with [`to_ascii_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_ascii_lowercase) in [15: spell_check](../part-3-aggregations/15-spell-check.md) and [24: fun_strings_ext](../part-4-strings-parsing/24-fun-strings-ext.md), rather than calling both “case-insensitive” without qualification.
- [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) provides a delimiter split, not a complete grammar.
  - Compare the different validation rules in [21: parse_srt_timestamp](../part-4-strings-parsing/21-parse-srt-timestamp.md) and [36: env_file_parser](../part-6-capstone/36-env-file-parser.md).
- [Rust Reference: integer overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow), [`u64::checked_mul`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul), and [`checked_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_add) separate arithmetic policy from iterator syntax.
  - Use with [19: list_sum](../part-3-aggregations/19-list-sum.md), [21: parse_srt_timestamp](../part-4-strings-parsing/21-parse-srt-timestamp.md), and [22: iterators](../part-4-strings-parsing/22-iterators.md).
- [Recursive types with `Box`](https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes) and [destructors](https://doc.rust-lang.org/reference/destructors.html) distinguish finite type layout from traversal and destruction stack behavior in [19: list_sum](../part-3-aggregations/19-list-sum.md).
- [`slice::sort_unstable`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) and [`sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort) document production sorting contracts and stability for the comparison in [25: quicksort](../part-4-strings-parsing/25-quicksort.md).

## Production history and design sources

- **Verified production refactor:** [Nushell PR #13832](https://github.com/nushell/nushell/pull/13832), replacing raw integer IDs with new types, is the source behind [31: typed_ids](../part-5-domain-modeling/31-typed-ids.md).
  - The [pinned generic `Id<T>` implementation](https://github.com/nushell/nushell/blob/f0c83a445977c894c4c46ca9e1c97e3372eddd69/crates/nu-protocol/src/id.rs#L7-L33) uses marker types and `PhantomData`, unlike the workshop's separate tuple wrappers and instructor-created `Engine`.
- **Historical production API adapted for teaching:** [hyper's buffer setters](https://github.com/hyperium/hyper/blob/73bff4e98c372ce04b006370c0b0d2af29ea8718/src/client/conn.rs#L616-L642) supply the starting idea for [30: buffer_policy](../part-5-domain-modeling/30-buffer-policy.md).
  - The standalone starter changes names, visibility, and surrounding machinery; the enum solution is a workshop redesign, not a claimed upstream PR.
- **Existing production design, not an upstream vulnerability fix:** [secrecy's pinned `SecretBox` implementation](https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/src/lib.rs#L141-L162) shows redacted debugging and explicit access behind [33: secret_token](../part-5-domain-modeling/33-secret-token.md).
  - [PR #1140](https://github.com/iqlusioninc/crates/pull/1140) restructures `SecretBox`; it does not document a real version of the workshop starter's derived-`Debug` leak.
  - The [`secrecy` documentation](https://docs.rs/secrecy/) describes the production-oriented library; the local toy wrapper does not reproduce its memory-handling facilities.
- **Historical typed API, not a verified before-and-after refactor:** [rustls's pinned client builder](https://github.com/rustls/rustls/blob/792045ba685965edc6f872cb8fcdd3136c8f0ef8/rustls/src/client/builder.rs#L16-L31) motivates [34: config_typestate](../part-5-domain-modeling/34-config-typestate.md).
  - Both local versions are instructor-created fixtures, and their labeled root store is not certificate validation.
- **External API example:** [the pinned `vimspell` iterator](https://github.com/exrok/vimspell/blob/361b63f71228af3d00291d3c257cd487cf5b95b8/src/lib.rs#L824) motivates lazy byte ranges in [15: spell_check](../part-3-aggregations/15-spell-check.md).
  - This citation illustrates a return contract, not a local implementation or evidence that its spelling rules equal the exercise's lowercase-set membership.
- The repository's `THIRD_PARTY_NOTICES.md` records pinned MIT notices for exercises 30, 31, 33, and 34 only; see [Acknowledgements](acknowledgements.md) for the credit distinctions.

## Teaching sources

- [Advent of Code 2015, Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1) is the problem source cited in `examples/10_floor.rs` and its slide page.
  - [10: floor](../part-2-option-result/10-floor.md) uses the floor-instruction exercise to compare a loop, fallible summation, and `try_fold`, not to report a production refactor.
- [Object Soup is Made of Indexes](https://jacko.io/object_soup.html), Jack O'Connor, develops the friendship and index-based ownership example behind [35: friends](../part-5-domain-modeling/35-friends.md).
  - Treat it as a teaching article, and compare it with [reference cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) when distinguishing links from ownership.
- [32: validation_feedback](../part-5-domain-modeling/32-validation-feedback.md) records an instructor-written fixture inspired by a pattern in beginner-course submissions, not a specific copied submission or an attributable learner implementation.

## Protocol references and limits

- [Redis `SET`](https://redis.io/docs/latest/commands/set/) and [RESP simple errors](https://redis.io/docs/latest/develop/reference/protocol-spec/#simple-errors) provide comparison points for replacement and error formatting in [37: mini_redis](../part-6-capstone/37-mini-redis.md).
  - They are specifications to check against, not evidence that either local implementation is wire-compatible with Redis.
- [`http::StatusCode`](https://docs.rs/http/latest/http/status/struct.StatusCode.html) is the library type discussed as a possible redesign in [28: http_response_router](../part-5-domain-modeling/28-http-response-router.md).
  - Its range and classifications do not supply the application's retry policy, and the chapter's main solution does not introduce this dependency.

## Editorial follow-up

- TODO(prose): Recheck the selected citations after chapter edits, deduplicating by resource while preserving distinct sections that support different claims.
- Retain commit pins for historical design and attribution claims, and distinguish those from version-moving API documentation.
- TODO(prose): Review quotation and adapted-code attribution separately from bibliography completeness; a reading link is not a license grant.
- Use the [Concept index](concept-index.md) to find an idiom and the [Trade-off matrix](trade-off-matrix.md) to locate the local comparisons these sources inform.
