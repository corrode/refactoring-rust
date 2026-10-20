# Refactoring Rust: Book Plan

Working document. Captures the design decisions made during the planning
interview on the `book` branch. Mark up freely; this is the contract we'll
build the book against, not a finished spec.

Where a decision is still soft, it's flagged **`TODO`** with a question.
Where a v2 idea was deliberately deferred, it's flagged **`v2`**.

---

## 1. Positioning

**A standalone self-study book** that also doubles as the workshop's
reference companion.

- A reader who never attends the workshop must be able to work through the
  book end-to-end and get value from it.
- Slides remain terse (bullet points + code); the book is the source of
  truth for all prose.
- The book is also the workshop's marketing surface: "loved the book? book
  the workshop for your team."

**Differentiators vs. existing resources:**

| | rustlings | 100 Exercises | Brown TRPL fork | **Refactoring Rust** |
|---|---|---|---|---|
| Self-serve | ✅ | ✅ | ✅ | ✅ |
| Idiom-level refactoring | ⚠️ | ⚠️ | ❌ | ✅ |
| Honest trade-off analysis | ❌ | ❌ | ❌ | ✅ |
| Models how to do code review | ❌ | ❌ | ❌ | ✅ |
| Judgement-style quizzes | ❌ | ❌ | ⚠️ recall-style | ✅ |
| Production edge cases | ⚠️ | ⚠️ | ❌ | ✅ |
| Concept index across chapters | ❌ | ⚠️ | ⚠️ | ✅ |

---

## 2. Structure

**Hybrid: themed parts containing per-exercise chapters.**

Proposed parts, **confirmed against `slides/slides.md` phase
dividers**:

- **Part I, Warm-ups** (01–05)
- **Part II, `Option` · `Result` · `?`** (06–12)
- **Part III, Aggregations & counting** (13–19)
- **Part IV, Strings, parsing & iterators** (20–25)
- **Part V, Domain modeling** (26–33)
- **Part VI, Capstone** (34–35)

Each part has a 1–3 paragraph intro page describing the theme and what the
reader will learn across its chapters.

---

## 3. Per-chapter anatomy

Use `src/part-1-warmups/01-is-yes.md` as the format reference.
Start with an unnumbered exercise-name title and a short task-specific introduction.
State constraints without giving away the diagnosis or solution. Avoid repeating
the first chapter's instructions for using the editor and repository.

1. **Code to refactor**: an editable include from `examples/`, followed by an expandable **Tests and local setup** editor.
   Each block has one explicitly attached suite; learners can edit callers when changing an API.
   No EXERCISE or TRY THIS callout.
2. **Discussion**: independent, initially closed question-and-answer disclosures.
   Put each explanation with its question; do not repeat the questions in a callout.
3. **Show Solutions**: the main solution, alternatives, and relevant edge cases in one
   collapsed box. Show a signature-preserving refactor first; reserve API changes for alternatives.
   Keep the original caller-facing types in the first solution too.
   Domain modeling is the exception: the first solution may redesign the API, with caller migration and updated tests explained.
   Use level-three subheadings within it.
   End the disclosure with **Trade-offs** under `### Trade-offs`, with any subheadings at level four or deeper.
   Compare named variants against an **Original** column, with the six criteria as rows and a short reason beside each score.
4. **Takeaways For Our Own Codebase**: questions for further work or team discussion.
5. **References**: named concepts with links to the concept index.

Preserve useful technical details when applying this format. Unwritten chapters
stay marked as drafts. Quizzes are optional and must meet the criteria in §13;
never add one merely to fill a section. Further reading is optional too.

---

## 4. Trade-off axes (load-bearing)

Every Trade-offs section scores against these six axes, in this order:

1. **Type-safety**: does the type system enforce the invariant, or do we
   rely on convention/runtime checks?
2. **Readability**: would a teammate reading this in six months understand
   it without context?
3. **API design**: does the signature communicate intent? Are illegal
   states representable?
4. **Performance**: allocations, copies, branches, asymptotic cost.
   Acknowledged honestly even when it doesn't matter.
5. **Flexibility**: how easy is it to extend, compose, or generalize?
6. **Error reporting**: when this fails, does the caller learn enough to
   act?

These axes are defined once in the appendix's Trade-off-axes chapter and
referenced as shorthand thereafter. Changing them later is expensive, they
are the format's spec.

**`TODO`:** confirm the axis list before bulk-writing. The list may need to
split or merge categories once Phase 2 (reference trio) exposes real
scoring problems.

---

## 5. Code as single source of truth

**Hybrid include strategy, biased toward `{{#include}}`.**

- **Starter listings:** `{{#include ../../../examples/NN_name.rs}}` from chapters
  under `book/src/part-*`.
- **Canonical solution listings:** `{{#include ../../../solutions/NN_name.rs}}` from
  the same location. These preserve the original function signatures except in domain modeling, where API redesign is part of the task.
  Keep API-changing versions in `_alt.rs` files, registered after the main solution in Cargo and shown afterward in the book and slides.
- **Substantial alternative solutions** (e.g. the `Rooms` newtype, the
  `NonZeroU16` `parse_port`, the alternative `excluded_path` APIs,
  the lazy `spell_check` iterator): get their own files such as
  `solutions/NN_name_alt.rs` and are included.
- **Small variations / partials:** inline ```rust blocks (runnable via
  `mdbook test`). Use `// ANCHOR:` / `// ANCHOR_END:` regions inside
  solution files to include specific blocks without duplication.
- **Deliberately broken / illustrative code:** ```rust,ignore.

**CI must run in this order, gating deploy:**

1. `make test` runs registered starters, solutions, and integration tests.
   Integration tests check the two intentional starter failures separately; every solution must pass.
2. `mdbook test` checks eligible inline Rust blocks.
3. `make book-runner-test` checks the browser runner and rendered-snippet checker.
4. `mdbook build` expands includes and renders the book.
5. `python3 scripts/check_book_snippets.py book/book` compiles and runs every editable snippet with its explicitly attached suite, using the same source concatenation as the browser.
   `data-expected-failures` on a suite declares intentional failing test names.
   Unexpected failures, unexpected successes, compilation errors, missing suites, empty suites, and ignored tests fail the check.
6. Deploy.

`make book-check` runs these checks locally (requires Node.js and Python 3.10+ in addition to Rust and mdBook).
The browser submits the current code and edited tests once, without selecting or retrying another suite.
Intentional failures remain ordinary failing tests in the browser; their metadata controls CI expectations only.
These checks cover executable examples, not the correctness of every claim or explicitly ignored illustration.

---

## 6. Interactivity

**mdBook + judgement-style quizzes + Rust Playground integration.**

- **Quizzes (optional):** only when a new application or decision adds value.
  Place them outside the Show Solutions disclosure, after its Trade-offs section and before Takeaways For Our Own Codebase. Do not repeat
  points already covered in the chapter. Question style is **judgement**, not recall:
  - "Which of these signatures best expresses the invariant?"
  - "Which axis does change X improve at the cost of which other axis?"
  - "Spot the smell in this related snippet."
  - **Avoid** "what does this print", Brown overuses it; it's not what
    this book teaches.
  - **`TODO`:** pick the plugin, `mdbook-quiz` (Brown's, batteries
    included, heavier) vs. a lightweight custom shortcode. Decide during
    Phase 0.
- **Playground:** mdBook's built-in play button on `rust` blocks. Only
  enabled on blocks the reader is meant to run (starters, snippets); use
  `noplayground` for illustrative code. Pattern documented in `STYLE.md`.
- **No CLI fetcher** in v1. `git clone` is good enough; mentioned in
  How-to-use.
- **No ownership visualizer.** Wrong fit, this book teaches taste, not
  the borrow checker.

---

## 7. Voice and tone

**Senior-engineer-reviewing-with-you voice**, written from the reader's
perspective. Direct address belongs throughout the explanation when it
makes a consequence or decision easier to understand.

Style rules (the full version lives in `book/STYLE.md`):

- Use **you** naturally when walking through code, naming a choice, or
  explaining what a caller sees.
- Use **I** for judgement and reassurance, especially when reasonable
  reviewers may disagree.
- Use **we** only for work that the author and reader are doing together.
- Answer likely questions where they arise instead of postponing them.
- Do not write **the reader** / **one** or avoid pronouns through passive
  voice.
- Disagreement with Clippy, the canonical solution, or earlier chapters is
  welcome when the reason is explicit.
- Contractions are on. Em dashes are out. Exclamation marks appear only in
  code.

---

## 8. Front matter and back matter

**Front matter (in order, before Part I):**

1. **Refactoring Rust.** Introduces the book and its approach.
   Its **How to use this book** section covers **Solo**, **Pair**, **Book club / team**, and **Workshop facilitator** modes.
**Back matter (appendix, in order):**

3. **How to run a code review.** ~5–8 pages, pure prose, no Rust-specific
   code. The book's meta-thesis: ask questions, don't dictate; every
   refactor has a cost; type-driven design is a tool not a religion.
4. **The trade-off axes.** Defines the six axes once and for all, with a
   worked example.
5. **Roadmap.** One page listing the parts and what each teaches.

6. **Concept index**: curated, not auto-generated. Every named concept
   listed with the chapters that introduce and reuse it. Doubles as the
   target of every chapter's References section. Surfaces in the top
   nav as **"Cheatsheet"** (friendlier label).
7. **Trade-off matrix appendix**: one big table; rows are exercises,
   columns are the six axes, with one row per exercise and solution variant. The whole-forest view no competitor publishes.
8. **Further reading**: consolidated, deduplicated bibliography.
9. **Contributing & errata**: short; points at the issue templates.
10. **Acknowledgements**: every accepted issue, alternative-solution PR,
    typo fix gets a name. Closes the loop with the community even when we
    don't accept prose PRs.

**Glossary** is deliberately *not* a separate structure, definitions live
inside Concept index entries.

---

## 9. Repo layout, build, deployment

**`book/` at the repo root** (this directory). Single repo, single source
of truth, `{{#include ../../examples/...}}` works.

```
refactoring/
├── book/
│   ├── PLAN.md                 # this file
│   ├── STYLE.md                # writing style guide
│   ├── book.toml
│   ├── theme/                  # custom CSS, favicon, logo
│   │   ├── css/
│   │   │   ├── corrode.css
│   │   │   └── print.css
│   │   ├── favicon.png
│   │   └── logo.svg
│   ├── src/
│   │   ├── SUMMARY.md
│   │   ├── intro.md


│   │   ├── part-1-warmups/
│   │   │   ├── README.md
│   │   │   ├── 01-is-yes.md
│   │   │   ├── 02-starts-with-uppercase.md
│   │   │   └── ...
│   │   ├── part-2-control-flow/
│   │   ├── ...
│   │   └── appendix/
│   │       ├── how-to-run-a-code-review.md
│   │       ├── trade-off-axes.md
│   │       ├── roadmap.md
│   │       ├── concept-index.md
│   │       ├── trade-off-matrix.md
│   │       ├── further-reading.md
│   │       ├── contributing.md
│   │       └── acknowledgements.md
│   └── LICENSE.md              # all rights reserved, © corrode
├── LICENSE                     # top-level for code: MIT (or dual MIT/Apache)
├── .github/workflows/book.yml
├── examples/                   # unchanged
├── solutions/                  # unchanged, may grow alt solutions
└── slides/                     # unchanged
```

**Branch:** `book` (this one). Long-lived. Big-bang merge to `main` when
v1 is complete. No public release until merge.

**Deploy:** GitHub Pages via `actions/deploy-pages` (no committed
`gh-pages` branch). Source on `main`; built HTML never lives in git.

**URL:** `refactoring.corrode.dev` (custom subdomain). DNS work happens at
launch.

**`TODO`:** confirm `corrode.dev` DNS access at launch time.

---

## 10. Licensing

- **Prose:** all rights reserved. © corrode. Quotation for review and
  educational purposes permitted with attribution. Lives in
  `book/LICENSE.md`.
- **Code** (`examples/`, `solutions/`, runnable inline snippets):
  **MIT** (or dual MIT/Apache-2.0, defaulting to dual to match the Rust
  ecosystem; flag if you prefer MIT-only). Lives in repo-root `LICENSE`.
- The split is explained in the Preface and `README.md`.

**`TODO`:** pick MIT vs. dual MIT/Apache-2.0 for the code license.

---

## 11. Contribution policy

**Closed prose authorship; open code contributions.**

- **Accepted as PRs:** typo fixes, technical/factual corrections,
  broken-link fixes, new alternative solutions in `solutions/*_alt*.rs`,
  CI improvements.
- **Not accepted as PRs:** new prose sections, prose rewrites, new
  chapters. **File an issue instead**, substantive suggestions are
  credited in Acknowledgements when adopted.
- **Issue templates** in `.github/ISSUE_TEMPLATE/`:
  - Suggest a Discussion question
  - Suggest a Trade-off I missed
  - Suggest an Edge case
  - Found a typo / bug
  - Suggest an alternative solution
- **No CLA, no DCO.** Inbound contributions to `examples/` and
  `solutions/` are licensed under MIT (or dual MIT/Apache). Issues grant
  corrode permission to incorporate ideas with attribution.

---

## 12. Visual identity

**Idiomatic mdBook with a brand-color CSS layer.** No Handlebars template
overrides in v1.

v1 scope:
- Default mdBook `light` + `navy` (or similar) themes restyled with the
  corrode accent color and brand fonts.
- Search on; sidebar on; prev/next on; stock playground button on `rust`
  blocks.
- Blockquote-based callouts with a small CSS treatment to differentiate
  the callout labels (`EDGE CASE`, `FOLLOW-UP`, `CONCEPT`).
- Native question-and-answer disclosures in Discussion, with visible focus styling.
- Light + dark only; kill the other three default themes (coal, ayu,
  rust) unless that creates friction.

`v2` flagged for later (post-launch iteration):

- Custom Handlebars top nav with three-level breadcrumb
  (`Refactoring / Part / Chapter`).
- "Playground" top-nav link → play.rust-lang.org.
- "Cheatsheet" top-nav link → Concept index.
- "CHAPTER N" eyebrow + big magenta chapter title.
- Custom syntax highlighting theme matching the Rustfinity reference.
- Prominent in-page "Run" button styling.
- Per-chapter trade-off radar chart, programmatically generated from the
  same data that feeds the back-matter matrix.

**Assets needed from Matthias:**
- Body + display fonts (the slides may already define them, check first).
- Code font (likely JetBrains Mono or Berkeley Mono).
- Exact hex codes for the magenta and cyan accents.
- Corrode logo + favicon (probably exist in `slides/public/`).

**Diagrams:** tables everywhere they help; mermaid (~6–10 diagrams across
the whole book, only where genuinely useful, e.g. state machines in ch. 23
or type relationships in ch. 35). No hand-drawn illustrations. No
screenshots.

---

## 13. Quiz philosophy

(Cross-referenced from §6, expanded here.)

- **Judgement over recall.** Test whether the reader can choose between
  valid alternatives, not whether they memorized syntax.
- **Trade-off-aware.** Many questions take the form "what does change X
  improve, and what does it cost?", directly operationalizing the
  trade-off axes.
- **Spot the smell.** Some questions use a *related but different* snippet
  to test transfer of the chapter's idea, not pattern-matching on the
  chapter's exact code.
- **Optional, with no minimum count.** Omit quizzes that repeat the explanation
  or ask obvious questions. Keep a question only when it requires applying the
  idea to a new situation or making a decision not already discussed.
- **No "what does this print"** unless the surprise is genuinely the
  point.

---

## 14. Writing process

**Order:** Scaffolding → Front matter → Reference trio → Bulk → Back
matter → Final pass → Launch.

The front matter must be written **before** the reference trio so the
six trade-off axes and the meta-position on code review are committed to
prose before any chapter operationalizes them.

The reference trio is **ch. 02, ch. 16, ch. 14**:
- **02** tests "does the format collapse on a trivial exercise?"
- **16** tests whether the format handles repeated aggregations, a semantic edge
  case, and a domain-newtype alternative.
- **14** tests whether *Alternative approaches* can handle a multi-signature API
  design space.

Once the trio is locked, *they are the format spec.* New chapters follow
their structure. No separate format spec document.

### Phase 0, Scaffolding (~1 day)

- Directory layout, `book.toml`, brand CSS, `SUMMARY.md` with all 35
  chapter entries (unwritten ones as plain text `- [Title]()`).
- All part-intro stubs.
- CI workflow (`cargo test --examples`, `mdbook test`, `mdbook build`,
  deploy).
- `STYLE.md` skeleton (extended through Phase 2).
- `CONTRIBUTING.md` + issue templates.
- `LICENSE` (root) + `book/LICENSE.md`.
- Rename "Observations" → "Review" in `slides/pages/*.md` (TODO item
  #1 in the existing `TODO.md`).

### Phase 1, Thesis (~3 days)

- Preface
- How to use this book
- How to run a code review
- The trade-off axes
- Roadmap

### Phase 2, Format lock / reference trio (~4 days)

- Chapter 02 (`starts_with_uppercase`)
- Chapter 16 (`room_occupancy`)
- Chapter 14 (`excluded_path`)
- Iterate on `STYLE.md` until it reflects what the trio actually does.

### Phase 3, Bulk write (~12 days)

- Write the remaining chapters.
- Go part-by-part so part intros can be tightened against the chapters
  they introduce.
- Add real solution files for the remaining alternatives the TODO identifies
  (`NonZeroU16` for ch. 12 and a lazy iterator for ch. 15). The `Rooms` newtype now has its own domain-modeling chapter (26), following the aggregation work in ch. 16.
  Alternative exclusion APIs remain in ch. 14.

### Phase 4, Back matter (~3 days)

- Concept index (aggregated from per-chapter References).
- Trade-off matrix (aggregated from per-chapter Trade-offs).
- Further reading (consolidated).
- Contributing, Acknowledgements.

### Phase 5, Final pass (~2 days)

- End-to-end read for tone drift.
- Verify every Concept index entry resolves.
- Verify every cross-chapter reference.
- Verify every `{{#include}}` anchor still exists.
- Verify every quiz answer.

### Phase 6, Launch (~1 day)

- Merge `book` → `main`.
- DNS + deploy at `refactoring.corrode.dev`.
- Announce: blog post on corrode.dev, LinkedIn, r/rust if appropriate.
- Switch to chapter-improvement issue triage cadence.

**Total budget:** ~26 working days of focused writing. At a sustainable
part-time pace, 2–3 months elapsed.

---

## 15. Open questions to resolve before / during Phase 0

Collected from inline `TODO`s above:

1. ~~Confirm the proposed part grouping matches the slide phase dividers.~~
   **Confirmed during Phase 0.**
2. Confirm the six trade-off axes survive the reference trio; expect at
   least one to split or merge.
3. Pick the quiz plugin: `mdbook-quiz` vs. lightweight custom.
4. Pick MIT vs. dual MIT/Apache-2.0 for the code license.
5. Confirm `corrode.dev` DNS access at launch time.
6. Receive brand assets: fonts, accent hex codes, logo, favicon.

---

## 16. Explicitly out of scope for v1

- CLI fetcher (rustlings-style `cargo install refactoring-rust`).
- Ownership visualizer or any borrow-checker-teaching tooling.
- Hand-drawn illustrations.
- Custom in-browser code editor (Monaco / CodeMirror with WASM exec).
- Per-chapter radar-chart trade-off visualizations.
- Custom Handlebars top nav, "Cheatsheet" / "Playground" top-nav links,
  big magenta chapter title, all flagged as `v2` in §12.
- Translation / i18n.
- Print / PDF distribution (mdBook's built-in print works; not a v1
  feature).
- Traditional publisher edition.

Each of these is a real product decision; none of them is *wrong*. They
are deferred to keep v1 shippable.
