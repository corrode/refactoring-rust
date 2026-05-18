---
layout: default
---

# 05 · `increment`

<div class="opacity-80 mb-4">
Add one to an <code>Option&lt;u32&gt;</code>. Propagate <code>None</code>.
</div>

```rust
pub fn increment(n: Option<u32>) -> Option<u32> {
    let x = match n {
        Some(x) => x,
        None => return None,
    };
    Some(x + 1)
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 05_let_else</code>
</div>

<!--
Two natural answers:
  - `Option::map` is the idiomatic combinator.
  - `let Some(x) = n else { return None }` is the language feature
    the file's name hints at - useful for any "unwrap-or-bail" shape.
-->

---

# 05 · Observations

- The whole function does one thing: apply `+1` inside the `Option`. Is there a combinator for that?
- The match is `Some(x) => x, None => return None`. That pattern has a dedicated syntax now.
- Which form do you prefer when the body is one expression vs. many statements?


---

# 05 · Possible solution

```rust
pub fn increment(n: Option<u32>) -> Option<u32> {
    n.map(|x| x + 1)
}
```

<div class="mt-8 text-base opacity-80">

- `Option::map` is exactly "transform the inner value, keep `None` as `None`".
- No early return, no temporary, no match.
- Alternatively, use `let … else` when the rest of the body is long.

</div>
