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

Use the section order from `PLAN.md` §3. An introduction is optional: keep it
only when it adds useful context beyond the exercise prompt and starter code.
Do not describe the same behavior in all three places or preview the solution
in the introduction. Short exercises can start with **Starter code**.

1. The problem (optional)
2. Starter code
3. Review
4. A possible solution
5. Trade-offs (six fixed axes; same order every time)
6. Quiz
7. Follow-up questions
8. Concepts recap

Optional, included only when there's something real to say:

- Alternative approaches
- Edge cases & production concerns
- Further reading

## Spine sections as callouts

Prompt-oriented spine sections open with a labelled blockquote. Use badges for
*Starter code*, *Review*, *Trade-offs*, *Follow-up questions*, and *Concepts recap*.
*A possible solution* and *Quiz* do not need badges because their headings already
name the content. The CSS in `theme/css/corrode.css` styles the leading `**LABEL**`
as a small-caps badge.

```markdown
> **REVIEW**
>
> - Does the function need to own the string?
> - What does `chars().nth(0)` do, and is there a more direct way?
```

Standard labels:

- `EXERCISE`: wraps the starter listing
- `REVIEW`: the questions a thoughtful reviewer would ask
- `TRY THIS`: an inline call to action
- `TRADE-OFF`: the per-chapter scoring section
- `EDGE CASE`: production concerns
- `FOLLOW-UP`: open-ended prompts
- `CONCEPT`: concepts recap entries

## Hidden solutions

Keep each canonical solution and its walkthrough collapsed by default. Readers should
have a clear chance to try the exercise before seeing an answer. Use this form:

````markdown
## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/NN_name.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/NN_name.rs:tests}}
</script>

Walk through the code here.

</details>
````

Leave *Alternative approaches*, *Edge cases and production concerns*, *Trade-offs*,
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

- Link Concepts recap entries to the appendix Cheatsheet with
  `[newtype](../appendix/concept-index.md#newtype)` style anchors.
- Link forward and backward between chapters generously
  (`see [ch. 13](../part-3-aggregations/13-excluded-path.md)`). The
  book is a graph, not a list.

## Trade-off scoring vocabulary

In the *Trade-offs* section, score the canonical solution against each
of the six axes with one of:

- **wins**: solution improves this axis vs. the starter
- **loses**: solution regresses this axis
- **neutral**: no meaningful change
- **N/A**: axis doesn't apply (use sparingly; usually means we should
  reconsider whether the axis really doesn't apply)

Include a one-sentence justification rather than only the label.

## Quizzes

- 2–4 questions per chapter, placed between *Trade-offs* and
  *Follow-up questions*.
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
