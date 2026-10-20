---
layout: default
---

# 19 · `list_sum`

Sum the values in this owned list. Keep the signature, type, and behavior.

```rust
pub enum List {
    Item(i32, Box<List>),
    End,
}

pub fn sum(list: List) -> i32 {
    let mut sum = 0;
    let mut curr = list;
    loop {
        match curr {
            List::End => return sum,
            List::Item(i, list) => {
                sum += i;
                curr = *list;
            }
        }
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 19_list_sum</code>
</div>

---

# 19 · Review

- Why does the recursive enum need `Box<List>`?
- What moves when the match takes an item apart?
- Does `curr = *list` clone anything?
- Which pattern keeps the loop running? Which one stops it?
- If we use recursion instead, what changes besides the syntax?

<!--
Box gives List a finite size by storing the next node behind an owning pointer.
Matching the owned enum moves out the box. Dereferencing that owned Box moves
its List out, without cloning or allocating a replacement chain.
Only Item repeats. End stops. Ask about addition order and call-stack growth
before presenting recursion as a possible implementation.
-->

---

# 19 · Preserve behavior with `while let`

```rust
pub fn sum(list: List) -> i32 {
    let mut sum = 0;
    let mut curr = list;
    while let List::Item(value, next) = curr {
        sum += value;
        curr = *next;
    }
    sum
}
```

- Repeat while the pattern matches; `End` exits the loop.
- Same `List`, same signature, same head-to-tail addition order.
- Move the next node out of its box. No clone or new allocation.
- O(n) work; constant call stack during normal traversal.

<!--
The enum is unchanged from the starter.
Normal traversal consumes and releases each Box individually. Dropping an
unconsumed list can still recurse through its tail. If addition panics,
unwinding can recursively drop a long remainder. Constant traversal stack
is not a guarantee about every destruction path.
-->

---

# 19 · Alternative: recursive match

```rust
pub fn sum(list: List) -> i32 {
    match list {
        List::End => 0,
        List::Item(value, next) => value + sum(*next),
    }
}
```

- Matches the equations: empty sum is zero; item sum is value plus tail sum.
- Same API, **different behavior**: tail additions finish first.
- O(n) work, O(n) call-stack requirement. Long lists can exhaust the stack.
- Not tail-recursive: addition remains after the call returns.
- Rust doesn't guarantee tail-call optimization.

<!--
This alternative is solutions/19_list_sum_alt.rs. It has its own tests,
including the overflow-order comparison on the next slide. It isn't the
behavior-preserving answer, and shorter code isn't categorically better.
Both versions consume their input without cloning list nodes.
-->

---

# 19 · Same total, different intermediate sums

With overflow checks enabled, let `MAX = i32::MAX`:

| Values, head to tail | Original / `while let` | Recursive match |
| --- | --- | --- |
| `[MAX, 1, -1]` | Panics at `MAX + 1` | Returns `MAX` |
| `[-1, MAX, 1]` | Returns `MAX` | Panics at `MAX + 1` |

- Loop: `((0 + a) + b) + c`. Recursion: `a + (b + (c + 0))`.
- Equivalence requires fitting intermediate sums in both orders and enough stack.
- Checks disabled: wrapping gives the same modular result, if execution completes.

**Discussion:** Is a small final total enough to justify recursion?
What bounds would you need on values and list length?

<!--
No: the first two rows have the same representable mathematical total, but
opposite panic behavior. For checked arithmetic, examine prefixes and suffixes,
not only the final total. Default debug and release profiles differ in overflow
checks, but custom flags can override them. The alternative tests assume defaults.
Neither addition order is always safer. Keep while let for this exercise's
original contract; use recursion only with justified arithmetic and depth bounds.
Normal iterative consumption has constant traversal stack, but dropping an
entire unconsumed chain or unwinding with a remainder can still recurse.
-->
