# Notes on Slidev for future sessions

Things I learned setting up this deck. Optimised for "future me, six
months from now, in a hurry".

## Project shape

```
slides/
├── package.json          # @slidev/cli + theme + vue
├── slides.md             # entry: cover + section dividers + src: includes
└── pages/
    ├── 00_motivation.md
    ├── 01_starts_with_uppercase.md
    └── ...
```

Each page in `pages/` is one or more slides separated by `---` on a line
by itself. The entry deck pulls them in with frontmatter:

```md
---
src: ./pages/01_starts_with_uppercase.md
---
```

The `src:` page itself is ignored - the file contents replace it. You
can also reuse the same file multiple times, or import a slice
(`./foo.md#2,5-7`) if you only want some of its slides.

## Running it

```sh
cd slides
npm install
npm run dev      # localhost:3030, hot reload
npm run build    # static SPA in ./dist
npm run export   # PDF via playwright
```

For PDF export, install `playwright-chromium` once:
`npm i -D playwright-chromium`.

## Frontmatter that matters

- `theme: seriph` - clean serif look, decent contrast.
- `colorSchema: dark` - forces dark mode regardless of OS.
- `transition: slide-left` - gentle slide between pages.
- `mdc: true` - enables `{class-name}` shorthand on inline elements.
- `defaults.layout: default` - what every page gets unless it overrides.
- Per-slide `layout: center` (and others: `cover`, `two-cols`,
  `image-right`, `quote`, `section`, `end`) override the default.

## Animations: `<v-click>` and `<v-clicks>`

The single most useful feature. Spacebar advances clicks within a slide
before moving to the next slide.

```html
<v-click>Shown after 1st click</v-click>

<v-clicks>

- Item one
- Item two
- Item three

</v-clicks>
```

- `<v-clicks>` wraps a list and reveals each bullet on successive clicks.
- `<v-click hide>` (or `.hide` modifier) hides an element after another click.
- `<v-click at="3">` for absolute timing; `<v-click at="+2">` for relative.
- Inside the wrapper, **leave a blank line** between the tag and the
  Markdown list - otherwise the list won't be parsed as Markdown.

## Code blocks

Triple-backtick fenced code blocks Just Work and are syntax-highlighted
by Shiki. Useful extras:

- `<<< @/path/to/file.ext` - import a whole file as a code block.
  `@` is the project root (where `package.json` lives).
- `<<< @/path/to/file.ext#region-name` - import a VS Code fold region
  (lines between `// #region region-name` and `// #endregion region-name`).
- ` ```rust {1,3-5|6}{lines:true} ` - Shiki line highlighting with
  step-through (`|` separates click steps).
- ` ```md magic-move ` block with multiple sub-blocks - Shiki Magic Move
  animates between code versions. Great for showing diffs.
- ` ```rust {monaco} ` turns a code block into an inline Monaco editor.

I deliberately did **not** use file imports for the per-exercise task
slides, because:

1. The example files include `main`, tests, and `use` lines we don't
   want on the slide.
2. Adding `#region` markers to every example file pollutes the source
   for the workshop attendees.
3. The per-exercise slides have to fit on screen, so they sometimes
   need a trimmed version (`// ...`) anyway.

If you decide the duplication is too painful, the cleanest fix is to
add `// #region task` / `// #endregion task` around the function under
refactor in each example, then use
`<<< @/../examples/NN_name.rs#task rust` in the slide. The build will
fail loudly if the region doesn't exist, so it stays in sync.

## Speaker notes

Anything in an HTML comment `<!-- ... -->` **at the end of a slide**
becomes the presenter note shown in the presenter view (press `p`
during the dev server). Put them at the end, not the middle, or
Slidev treats them as plain comments.

```md
# Slide title

Some content.

<!--
This is a presenter note.
-->
```

The presenter view also supports `[click]` markers in notes for
click-synced highlighting:

```md
<!--
First thing to say.

[click] Now the bullets appear.

[click] And so on.
-->
```

## Styling

- The theme provides sensible defaults. Override with a `<style>` block
  inside the slide for slide-scoped CSS, or edit `style.css` /
  `styles/index.ts` for global tweaks.
- Layout helpers: UnoCSS is built in, so Tailwind-style classes like
  `mt-8`, `text-xl`, `opacity-80`, `grid grid-cols-2 gap-12` work out
  of the box.
- The cover slide uses a manual accent stripe via a `<style>` block; see
  `slides.md` at the top.
- `mdc: true` lets you write `[text]{.class-name}` for inline styled
  spans (didn't use much here, but useful for ad-hoc highlights).

## Pitfalls I hit

1. **Blank lines inside `<v-clicks>` matter.** Without a blank line
   between the opening tag and the Markdown list, the list is rendered
   as inline HTML and `v-clicks` doesn't see children. Always leave
   the blank line.
2. **`---` inside a code fence is fine**, but `---` on its own line
   inside a comment block can be interpreted as a slide separator. Keep
   `---` only where you mean a slide break.
3. **Speaker notes only render if they're at the end of the slide.** A
   comment in the middle is just a comment.
4. **`src:` pages can't have their own frontmatter for the *parent*
   slot.** If you want a layout on the imported page, set it on the
   first slide of the imported file. The parent's `src:` slide is
   discarded entirely.
5. **`npm install` is slow** the first time (Slidev pulls Monaco,
   Shiki, full Vue, Vite, Playwright bindings). Budget ~3 min on a
   cold cache.

## Useful CLI flags

```sh
slidev export --with-clicks      # one PDF page per click step
slidev export --range "1,4-5,6"  # subset of slides
slidev build --base /workshops/refactoring/   # for sub-path hosting
slidev --remote                  # presenter view accessible on LAN
slidev --port 4000
```

## Workshop-specific conventions used here

- Three slides per exercise: **task**, **hints**, **solution**.
- Hints and solutions live behind `<v-click>` so you can't accidentally
  project the answer by tapping spacebar twice.
- Solutions are reasonable, not "the One True Answer". The speaker
  notes call out alternatives.
- Section dividers between phases give a natural breakpoint to look at
  the clock.
- File numbering matches `examples/NN_name.rs` exactly, so saying
  "exercise 16" works for both the deck and the codebase.

## Things to add later

- A solution diff using Shiki Magic Move (` ```md magic-move `) for the
  meatier refactors - would animate starter → solution beautifully on
  exercises 11, 23, 25, 26.
- Per-phase recap slides that summarize the idioms introduced.
- A "cheatsheet" appendix slide with all the `Option`/`Result`
  combinators, useful as a final hand-out.
- Workshop survey QR code on the closing slide.
