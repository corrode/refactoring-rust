---
layout: default
zoom: 0.9
---

# 17 · `text_stats`

<div class="opacity-80 mb-3">
Compute the total word count, unique word count, and average word length.
</div>

```rust
pub fn text_stats(text: &str) -> (usize, usize, f64) {
    let counts = count_words(text);
    let total_words = counts.values().count();
    let unique_words = counts.iter()
        .filter(|&(_, &count)| count == 1).count();
    let total_length: usize = counts.keys()
        .map(|word| word.chars().count()).sum();

    (total_words, unique_words,
     total_length as f64 / total_words as f64)
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 17_text_stats</code>
</div>

<!--
The checked-in test uses only words that occur once, so this implementation is green.
Do not start by replacing methods. First ask what each number means and which part of
the map represents it.
-->

---

# 17 · Review

- What does a key in `counts` represent? What does its value represent?
- Does `values().count()` count words or map entries?
- Does `count == 1` mean “unique,” or “appears exactly once”?
- Should repeated words contribute once or repeatedly to the average?

Add a test that distinguishes the possible meanings:

```rust
let (total, unique, average) = text_stats("a a rust");
assert_eq!((total, unique), (3, 2));
assert!((average - 2.0).abs() < 0.001);
```

<!--
Let attendees predict all three values before running the new test. Verbose code is
a distraction here. The original test accepts three different aggregation policies.
-->

---
zoom: 0.9
---

# 17 · Possible solution

```rust
pub fn text_stats(text: &str) -> (usize, usize, f64) {
    let counts = count_words(text);
    let total_words: usize = counts.values().sum();
    let unique_words = counts.len();
    let total_characters: usize = counts.iter()
        .map(|(word, count)| word.chars().count() * count)
        .sum();
    let average = if total_words == 0 {
        0.0
    } else {
        total_characters as f64 / total_words as f64
    };

    (total_words, unique_words, average)
}
```

<div class="mt-6 text-base opacity-80">

- Values sum to the number of occurrences; keys count distinct words.
- Lengths are weighted by occurrence count.
- Empty input has an explicit result instead of producing `NaN`.

</div>

<!--
Empty input still needs a contract. We could return `Option<TextStats>`, make only
the average optional, or return an error. A named `TextStats` type is worth discussing
once the metrics are settled.
-->
