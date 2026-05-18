---
layout: default
---

# 04 · `truncate`

<div class="opacity-80 mb-4">
Truncate a string to at most <code>max_bytes</code> bytes.
</div>

```rust
pub fn truncate(s: &str, max_bytes: usize) -> String {
    let mut owned = s.to_string();
    owned.truncate(max_bytes);
    owned
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 04_truncate_string</code>
</div>

<!--
The bug is hidden until you run main: `truncate("café", 4)` panics
because byte 4 is in the middle of `é`. `String::truncate` requires
a char boundary. The "fix" depends on the spec we actually want:
  - byte cap that respects char boundaries
  - or a char-count cap
-->

---

# 04 · Observations

- Try `cargo run --example 04_truncate_string` - what happens, and why?
- What does the doc of `String::truncate` say about char boundaries?
- Do you actually want "first N bytes" or "first N characters"?
- Is there a `str` method that tells you whether an index is safe to cut at?

<div class="mt-10 opacity-70">
Hint: look for <code>is_char_boundary</code>, or step through <code>chars()</code>.
</div>

---

# 04 · Possible solution

```rust
pub fn truncate(s: &str, max_bytes: usize) -> String {
    if max_bytes >= s.len() {
        return s.to_string();
    }
    let mut end = max_bytes;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    s[..end].to_string()
}
```

<div class="mt-6 text-base opacity-80">

- `is_char_boundary` lets us back up to the nearest safe cut.
- Slicing a `&str` at a boundary is panic-free and allocation-cheap.
- No more mid-codepoint crash on `"café"`.

</div>
<!--
Alternative interpretations are equally valid:
  - char-count semantics: `s.chars().take(n).collect()`
  - nightly: `s.floor_char_boundary(max_bytes)`
Mention both so people see the spec ambiguity is the real lesson.
-->
