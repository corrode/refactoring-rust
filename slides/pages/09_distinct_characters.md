---
layout: default
---

# 09 · `count_distinct`

<div class="opacity-80 mb-4">
Count the number of distinct characters in a string.
</div>

```rust
pub fn count_distinct(s: &str) -> usize {
    let mut seen: Vec<char> = Vec::new();
    for c in s.chars() {
        if !seen.contains(&c) {
            seen.push(c);
        }
    }
    seen.len()
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 09_distinct_characters</code>
</div>

<!--
Two things to discover:
  - `Vec::contains` is O(n) per lookup -> O(n²) overall. A set is O(1).
  - Once a `HashSet` is in play, the whole loop is `chars().collect::<HashSet<_>>().len()`.
-->

---

# 09 · Observations

- What's the complexity of `Vec::contains` inside a loop?
- What collection answers "have I seen this before?" in constant time?
- Can you `collect` an iterator of `char` straight into that collection?


---

# 09 · Possible solution

```rust
use std::collections::HashSet;

pub fn count_distinct(s: &str) -> usize {
    s.chars().collect::<HashSet<_>>().len()
}
```

<div class="mt-8 text-base opacity-80">

- A `HashSet` deduplicates automatically and gives O(1) membership.
- `collect` builds it directly from the `chars()` iterator.
- Complexity goes from O(n²) to O(n); the code is shorter too.

</div>
