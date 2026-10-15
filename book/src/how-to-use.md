# How to use this book

The useful part of this book begins when you stop reading it.

Each chapter gives you a small piece of Rust to review and refactor. If you read
straight through to the solution, you'll understand the explanation but miss
the practice. Pick one of the four modes below, then make sure your answer comes
before ours.

## Solo

This is the default: you, a keyboard, and five to twenty minutes per chapter.
A couch, desk, or train all work.

1. Read *The problem*.
2. Read the starter code carefully, as if it were a pull request on GitHub.
3. **Stop reading the chapter.** Write down what you'd say in a review, then
   refactor the code in your editor. Spend five to twenty minutes, depending on
   the size of the chapter.
4. Read *Review*. Compare its questions with yours. Which concerns did you both
   notice? Which ones did either of you miss?
5. Open *A possible solution* and compare it with your version.
6. Read *Trade-offs* and decide whether you agree with the scores.
7. Do the quiz.
8. Stay with the *Follow-up questions* until you find at least one worth
   discussing with someone else.

Step 3 is the important one. Skip it and the book becomes entertainment.
Internalising the rubric takes more than recognising a good answer after you've
seen it.

One useful constraint is to delete your first refactor before trying again
after the chapter. You can't copy and paste your way through the second attempt,
so you practise making the same kind of change from memory.

## Pair

Put two engineers at one keyboard. One drives and writes the refactor. The other
reviews by asking the chapter's *Review* questions out loud, with room for a real
answer. Swap roles every chapter, or every other chapter.

The questions give the reviewer a place to start, not a script to obey. Go
off-script when the code gives you a better question. Pairing adds the part a
written chapter can only describe: a real disagreement with someone who can
answer back.

## Book club / team

For a group of three to eight people, read one chapter per week. Anyone who
wants to can bring a refactored version. Keep the meeting to an hour and spend
it on four things:

1. What each person noticed before reading *Review*.
2. Which solution is closest to the canonical one, which is furthest away, and
   why.
3. Where the group disagrees with the *Trade-offs* scores.
4. The *Follow-up questions*. They were written for this conversation.

Don't race through the book. The recurring practice matters more than finishing
quickly, and the gap between meetings gives the ideas time to show up in real
code. Twenty-seven weeks is fine. If your team is past the warm-ups, skip them.

## Workshop facilitator

If you're running an internal workshop, use the slides in
[`slides/`](https://github.com/corrode/refactoring-rust/tree/main/slides) for the live
session and keep the book nearby as the reference. The chapter
sections already divide the work:

- Use *Review* to prompt the room.
- Use *Trade-offs* when the discussion starts treating a preference as a rule.
- Use *Follow-up questions* for breakout rooms.

If your team would rather have the workshop delivered by someone who has run it
before, the booking form is at <https://corrode.dev>.

## Running the code locally

The starters live in `examples/`; the canonical solutions live in `solutions/`.
Both are wired up as Cargo examples:

```sh
git clone https://github.com/corrode/refactoring-rust
cd refactoring-rust

# Test a starter (these are the tests every refactor has to keep passing)
cargo test --example 01_starts_with_uppercase

# Run the canonical solution's tests
cargo test --example 01_starts_with_uppercase_solution

# Run every test in the repo
make test
```

Runnable code blocks also have a small play button in the top-right corner. It
opens the snippet in the [Rust Playground](https://play.rust-lang.org), which is
handy for a quick "what if" experiment without leaving the page.

## Building the book locally

To read the book offline or preview an edit, install
[mdBook](https://rust-lang.github.io/mdBook/):

```sh
cargo install mdbook
make book           # live-serve at http://localhost:3000, opens browser
make book-build     # static HTML in book/book/
make book-open      # build once, open the static HTML
```

`make help` lists the other commands.

## Conventions

The same callouts recur throughout the book:

> **EXERCISE**
>
> The starter code. This is what you're refactoring.

> **REVIEW**
>
> The questions a thoughtful reviewer would ask. This is the book's signature
> section.

> **TRADE-OFF**
>
> The canonical solution scored against the same six axes, in the same order,
> with a sentence explaining each score.

> **EDGE CASE**
>
> Production concerns the canonical solution skipped on purpose. This appears
> only when there's something worth flagging.

> **FOLLOW-UP**
>
> Open-ended questions to take back to your team's code.

> **CONCEPT**
>
> A named idea introduced by the chapter, with a link to its
> [Cheatsheet](appendix/concept-index.md) entry and every other chapter that
> uses it.

Some chapters open with a **BONUS** callout. They're optional. Skip them when
you're short on time because no later chapter depends on them.

## Filing issues

Found a typo, broken link, missed refactor, or trade-off you'd score differently?
Use the [issue templates](https://github.com/corrode/refactoring-rust/issues/new/choose).
They sort feedback into categories we can act on.

We don't accept prose pull requests. The book has a single author on purpose,
as explained in [Contributing](appendix/contributing.md). When a substantive
issue is adopted, its author is credited in the
[Acknowledgements](appendix/acknowledgements.md).

## Where to start

If you can follow the starter code in
[chapter 01](part-1-warmups/01-starts-with-uppercase.md), begin with the
[Preface](preface.md) and read forward. If not, work through *The Book* or
*100 Exercises to Learn Rust* first. This book assumes you can read Rust without
help.

If you've written Rust for a while and want to get to the harder exercises,
skim [How to run a code review](how-to-run-a-code-review.md) and
[The trade-off axes](trade-off-axes.md) first. They define the vocabulary used
throughout the book. Then use the [Roadmap](roadmap.md) to choose a part and
jump in.
