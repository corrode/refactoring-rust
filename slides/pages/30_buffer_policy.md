---
layout: default
zoom: 0.9
---

# 30 · `buffer_policy`

```rust
#[derive(Debug, Default, PartialEq)]
pub struct BufferConfig {
    pub exact_size: Option<usize>,
    pub max_size: Option<usize>,
}

impl BufferConfig {
    pub fn exact_size(&mut self, size: Option<usize>) {
        self.exact_size = size;
        self.max_size = None;
    }

    pub fn max_size(&mut self, max: usize) {
        assert!(max >= 8192, "maximum buffer size must be at least 8192");
        self.max_size = Some(max);
        self.exact_size = None;
    }
}
```

<div class="absolute bottom-4 left-12 text-xs opacity-60">
Inspired by <a href="https://github.com/hyperium/hyper/blob/73bff4e98c372ce04b006370c0b0d2af29ea8718/src/client/conn.rs#L616-L642">hyper's buffer configuration</a>.
</div>

<!--
Review the representation while preserving setter signatures and behaviour.
The complete starter and tests are in examples/30_buffer_policy.rs.
This extracts two private fields from hyper's connection builder, makes them
public for inspection, shortens names, and removes builder chaining and feature
gates. The panic message is workshop-specific. The upstream MIT attribution
and full notice are preserved in THIRD_PARTY_NOTICES.md.
The historical setters already clear the other option; do not present this
as an upstream bug. Let participants discover the relationship between fields.
-->

---

# 30 · Review

- Which configurations can the setters produce?
- What changes when you reverse setter order?
- Does `exact_size(None)` preserve a previous maximum?
- What must remain unchanged after an invalid maximum?

<!--
Starting from default, the setters produce three policies, never both fields set.
The last successful setter wins. exact_size(None) resets both options.
Zero is accepted as an exact size. The maximum must be at least 8192.
Validation happens before mutation, so a rejected maximum leaves the old state.
Ask whether these rules belong in individual assignments or the representation.
-->

---
zoom: 0.85
---

# 30 · Possible solution: a policy enum

```rust
#[derive(Debug, Default, PartialEq)]
pub enum BufferConfig {
    #[default]
    Adaptive,
    Exact(usize),
    AdaptiveWithMax(usize),
}

impl BufferConfig {
    pub fn exact_size(&mut self, size: Option<usize>) {
        *self = match size {
            Some(size) => Self::Exact(size),
            None => Self::Adaptive,
        };
    }

    pub fn max_size(&mut self, max: usize) {
        assert!(max >= 8192, "maximum buffer size must be at least 8192");
        *self = Self::AdaptiveWithMax(max);
    }
}
```

- One policy at a time; setter calls stay unchanged.
- Public variants still allow `AdaptiveWithMax(1)`.

<div class="absolute bottom-4 left-12 text-xs opacity-60">
Inspired by <a href="https://github.com/hyperium/hyper/blob/73bff4e98c372ce04b006370c0b0d2af29ea8718/src/client/conn.rs#L616-L642">hyper's buffer configuration</a>. Workshop solution.
</div>

<!--
The full implementation and tests are in solutions/30_buffer_policy.rs.
Field readers now match variants; struct literals must choose a policy.
Adaptive means no explicit size or maximum, not unlimited growth.
The enum enforces mutual exclusion, not numeric validity on every construction
path. A validated payload or private wrapper would be another API decision.
No allocation occurs here, and we make no speed or memory-layout claim.
Tests retain defaults, both setter orders, reset, zero, numeric boundaries,
and unchanged state after rejected maxima. No upstream refactor PR is claimed.
-->
