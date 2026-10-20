# `list_sum`

This function consumes a linked list and returns the sum of its values, or zero for an empty list.
Refactor it while keeping `sum(List) -> i32`, the `List` type, and the original behavior.
The first solution preserves those constraints; the alternative explores a different implementation with the same API.

## Code to refactor

```rust,editable
{{#include ../../../examples/19_list_sum.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/19_list_sum.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

Edit `examples/19_list_sum.rs` in the [workshop repository](https://github.com/corrode/refactoring-rust).
From the repository root:

```sh
cargo test --example 19_list_sum
```

</div>

## Discussion

<details class="discussion-question">
<summary>Why does <code>Item</code> contain a <code>Box&lt;List&gt;</code>?</summary>

`List` is a recursive enum: an item contains a value and the rest of the list, which is another `List`.
`End` marks the empty list.
Without indirection, storing a `List` inside itself would require space for another `List`, then another, with no finite size the compiler could assign.
`Box<List>` stores an owning pointer of known size; the next `List` lives on the heap.
The enum therefore has a finite size even though lists can have different lengths.

</details>

<details class="discussion-question">
<summary>Does <code>*list</code> copy the remaining list?</summary>

No.
The function owns its input, and matching an `Item` moves its box into the local binding named `list`.
Dereferencing that owned `Box<List>` in `curr = *list` moves the contained `List` out of the box.
It doesn't clone the remaining nodes or allocate a replacement list.
The box's allocation can then be released without dropping the moved-out contents.

The name `list` is doing two jobs here: the function parameter and the remaining list in the match arm.
Calling the latter `next` makes the traversal easier to follow.

</details>

<details class="discussion-question">
<summary>Which pattern keeps the loop running?</summary>

Only `List::Item` does more work.
`List::End` stops the traversal and returns the accumulator.
A `while let` loop expresses this directly: repeat the body while the current value matches `List::Item(value, next)`.
When it doesn't match, return the accumulated sum after the loop.
Unlike a one-time `if let`, `while let` tests the pattern again on every iteration.

</details>

<details class="discussion-question">
<summary>Would a recursive definition preserve the behavior?</summary>

The mathematical equations are appealing: the sum of `End` is zero, and the sum of an `Item` is its value plus the sum of the rest.
But `i32` addition isn't unbounded integer arithmetic.
The loop adds from head to tail; `value + sum(*next)` must obtain the tail's sum before completing the current addition.
That changes the intermediate sums and can change whether overflow checks panic.
Recursion also introduces a call-stack requirement that grows with the list's length.
The same signature doesn't guarantee the same behavior.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Repeat the item pattern with `while let`

```rust,editable
{{#include ../../../solutions/19_list_sum.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/19_list_sum.rs:tests}}
</script>

The loop condition now contains the pattern that allows traversal to continue.
Each iteration adds `value` before moving `*next` into `curr`, just as the original does.
`End` exits the loop, and the final expression returns `sum`.
For an empty list, the body never runs and the result stays zero.

The signature, ownership, and addition order are unchanged.
Existing callers and tests need no migration.
The accumulator is still mutable, but the separate `loop`, `match`, and early return no longer obscure the single repeating case.

### Alternative: follow the recursive definition

```rust,editable
{{#include ../../../solutions/19_list_sum_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/19_list_sum_alt.rs:tests}}
</script>

This match follows the two equations directly:

- `sum(End) = 0`
- `sum(Item(value, next)) = value + sum(*next)`

There is no explicit accumulator or cursor.
The recursive call takes ownership of the rest of the list, again without cloning it.
This can be a useful way to explain the data structure, but it is not a behavior-preserving replacement for the loop.
It keeps the same API, not the same arithmetic and resource behavior.

### Addition order and overflow

For three values `a`, `b`, and `c`, the loop computes `((0 + a) + b) + c`.
The recursive version computes `a + (b + (c + 0))`.
With overflow checks enabled, a representable final mathematical total doesn't guarantee that both calculations succeed.

Here `MAX` means `i32::MAX`, and the bracket notation lists values from head to tail:

| Input | Original and `while let` | Recursive match |
| --- | --- | --- |
| `[MAX, 1, -1]` | Panics at `MAX + 1` | Returns `MAX`: `1 + (-1)` is zero |
| `[-1, MAX, 1]` | Returns `MAX`: `-1 + MAX` fits | Panics at `MAX + 1` in the tail |

Neither order is categorically safer.
To justify equivalence by ordinary integer arithmetic, all relevant intermediate sums in both orders must fit in `i32`, and the recursive version must have enough stack.
A small final total alone is not enough.

When overflow checks are disabled, these additions wrap modulo 2³².
Both orders then produce the same final `i32` result, provided execution completes with enough stack.
Overflow checks are a compiler setting, not an inherent debug-versus-release distinction, though Cargo enables them in the default debug profile and disables them in the default release profile.
The attached overflow tests use `cfg(debug_assertions)` and assume those default settings; custom overflow flags can invalidate that assumption.
The recursive suite records the two behavioral differences rather than pretending it passes the original contract unchanged.

### Stack use and dropping the list

All three versions visit each node once and take O(n) time.
None allocates new list nodes or clones the input during summation.
The input already occupies O(n) heap space.

The original and `while let` versions use a constant call stack during normal traversal.
They consume each box, move its contents out, and release its allocation as they advance.
There isn't an intact chain of consumed nodes left to drop at the end.
The main solution's long-list test exercises that normal path.

The recursive version needs O(n) call-stack space unless an optimization removes it; don't rely on that optimization.
The addition still has to happen after `sum(*next)` returns, so this function isn't tail-recursive.
Rust doesn't guarantee tail-call optimization anyway.
A sufficiently long list can exhaust the stack.

Iterative traversal doesn't make every use of this type stack-safe.
Dropping an entire unconsumed list can recursively drop its boxed tail.
If addition panics while a loop still owns a long remainder, unwinding can drop that remainder recursively too.
For unbounded or untrusted list lengths, review destruction and failure paths as well as traversal; a successful long-list sum doesn't test them all.

### Trade-offs

Compare each implementation with the **Original** using the [six trade-off axes](../appendix/trade-off-axes.md).

<img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage; **N/A** means not applicable.

| Criterion | Original | `while let` | Recursive match |
| --- | --- | --- | --- |
| Type-safety | Owns nodes; `i32` can overflow | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same types and invariants | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same types; no length or sum bound |
| Readability | Explicit loop and two match arms | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Repeating pattern is the condition | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Direct equations; less visible evaluation cost |
| API design | `sum(List) -> i32`; consumes input | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same API and behavior | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Same API conceals changed behavior |
| Performance | O(n) work; constant traversal stack | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same work and traversal stack | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> O(n) work and call-stack requirement |
| Flexibility | Can extend either match arm | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Fits one repeating variant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Explicit cases; requires bounded depth |
| Error reporting | Overflow panic with checks; no `Result` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same panic conditions | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Changed panic conditions; stack exhaustion risk |

Readability depends on familiarity with pattern matching and recursion.
The recursive equations are shorter, but their arithmetic and stack caveats matter to a caller.
The performance row describes work and storage requirements, not a measured speed difference; constant traversal stack doesn't cover recursive destruction during unwinding.
The alternatives are not interchangeable contracts, even though all three accept the same arguments.

For this exercise's behavior-preserving refactor, I'd use `while let`.
Keep the recursive version as a comparison, or use it only where input bounds and arithmetic requirements justify its different behavior.

</details>

## Takeaways For Our Own Codebase

> Which loops repeat one enum pattern and stop on another?
> Try expressing the continuing case in the loop condition without changing operation order.
>
> Before replacing a loop with recursion, check intermediate arithmetic and maximum depth, not just the final mathematical result.
> Add tests that distinguish evaluation orders.
>
> For recursively owned data, inspect both normal traversal and destruction after early exits or panics.

## References

- [Recursive types with `Box<T>`](https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes) explains why indirection gives a recursive enum a finite size.
- [Conditional loops with `while let`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-pattern-loops) repeats a body while a pattern matches.
- [`Box<T>`](https://doc.rust-lang.org/std/boxed/index.html) describes heap ownership and moving values out of boxes.
- [Integer overflow](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow) documents checked arithmetic and compiler settings.
- [Destructors](https://doc.rust-lang.org/reference/destructors.html) describes how fields are dropped when owned values leave scope.
