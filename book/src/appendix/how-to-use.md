# How to use this book

Each chapter starts with *Code to refactor*, followed by the task and its constraints.
Try the exercise before reading the solution so you can compare your decisions with those in the chapter.
Sit with the code for a bit and look for problems.

- Do you understand the logic?
- Is the code overly complex or verbose?
- Is the API clear and precise?

Start by preserving the function signatures and caller-facing types so callers don't need to change.
The first solution follows that rule; alternatives may change the API and explain what callers gain or lose.
[Domain modeling](../part-5-domain-modeling/index.html) is the exception: there, you can redesign the API from the start and update callers and tests to match.

## Solo

Allow five to twenty minutes for your first attempt at each exercise, plus time to read the explanation and answer the questions.

1. Read the code to refactor carefully, as if it were a pull request on GitHub.
2. Read the task and constraints that follow the starter code.
3. **Stop reading the chapter.**
   Write down what you'd say in a review, then refactor the code in your editor.
4. Read the *Discussion* questions and compare them with yours.
   Expand each question when you want to read its answer.
5. Open *Show solution* under *Solution* and compare the implementations with your version.
6. Read *Trade-offs* inside the same disclosure and decide whether you agree with the scores.
7. Choose a takeaway to investigate or discuss with someone else.

For more practice, return to the starter after reading the chapter and refactor it again without referring to the solution.

## Pair

Work together at one keyboard.
One person writes the refactor; the other asks questions and reviews the changes.
Use the chapter's *Discussion* questions as a starting point and add your own.
Discuss disagreements and explain each choice.
Swap roles every chapter, or every other chapter.

## Book club / team

For a group of three to eight people, read one chapter per week.
Anyone who wants to can bring a refactored version.
Keep the meeting to an hour and spend it on four things:

1. What each person noticed before reading *Discussion*.
2. Which solution is closest to the canonical one, which is furthest away, and why.
3. Where the group disagrees with the *Trade-offs* scores.
4. The *Takeaways For Our Own Codebase*.
   They were written for this conversation.

Between meetings, look for related examples in your team's code.
Take more than one session for a chapter if needed, and skip the warm-ups if the team already knows the material.

## Workshop facilitator

I teach this material as a live workshop for teams, using the same exercises with more time for discussion.
If you're running an internal workshop, use the slides in [`slides/`](https://github.com/corrode/refactoring-rust/tree/main/slides) for the live session and keep the book nearby as the reference.
The chapter sections already divide the work:

- Use *Discussion* to prompt the room.
- Use *Trade-offs* inside *Show solution* when the discussion starts treating a preference as a rule.
- Use *Takeaways For Our Own Codebase* for breakout rooms.

To book a facilitated workshop for your team, visit <https://corrode.dev>.

## Running the code in the browser

Each editable block has a **Run** button.
It sends the current code and its attached tests to the [Rust Playground](https://play.rust-lang.org) and shows the results below the code on the same page.
Open **Tests and local setup** to inspect the assertions, add a case, or change a caller.
If you try a different signature, update the callers yourself; the editor won't replace them for you.

## Running the code locally

The starters live in `examples/`; the canonical solutions live in `solutions/`.
Both are wired up as Cargo examples:

```sh
git clone https://github.com/corrode/refactoring-rust
cd refactoring-rust

# Test a starter
cargo test --example 01_is_yes

# Run the canonical solution's tests
cargo test --example 01_is_yes_solution

# Try an alternative that changes the API
cargo test --example 01_is_yes_alt_solution

# Run source tests, including checks for intentional starter failures
make test
```

> [!NOTE]
> Some starters have intentional test failures, documented in [chapter 33](../part-5-domain-modeling/33-typed-ids.md), [35](../part-5-domain-modeling/35-secret-token.md), [37](../part-5-domain-modeling/37-temp-file.md), and [39](../part-5-domain-modeling/39-friends.md).
> They remain ordinary failures in the browser and when you run those starters' tests directly.
> `make test` checks these failures separately through integration tests, while plain `cargo test --examples` can fail by design.
> See the [current validation workflow](contributing.md#current-validation-workflow) for the exact failures and contributor checks.

## Building the book locally

To read the book offline or preview an edit, install [mdBook](https://rust-lang.github.io/mdBook/).
Run these commands from the repository root:

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

Each chapter has a visible *Solution* heading and a short introduction that doesn't give away the refactor.
The *Show solution* disclosure below it starts closed.
Open it to read the implementations, explanations, alternatives, production concerns, and trade-off comparisons together.

Exercises come with tests, but passing them doesn't prove you've covered every input.
A **NOTE** callout identifies expected starter-test failures.
A **WARNING** callout flags important production or safety limitations.
Solution-specific warnings stay inside *Show solution*.

*Takeaways For Our Own Codebase* contains questions and actions in an unlabelled blockquote.
*References* links to the [Cheatsheet](concept-index.md) and other relevant documentation.

Chapters marked **BONUS** are optional.
Skip them when you're short on time because no later chapter depends on them.

## Where to start

If you can follow the code to refactor in [chapter 01](../part-1-warmups/01-is-yes.md), start there and read forward.
If not, work through *The Book* or *100 Exercises to Learn Rust* first.

If you've written Rust for a while and want to get to the harder exercises, skim [How to run a code review](how-to-run-a-code-review.md) and [The trade-off axes](trade-off-axes.md) first.
They define the vocabulary used throughout the book.
Then use the [Roadmap](roadmap.md) to choose a part.
