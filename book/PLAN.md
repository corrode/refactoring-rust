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

- **Part I, Warm-ups** (01–04)
- **Part II, `Option` · `Result` · `?`** (05–10)
- **Part III, Aggregations & counting** (11–16)
- **Part IV, Strings, parsing & iterators** (17–22)
- **Part V, Domain modeling** (23–25)
- **Part VI, Capstone** (26–27)

Each part has a 1–3 paragraph intro page describing the theme and what the
reader will learn across its chapters.

---

## 3. Per-chapter anatomy

A fixed **7-section spine** every chapter follows, plus **3 optional
sections** included only when there's something real to say.

**Spine (always present):**

1. **The problem**: one paragraph framing, where you'd plausibly meet this
   in real code.
2. **Starter code**: `{{#include}}` from `examples/NN_*.rs`. Includes an
   inline "Try it yourself" callout with the `cargo` invocations.
3. **Review**: the questions a thoughtful reviewer would ask, framed
   as prompts to the *reader*. (Renamed from "Observations", also update
   the slides under TODO item #1 in `TODO.md`.)
4. **A possible solution**: the canonical solution from `solutions/`,
   walked through.
5. **Trade-offs**: scored against the fixed axes (see §4). Always the same
   axes; same order.
6. **Follow-up questions**: open-ended prompts for the reader / their team.
7. **Concepts recap**: named ideas this chapter introduced
   (e.g. *newtype*, *let-else*, *is_some_and*), each linked to the Concept
   index so they resolve to every other chapter that touches them.

**Optional (only when warranted, never as filler):**

- **Alternative approaches**: other valid solutions. Skipped where the
  canonical solution is genuinely the only good one.
- **Edge cases & production concerns**: what we didn't consider; what
  would break under load, weird input, i18n, concurrency. Included for most
  chapters from ~10 onward.
- **Further reading**: RFCs, std docs, blog posts, related crates.
  Skipped if no good external resource exists.

**Quizzes** sit between Trade-offs and Follow-ups. See §6.

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

These axes are defined once in the front-matter Trade-off-axes chapter and
referenced as shorthand thereafter. Changing them later is expensive, they
are the format's spec.

**`TODO`:** confirm the axis list before bulk-writing. The list may need to
split or merge categories once Phase 2 (reference trio) exposes real
scoring problems.

---

## 5. Code as single source of truth

**Hybrid include strategy, biased toward `{{#include}}`.**

- **Starter listings:** `{{#include ../../examples/NN_name.rs}}`.
- **Canonical solution listings:** `{{#include ../../solutions/NN_name.rs}}`.
- **Substantial alternative solutions** (e.g. the `Rooms` newtype, the
  `NonZeroU16` `parse_port`, the alternative `excluded_path` signatures,
  the lazy `spell_check` iterator): get their own files such as
  `solutions/NN_name_alt.rs` and are included.
- **Small variations / partials:** inline ```rust blocks (runnable via
  `mdbook test`). Use `// ANCHOR:` / `// ANCHOR_END:` regions inside
  solution files to include specific blocks without duplication.
- **Deliberately broken / illustrative code:** ```rust,ignore.

**CI must run in this order, gating deploy:**

1. `cargo test --examples`, covers both starters and solutions, since
   solutions are already wired as `NN_*_solution` `[[example]]` entries in
   `Cargo.toml`.
2. `mdbook test` (compiles inline rust blocks).
3. `mdbook build`.
4. Deploy.

The book is physically incapable of shipping broken code.

---

## 6. Interactivity

**mdBook + judgement-style quizzes + Rust Playground integration.**

- **Quizzes:** one per chapter, 2–4 questions, placed after Trade-offs and
  before Follow-up questions. Question style is **judgement**, not recall:
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

**Senior-engineer-reviewing-with-you voice** (first-person plural for
narration, first-person singular for contested opinions), with **light
direct address** in Review and Follow-up sections.

Style rules (full version goes in `book/STYLE.md`):

- Default to **we** for narration ("we'd reach for `&str` here because…").
- Use **I** sparingly, only for genuinely contested opinions
  ("I'll admit I find `is_some_and` slightly noisier than
  `map_or(false, …)`, but…").
- Use **you** only in Review prompts, Follow-up questions, and explicit
  instructions ("Try replacing the `Vec` with an iterator and see what
  breaks").
- Never **the reader** / **one** / passive-voice avoidance of pronouns.
- Allowed to disagree with Clippy, with the canonical solution, and with
  past chapters. **Required** to flag when doing so.
- Contractions on; em-dashes on; no exclamation marks except in code.

---

## 8. Front matter and back matter

**Front matter (in order, before Part I):**

1. **Preface, Why this book.** 1–2 pages, personal, opinionated,
   positions against rustlings / 100 Exercises / Brown.
2. **How to use this book.** Enumerates the modes: **Solo**, **Pair**,
   **Book club / team**, **Workshop facilitator**.
3. **How to run a code review.** ~5–8 pages, pure prose, no Rust-specific
   code. The book's meta-thesis: ask questions, don't dictate; every
   refactor has a cost; type-driven design is a tool not a religion.
4. **The trade-off axes.** Defines the six axes once and for all, with a
   worked example.
5. **Roadmap.** One page listing the parts and what each teaches.

**Back matter:**

6. **Concept index**: curated, not auto-generated. Every named concept
   listed with the chapters that introduce and reuse it. Doubles as the
   target of every chapter's Concepts recap section. Surfaces in the top
   nav as **"Cheatsheet"** (friendlier label).
7. **Trade-off matrix appendix**: one big table; rows are exercises,
   columns are the six axes, cells summarize how each canonical solution
   scored. The whole-forest view no competitor publishes.
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
│   │   ├── preface.md
│   │   ├── how-to-use.md
│   │   ├── how-to-run-a-code-review.md
│   │   ├── trade-off-axes.md
│   │   ├── roadmap.md
│   │   ├── part-1-warmups/
│   │   │   ├── README.md
│   │   │   ├── 01-starts-with-uppercase.md
│   │   │   └── ...
│   │   ├── part-2-control-flow/
│   │   ├── ...
│   │   └── appendix/
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
  - Suggest a Review question
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
  the 7-section spine labels (`EXERCISE`, `REVIEW`, `TRY THIS`,
  `TRADE-OFF`, `EDGE CASE`, `FOLLOW-UP`, `CONCEPT`).
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
the whole book, only where genuinely useful, e.g. state machines in ch. 20
or type relationships in ch. 27). No hand-drawn illustrations. No
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
- **2–4 questions per chapter.** Never zero (every chapter has a quiz);
  never more than four (quizzes are a checkpoint, not the experience).
- **No "what does this print"** unless the surprise is genuinely the
  point.

---

## 14. Writing process

**Order:** Scaffolding → Front matter → Reference trio → Bulk → Back
matter → Final pass → Launch.

The front matter must be written **before** the reference trio so the
six trade-off axes and the meta-position on code review are committed to
prose before any chapter operationalizes them.

The reference trio is **ch. 01, ch. 14, ch. 19**:
- **01** tests "does the format collapse on a trivial exercise?"
- **14** tests "does it handle genuinely contested error-handling
  trade-offs?"
- **19** tests "does Alternative approaches handle a 5-signature design
  space?"

Once the trio is locked, *they are the format spec.* New chapters are
written by copying their shape. No separate format spec document.

### Phase 0, Scaffolding (~1 day)

- Directory layout, `book.toml`, brand CSS, `SUMMARY.md` with all 27
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

- Chapter 01 (`starts_with_uppercase`)
- Chapter 14 (`error_handling`)
- Chapter 19 (`excluded_path`)
- Iterate on `STYLE.md` until it reflects what the trio actually does.

### Phase 3, Bulk write (~12 days)

- Chapters 02–13, 15–18, 20–27.
- Go part-by-part so part intros can be tightened against the chapters
  they introduce.
- Add real solution files for the alternative solutions TODO already
  identifies (`Rooms` newtype for ch. 10, `NonZeroU16` for ch. 15,
  alternative signatures for ch. 19, lazy iterator for ch. 21).

### Phase 4, Back matter (~3 days)

- Concept index (aggregated from per-chapter Concepts recap).
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
