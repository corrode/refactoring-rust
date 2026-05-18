---
layout: default
---

# 15 · `parse_port`

<div class="opacity-80 mb-4">
Parse a TCP port string into a <code>u16</code>.
</div>

```rust
pub fn parse_port(s: &str) -> Option<u16> {
    let n: i32 = s.parse().ok()?;
    if n < 1 || n > 65535 {
        return None;
    }
    Some(n as u16)
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 15_parse_port</code>
</div>

<!--
Discoveries:
  - `Option` throws away the reason. Why did it fail? Bad number? Out of range? Zero?
  - Parsing as `i32` and then bounds-checking is doing the type system's job by hand.
  - If we parsed straight into `u16`, the upper bound is free. Only "not zero" is left.
  - `NonZeroU16` encodes the remaining invariant in the type.
-->

---

# 15 · Observations

- The caller can't tell `"abc"` apart from `"99999"` apart from `"0"`. Should they be able to?
- Why parse into `i32` when the target is `u16`? What does that buy us?
- If `u16` already rules out values above 65535, what's the *only* remaining invariant?
- Is there a type in `std` that says "a `u16` that isn't zero"?

<div class="mt-12 opacity-70">
Hint: "Parse, don't validate": push invariants into the type.
</div>

---
zoom: 0.95
---

# 15 · Possible solution

```rust
use std::num::NonZeroU16;

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidPort {
    Invalid,
    Zero,
}

pub fn parse_port(s: &str) -> Result<NonZeroU16, InvalidPort> {
    let n: u16 = s.parse().map_err(|_| InvalidPort::Invalid)?;
    NonZeroU16::new(n).ok_or(InvalidPort::Zero)
}
```

<div class="mt-8 text-base opacity-80">

- Parsing directly as `u16` removes the manual upper-bound check.
- `NonZeroU16` makes "port 0" unrepresentable in the success type.
- A small `enum` tells the caller *why* parsing failed instead of swallowing it.
- Two variants (`Invalid` / `Zero`) keep the API tight; you could split `Invalid`
  into `NotANumber` / `OutOfRange` if callers actually need to distinguish.

</div>
<!--
Name-drop Alexis King's "Parse, don't validate" here. The transformation pattern
is: take a stringly-typed value, produce a value whose type proves the invariants.
Callers can no longer forget to check the port is non-zero - the type does it.
-->
