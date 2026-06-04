---
layout: default
zoom: 0.8
---

# 17 · `trim_log_line`

<div class="opacity-80 mb-4">
Trim outer whitespace, collapse internal runs of whitespace, strip a leading
<code>[INFO]</code> / <code>[WARN]</code> / <code>[ERROR]</code> tag.
</div>

```rust
pub fn trim_log_line(line: &str) -> String {
    let mut s = String::new();
    let mut started = false;
    for c in line.chars() {
        if !started && c.is_whitespace() { continue; }
        started = true;
        s.push(c);
    }
    while s.ends_with(' ') || s.ends_with('\t') || s.ends_with('\n') { s.pop(); }
    for tag in &["[INFO]", "[WARN]", "[ERROR]"] {
        if s.starts_with(tag) {
            s = s[tag.len()..].to_string();
            if s.starts_with(' ') { s = s[1..].to_string(); }
            break;
        }
    }
    // ... manual whitespace-collapse loop ...
    let collapsed = s.replace("\t", " ").replace("\n", " ");
    let mut out = String::new();
    let mut prev_space = false;
    for c in collapsed.chars() {
        if c == ' ' { if !prev_space { out.push(' '); } prev_space = true; }
        else { out.push(c); prev_space = false; }
    }
    out
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 17_trim_log_line</code>
</div>

<!--
Trimmed the body slightly to fit (kept all the structure, condensed braces).
Discoveries:
  - The first two loops are just `.trim()`.
  - The tag-stripping loop is `strip_prefix` in a fold.
  - The whitespace-collapse machinery is `split_whitespace().collect::<Vec<_>>().join(" ")`.
-->

---

# 17 · Review

- The first 10 lines hand-roll a function that already lives on `str`. Which one?
- For "does this string start with X, and if so what's after X?" - what's the standard idiom?
- `split_whitespace()` already knows what whitespace is. Could it replace the collapse loop?
- After the rewrite, how many lines are left?

<div class="mt-12 opacity-70">
Hint: <code>trim</code>, <code>strip_prefix</code>, <code>split_whitespace</code>.
</div>

---

# 17 · Possible solution

```rust
pub fn trim_log_line(line: &str) -> String {
    let trimmed = line.trim();
    let without_tag = ["[INFO]", "[WARN]", "[ERROR]"]
        .iter()
        .find_map(|tag| trimmed.strip_prefix(tag))
        .unwrap_or(trimmed);
    without_tag.split_whitespace().collect::<Vec<_>>().join(" ")
}
```

<div class="mt-8 text-base opacity-80">

- `trim()` replaces both the start-skipping loop and the end-popping loop.
- `strip_prefix` returns `Option<&str>` - exactly the "matched and here's the rest" shape.
- `split_whitespace()` treats any run of whitespace as one separator; `join(" ")` puts them back with single spaces.

</div>
<!--
An `itertools::join` or `intersperse` variant works too, but std-only is fine
here. Worth noting that this version produces no intermediate `String` until
the final `collect` + `join`.
-->
