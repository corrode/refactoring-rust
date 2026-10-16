# `even_numbers`

In this exercise, the task is to return a sequence of even numbers below a given upper bound. For example, `evens(10)` should produce the sequence `[0, 2, 4, 6, 8]`.


## Code to refactor

```rust,editable
{{#include ../../../examples/03_even_numbers.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/03_even_numbers.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/03_even_numbers.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>Does <code>evens</code> need to allocate and fill a complete <code>Vec</code> before returning?</summary>

No, it can return a lazy iterator instead.
You can then consume values as you need them without storing the complete sequence first.

</details>

<details class="discussion-question">
<summary>What work happens if you only want the first three values?</summary>

The original function still checks every integer below `max` and stores every even value before returning.
With a lazy iterator, the caller can stop after three values without generating the unused suffix.

</details>

<details class="discussion-question">
<summary>Can a range replace the counter and loop condition?</summary>

Yes, `0..max` replaces all three steps: start at zero, keep going while `n < max`, and increment after each pass.
`0..max` expresses those same bounds, including the empty range when `max` is zero.

</details>

<details class="discussion-question">
<summary>Can the traversal generate only even values instead of testing every integer?</summary>

Yes, start at zero and advance by two.
`step_by(2)` yields every other value instead of testing and discarding odd candidates.

</details>

<details class="discussion-question">
<summary>Which parts of the contract do the tests establish: start, bound, order, or storage type?</summary>

The tests check the values and their order: `evens(10)` produces `[0, 2, 4, 6, 8]`, `evens(0)` is empty, and `evens(7)` includes 6.
These cases cover the start at zero, the exclusive upper bound, and ascending order.
The starter tests compare the result directly with a `Vec`, so they need to change when the function returns an iterator.
The expected values stay the same.

</details>

<details class="discussion-question">
<summary>What do callers gain and lose when the function returns an iterator?</summary>

You choose how many values to consume and whether to collect them.
But you lose the owned storage, indexing, and repeated traversal that a `Vec` provides.
To keep those capabilities, collect the iterator or change how you use the result.
Returning a `Vec` makes `evens` generate the complete result before returning.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/03_even_numbers.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/03_even_numbers.rs:tests}}
</script>

`0..max` has the required bounds and is empty when `max` is zero.
`step_by(2)` yields every other value, starting with zero.
This removes the manual counter and parity check.

The return type, `impl Iterator<Item = u32>`, hides the concrete `StepBy<Range<u32>>` type.
You can change the adapters later as long as you preserve the promised sequence and public trait guarantees.

Calling `evens` creates the iterator without generating the full sequence.
`take(3)` limits a later traversal to at most three values; it does not consume them by itself.
`sum` consumes the sequence without building a temporary collection.
A caller that needs owned storage can use `collect::<Vec<_>>()`.

The solution tests call `collect` before comparing a complete sequence and use `count` for the empty case.
Callers that rely on a `Vec` for indexing, storage, or repeated traversal also need to collect the iterator or change how they use the result.

Do the shared tests prove that the result is lazy?
No, so the solution adds `stays_lazy`: it uses `u32::MAX` as the bound and consumes only three values.
The test shows that you can stop after a small prefix instead of constructing the full result.
It does not measure allocations.

### Alternative approaches

If callers rely on an owned collection, you can remove the manual loop without changing the API:

```rust,editable
pub fn evens(max: u32) -> Vec<u32> {
    (0..max).step_by(2).collect()
}
```

Use this version when you need to preserve the `Vec` contract.
It removes the manual loop without making callers change.
It remains eager, so it does not satisfy this exercise's lazy-result requirement.

A lazy filter is also correct:

```rust,editable
pub fn evens(max: u32) -> impl Iterator<Item = u32> {
    (0..max).filter(|n| n % 2 == 0)
}
```

`filter` is useful when a predicate determines which values to keep.
Here the values are evenly spaced, so `step_by(2)` states the rule directly without a parity test for each candidate.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Lazy `step_by` | `step_by` → `Vec` | Lazy `filter` |
|---|---|---|---|---|
| Type-safety | Evenness enforced by implementation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Evenness not encoded in type | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Evenness not encoded in type | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Evenness not encoded in type |
| Readability | Manual counter and parity check | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Direct stride, if adapter familiar | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Direct stride, if adapters familiar | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Explicit predicate, if adapters familiar |
| API design | Owned, indexable, reusable collection | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Meets lazy requirement; breaks callers | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Preserves collection API; remains eager | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Meets lazy requirement; breaks callers |
| Performance | Builds full result before returning | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Avoids unused suffix and collection | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Still eager; timing unmeasured | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Avoids unused suffix and collection |
| Flexibility | Composition after full generation | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Composition before generation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Composition after full generation | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Composition before generation |
| Error reporting | Every bound defines a sequence | N/A | N/A | N/A |

Both lazy variants give up direct indexing and repeated traversal; callers needing those must collect.
The `Vec` alternative preserves those capabilities but does not meet the exercise's lazy-result requirement.
`filter` tests candidate parity, whereas `step_by` expresses the stride directly.
Avoiding unused work helps partial consumers; fully collecting still allocates, and timing differences need measurement.

</details>

## Takeaways For Our Own Codebase

> - Check the call sites before changing a collection return type: do they need owned storage, or do they consume each value only once?
> - Try `take`, `map`, `sum`, or direct iteration where callers don't need a collection.
> - Identify callers that still need `collect` and decide whether moving that choice to them helps.
> - Is a helper named `evens` clearer than writing `(0..max).step_by(2)` at each call site?
> - Can you separate the loop cleanup from the API change so reviewers can assess each on its own?

## References

- [Half-open ranges](../appendix/concept-index.md#half-open-ranges): `0..max` excludes the upper bound and starts at zero unless the range is empty.
- [Lazy iterators](../appendix/concept-index.md#lazy-iterators): produce values as consumers request them instead of storing the full result first.
- [Return-position `impl Trait`](../appendix/concept-index.md#return-position-impl-trait): promise iterator behaviour while keeping the concrete adapter type private.
- [`Iterator::step_by`](../appendix/concept-index.md#iterator-step-by): traverse a sequence at a fixed stride.
- [Iterator consumers](../appendix/concept-index.md#iterator-consumers): `sum` and `collect` consume an iterator; adapters such as `take` limit later consumption.
