---
layout: default
---

# 02 · `describe`

<div class="opacity-80 mb-4">
Classify a number as "prime", "square", "nothing", or "something else".
</div>

```rust
pub fn describe(value: Option<u32>) -> &'static str {
    match value {
        Some(2) | Some(3) | Some(5) | Some(7) => "prime",
        Some(0) | Some(1) | Some(4) | Some(9) => "square",
        None => "nothing",
        _ => "something else",
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 02_better_match</code>
</div>

<!--
Tiny one. People usually spot:
  - `Some(2) | Some(3) | …` repeats `Some` - there's a tidier form.
  - Or-patterns nest: `Some(2 | 3 | 5 | 7)`.
  - The arms read like a table once the `Some` noise is gone.
-->

---

# 02 · Observations

- How many times does the word `Some` appear? Does it need to?
- Or-patterns can live *inside* a constructor - have you used that form?
- If you read the arms aloud, what's the actual information density?

---

# 02 · Possible solution

```rust
pub fn describe(value: Option<u32>) -> &'static str {
    match value {
        Some(2 | 3 | 5 | 7) => "prime",
        Some(0 | 1 | 4 | 9) => "square",
        None => "nothing",
        _ => "something else",
    }
}
```

<div class="mt-8 text-base opacity-80">

- Or-patterns nest inside `Some(_)` since Rust 1.53.
- The wildcard `_` still catches every other `Some(n)`.
- Same behaviour, half the noise.

</div>
