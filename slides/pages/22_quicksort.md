---
layout: default
---

# 22 · `quicksort` <span class="opacity-50 text-base">· bonus</span>

<div class="opacity-80 text-xs mb-4">
This works. Make it nicer.
</div>

```rust
pub fn quicksort(input: &[i32]) -> Vec<i32> {
    let mut values = input.to_vec();
    let len = values.len();
    if len > 1 {
        sort_range(&mut values, 0, len - 1);
    }
    values
}

fn sort_range(values: &mut [i32], lo: usize, hi: usize) {
    if lo >= hi { return; }
    let pivot = values[hi];
    let (mut i, mut j) = (lo, lo);
    while j < hi {
        if values[j] <= pivot { values.swap(i, j); i += 1; }
        j += 1;
    }
    values.swap(i, hi);
    if i > 0 { sort_range(values, lo, i - 1); }
    sort_range(values, i + 1, hi);
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 22_quicksort</code>
</div>

<!--
Lomuto partition with manual index juggling. Correct, but the algorithm
is buried. Ask the audience to make it read like the textbook definition.
-->

---

# 22 · Review

- Quicksort is "pick a pivot, recurse on the smaller, then the larger." How much of that survives in the code?
- Slice patterns can name the head and the tail at once. What would `[pivot, rest @ ..]` give us?
- Is there a single call that splits the rest into "<= pivot" and "> pivot"?
- What are we trading away by allocating instead of sorting in place?

---
zoom: 0.95
---

# 22 · Possible solution

```rust
pub fn quicksort(input: &[i32]) -> Vec<i32> {
    let [pivot, rest @ ..] = input else {
        return Vec::new();
    };
    let (less, greater): (Vec<i32>, Vec<i32>) =
        rest.iter().partition(|&&x| x <= *pivot);

    let mut sorted = quicksort(&less);
    sorted.push(*pivot);
    sorted.extend(quicksort(&greater));
    sorted
}
```

<div class="mt-4 text-base opacity-80">

- Slice pattern + `let ... else` names the pivot and folds in the base case.
- `partition` does the split in one pass; the recursion reads like the definition.
- Clearest, not fastest: for real work, `slice::sort_unstable`.

</div>
