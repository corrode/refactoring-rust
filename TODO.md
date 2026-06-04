# Remaining workshop slide todos

> Exercise numbers below use the **new** numbering (after removing the old `05_two_variables`).
> The "(was N)" hint shows the old number where useful.

---

## 1. Put review prompts next to the source code (on a dedicated slide)

Each exercise should be a 3-slide sequence:

1. **Source code only** &mdash; the starter, full width.
2. **Source code + review prompts side-by-side** &mdash; same starter on
   the left, review bullets on the right.
3. **Possible solution** &mdash; as today.

Use Slidev's `two-cols-header` layout for step 2:

```md
---
layout: two-cols-header
---

# 02 · `describe`

::left::

```rust
pub fn describe(...) { ... }
```

::right::

- bullet 1
- bullet 2
```

Notes:
- Review bullets and the hint already render all-at-once (no
  `v-clicks` animations).
- Decide whether this applies to **every** exercise or only the ones
  where the starter is short enough to leave room on the right.
- The current standalone `# NN · Review` slide goes away once
  the side-by-side version exists.

---

## 2. Exercise 10 · `room_occupancy` — also show the newtype solution

Currently the solution slide only shows the four iterator-chain helpers.
Add a second solution slide showing a `Rooms` newtype:

```rust
pub struct Rooms(Vec<RoomOccupancy>);

impl Rooms {
    pub fn adults_total(&self) -> i32 { ... }
    pub fn children_total(&self) -> i32 { ... }
    pub fn max_adults_in_a_room(&self) -> i32 { ... }
    pub fn child_ages(&self) -> Vec<i32> { ... }
}
```

Talking point: now the four helpers become methods on the type, and the
caller can't accidentally pass an unrelated `&[RoomOccupancy]`.

---

## 3. Exercise 15 · `parse_port` — show `NonZeroU16` + `InvalidPort` solution

Add this as a solution slide:

```rust
use std::num::NonZeroU16;

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidPort {
    Invalid,
    Zero,
}

pub fn parse_port(s: &str) -> Result<NonZeroU16, InvalidPort> {
    let n: u16 = s.parse().map_err(|_| InvalidPort::Invalid)?;
    NonZeroU16::new(n).ok_or(InvalidPort::Zero)
}
```

> If nobody asks, suggest changing it to `Invalid(String)` where the string
> is the offending input, so the error message can be more informative.

---

## 4. Exercise 19 · `excluded_path` — show alternative signatures

The current signature is:

```rust
pub fn is_excluded(excluded: &Option<HashSet<PathBuf>>, path: &Path) -> bool {
```

Add a solution slide that walks through better alternatives:

```rust
// 1. drop the Option (caller passes empty set if none excluded)
pub fn is_excluded(excluded: &HashSet<PathBuf>, path: &Path) -> bool

// 2. argument order: subject first
pub fn is_excluded(path: &Path, excluded: &HashSet<PathBuf>) -> bool

// 3. extract a real type
struct Excluder<T>(HashSet<T>);
pub fn is_excluded(path: &Path, excluder: &Excluder<PathBuf>) -> bool

// 4. extension trait on Path
trait Excludable {
    fn is_excluded(&self, excluded: &HashSet<PathBuf>) -> bool;
}
impl Excludable for Path {
    fn is_excluded(&self, excluded: &HashSet<PathBuf>) -> bool {
        excluded.contains(self)
    }
}

// 5. push the invariant into the type: an Excluder is non-empty by construction
struct Excluder(NonEmptyHashSet<PathBuf>);
```

Frame it as "what is the type *really* telling us, and what's it hiding?".

---

## 6. Exercise 21 · `spell_check` — show the lazy `vimspell` solution

Add an extra solution slide that shows returning an `impl Iterator` of
byte ranges instead of allocating a `Vec`. Source for reference:
<https://github.com/exrok/vimspell/blob/361b63f71228af3d00291d3c257cd487cf5b95b8/src/lib.rs#L824>

```rust
/// Checks text for spelling errors, returning an iterator of byte ranges.
pub fn spell_check<'a>(
    &'a self,
    input: &'a (impl AsRef<[u8]> + ?Sized),
) -> impl Iterator<Item = std::ops::Range<usize>> {
    SpellCheckIter::new(self, input.as_ref())
}
```

Talking point: **it's lazy** — caller can stop at the first misspelling,
no `Vec` allocation, ranges instead of copied strings.

---

## 7. Exercise 23 · `fun_strings` — trim vertical on the solution

The solution slide is tall. Tighten it (compact `emoji_case` /
`spongebob_case` iterator chain, drop blank lines, shorter takeaway).

---

## 8. Accent stripe on intro & phase slides ✅

Done via `slides/components/AccentStripe.vue` (scoped `<style>` in
the component). Used on the cover, all three `00_*` intro pages, and
all six phase divider slides via `<AccentStripe />`.

The earlier failure mode (lifting CSS into global `style.css` +
repeating the raw `<div>`) is avoided because the styles live inside
the scoped component.

