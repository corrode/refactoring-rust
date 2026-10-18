---
layout: default
---

# 05 · `truncate`

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
<code>cargo test --example 05_truncate_string</code>
</div>

<!--
The bug is hidden until you add a boundary test: `truncate("café", 4)` panics
because byte 4 is in the middle of `é`. `String::truncate` requires
a char boundary. The "fix" depends on the spec we actually want:
  - byte cap that respects char boundaries
  - or a char-count cap
-->

---

# 05 · Review

- Add `assert_eq!(truncate("café", 4), "caf")`. What happens, and why?
- What does the documentation for `String::truncate` say about char boundaries?
- Do you actually want "first N bytes" or "first N characters"?
- Is there a `str` method that tells you whether an index is safe to cut at?

```rust
#[test]
fn backs_up_to_a_character_boundary() {
    assert_eq!(truncate("café", 4), "caf");
}
```

<div class="mt-6 opacity-70">
Hint: look for <code>is_char_boundary</code>, or step through <code>chars()</code>.
</div>

---

# 05 · Possible solution

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
- Slicing at a character boundary does not panic and allocates only the result.
- `"café"` no longer crashes when the limit falls inside `é`.

</div>
<!--
Both contracts can make sense:
  - count characters with `s.chars().take(n).collect()`
  - cap bytes with `s.floor_char_boundary(max_bytes)` on Rust 1.91 and newer
The contract matters more than either implementation.
-->
