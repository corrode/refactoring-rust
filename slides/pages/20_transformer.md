---
layout: default
zoom: 0.85
---

# 20 · `apply_commands`

<div class="opacity-80 mb-4">
Apply a sequence of <code>Command</code>s to strings.
</div>

```rust
pub enum Command { Uppercase, Trim, Append(usize) }

pub fn apply_commands(input: Vec<(String, Command)>) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let item = input[i].clone();
        let s = item.0;
        let c = item.1;

        if let Command::Uppercase = c {
            result.push(s.to_uppercase());
        } else if let Command::Trim = c {
            result.push(s.trim().to_string());
        } else if let Command::Append(n) = c {
            let mut temp_string = s;
            for _ in 0..n {
                temp_string.push_str("bar");
            }
            result.push(temp_string);
        }
    }
    result
}
```

<div class="absolute top-25 right-12 text-sm opacity-60">
<code>cargo test --example 20_transformer</code>
</div>

<!--
Discoveries:
  - Index loop + `.clone()` when we already own the `Vec` - pure waste.
  - Chained `if let` on an enum should be `match`, with exhaustiveness checking.
  - Manual loop appending "bar" n times is `"bar".repeat(n)` or `s.push_str(&"bar".repeat(n))`.
  - "Map every element through a function" is `iter.map(...).collect()`.
-->

---

# 20 · Review

- We own <code>input</code> - why are we cloning each element?
- A chain of <code>if let</code> on the same enum… what's the better tool?
- Pushing <code>"bar"</code> in a loop, <code>n</code> times - does <code>str</code> have a helper for that?
- Build a <code>Vec</code> by walking another <code>Vec</code>: that's almost always <code>iter().map(…).collect()</code>.


---

# 20 · Possible solution

```rust
pub fn apply_commands(input: Vec<(String, Command)>) -> Vec<String> {
    input
        .into_iter()
        .map(|(s, cmd)| match cmd {
            Command::Uppercase => s.to_uppercase(),
            Command::Trim => s.trim().to_string(),
            Command::Append(n) => s + &"bar".repeat(n),
        })
        .collect()
}
```

<div class="mt-8 text-base opacity-80">

- `into_iter()` consumes the input - no clones.
- `match` on the enum is exhaustive: add a `Command` variant and the compiler complains here.
- `"bar".repeat(n)` replaces the inner loop. `String + &str` reuses the existing buffer.

</div>
<!--
Alternative for the `Append` arm: `let mut s = s; s.push_str(&"bar".repeat(n)); s`.
Both fine. The `s + &"bar".repeat(n)` form is shorter and reuses `s`'s allocation.
-->
