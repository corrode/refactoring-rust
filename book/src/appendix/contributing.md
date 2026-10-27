# Contributing & errata

> **Draft outline.** Bullet notes for the appendix; explanations still to be written.

- The repository's `CONTRIBUTING.md` is the policy source: closed book-prose authorship, open code contributions, and issue-based feedback on the text.
- This outline follows that policy while documenting the current `Makefile` checks, including intentional starter failures.

## Which changes belong in a pull request?

- Code corrections, comment typo or clarity fixes, and alternative solutions in `examples/` and `solutions/` are welcome.
- CI, build-script, tooling, and cross-browser stylesheet fixes are also listed as accepted contribution categories.
- Book prose changes, including rewrites, new chapters, and prose typo fixes, should be reported as issues rather than submitted as edits to `book/src/**.md`.
- `book/PLAN.md` and `book/STYLE.md` are also outside the accepted prose-PR route under the current policy.
- A new `solutions/NN_name_alt.rs` needs a matching `[[example]]` entry in `Cargo.toml` so the repository tests exercise it.
- An accepted alternative does not automatically become the chapter's recommended implementation or appear in the book.
- Keep a PR focused on one change, and describe changed contracts rather than claiming a shorter implementation is universally better.

## Choosing an issue category

- **Suggest a Discussion question:** `.github/ISSUE_TEMPLATE/review.yml` asks for the chapter, the proposed question, and why asking it would help.
- **Suggest a Trade-off:** `.github/ISSUE_TEMPLATE/tradeoff.yml` asks for the chapter, the affected axis, and what the comparison missed.
- **Suggest an Edge case:** `.github/ISSUE_TEMPLATE/edge-case.yml` asks for the failing input, load, or environment, with an optional mitigation.
- **Suggest an alternative solution:** `.github/ISSUE_TEMPLATE/alternative-solution.yml` accepts compilable code or a draft PR link and asks how it compares on the six axes.
- **Found a typo or bug:** `.github/ISSUE_TEMPLATE/typo.yml` asks where the problem occurs and what is wrong, covering prose, links, code includes, doctests, and CSS.
- Blank issues are disabled in `.github/ISSUE_TEMPLATE/config.yml`; use the relevant form rather than relying on an unstructured report.
- For technical reports, identify the chapter, input, expected result, actual result, and relevant toolchain or environment when it affects reproduction.
- A report about a deliberately broken starter should explain what is inconsistent with the teaching contract, not merely that its designated failure occurs.

## Code contracts and test changes

- Outside domain-modeling exercises, keep the first solution's original signatures and caller-facing types; place API-changing designs after it as alternatives.
- Domain-modeling solutions may change the main API, but must explain the migration and update callers and tests for that implementation.
- Distinguish a deliberate correctness fix from a behavior-preserving refactor, including differences in Unicode rules, tie selection, empty inputs, and error messages.
- Do not weaken an assertion or ignore a test to make an intentionally broken starter appear correct.
- When changing an exercise's intended failure, review its integration test and browser expectation metadata together rather than treating either as a blanket exemption.
- The [Concept index](concept-index.md) and [Trade-off matrix](trade-off-matrix.md) can help name the changed contract without replacing the exercise-specific tests.

## Current validation workflow

- From the repository root, `make test` runs Cargo examples and integration tests with all features, excluding four exact unit-test names from the broad run.
- `tests/domain_modeling.rs` separately compiles and runs the relevant starter tests, requiring their expected failure messages and matching passing solution cases.
- The four designated starter failures concern distinct ID categories in exercise 31, secret leakage through `Debug` in exercise 33, and self-friendship plus equal-name identity in exercise 35.
- Integration fixtures also test actual solution callers that must compile, run, or fail compilation with a particular diagnostic code.
- Plain `cargo test --examples` can fail on these starters by design, so it is not the current all-green acceptance command.
- `make book-test` runs `mdbook test` for eligible Rust listings; it does not replace the rendered editable-snippet checks.
- `make book-runner-test` runs the Node.js browser/theme tests and Python tests for the snippet checker.
- `make book-snippet-check` builds the book, then compiles and runs rendered editable implementations with their attached test suites.
- Each editable implementation needs its own compatible adjacent suite; `data-expected-failures` declares exact intended failing test names on the starter suite.
- Compilation failures, undeclared failures, unexpectedly passing designated failures, missing suites, and ignored tests are not accepted as successful validation.
- `make book-check` combines source tests, mdBook tests, runner tests, the book build, and rendered-snippet checks; it does not publish the book.
- The full local workflow needs Rust/Cargo, mdBook, Node.js, and Python 3.10 or newer, with any specially required Rust toolchain selected by the relevant snippet.
- `make book-build` builds static output only, while `make book` starts a live server; neither alone is the full validation workflow.
- TODO(prose): Reconcile the older plain-`cargo test --examples` instruction in `CONTRIBUTING.md` with the current `Makefile` and `.github/workflows/book.yml` before publishing the final contributor instructions.

## Licenses and reuse

- The root `LICENSE` specifies MIT for Rust files and Rust snippets in the book and slides, with the copyright and permission notice retained as required.
- Book prose is separate, with all rights reserved by corrode under `book/LICENSE.md`; do not treat the Rust code license as a license for whole chapters or the book's design.
- The book license permits short quotations with attribution and personal local study, but not substantial republication, translation without permission, or distribution of modified prose.
- The attribution named by that license is *Refactoring Rust* by Matthias Endler (corrode), at `https://refactoring.corrode.dev`.
- The contribution policy requires neither a CLA nor DCO signoff for code and states that submitted code is contributed under MIT.
- Existing third-party notices remain relevant when adapted code is redistributed; `THIRD_PARTY_NOTICES.md` covers four specific exercises, not every source or dependency in the repository.
- TODO(prose): Resolve the provisional dual-license wording still present in `book/LICENSE.md`; the root license currently specifies MIT, so this outline does not promise an Apache-2.0 option.

## Credit, contact, and publication details

- Adopted suggestions receive credit according to the repository policy; the [Acknowledgements](acknowledgements.md) outline separates these contributions from upstream source attribution.
- Security issues in dependencies or the CI pipeline should go privately to <matthias@corrode.dev>, not into a public issue.
- The same contact is listed for conduct concerns and book-permission inquiries; disagree about ideas rather than people.
- TODO(prose): Confirm the canonical repository and issue links before publication, since the current Git origin names `corrode/refactoring-rust` while `book/book.toml` points to `corrode/refactoring`.
- Until that URL decision is settled, the source paths above identify the actual checked-in policies and issue forms without copying potentially stale edit links.
