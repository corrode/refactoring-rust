---
layout: default
---

# 05 · `is_rust_source_file`

<div class="opacity-80 mb-4">
Does this path point to a Rust source file?
</div>

```rust
pub fn is_rust_source_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(s) => s == "rs",
            None => false,
        },
        None => false,
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 05_path</code>
</div>

<!--
Pyramid of nested `match` on `Option`. Discoveries:
  - Both `None` arms collapse to `false` - that's `unwrap_or(false)` territory.
  - `Option::is_some_and` / `map_or` flatten it nicely.
  - Or: just compare against `OsStr::new("rs")` and skip `to_str` entirely.
-->

---

# 05 · Review

- Both `None` branches do the same thing - what does that suggest?
- What's `Path::extension()`'s return type, and what helpers does `Option` give you?
- Do you actually need `to_str`? `OsStr` has its own `==`.
- The shape is "if some, check predicate, else false" - there's a one-liner for that.


---

# 05 · Possible solution

```rust
pub fn is_rust_source_file(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "rs")
}
```

<div class="mt-8 text-base opacity-80">

- `is_some_and` collapses both `None` arms into a single `false`.
- `OsStr` implements `PartialEq<str>`, so no `to_str` dance is required.
- One line, no nesting, same semantics.

</div>
<!--
Equally fine: `path.extension().map_or(false, |e| e == "rs")`.
-->
