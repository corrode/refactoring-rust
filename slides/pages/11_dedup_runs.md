---
layout: default
---

# 11 · `dedup_runs` <span class="opacity-50 text-base">· bonus</span>

<div class="opacity-80 text-xs mb-4">
This works. Make it nicer.
</div>

```rust
pub fn dedup_runs(values: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < values.len() {
        if result.is_empty() || *result.last().unwrap() != values[i] {
            result.push(values[i]);
        }
        i += 1;
    }
    result
}
// [1, 2, 4, 5, 5, 5, 5, 25, 5] -> [1, 2, 4, 5, 25, 5]
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 11_dedup_runs</code>
</div>

<!--
Only adjacent duplicates collapse; the trailing 5 survives.
The arc: this is exactly what `Vec::dedup` does.
-->

---

# 11 · Review

- What does the standard library call "collapse consecutive equal elements"?
- The manual loop tracks `result.last()`. Does anything already do that for us?
- Would the answer change if we only had a `&[i32]` instead of an owned `Vec`?

---

# 11 · Possible solution

```rust
pub fn dedup_runs(mut values: Vec<i32>) -> Vec<i32> {
    values.dedup();
    values
}
```

<div class="mt-6 text-base opacity-80">

- `Vec::dedup` removes consecutive equal elements in place.
- It touches only *adjacent* duplicates, which is exactly the spec.
- Lazy, iterator-based equivalent: `Itertools::dedup`.

</div>

<!--
`dedup_by` / `dedup_by_key` cover the "equal enough" variations.
-->
