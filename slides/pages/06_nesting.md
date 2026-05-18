---
layout: default
---

# 06 · `shout`

<div class="opacity-80 mb-4">
If the input is <code>Some</code> and non-empty after trimming, return it uppercased. Otherwise <code>None</code>.
</div>

```rust
pub fn shout(s: Option<&str>) -> Option<String> {
    if let Some(s) = s {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_uppercase());
        } else {
            return None;
        }
    } else {
        return None;
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 06_nesting</code>
</div>

<!--
Two nested `if` + three `return`s. Discoveries:
  - Both failure paths produce `None` - that's `and_then` / `filter`.
  - Trimming first, then filtering on non-empty is a chain.
  - No explicit `return`s needed.
-->

---

# 06 · Observations

- How many places does this function return `None`? Could they all be the *same* `None`?
- "Keep the value only if it satisfies a predicate" - what's that called on `Option`?
- The flow is: trim → reject empty → uppercase. Read top to bottom, can you make the code read the same way?


---

# 06 · Possible solution

```rust
pub fn shout(s: Option<&str>) -> Option<String> {
    s.map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_uppercase)
}
```

<div class="mt-8 text-base opacity-80">

- `map(str::trim)` transforms the inner string.
- `filter` turns the `Some("")` case into `None` declaratively.
- Final `map(str::to_uppercase)` allocates only when we actually have content.
- The big insight: all of these things are what you'd do on iterators, because `Option` is one!

</div>
