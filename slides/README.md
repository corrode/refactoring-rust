# Workshop slides

Slidev deck for the "Writing Better Rust" refactoring workshop.

```sh
npm install
npm run dev      # http://localhost:3030
npm run build    # static SPA in ./dist
npm run export   # PDF (needs `npm i -D playwright-chromium` once)
```

## Layout

- `slides.md` - entry deck. Cover + section dividers + `src:` includes.
- `pages/` - per-exercise pages, one file per exercise (`NN_name.md`).
- `NOTES.md` - Slidev cheat sheet and lessons learned for next time.

Each per-exercise page contains three slides: task, hints, solution.
Hints and solutions use `<v-click>` reveals so the answer never shows
up by accident when you advance through the deck.

The exercises themselves live in `../examples/NN_name.rs`. Slide
numbers match file numbers exactly.

## Presenter view

Run `npm run dev`, then press `p` to open the presenter view in a
second tab/window. Speaker notes (the `<!-- ... -->` blocks at the end
of each slide) show up there.
