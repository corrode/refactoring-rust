---
layout: default
---

# 11 · `highest_and_lowest`

<div class="opacity-80 mb-4">
Given a string of space-separated integers, return the highest and lowest value.
</div>

```rust
pub fn highest_and_lowest(input: &str) -> (i64, i64) {
    let parts: Vec<&str> = input.split(' ').collect();
    let mut max: i64 = parts[0].parse().unwrap();
    let mut min: i64 = parts[0].parse().unwrap();
    for part in &parts[1..] {
        let n: i64 = part.parse().unwrap();
        if n > max { max = n; }
        if n < min { min = n; }
    }
    (max, min)
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 11_highest_and_lowest</code>
</div>

<!--
Three things to notice:
  - `split(' ')` vs `split_whitespace()` - what about double spaces, tabs?
  - `unwrap()` everywhere - what happens on empty input or non-numeric tokens?
  - The min/max bookkeeping is a textbook `fold`.
Solution choice: keep the (i64, i64) signature and use a single fold.
A Result-returning version is just as valid; mention it.
-->

---

# 11 · Observations

- What happens with `highest_and_lowest("")`? Or `"7  3"` (two spaces)? Or `"oops"`?
- Notice the parse runs three times on `parts[0]` - does it need to?
- The loop tracks `(max, min)` together. What iterator method threads a running value through?
- Should this really return `(i64, i64)`, or admit failure with a `Result`?


---

# 11 · Possible solution

```rust
pub fn highest_and_lowest(input: &str) -> (i64, i64) {
    let mut nums = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let first = nums.next().expect("at least one number");
    nums.fold((first, first), |(max, min), n| {
        (max.max(n), min.min(n))
    })
}
```

<div class="mt-6 text-base opacity-80">

- `split_whitespace` handles tabs and runs of spaces.
- Pull the first number to seed both `max` and `min`, then `fold` the rest.
- `i64::max` / `i64::min` replace the two `if` statements.

</div>
<!--
A `Result<(i64, i64), _>` signature is the honest version:
the panics become `?`-friendly errors and empty input is a real case.
Show the simpler one here, mention the Result variant verbally.
-->
