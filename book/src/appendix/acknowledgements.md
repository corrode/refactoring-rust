# Acknowledgements

> **Draft outline.** Bullet notes for the appendix; explanations still to be written.

- Separate contributions to this book from the sources that informed its exercises and from the legal notices attached to adapted code.
- `book/book.toml` names Matthias Endler as the author, and the book license attributes the work to Matthias Endler (corrode).
- This outline records supported source attribution and credit categories, not an invented list of reviewers or a completed contributor register.

## Contributions to record

- Adopted Discussion questions: connect the credited person or handle with the question and chapter rather than offering an unexplained list of names.
- Accepted alternative solutions and code fixes: retain the PR or commit reference, the exercise, and the contribution's purpose.
- Corrections and edge cases: credit reports that change a contract explanation, test, comparison, or implementation, including small but useful typo and link reports.
- Browser-runner, build, theme, and accessibility work: identify the actual contribution rather than crediting every tool author as a book reviewer.
- Preview readers and workshop participants: record specific feedback where available, without assuming that attending a session means contributing text or endorsing the book.
- TODO(prose): Assemble names and evidence from accepted issues, PRs, and author-held review notes; the policy promises credit for adopted suggestions, but these source records do not supply that completed list.
- TODO(prose): Confirm preferred names or handles and whether private participants want public credit before adding them to the final acknowledgements.

## Named teaching sources

- **Jack O'Connor, Object Soup is Made of Indexes:** the teaching article credited in [35: friends](../part-5-domain-modeling/35-friends.md), its slide page, and both Rust files.
  - The example develops the distinction between object relationships and ownership through indices, not a report of a production refactoring PR.
  - Retain the [article link](https://jacko.io/object_soup.html) alongside the acknowledgement so the source remains identifiable.
- **Advent of Code 2015, Day 1: Not Quite Lisp:** the problem source named in `examples/10_floor.rs` and the slide page for [10: floor](../part-2-option-result/10-floor.md).
  - Credit the [puzzle source](https://adventofcode.com/2015/day/1) without presenting the workshop's iterator variants as copied puzzle solutions or a production change.
- **Beginner-course feedback patterns:** [32: validation_feedback](../part-5-domain-modeling/32-validation-feedback.md) records an instructor-written fixture inspired by a recurring submission pattern.
  - The source explicitly says no learner code was copied, so do not invent a named student, particular submission, or permission history.

## Upstream projects and recorded notices

- **hyper and Sean McArthur:** [30: buffer_policy](../part-5-domain-modeling/30-buffer-policy.md) adapts historical buffer setters, with a workshop-specific enum redesign rather than an upstream refactoring PR.
  - `THIRD_PARTY_NOTICES.md` records the pinned MIT notice, copyright 2014–2021 Sean McArthur, and the standalone adaptation's changes.
- **The Nushell Project Developers:** [31: typed_ids](../part-5-domain-modeling/31-typed-ids.md) is inspired by the verified ID refactor in PR #13832.
  - The recorded MIT notice names The Nushell Project Developers, copyright 2019–2023; the local `Engine` and tuple newtypes are teaching fixtures rather than upstream excerpts.
- **secrecy and iqlusion:** [33: secret_token](../part-5-domain-modeling/33-secret-token.md) draws on existing explicit-access and redacted-debugging design.
  - The recorded MIT notice is copyright 2019 iqlusion; PR #1140 restructured `SecretBox` and is not evidence of an upstream version of the starter's secret leak.
- **rustls and Joseph Birr-Pixton:** [34: config_typestate](../part-5-domain-modeling/34-config-typestate.md) simplifies a pinned generic typed transition into instructor-created concrete states.
  - The recorded MIT notice is copyright 2016 Joseph Birr-Pixton; the historical builder is a design source, not a verified before-and-after refactoring report.
- These names identify projects or copyright holders documented by the source records, not people claimed to have reviewed, approved, or contributed to this book.
- Keep the full applicable notices with redistributed adapted code; a short acknowledgement does not replace a required copyright and permission notice.

## Other documented influences

- The Google Rust Style Guide supplies cited API guidance on borrowing, iterators, enum parameters, standard traits, and newtypes, with chapter-specific qualifications.
- Rust documentation supplies the language and library contracts used throughout the examples; citing those contracts is different from claiming an endorsement by their authors.
- The pinned `vimspell` API is an external lazy-result example discussed in [15: spell_check](../part-3-aggregations/15-spell-check.md), not a checked-in local alternative or a verified source of a book contribution.
- [Further reading](further-reading.md#production-history-and-design-sources) keeps the project, PR, and commit links together with their actual role in the book.

## Attribution review before publication

- `THIRD_PARTY_NOTICES.md` explicitly covers exercises 30, 31, 33, and 34 only, so it cannot stand in for a repository-wide attribution or dependency review.
- TODO(prose): Review any retained article quotations, puzzle wording, or adapted snippets outside those four exercises and record the applicable terms rather than assuming the repository's MIT license grants upstream rights.
- TODO(prose): Verify individual PR-author attribution before adding personal names to the project credits; a repository name, copyright holder, and change author are not interchangeable.
- Check that chapter notes, slides, source comments, bibliography entries, and acknowledgements describe the same provenance when examples are revised.
- Do not turn source credit into a statement of permission, sponsorship, endorsement, or participation unless the author has evidence for that statement.
- Keep the final list consistent with [Contributing & errata](contributing.md), especially the promise to credit adopted feedback despite closed prose authorship.
