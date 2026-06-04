---
layout: default
---

# 19 · `Fibonacci`

<div class="opacity-80 text-xs mb-4">
This works. Make it nicer.
</div>

```rust
pub fn fib(n: u32) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    return fib(n - 1) + fib(n - 2);
}

pub fn first_n(n: usize) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        result.push(fib(i as u32));
        i = i + 1;
    }
    return result;
}
```

<div class="absolute top-25 right-12 text-sm opacity-60">
<code>cargo test --example 19_iterators</code>
</div>

<!--
Lots of beginner smells in one place:
  - Exponential recursion (fib(40) is already painful).
  - Manual `while` with `i = i + 1` instead of a `for` / iterator.
  - `return` on the last expression.
  - Eagerly building a `Vec` when a lazy producer would do.
The arc: spot the smells -> reach for `Iterator`.
-->

---

# 19 · Review

- `fib(n)` recomputes the same subproblems over and over - what's the complexity?
- `first_n` allocates a whole `Vec` up front. What if the caller only wants the first few?
- That `while` + counter pattern - is there a more Rust-y way to produce a sequence?
- If we had an `Iterator`, what would we get for free?

---
zoom: 0.85
---

# 19 · Possible solution

```rust
struct Fibonacci { current: u64, next: u64 }

impl Fibonacci {
    fn new() -> Self { Self { current: 0, next: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

// Same result, lazy and O(n):
// let fib: Vec<u64> = Fibonacci::new().take(10).collect();
```

<div class="mt-6 text-base opacity-80">

- One `struct` holds the state; `next` advances it in O(1).
- Implementing `Iterator` unlocks `take`, `map`, `zip`, `collect`, …
- For overflow safety, swap `+` for `checked_add` and return `None`.

</div>
<!--
A tuple-destructure form is even tidier on recent Rust:
  (self.current, self.next) = (self.next, self.current + self.next);
-->
