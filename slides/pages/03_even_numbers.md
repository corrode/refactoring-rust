---
layout: default
---

# 03 · `evens`

<div class="opacity-80 text-xs mb-4">
This works. Make it nicer.
</div>

```rust
pub fn evens(max: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut n = 0;
    while n < max {
        if n % 2 == 0 {
            result.push(n);
        }
        n += 1;
    }
    result
}
```

<div class="absolute top-25 right-12 text-sm opacity-60">
<code>cargo test --example 03_even_numbers</code>
</div>

<!--
Smells:
  - Manual `while` + counter instead of a range.
  - `% 2 == 0` to recover a pattern we could express directly.
  - Eagerly builds a `Vec`; the caller can't stop early.
The arc: return an iterator, say what we mean with `step_by`.
-->

---

# 03 · Review

- `evens` allocates a whole `Vec` up front. What if the caller only wants the first few?
- `n % 2 == 0` filters for evens - is there a way to *step* over them directly?
- What does returning `impl Iterator` buy the caller over a `Vec`?

---

# 03 · Possible solution

```rust
pub fn evens(max: u32) -> impl Iterator<Item = u32> {
    (0..max).step_by(2)
}

// Lazy and allocation-free:
// let first_three: Vec<u32> = evens(u32::MAX).take(3).collect();
```

<div class="mt-6 text-base opacity-80">

- `impl Iterator<Item = u32>` returns the iterator without allocating.
- `step_by(2)` says exactly what we mean: walk in steps of two.
- Callers stay in control: `take`, `map`, `sum`, or `collect` as they like.

</div>

<!--
A first pass might keep the filter:
  (0..max).filter(|n| n % 2 == 0)
That's already lazy; `step_by` then removes the predicate entirely.
-->
