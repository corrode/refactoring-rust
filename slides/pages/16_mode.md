---
layout: default
---

# 16 · `most_common`

<div class="opacity-80 mb-4">
Find the value that occurs most often in the input.
</div>

```rust
pub fn most_common(numbers: &[i32]) -> i32 {
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for &x in numbers {
        *occurrences.entry(x).or_insert(0) += 1;
    }

    let mut best: (i32, i32) = (
        *occurrences.iter().next().expect("empty input").0,
        *occurrences.iter().next().expect("empty input").1,
    );
    for (&value, &count) in &occurrences {
        if count > best.1 {
            best = (value, count);
        }
    }
    best.0
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 16_mode</code>
</div>

<!--
The counting half is already idiomatic (`entry().or_insert(0)`).
The "find the maximum" half re-implements `Iterator::max_by_key` by hand,
and calls `iter().next()` twice to seed `best`.
-->

---

# 16 · Review

- The first loop is already idiomatic - focus on the second one.
- "Iterate, pick the item with the highest something" - there's an iterator method for that.
- Why call `iter().next()` twice to seed `best`?
- After replacing the second loop, does `most_common` even need the temporary `best`?

---

# 16 · Possible solution

```rust
pub fn most_common(numbers: &[i32]) -> i32 {
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for &x in numbers {
        *occurrences.entry(x).or_insert(0) += 1;
    }
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .expect("empty input")
}
```

<div class="mt-6 text-base opacity-80">

- `HashMap::entry(...).or_insert(0)` is the canonical counting idiom.
- `max_by_key` replaces the hand-rolled tracking - no temporaries, no double seeding.
- The empty-input case is now a single, honest `expect`.
- Ideally, you'd return an `Option` and let the caller decide what to do. 

</div>
