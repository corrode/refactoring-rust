# `quicksort`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

> **BONUS.** Optional exercise.
> Skip it if you're short on time; nothing later in the book depends on it.

- Sort integers into ascending order while leaving the supplied slice unchanged.
- Keep `quicksort(input: &[i32]) -> Vec<i32>` for the first solution.
- Preserve every element, including duplicates, and consider other interfaces only as alternatives.

## Code to refactor

- Starter and its tests: `examples/25_quicksort.rs`.
- Workshop prompts and speaker notes: `slides/pages/25_quicksort.md`.
- `ideas.md` records the original goal of a more declarative, Haskell-like presentation of quicksort.
- The public function copies the input into a vector before sorting it.
- `sort_range` uses inclusive bounds and the last element as a pivot.
- A Lomuto partition moves values less than or equal to the pivot to the left, then places the pivot between the partitions.
- Recursive calls sort the ranges on either side of that pivot.
- Distinguish mutation of the private copy from mutation of the caller's slice.

## Discussion

<details class="discussion-question">
<summary>Which index guards are doing real work?</summary>

- The outer `len > 1` check avoids computing `len - 1` for an empty slice.
- The helper returns when `lo >= hi`, before reading a pivot from an empty or singleton range.
- The scan stops before `hi`, keeping the pivot outside the scanned section until the final swap.
- `i > 0` prevents unsigned underflow in the left recursion's `i - 1` expression.
- The right recursion may begin past the final element, but its base-case check returns before indexing.
- These guards make the current algorithm work; do not invent an indexing bug to justify a rewrite.

</details>

<details class="discussion-question">
<summary>What guarantees that recursive partitions get smaller?</summary>

- Set aside exactly one pivot before partitioning the remaining values.
- Every remaining value belongs to either the less-than-or-equal partition or the greater partition.
- Equal values must remain in the result, not disappear with a strict-less/strict-greater split.
- The pivot's exclusion guarantees smaller recursive inputs even when every value is equal.
- Smaller does not mean balanced: a partition of size n minus one still creates a long recursion chain.

</details>

<details class="discussion-question">
<summary>Is either implementation an in-place sort for its caller?</summary>

- The starter sorts a private vector in place after copying the input once.
- Its public contract still allocates and returns a new result.
- The canonical solution builds partition vectors recursively rather than mutating that one copy.
- Compare the actual allocation patterns instead of contrasting “in-place” with “allocating” at the API level.
- If the caller can allow mutation, changing the interface is a separate decision.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/25_quicksort.rs`, with the same borrowed-input, owned-output signature.
- `[pivot, rest @ ..]` binds the first element and remaining slice without allocating.
- `let ... else` returns an empty vector for the empty-input base case.
- This chooses the first pivot rather than the starter's last pivot, while preserving the sorted integer result.
- `partition` visits the remainder once and builds `less` and `greater` vectors using the `<=` predicate.
- `rest.iter()` yields `&i32`, and the predicate receives a reference to that item, explaining `|&&x|`.
- The destination `Vec<i32>` values copy the referenced integers through `Extend<&i32>`; partitioning here is not zero-copy.
- Recursively sort `less`, append the pivot once, then extend with sorted `greater`.
- Reconstruct the argument for ascending order and element preservation from those three steps.
- Singleton input follows the same path with empty partitions instead of needing another explicit branch.
- Both versions handle the entire `i32` comparison domain without subtracting values or risking value-arithmetic overflow.

### Cost and recursion limits

- For n elements, balanced partitions give O(n log n) work and O(log n) recursion depth.
- Both deterministic endpoint pivots permit O(n²) work and O(n) depth on sorted, reversed, or all-equal inputs.
- The starter holds its O(n) result vector plus the recursion stack, without allocating separate partition vectors.
- The canonical solution allocates and copies across recursion levels, with O(n log n) aggregate element work for balanced partitions.
- Peak auxiliary heap storage is O(n) for balanced recursion, but can reach O(n²) for skewed recursion.
- In the all-equal case, live ancestor vectors retain sizes n minus one, n minus two, and so on while their borrowed partitions recurse.
- Both versions can exhaust the stack on large unbalanced inputs, and Rust does not guarantee tail-call elimination.
- Treat “clearest” in the source and slides as a teaching preference, not a universal readability verdict or a benchmark result.

### Alternative approaches

- Production direction, not a separate repository solution: copy the slice and call `sort_unstable` on the resulting vector to preserve the public signature.
- That retains one output copy while delegating sorting and worst-case O(n log n) guarantees to the standard library.
- Use stable `sort` when a future record-sorting API must preserve the order of equal keys.
- Equal integers have no distinguishable identity here, so the existing tests cannot establish record-sort stability.
- A separate `&mut [i32]` interface could avoid the initial copy, but callers would need mutable storage and could no longer expect an unchanged input.
- Do not present that mutating interface as an interchangeable first solution.

### Edge cases and test coverage

- Starter and canonical suites have the same tests: a typical slice, empty/singleton input, sorted/reversed triples, and duplicates with negatives.
- Those small inputs check results but do not expose large-input stack or allocation behavior.
- Proposed correctness cases: all-equal values, two-element permutations, repeated pivots, and `i32::MIN` with `i32::MAX`.
- Proposed property checks: compare bounded generated inputs against standard sorting, preserve length and multiplicities, and verify the input remains unchanged.
- Keep generated test sizes bounded so ordinary tests do not intentionally exhaust the stack.
- Use a separate measurement plan for allocation volume and worst-case behavior if performance comparisons are added later.
- There is no malformed-input error for an integer slice; stack exhaustion and allocation failures are resource failures, not recoverable results from this API.

### Trade-offs

<!-- Final comparison matrices are deferred while this chapter is a bullet outline. -->

- **Type-safety:** **Original:** safe indexed slices with hand-maintained bounds; main removes those index invariants with slice patterns, but neither type system encoding guarantees shallow recursion.
- **Readability:** **Original:** explicit Lomuto mechanics; main exposes partition, recursive sorting, and concatenation at the cost of hiding allocation work behind collection operations.
- **API design:** **Original:** unchanged borrowed input and a fresh vector; main preserves that contract, while a mutating slice API would require caller changes.
- **Performance:** **Original:** one copied buffer plus stack; main adds recursive allocations and can retain quadratic heap storage, while standard sorting guarantees O(n log n) worst-case time.
- **Flexibility:** **Original:** fixed `i32` sorting; main stays equally specialized, and generalizing to non-`Copy` records would require a new ownership and ordering discussion.
- **Error reporting:** **Original:** no error channel and possible resource exhaustion; main keeps that limitation and may require much more memory on skewed partitions.

</details>

## Takeaways For Our Own Codebase

> - Compare a clearer algorithm's allocation lifetime, not just its line count.
> - Test duplicate handling separately from sortedness and preserve multiplicities.
> - Prefer a standard sorting implementation unless a measured requirement justifies owning the algorithm.

## References

- [Slice patterns](https://doc.rust-lang.org/reference/patterns.html#slice-patterns): destructure a pivot and the remaining slice.
- [`Iterator::partition`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.partition): distribute items into two collections.
- [`slice::sort_unstable`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) and [`sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort): production sorting contracts and stability.
- [Fibonacci's algorithmic refactor](22-iterators.md#solutions): removing recursion and rewriting recursion have different cost consequences.
