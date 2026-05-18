---
layout: default
zoom: 0.9
---


# 23 · `fun_strings`

<div class="opacity-80 mb-4">
A bunch of little string helpers. Notice how call sites read inside-out:
<code>sparkle(&shout(s.trim()))</code>.
</div>

```rust
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn sparkle(s: &str) -> String {
    format!("✨ {} ✨", s)
}

pub fn spongebob_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            out.extend(c.to_uppercase());
        } else {
            out.extend(c.to_lowercase());
        }
    }
    out
}

// call site:
// let banner = sparkle(&shout(s.trim()));
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 23_fun_strings_ext</code>
</div>

<!--
Discoveries:
  - Call sites nest right-to-left, which is hard to read.
  - Rust's method-chaining gives left-to-right flow - but we can't add inherent
    methods to `str`.
  - Extension trait pattern: define a trait, impl it for `str`.
  - Orphan rule: we own the trait, so we can impl for any type (including foreign ones).
-->

---

# 23 · Observations

- Why does <code>sparkle(&shout(s.trim()))</code> read backwards?
- Method calls chain left-to-right. Could these *be* methods on <code>str</code>?
- We can't add inherent methods to a foreign type - what's the workaround?
- What does the orphan rule allow us to do here?

---
zoom: 0.85
---

# 23 · Possible solution

```rust
pub trait FunStr {
    fn shout(&self) -> String;
    fn sparkle(&self) -> String;
    fn spongebob_case(&self) -> String;
}

impl FunStr for str {
    fn shout(&self) -> String { self.to_uppercase() }
    fn sparkle(&self) -> String { format!("✨ {} ✨", self) }
    fn spongebob_case(&self) -> String {
        self.chars()
            .enumerate()
            .map(|(i, c)| if i % 2 == 0 { c.to_ascii_uppercase() } else { c.to_ascii_lowercase() })
            .collect()
    }
}

// Now can write this! 
// let banner = "hello".trim().shout().sparkle();
```

<div class="mt-8 text-base opacity-80">

- An extension trait lets us add methods to a foreign type (`str`) because we own the trait.
- Call sites flow in reading order: `s.trim().shout().sparkle()`.
- `spongebob_case` becomes a single iterator pipeline - no manual buffer.

</div>
<!--
Orphan rule recap: you can `impl Trait for Type` if either the trait or the
type is local to your crate. Here the trait is ours, so impl-ing for `str`
is allowed. This pattern shows up everywhere - `itertools::Itertools`,
`anyhow::Context`, `tokio::AsyncReadExt`, etc.
-->
