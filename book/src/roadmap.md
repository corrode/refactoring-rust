# Roadmap

The body of the book is twenty-seven exercises in six parts. Each part
has a theme; each chapter inside a part stands on its own. You can
read straight through, jump to a part that interests you, or treat
the sidebar as a buffet.

This page is the shortest possible answer to *what does each part
teach, and what should I expect by the end of it?*

## Part I. Warm-ups (chapters 01–04)

Four short refactors that Clippy would mostly catch. The point isn't
the refactors. It's the format. By the end of part I you'll have seen
the *Review / Trade-offs / Quiz / Follow-up / Concepts recap*
cycle four times and know how the rest of the book reads.

You'll meet: `&str` vs. `String`, `is_empty`, `chars().next()` over
`chars().nth(0)`, `matches!`, returning `impl Iterator` instead of a
`Vec`, char boundaries, and the rule that an `if x { true } else
{ false }` is always wrong.

If you've been writing Rust for more than a few months, you can read
part I in under an hour.

## Part II. `Option`, `Result`, `?` (chapters 05–10)

The transition from "I know the syntax" to "I think in idioms." We
look at the cost of deeply nested `if let`, when to reach for
combinators (`is_some_and`, `map`, `and_then`, `filter_map`) and when
a loop is honestly clearer, what the standard library does with
iterators of `Result`, and how `?` collapses a page of `match` into a
line.

By the end you should be able to read a complicated `Option`/`Result`
pipeline at a glance, and (more importantly) write one a teammate
will be able to read. Chapter 09 is the reference chapter for the
book's whole approach to trade-offs. Read it slowly.

## Part III. Aggregations & counting (chapters 11–16)

Real-world iterator work: collapsing runs with `dedup`, membership
checks against a `HashSet`, rolling up a `Vec` of structs into totals,
finding extremes, and computing modes with the `entry` API. Once
you've seen the canonical solutions, much of the everyday Rust you
write afterwards gets shorter and easier to maintain.

This part also introduces the **newtype** pattern in a setting where
it earns its keep, and shows when a small `struct` with methods beats
a free function that takes a `&[T]`.

Chapter 11 (`dedup_runs`) is an optional bonus exercise. Skip it if
you're short on time.

## Part IV. Strings, parsing & iterators (chapters 17–22)

Reach for the standard library before hand-rolling a loop. We clean
up log lines with `split_whitespace` and `strip_prefix`, parse a
timestamp without a single `unwrap`, implement `Iterator` for a type
of our own, drive a small transformer off an `enum`, turn a pile of
free functions into a fluent extension trait, and rewrite a verbose
quicksort until it reads like its textbook definition.

Chapter 19 in particular is the part's payoff: once you can implement
`Iterator`, every adaptor in the standard library (`take`, `map`,
`zip`, `collect`) becomes yours for free.

Chapter 22 (`quicksort`) is an optional bonus exercise. Skip it if
you're short on time.

## Part V. Domain modeling (chapters 23–25)

Where Rust pays its biggest dividends. *Make illegal states
unrepresentable* gets a thorough working-out across three exercises:
validating structure with the type system (ch. 23), enum-driven
routers that separate decisions from side effects (ch. 24), and
layered configuration with `FromStr` and `Default` (ch. 25).

If your team's Rust review comments are mostly about types and design
rather than syntax, this is where you'll find your vocabulary.

## Part VI. Capstone (chapters 26–27)

Two longer refactors that pull on most of what came before. Chapter
26 is a parser; chapter 27 is a small `redis`-style server. Both end
where production code starts: with the things you'd want to test, the
edge cases you'd want a colleague to flag, and the design decisions
that aren't done just because the code compiles.

The capstone chapters don't introduce new concepts. They show what it
looks like to do the rest of the book's work *together*, at scale.

## Suggested pace

Solo, one chapter per sitting: about two months at a chapter every
other day.

Pair, one chapter per session: about three months at a session per
week.

Team book club, one chapter per week: about six months. Skip part I
if your team is past it; that buys you a month back.

Workshop, end to end: two full days, with the slides for delivery and
the book as the post-workshop reference.

## And then

When you finish, the back matter is where the book stops being a
sequence and starts being a tool:

- The [Cheatsheet](appendix/concept-index.md): every named concept,
  linked from every chapter that touches it. Use it as a lookup.
- The [Trade-off matrix](appendix/trade-off-matrix.md): all
  twenty-seven canonical solutions scored side-by-side. Use it as a
  map.
- [Further reading](appendix/further-reading.md): every external
  resource cited in the book, deduplicated.

Then come back to whichever chapter your team's next PR reminds you
of.
