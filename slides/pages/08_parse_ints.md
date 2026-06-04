---
layout: default
---

# 08 · `parse_values`

<div class="opacity-80 mb-4">
Parse every string as <code>i32</code>. Return all parsed integers, or the first error.
</div>

```rust
pub fn parse_values(values: Vec<String>) -> Result<Vec<i32>, ParseIntError> {
    let mut result = Vec::new();
    for value in values {
        match value.parse::<i32>() {
            Ok(num) => result.push(num),
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 08_parse_ints</code>
</div>

<!--
The trick everyone enjoys discovering:
`Iterator<Item = Result<T, E>>` can be `collect`ed into
`Result<Vec<T>, E>` - short-circuits on the first error.
Also: `Vec<String>` -> `&[String]` (or `&[impl AsRef<str>]`) is a fair callout
but the headline is `collect`.
-->

---

# 08 · Review

- What does `collect` do when the items are `Result`s?
- Does the function need to *own* the `Vec<String>` to read from it?
- The early return on `Err` is doing the same thing `collect` does for free.

---
zoom: 1.2
---

# 08 · Possible solution

```rust
pub fn parse_values(values: &[String]) -> Result<Vec<i32>, ParseIntError> {
    values.iter().map(|v| v.parse()).collect()
}
```

<div class="mt-8 text-base opacity-80">

- `collect` is overloaded: from `Iterator<Item = Result<T, E>>` it builds `Result<Vec<T>, E>` and stops at the first `Err`.
- The return type drives type inference for `parse` and `collect`.
- Borrowed slice means callers don't have to give up ownership.

</div>
