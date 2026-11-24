# Unpublished book notes

These drafts and editorial follow-ups are outside `book/src/` and are not included in the published book.
The archived drafts describe earlier plans and may be out of date.

- [How to run a code review](how-to-run-a-code-review.md): general review guidance, kept for possible use outside the book.
- [Roadmap](roadmap.md): an earlier overview, replaced by the part introductions and the usage guide.
  Its chapter-31 description promises layered configuration, but the exercise converts one map and does not load or merge sources.
- [Trade-off matrix](trade-off-matrix.md): an unfinished comparison outline.
  The book keeps comparisons with their exercises rather than maintaining a second scorecard.

## Reference maintenance

- Recheck the [Concept index](../src/appendix/concept-index.md) against chapter References after drafts settle, adding definitions where an entry only points to an example.
  Preserve existing anchors and check incoming links before removing or merging entries.
- Recheck [Further reading](../src/appendix/further-reading.md) after chapter edits, deduplicating resources while preserving sections that support different claims.
  Keep historical commit pins distinct from version-moving API documentation.
- Keep chapter notes, slides, source comments, bibliography entries, and acknowledgements consistent about provenance.

## Credits and permissions before publication

- Assemble contributor credits from accepted issues, PRs, and review notes, identifying the chapter and contribution.
  Confirm preferred names or handles and consent to public credit from private participants.
  Do not treat attendance as authorship or endorsement.
- Verify individual PR-author attribution before adding personal names to project credits.
  Copyright holders, change authors, and book reviewers are distinct roles.
- Review quotations, puzzle wording, and adapted snippets separately from bibliography completeness.
  The [third-party notices](../../THIRD_PARTY_NOTICES.md) cover exercises 32, 33, 35, 36, and 38 only, not every source or dependency.
  A citation is not a license grant.
- Resolve the provisional dual-license wording in [the book license](../LICENSE.md) and [the plan](../PLAN.md).
  The [root license](../../LICENSE) currently specifies MIT; the reader-facing page must not promise an Apache-2.0 option.
- Confirm the canonical repository and issue URLs before publication.
  Existing prose links use `corrode/refactoring-rust`, while `book.toml` points to `corrode/refactoring`.
