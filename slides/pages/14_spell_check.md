---
layout: default
---

# 14 · `spell_check`

<div class="opacity-80 mb-4">
Return the words from <code>words</code> that are not in <code>dict</code>
(case-insensitive).
</div>

```rust
pub fn spell_check(words: &Vec<String>, dict: &Vec<String>) -> Vec<String> {
    let mut misspelled = Vec::new();
    for word in words {
        let mut found = false;
        for d in dict {
            if d.to_lowercase() == word.to_lowercase() {
                found = true;
                break;
            }
        }
        if !found {
            misspelled.push(word.clone());
        }
    }
    misspelled
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 14_spell_check</code>
</div>

<!--
Discoveries:
  - This is O(n·m) with a hidden allocation on every comparison (`to_lowercase`).
  - A `HashSet` lookup is O(1); build it once.
  - `&Vec<T>` should almost always be `&[T]`.
  - We clone every output; we could borrow with a lifetime instead.
  - ASCII casing is a separate contract, not a substitute for Unicode lowercase comparison.
-->

---

# 14 · Review

- What's the time complexity right now? What changes if <code>dict</code> were a <code>HashSet</code>?
- <code>to_lowercase()</code> is called inside the inner loop - for the same word, every time. Why?
- <code>&Vec&lt;T&gt;</code> vs <code>&[T]</code> - does the function need <code>Vec</code> capabilities?
- Do we need to <code>clone</code> the output strings, or could we return borrows?

<div class="mt-12 opacity-70">
Hint: build the lookup set once, outside.
</div>

---

# 14 · Possible solution

```rust
use std::collections::HashSet;

pub fn spell_check(words: &Vec<String>, dict: &Vec<String>) -> Vec<String> {
    let dict: HashSet<String> = dict.iter().map(|w| w.to_lowercase()).collect();
    words.iter()
        .filter(|word| !dict.contains(&word.to_lowercase()))
        .cloned()
        .collect()
}
```

<div class="mt-8 text-base opacity-80">

- Build the lookup set once per call, not once per comparison.
- Keep Unicode lowercase comparison, output spelling, and order.
- Keep `&Vec<String>` and owned results for existing callers.

</div>

<!--
This compares Unicode lowercase strings, not full Unicode case folding.
The unusual &Vec arguments deliberately preserve the starter's API.
-->

---

# 14 · Alternative: reusable ASCII dictionary

```rust
use std::collections::HashSet;

pub fn spell_check<'a>(words: &[&'a str], dict: &HashSet<String>) -> Vec<&'a str> {
    words
        .iter()
        .copied()
        .filter(|w| !dict.contains(&w.to_ascii_lowercase()))
        .collect()
}
```

<div class="mt-8 text-base opacity-80">

- The caller builds the `HashSet` once (lowercased) and reuses it - O(n) lookups.
- `&[&str]` and `Vec<&'a str>` avoid the per-word `clone`.
- ASCII-only casing changes non-ASCII behavior; the dictionary must already be ASCII-lowercased.

</div>
<!--
The big win is moving the "build the lookup table" cost out of the function.
That's a recurring API design lesson: take the data structure that fits the
algorithm, not the one the caller happens to have.
-->
