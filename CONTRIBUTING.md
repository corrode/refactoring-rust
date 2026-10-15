# Contributing to Refactoring Rust

Thanks for the interest. This repository contains two distinct kinds of
content with two distinct contribution policies. Please skim both
sections before opening a PR.

## TL;DR

- **Code** (`examples/`, `solutions/`, CI) — PRs welcome, MIT-licensed.
- **Book prose** (`book/src/**.md`) — closed authorship; PRs not
  accepted. **File an issue** with the suggestion; substantive
  contributions are credited in the Acknowledgements.

## Why the split

The book has a single author so it can carry a consistent voice,
opinion, and pedagogical arc end-to-end. Opening prose authorship
fragments that. The code, in contrast, benefits from many hands and is
permissively licensed so anyone can use it.

See `book/LICENSE.md` and the repository-root `LICENSE` for the legal
details.

## Code: PRs welcome

We're happy to receive PRs for:

- **Typo and clarity fixes** in `examples/` or `solutions/` comments.
- **Technical / correctness fixes** to any starter or solution.
- **New alternative solutions** as `solutions/NN_name_alt.rs`. Add the
  matching `[[example]]` entry to `Cargo.toml`. The book may or may not
  link to them; either way the file is useful.
- **CI improvements**, build-script fixes, and tooling upgrades.
- **`book/theme/css/corrode.css` tweaks** for cross-browser issues.

By submitting code, you agree it's licensed under MIT (matching the
repository-root `LICENSE`). No CLA, no DCO signoff required.

**Before opening a code PR:**

1. `cargo test --examples` must pass on your branch.
2. `cd book && mdbook build` must pass.
3. Keep PRs focused — one fix per PR is easier to review than ten.

## Book prose: file an issue

We don't accept PRs that modify `book/src/**.md` (or `book/PLAN.md`,
`book/STYLE.md`).

We *do* want your feedback, and we credit contributors in the
Acknowledgements appendix when their suggestions are adopted. Please
use the issue templates at
`.github/ISSUE_TEMPLATE/` — they correspond to the kinds of feedback
that genuinely help:

- **Suggest a Review question** — a question a thoughtful reviewer
  would ask on a chapter that we missed.
- **Suggest a Trade-off** — an axis or consideration we didn't score.
- **Suggest an Edge case** — production concerns that aren't covered.
- **Suggest an alternative solution** — usually pair this with a PR
  adding `solutions/NN_name_alt.rs`.
- **Found a typo or bug** — typo in prose; broken link; broken
  `{{#include}}` anchor; broken doctest.

## Reporting security issues

Security issues in any dependency or in the CI pipeline: please email
<matthias@corrode.dev> rather than open a public issue.

## Code of conduct

Be kind, be patient, assume good faith. Disagree about ideas, not
about people. If something feels off, email
<matthias@corrode.dev>.
