# Roadmap

The book contains thirty-four exercises in six parts. Read them in order, jump
to the topic you need, or use this page to choose where to start. Each chapter
stands on its own, while each part develops one group of related ideas.

## Part I. Warm-ups (chapters 01–05)

These five short refactors are mostly changes Clippy could find. Use them to
learn the chapter format: *Discussion / Trade-offs / Follow-up / Important Concepts*.

You'll work with `&str` and `String`, `is_empty`, `chars().next()` instead of
`chars().nth(0)`, `matches!`, lazy `impl Iterator` returns instead of `Vec`, char
boundaries, and replacing `if x { true } else { false }` with `x`.
If you've written Rust for a few months, part I should take less than an hour.

## Part II. `Option`, `Result`, `?` (chapters 06–12)

This part moves from knowing the syntax to choosing idiomatic control flow. It
covers deeply nested `if let`, combinators such as `is_some_and`, `map`,
`and_then`, and `filter_map`, and the cases where a loop is clearer. You'll see
how the standard library handles iterators of `Result`, how `?` replaces repeated
`match` expressions for error propagation, and why fail-fast parsing differs
from validation that must run every rule and collect every error.

By the end, you should be able to read a complicated `Option` or `Result`
pipeline and write one a teammate can follow. Give chapter 11 extra time: its short
final implementation involves several decisions about interfaces and error contracts.

## Part III. Aggregations & counting (chapters 13–18)

This part covers common iterator tasks: collapsing runs with `dedup`,
checking membership with a `HashSet`, summing fields across structs, defining
occurrence-weighted text statistics, and computing modes with the `entry` API.
You'll compare these methods with explicit loops.


Chapter 13 (`dedup_runs`) is an optional bonus exercise. Skip it when time is
short.

## Part IV. Strings, parsing & iterators (chapters 19–24)

Part IV covers standard library methods and custom iterators. You'll clean up
log lines with `split_whitespace` and `strip_prefix`, parse
a timestamp without `unwrap`, implement `Iterator` for your own type, control a
small transformer with an `enum`, and remove a sentinel from a command pipeline
by choosing its identity value. You'll also turn free functions into a fluent
extension trait and reduce a verbose quicksort to its textbook definition.

In chapter 21, implementing `Iterator` makes methods such as `take`, `map`,
`zip`, and `collect` available to your type. Chapter 24
(`quicksort`) is an optional bonus exercise. Skip it when time is short.

## Part V. Domain modeling (chapters 25–32)

Part V moves from syntax to data and API design across eight exercises.
Chapter 25 groups room aggregates on an owned **newtype**, revisiting the iterator functions from chapter 16.
Chapter 26 validates IBAN prefixes, chapter 27 uses enums to separate routing decisions from side effects, and chapter 28 combines visibility with layered configuration through `TryFrom` and `FromStr`.
Chapter 29 models buffer sizing policies, chapter 30 separates ID types, chapter 31 controls access to secrets and their diagnostic output, and chapter 32 uses typestate to constrain configuration steps.

If your team's Rust reviews spend more time on types and design than syntax,
start here.

## Part VI. Capstone (chapters 33–34)

The final two refactors combine most of the earlier work. Chapter 33 is a
parser. Chapter 34 is a small `redis`-style server. Both include production
concerns to investigate: additional tests, edge cases, and design decisions
that compilation alone can't settle.

The capstones introduce no new concepts. They show how the rest of the book's
techniques interact in larger code.

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
- The [Trade-off matrix](trade-off-matrix.md) puts all thirty-four
  canonical solutions and their scores side by side.
- [Further reading](further-reading.md) collects and deduplicates every
  external resource cited in the book.

Use these references to find chapters relevant to your own code reviews.
