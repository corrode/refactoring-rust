# Roadmap

The book contains thirty-eight exercises in six parts. Read them in order, jump
to the topic you need, or use this page to choose where to start. Each chapter
stands on its own, while each part develops one group of related ideas.

## Part I. Warm-ups (chapters 01–06)

These six short refactors are mostly changes Clippy could find. Use them to
learn the chapter format: *Discussion / Trade-offs / Follow-up / Important Concepts*.

You'll work with `&str` and `String`, `is_empty`, `chars().next()` instead of
`chars().nth(0)`, `matches!`, nested and struct patterns, range patterns, match ergonomics, lazy `impl Iterator` returns instead of `Vec`, char
boundaries, and replacing `if x { true } else { false }` with `x`.
If you've written Rust for a few months, part I should take less than an hour.

## Part II. `Option`, `Result`, `?` (chapters 07–13)

This part moves from knowing the syntax to choosing idiomatic control flow. It
covers deeply nested `if let`, combinators such as `is_some_and`, `map`,
`and_then`, and `filter_map`, and the cases where a loop is clearer. You'll see
how the standard library handles iterators of `Result`, how `?` replaces repeated
`match` expressions for error propagation, and why fail-fast parsing differs
from validation that must run every rule and collect every error.

By the end, you should be able to read a complicated `Option` or `Result`
pipeline and write one a teammate can follow. Give chapter 12 extra time: its short
final implementation involves several decisions about interfaces and error contracts.

## Part III. Aggregations & counting (chapters 14–20)

This part covers common iterator tasks: collapsing runs with `dedup`,
checking membership with a `HashSet`, summing fields across structs, defining
occurrence-weighted text statistics, and computing modes with the `entry` API.
You'll compare these methods with explicit loops across seven exercises.
Chapter 20 ([`list_sum`](../part-3-aggregations/20-list-sum.md)) consumes a recursive list to aggregate its values.
Iteration and recursion can evaluate additions in different orders, changing which intermediate sums overflow when overflow checks are enabled.
Rust does not guarantee tail-call optimization; don't assume recursion uses constant stack space across compilers and build settings.

Chapter 14 (`dedup_runs`) is an optional bonus exercise. Skip it when time is
short.

## Part IV. Strings, parsing & iterators (chapters 21–26)

Part IV covers standard library methods and custom iterators. You'll clean up
log lines with `split_whitespace` and `strip_prefix`, parse
a timestamp without `unwrap`, implement `Iterator` for your own type, control a
small transformer with an `enum`, and remove a sentinel from a command pipeline
by choosing its identity value. You'll also turn free functions into a fluent
extension trait and reduce a verbose quicksort to its textbook definition.

In chapter 23, implementing `Iterator` makes methods such as `take`, `map`,
`zip`, and `collect` available to your type. Chapter 26
(`quicksort`) is an optional bonus exercise. Skip it when time is short.

## Part V. Domain modeling (chapters 27–36)

Part V moves from syntax to data and API design across ten exercises.
Chapter 27 groups room aggregates on an owned **newtype**, revisiting the iterator functions from chapter 17.
Chapter 28 validates IBAN prefixes, chapter 29 uses enums to separate routing decisions from side effects, and chapter 30 combines visibility with layered configuration through `TryFrom` and `FromStr`.
Chapter 31 models buffer sizing policies, and chapter 32 separates ID types.
Chapter 33 separates validation categories from display messages in a fresh instructor-created fixture inspired by beginner submissions, not copied from a learner's code.
Chapter 34 controls access to secrets and their diagnostic output, and chapter 35 uses typestate to constrain configuration steps.
Chapter 36 (`friends`) closes the part with an advanced exercise.

If your team's Rust reviews spend more time on types and design than syntax,
start here.

## Part VI. Larger refactors (chapters 37–38)

The final two refactors combine most of the earlier work.
Chapter 37 is a parser.
Chapter 38 is a small `redis`-style server.
Both include production concerns to investigate: additional tests, edge cases, and design decisions that compilation alone can't settle.

These exercises introduce no new concepts.
They show how the rest of the book's techniques interact in larger code.

## Suggested pace

- **Solo:** one chapter every other day, about two months.
- **Pair:** one chapter per weekly session, about eight months.
- **Team book club:** one chapter per week, about eight months. Skip part I if the
  team is past it and save a month.
- **Workshop:** two full days, using the slides for delivery and the book as the
  reference afterward.

## Reference material

After the final chapter, the back matter gives you three ways to revisit the
book:

- The [Cheatsheet](concept-index.md) lists every named concept and links
  to each chapter that uses it.
- The [Trade-off matrix](trade-off-matrix.md) puts all thirty-eight
  canonical solutions and their scores side by side.
- [Further reading](further-reading.md) collects and deduplicates every
  external resource cited in the book.

Use these references to find chapters relevant to your own code reviews.
