---
layout: default
---

# 01 · `starts_with_uppercase`

<div class="opacity-80 mb-4">
Does this string start with an uppercase letter?
</div>

```rust
pub fn starts_with_uppercase(s: String) -> bool {
    if s.len() == 0 {
        return false;
    }
    let first = s.chars().nth(0).unwrap();
    if first.is_uppercase() == true {
        return true;
    } else {
        return false;
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 01_starts_with_uppercase</code>
</div>

<!--
Easiest possible warm-up. Several things to find. Don't preempt them.
Common discoveries (in roughly the order people will spot them):
  - `String` -> `&str`
  - `len() == 0` -> `is_empty()`
  - `chars().nth(0)` -> `chars().next()`
  - `== true` -> just drop it
  - `if x { true } else { false }` -> just `x`
-->

---

# 01 · Review

- Does the function need to **own** the string?
- What does `chars().nth(0)` do, and is there a more direct way?
- That `if … { true } else { false }` could be simpler.
- What's the right answer for `""` ?

<div class="mt-12 opacity-70">
Hint: Clippy will tell you most of these. Run it.
</div>

---

# 01 · Possible solution

```rust
pub fn starts_with_uppercase(s: &str) -> bool {
    s.chars().next().is_some_and(char::is_uppercase)
}
```
```rust
pub fn starts_with_uppercase(s: &str) -> bool {
    s.starts_with(char::is_uppercase)
}
```

<div class="mt-8 text-base opacity-80">

- `&str` is the natural parameter type for a read-only check.
- `chars().next()` returns `Option<char>`; `is_some_and` answers
  "is there a first char, and does it satisfy the predicate?"
- Empty string → `None` → `false`. The branch disappears.

</div>
<!--
There are other valid answers - e.g. `s.starts_with(char::is_uppercase)`
on stable, which is even shorter. Worth mentioning.
-->
