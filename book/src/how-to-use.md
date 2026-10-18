# How to use this book

Each chapter gives you a small piece of Rust to review and refactor. Try the
exercise before reading the solution so you can compare your decisions with
those in the chapter.
Start by preserving the function signatures so callers don't need to change.
The first solution follows that rule; alternatives may change the API and explain what callers gain or lose.
[Domain modeling](part-5-domain-modeling/README.md) is the exception: there, you can redesign the API from the start and update callers and tests to match.
Choose one of the four modes below.

## Solo

Allow five to twenty minutes for your first attempt at each exercise, plus
time to read the explanation and answer the questions.

1. Read the introduction for the task and any constraints.
2. Read the code to refactor carefully, as if it were a pull request on GitHub.
3. **Stop reading the chapter.** Write down what you'd say in a review, then
   refactor the code in your editor. Spend five to twenty minutes, depending on
   the size of the chapter.
4. Read the *Discussion* questions and compare them with yours.
   Expand each question when you want to read its answer.
5. Open *Show Solutions* and compare the implementations with your version.
6. Read *Trade-offs* inside the same disclosure and decide whether you agree with the scores.
7. Choose a *takeaway* to investigate or discuss with someone else.

For more practice, return to the starter after reading the chapter and refactor
it again without referring to the solution.

## Pair

Work together at one keyboard. One person writes the refactor; the other asks
questions and reviews the changes. Use the chapter's *Discussion* questions as a
starting point and add your own. Discuss disagreements and explain each choice.
Swap roles every chapter, or every other chapter.

## Book club / team

For a group of three to eight people, read one chapter per week. Anyone who
wants to can bring a refactored version. Keep the meeting to an hour and spend
it on four things:

1. What each person noticed before reading *Discussion*.
2. Which solution is closest to the canonical one, which is furthest away, and
   why.
3. Where the group disagrees with the *Trade-offs* scores.
4. The *Takeaways For Our Own Codebase*. They were written for this conversation.

Between meetings, look for related examples in your team's code. Take more
than one session for a chapter if needed, and skip the warm-ups if the team
already knows the material.

## Workshop facilitator

If you're running an internal workshop, use the slides in
[`slides/`](https://github.com/corrode/refactoring-rust/tree/main/slides) for the live
session and keep the book nearby as the reference. The chapter
sections already divide the work:

- Use *Discussion* to prompt the room.
- Use *Trade-offs* inside *Show Solutions* when the discussion starts treating a preference as a rule.
- Use *Takeaways For Our Own Codebase* for breakout rooms.

To book a facilitated workshop for your team, visit <https://corrode.dev>.

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

# Try an alternative that changes the API
cargo test --example 01_starts_with_uppercase_alt

# Run every test in the repo
make test
```

Runnable code blocks also have a small play button in the top-right corner. It
opens the snippet in the [Rust Playground](https://play.rust-lang.org), where you
can try changes to the code.

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

In *Discussion*, click a question to expand its answer.
You can open several answers at once, or leave them closed while you work.

*Show Solutions* opens the implementations, explanations, and trade-off comparisons together.
They stay hidden until you open the disclosure.

The following callouts appear elsewhere in the book:

> **EDGE CASE**
>
> Production concerns the canonical solution does not address.

*Takeaways For Our Own Codebase* contains questions and actions in an unlabelled blockquote.
*References* lists concepts with links to their [Cheatsheet](appendix/concept-index.md) entries.

Some chapters open with a **BONUS** callout. They're optional. Skip them when
you're short on time because no later chapter depends on them.

## Filing issues

Found a typo, broken link, missed refactor, or trade-off you'd score differently?
Use the [issue templates](https://github.com/corrode/refactoring-rust/issues/new/choose).
Choose the template that matches your feedback.

The book has a single author and does not accept prose rewrites as pull
requests. The contribution policy is explained in
[Contributing](appendix/contributing.md). When a substantive issue is adopted,
its author is credited in the
[Acknowledgements](appendix/acknowledgements.md).

## Where to start

If you can follow the code to refactor in
[chapter 01](part-1-warmups/01-starts-with-uppercase.md), begin with the
[Intro](intro.md) and read forward. If not, work through *The Book* or
*100 Exercises to Learn Rust* first. This book assumes you can read Rust without
help.

If you've written Rust for a while and want to get to the harder exercises,
skim [How to run a code review](appendix/how-to-run-a-code-review.md) and
[The trade-off axes](appendix/trade-off-axes.md) first. They define the vocabulary used
throughout the book. Then use the [Roadmap](appendix/roadmap.md) to choose a part.
