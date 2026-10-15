# AGENTS.md

Instructions for AI coding agents working in this repository.

This repo is the source for the **Refactoring Rust** workshop, slides,
and book. Most of what you need is in three places:

- [`book/PLAN.md`](book/PLAN.md). The design contract for the book.
  Read this first if you're doing anything in `book/`.
- [`book/STYLE.md`](book/STYLE.md). **The writing style guide. Read it
  before touching any prose in `book/src/**.md`.** Non-negotiable
  rules live here, including:
  - **Never use em-dashes (—).** Use commas, parentheses, colons, or
    a period and a new sentence instead.
  - **Avoid the word "shape" / "shapes".** Reach for the precise word
    (*signature*, *structure*, *layout*, *form*, *interface*,
    *design*, etc.).
  - Voice rules (we / I / you discipline).
  - Banned filler words (*leverage*, *utilise*, *in order to*,
    *simply*, *obviously*, *of course*, …).
- [`CONTRIBUTING.md`](CONTRIBUTING.md). What kinds of changes are
  accepted, and the prose-vs-code license split. Closed prose
  authorship; open code contributions.

## Layout

- `book/`: the mdBook (work-in-progress on the `book` branch).
- `examples/`: starter code, one file per exercise.
- `solutions/`: canonical solutions, one file per exercise.
  Both `examples/NN_*.rs` and `solutions/NN_*.rs` are wired as
  `[[example]]` entries in `Cargo.toml` and run on every `cargo test
  --examples`.
- `slides/`: Slidev presentation.
- `Makefile`: common tasks (`make help` to list).

## Common tasks

```sh
make test         # cargo test --examples (starters + solutions)
make book         # serve the book with live reload, opens browser
make book-check   # full CI replica: test + book-test + book-build
```

## When editing prose

1. Open `book/STYLE.md` and re-read the *Tone* and *Word choices to
   avoid* sections.
2. Audit your draft before claiming it's done:
   ```sh
   grep -n '—' book/src/your-file.md         # must return nothing
   grep -ni 'shape' book/src/your-file.md    # justify every survivor
   ```
3. Run `make book-check` to confirm `mdbook build` and `mdbook test`
   still pass.

## When editing code

1. Keep starter and solution behaviour aligned. Both are tested.
2. Run `make test` before committing.
3. If you add a new alternative solution, add the `[[example]]` entry
   in `Cargo.toml` so CI picks it up.

## When in doubt

`book/PLAN.md` lists open questions in §15 and explicit non-goals in
§16. Don't expand scope beyond what's planned without flagging it.
