# Writing style for Refactoring Rust

> **Status:** living document. Established from the reference trio
> (chapters 01, 15, 13) and updated as the book grows.
> See `book/PLAN.md` §7 for the rationale.

## Voice

- **Use "you" naturally** when walking through code, naming a choice, or explaining
  what a caller sees: *"If you pass a borrowed `String`, this function reads it without
  taking ownership."*
- **Use "I" for judgement and reassurance**, especially when reasonable reviewers may
  disagree: *"I'd keep the loop if your team doesn't use `is_some_and` yet."*
- **Use "we" only for genuinely shared work**, not as the default narrator: *"Before we
  refactor, we need to settle what an empty input means."*
- Anticipate likely questions and answer them where they arise. *"Does this allocate?
  No. `any` borrows each prefix and stops at the first match."*
- Write from the reader's perspective. Prefer the consequence they care about over a
  detached description of the code.
- Do not write "the reader," "one," or avoid pronouns through passive voice.

## Tone

- Keep explanations matter-of-fact. Start with what the function does, what needs
  changing, and why. Don't invent a colleague, review meeting, or cleanup ticket to
  introduce an exercise.
- Use simple words and direct explanations. Keep examples that explain the code;
  cut stories, metaphors, and repeated summaries that don't help explain it.
- Allowed, and encouraged, to disagree with Clippy, the canonical
  solution, the previous chapter, or your past self. **Required** to
  flag when you do, and why.
- Contractions are on.
- Do not use em dashes (`U+2014`). Replace them with commas, parentheses,
  colons, or a period and a new sentence, depending on what the dash
  was doing. Em dashes are a tell for AI-generated prose and unedited
  drafts. This book does not use them.
- En-dashes (–) are fine for numeric ranges (`chapters 01–04`) and
  nothing else.
- No exclamation marks outside of code.
- Don't apologise for opinions. Justify them.
- Don't hedge with "arguably," "perhaps," "maybe." Either say it or
  don't.
- Be empathetic without performing friendliness. Acknowledge when an API is unfamiliar,
  a distinction is easy to miss, or two answers are both reasonable. Then explain the
  deciding factor.
- Respect the reader's time. Lead with the useful fact, cut repeated transitions, and
  vary sentence length. Keep enough explanation for the reasoning to stand on its own.
- Prefer ordinary verbs such as "is," "has," "does," and "uses" over inflated phrasing.

## Word choices to avoid

- **"shape" / "shapes"**. Vague, overused, AI-flavoured. Almost
  always there is a more accurate word:
  - the *shape* of a function: its *signature*
  - the *shape* of an API: its *interface* or *surface*
  - the *shape* of a conversation: its *form*, *style*, or *register*
  - the *shape* of an argument: its *structure*
  - the *shape* of a refactor: the *change* or the *design*
  - the *shape* of data: its *layout* or *schema*

  Reach for the precise word. "Shape" survives only when the
  geometric metaphor is genuinely load-bearing, which is almost
  never.

- **"leverage"** as a verb. Use *use*.
- **"utilise"**. Use *use*.
- **"in order to"**. Use *to*.
- **"facilitate"**, **"enable"** when *let* would do. Use *let*.
- **"robust"**, **"powerful"**, **"elegant"** as adjectives applied
  to our own code. These are claims the reader should make about our
  code, not claims we make about our own. Say what the code *does*
  and let the adjective land where it lands.
- **"simply"** as a hedge before something that isn't simple. Cut it.
- **"of course"**, **"obviously"**. Cut them. If it's obvious, the
  reader will notice. If it isn't, we've just made them feel stupid.

## Structure

Use `src/part-1-warmups/01-starts-with-uppercase.md` as the format reference.
The chapter title is the exercise name in backticks, without its number. Keep
exercise numbers in filenames, commands, and the table of contents.

Start with a short introduction that states the task and any constraints. Adapt it
to the exercise: don't repeat the first chapter's Run-button instructions, cloning
instructions, or encouragement. Include the local test command where useful.
There is no separate “The problem” heading or EXERCISE / TRY THIS callout.

Do not identify the bug, name the refactor, or supply a failing input in the
introduction. Put those discoveries in Discussion or an optional hint. State
whether API changes are allowed; preserving results does not necessarily mean
preserving the signature. Don't promise that code works when the task includes
finding and fixing a bug.

Use this section order:

1. Code to refactor: editable listing followed by its hidden test scripts.
2. Discussion: a DISCUSSION callout, followed by explanations.
3. Solutions: one collapsed box with implementations and explanations.
4. Trade-offs: comparison matrices with an **Original** baseline column.
5. Follow-up questions: a FOLLOW-UP callout.
6. Important Concepts: a CONCEPT callout.

Keep alternatives and relevant edge cases inside Solutions as level-three
subheadings. Preserve useful caveats rather than deleting them to fit the format.
Unwritten chapters remain marked as drafts; do not add empty sections or invented
content just to fill the template. A quiz remains optional under the rules below.

## Callouts

Use these labels where the reference chapter uses them:

- `DISCUSSION`: questions about the code and refactoring choices.
- `FOLLOW-UP`: questions for further work or team discussion.
- `CONCEPT`: important concepts and links.

Code to refactor and Trade-offs do not need a callout repeating the heading.
Within Solutions, an EDGE CASE callout can draw attention to a constraint.

## Hidden solutions

Keep the main solution, alternative implementations, and their explanations together
in one disclosure, collapsed by default. Use subheadings to distinguish approaches
inside it; do not nest another disclosure for each alternative. Readers should
have a clear chance to try the exercise before seeing an answer. Put the heading inside
the summary so it serves as the disclosure control without a repeated label. The
`solution` class adds a tinted box, and the decorative icon is hidden from screen
readers. Do not add a separate Markdown heading above it. Use this form:

````markdown
<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/NN_name.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/NN_name.rs:tests}}
</script>

Walk through the code here.

</details>
````

Put *Alternative approaches* inside the disclosure, under a level-three heading.
Put relevant *Edge cases and production concerns* inside it too. Leave *Trade-offs*,
*Quiz*, and later sections outside the disclosure. The native `details` element keeps
the summary keyboard accessible, and mdBook still renders included code as a Playground
block.

## Code blocks

- **Starter listings:** `{{#include ../../../examples/NN_name.rs:listing}}` from
  a chapter under `book/src/part-*`.
- **Canonical solutions:** `{{#include ../../../solutions/NN_name.rs:listing}}` from
  the same location. Put the code readers should see between `// ANCHOR: listing` and
  `// ANCHOR_END: listing`. Put the test module between `// ANCHOR: tests` and
  `// ANCHOR_END: tests`, then include it in an adjacent inert
  `<script class="playground-tests">` element. The editor hides that element, while the
  Run button sends its contents to the Rust Playground with the edited code. Starter
  blocks also include the solution tests as `playground-tests-fallback`, so an intended
  signature change can switch to the updated test API. Cargo and the slides still use
  the complete source file. Include paths are relative to the Markdown file, so
  front-matter pages need a different number of parent traversals.
- **Alternative solutions worth keeping runnable:** add a real file
  `solutions/NN_name_alt.rs` (and a matching `[[example]]` entry in
  `Cargo.toml`), then `{{#include}}` it.
- **Inline snippets:** ` ```rust,editable ` for complete runnable code,
  ` ```rust,ignore ` for deliberately broken or illustrative code, and
  ` ```rust,no_run ` for code that should compile without running.
- **`noplayground`** on any block the reader is *not* meant to run
  (avoids tempting them with a Run button on illustrative code).
- Prefer **`// ANCHOR:` / `// ANCHOR_END:`** regions in solution files
  over duplication, when only a fragment is needed.

## Cross-references

- Link Important Concepts entries to the appendix Cheatsheet with
  `[newtype](../appendix/concept-index.md#newtype)` style anchors.
- Link forward and backward between chapters generously
  (`see [ch. 13](../part-3-aggregations/13-excluded-path.md)`). The
  book is a graph, not a list.

## Trade-off comparisons

Compare the original and the implementations shown in Solutions side by side.
Use the six criteria as rows, in their usual order, and name each implementation
in a column heading. The original column describes the baseline; other cells give
a relative score and a short reason:

- **+**: advantage over the original.
- **=**: comparable on this criterion.
- **−**: disadvantage relative to the original.
- **N/A**: the criterion does not apply.

Keep the reason in the cell so the comparison stands on its own. Avoid sentences
that make the table hard to scan. Use short phrases such as “+ Borrows input” or
“= Same error type,” rather than a score with no explanation.

Include a brief legend. Readability depends on familiarity; performance claims
must name the affected work, not assume a speed-up. Do not rate different contracts
as interchangeable. Explain any required qualifications below the table.

Split wide comparisons into smaller tables with the original baseline repeated.
Use separate tables for functions with different requirements. Do not combine their
scores or calculate an overall winner.

## Quizzes

- Optional. Include a quiz only when it adds a new application or decision.
  Omit the heading and section when no question earns its place.
- Remove questions answered by repeating the preceding explanation, obvious
  yes/no questions, and generic questions about team preferences.
- There is no minimum question count. Put any retained questions between
  *Trade-offs* and *Follow-up questions*.
- **Judgement** style, not recall. See `PLAN.md` §13.
- Never "what does this print" unless the surprise is the point.
- Until the quiz plugin is chosen, use numbered `###` question headings followed by
  an italic `*Answer:*` paragraph. This keeps the questions readable and leaves a
  consistent structure for a later migration.

## TODOs

When something isn't right yet, leave a literal `TODO(scope): …`
comment in the source markdown rather than glossing over it. Reviewers
should be able to grep for them. Scopes:

- `TODO(prose)`: wording, structure, an idea half-baked
- `TODO(theme)`: visual / CSS / theme work, often `v2`
- `TODO(plugins)`: mdBook plugin decisions
- `TODO(slides)`: needs a matching change in `slides/`
