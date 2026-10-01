# Roadmap

The book contains twenty-seven exercises in six parts. Read them in order, jump
to the topic you need, or use this page to choose where to start. Each chapter
stands on its own, while each part develops one group of related ideas.

## Part I. Warm-ups (chapters 01–04)

These four short refactors are mostly changes Clippy could find. The code isn't
the hard part. The point is to learn the book's rhythm: *Review / Trade-offs /
Quiz / Follow-up / Concepts recap*.

You'll work with `&str` and `String`, `is_empty`, `chars().next()` instead of
`chars().nth(0)`, `matches!`, lazy `impl Iterator` returns instead of `Vec`, char
boundaries, and the rule that `if x { true } else { false }` is always wrong.
If you've written Rust for a few months, part I should take less than an hour.

## Part II. `Option`, `Result`, `?` (chapters 05–10)

This part moves from knowing the syntax to choosing idiomatic control flow. It
covers deeply nested `if let`, combinators such as `is_some_and`, `map`,
`and_then`, and `filter_map`, and the cases where a loop is clearer. You'll see
how the standard library handles iterators of `Result`, how `?` turns a page of
`match` into a line, and why fail-fast parsing differs from validation that must
run every rule and collect every error.

By the end, you should be able to read a complicated `Option` or `Result`
pipeline and write one a teammate can follow. Give chapter 09 extra time: its short
final implementation hides several decisions about interfaces and error contracts.

## Part III. Aggregations & counting (chapters 11–16)

Here the work becomes everyday iterator code: collapsing runs with `dedup`,
checking membership with a `HashSet`, rolling structs into totals, defining
occurrence-weighted text statistics, and computing modes with the `entry` API.
These patterns make later Rust code shorter and easier to maintain.

This part also introduces the **newtype** pattern where it earns its keep, and
shows when a small `struct` with methods is a better home than a free function
that takes `&[T]`.

Chapter 11 (`dedup_runs`) is an optional bonus exercise. Skip it when time is
short.

## Part IV. Strings, parsing & iterators (chapters 17–22)

Part IV asks how far the standard library can take you before you write another
loop. You'll clean up log lines with `split_whitespace` and `strip_prefix`, parse
a timestamp without `unwrap`, implement `Iterator` for your own type, drive a
small transformer from an `enum`, and remove a sentinel from a command pipeline
by choosing its identity value. You'll also turn free functions into a fluent
extension trait and reduce a verbose quicksort to its textbook definition.

Chapter 19 is the payoff: once your type implements `Iterator`, adaptors such as
`take`, `map`, `zip`, and `collect` work with it for free. Chapter 22
(`quicksort`) is an optional bonus exercise. Skip it when time is short.

## Part V. Domain modeling (chapters 23–25)

Part V moves from syntax to data and API design. *Make illegal states
unrepresentable* runs through three exercises: validating structure with the
type system in chapter 23, building enum-driven routers that separate decisions
from side effects in chapter 24, and combining visibility with layered
configuration through `TryFrom` and `FromStr` in chapter 25.

If your team's Rust reviews spend more time on types and design than syntax,
start here.

## Part VI. Capstone (chapters 26–27)

The final two refactors combine most of the earlier work. Chapter 26 is a
parser. Chapter 27 is a small `redis`-style server. Both stop where production
work begins: tests still worth writing, edge cases a colleague should flag, and
design decisions that compilation alone can't settle.

The capstones introduce no new concepts. They show how the rest of the book's
techniques interact in larger code.

## Suggested pace

- **Solo:** one chapter every other day, about two months.
- **Pair:** one chapter per weekly session, about six months.
- **Team book club:** one chapter per week, about six months. Skip part I if the
  team is past it and save a month.
- **Workshop:** two full days, using the slides for delivery and the book as the
  reference afterward.

## And then

After the final chapter, the back matter gives you three ways to revisit the
book:

- The [Cheatsheet](appendix/concept-index.md) lists every named concept and links
  to each chapter that uses it.
- The [Trade-off matrix](appendix/trade-off-matrix.md) puts all twenty-seven
  canonical solutions and their scores side by side.
- [Further reading](appendix/further-reading.md) collects and deduplicates every
  external resource cited in the book.

Then return to the chapter that best matches the next pull request in front of
you.
