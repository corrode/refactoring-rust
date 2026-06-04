# How to use this book

The book is designed to be read in any of four modes. Pick the one
that matches your situation; the chapters work the same way in all
four.

## Solo

You at a keyboard, by yourself, on a couch or a desk or a train. The
default mode. For each chapter:

1. Read *The problem*.
2. Read the starter code carefully, slowly, the way you'd read it on
   GitHub before commenting on a PR.
3. **Stop reading the chapter.** Make a list of what you'd say in a
   review. Try to refactor the code yourself in your editor. Spend
   five to twenty minutes, depending on the chapter's size.
4. Then read *Review* and see which of your observations match
   ours, and which we surfaced that you didn't.
5. Read *A possible solution* and compare it to yours.
6. Read *Trade-offs* and decide whether you agree with our scoring.
7. Do the quiz.
8. Sit with the *Follow-up questions* until you have at least one you
   want to discuss with someone.

Skipping step 3 turns the book into entertainment. The whole point of
the format is that *your* answer comes before ours, because
internalising the rubric requires producing answers, not consuming
them. A little trick is to delete your first attempt at the refactor before trying again after reading the chapter. You won't be able to copy-paste, and you'll build the muscle memory to improve similar code in the future.

## Pair

Two engineers, one keyboard. One drives (writes the refactor), one
reviews (asks the *Review* questions out loud, gently). Swap roles
every chapter, or every other chapter. The chapter's *Review*
section gives the reviewer their script, but the best discussions
happen when the reviewer goes off-script.

Pairing on this book is genuinely better than reading it alone. You
get the disagreements the chapter can only describe.

## Book club / team

A small group, three to eight people, reads one chapter per week.
Anyone who wants to brings their refactored version. The meeting (an
hour, max) walks through:

1. What each person noticed in *Review* before reading ours.
2. Whose solution is closest to the canonical, whose is furthest, why.
3. Where the group disagrees with the *Trade-offs* scoring.
4. The *Follow-up questions*. These were chosen for exactly this
   meeting.

Don't try to read the whole book in a sprint. The point is the
recurring practice; the gap between meetings is where the ideas
actually land. Twenty-seven weeks is fine. Skip the warm-up chapters
if your team is past them.

## Workshop facilitator

If you're running an internal workshop, the slides in `slides/`
(<https://github.com/corrode/refactoring>) are the live-delivery
artifact and the book is your reference companion. Use the chapter's
*Review* questions as your prompts to the room; use *Trade-offs*
to keep the discussion honest when it veers into religion; use
*Follow-up questions* as the breakout-room material.

If your team would rather have the workshop delivered by someone who
has run it before, the booking form is at <https://corrode.dev>.

## Running the code locally

The starters and canonical solutions live in `examples/` and
`solutions/` in the repo. Both are wired as `cargo` examples, so:

```sh
git clone https://github.com/corrode/refactoring
cd refactoring

# Run a starter
cargo run --example 01_starts_with_uppercase

# Run its tests (these are the tests every refactor has to keep passing)
cargo test --example 01_starts_with_uppercase

# Run the canonical solution's tests
cargo test --example 01_starts_with_uppercase_solution

# Run every test in the repo
make test
```

Every code block in the book that's runnable also has a small "play"
button in the top-right corner that opens the snippet in the [Rust
Playground](https://play.rust-lang.org). Useful for quick "what if"
experiments without leaving the page.

## Building the book locally

If you want to read the book offline, or to preview an edit, you'll
need [mdBook](https://rust-lang.github.io/mdBook/):

```sh
cargo install mdbook
make book           # live-serve at http://localhost:3000, opens browser
make book-build     # static HTML in book/book/
make book-open      # build once, open the static HTML
```

`make help` lists everything.

## Conventions

You'll see a handful of recurring callouts in every chapter. Each is a
labelled block:

> **EXERCISE**
>
> Wraps the starter listing. *"Here's what you're refactoring."*

> **REVIEW**
>
> The questions a thoughtful reviewer would ask. The book's signature
> section.

> **TRADE-OFF**
>
> Where we score the canonical solution against the six axes. Always
> the same axes, always the same order, always a sentence of
> justification.

> **EDGE CASE**
>
> Production concerns the canonical solution skipped on purpose. Only
> present when there's something real to flag.

> **FOLLOW-UP**
>
> Open-ended questions to take back to your team's code.

> **CONCEPT**
>
> Named idea this chapter introduced, with a pointer to the
> [Cheatsheet](appendix/concept-index.md) entry where it's defined
> properly and linked from every other chapter that touches it.

A few chapters open with a **BONUS** callout. Those are optional: skip
them if you're short on time, since nothing later in the book depends
on them.

## Filing issues

If you spot a typo, a broken link, a refactor we missed, or a
trade-off we scored wrongly: [the issue
templates](https://github.com/corrode/refactoring/issues/new/choose)
channel feedback into the buckets we can actually act on. We don't
accept prose PRs (the book has a single author on purpose, see
[Contributing](appendix/contributing.md)), but every substantive
issue is credited in the [Acknowledgements](appendix/acknowledgements.md)
when it's adopted.

## Where to start

If you've read enough Rust to follow the starter code in
[chapter 01](part-1-warmups/01-starts-with-uppercase.md), start at
[the Preface](preface.md) and read forward. If you haven't, work
through *The Book* or *100 Exercises to Learn Rust* first; the format
here assumes you can read Rust without help.

If you've been writing Rust for a while and you're impatient, skim
[How to run a code review](how-to-run-a-code-review.md) and
[The trade-off axes](trade-off-axes.md). Those two chapters define
everything the rest of the book trades on. Then jump straight into
whichever part interests you most. The [Roadmap](roadmap.md) is the
shortest way to decide where to land.
