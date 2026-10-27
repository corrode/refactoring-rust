# Part VI. Larger refactors

> **Draft outline.** Notes for the final part introduction; the chapters combine earlier techniques in two larger examples.

## What this part covers

- Work through several related decisions in one change: parsing, state, data representation, error handling, and boundaries between functions.
- Start by recording the existing behavior, including awkward cases that callers can observe.
- First improve the implementation without changing its signatures or caller-facing types.
- Then consider a separate redesign, explaining which callers, tests, accepted inputs, or error policies must change.

## Exercises

- [`env_file_parser`](36-env-file-parser.md): refactor a small line-based parser, then compare it with a state-machine alternative that supports a different grammar and reports errors.
  - Keep silent skipping and existing delimiter rules separate from decisions about quotes, escapes, multiline values, and malformed input.
  - The alternative is a limited `.env` dialect, not a claim of compatibility with every dotenv implementation.
- [`mini_redis`](37-mini-redis.md): separate command handling in a small in-memory store, then explore a different representation for values, commands, and responses.
  - Preserve the public storage fields in the first solution; changing to one value per key is a separate migration.
  - The exercise handles already-tokenized commands and returns reply strings; it is not a network server or a complete Redis implementation.

## Review and writing notes

- Connect decisions back to fallible parsing, enum-based dispatch, validation boundaries, and the cost of exposing representation through public fields.
- Distinguish characterization tests for preserved behavior from new tests that specify deliberate corrections or extensions.
- Show why extracting a function can be useful even when it does not repair a grammar or data-model problem.
- Compare each approach with Original on all six trade-off axes, keeping incompatible contracts visibly separate.
- Carry the unresolved grammar, diagnostic, and compatibility questions from the chapter outlines into the final writing pass; passing tests alone does not settle them.
- End with review questions readers can apply to their own larger changes, rather than presenting either alternative as production-ready.
