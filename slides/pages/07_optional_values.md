---
layout: default
---

# 07 · `sum_options`

<div class="opacity-80 mb-4">
Sum all <code>Some</code> values in the vector. Ignore <code>None</code> entries.
</div>

```rust
pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    for option in options {
        match option {
            Some(value) => sum += value,
            None => {}
        }
    }
    sum
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 07_optional_values</code>
</div>

<!--
Imperative accumulator + match-with-empty-arm. Discoveries:
  - `Option` is iterable - `flatten` on an iterator of options drops `None`.
  - `filter_map` is the explicit alternative.
  - `.sum()` removes the mutable accumulator.
-->

---

# 07 · Review

- Did you know `Option<T>` implements `IntoIterator`? What can you do with an iterator of iterators?
- The `None => {}` arm is doing nothing - what does that suggest about *filtering*?
- Once the `None`s are gone, the loop is just an addition.


---

# 07 · Possible solution

```rust
pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    options.into_iter().flatten().sum()
}
```

<div class="mt-8 text-base opacity-80">

- `Option<T>` is an iterator of 0 or 1 element; `flatten` drops the empties.
- `sum` infers the result type from the return type.
- No mutable state, no match.

</div>
<!--
Equivalent: `options.into_iter().filter_map(|x| x).sum()`.
The `flatten` version reads slightly better because it doesn't need a closure.
-->
