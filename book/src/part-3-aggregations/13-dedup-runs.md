# `dedup_runs`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

> **BONUS.** Optional exercise.
> Skip it if you're short on time; nothing later in the book depends on it.

- Collapse consecutive equal integers to one value per run, preserving their order.
- Keep the first solution's `dedup_runs(values: Vec<i32>) -> Vec<i32>` signature and caller-facing types.

## Code to refactor

- Source and tests: `examples/13_dedup_runs.rs`.
- Starting approach: walk the owned vector with an index, compare each value with the last output value, and push retained values into a new vector.
- Companion source: `slides/pages/13_dedup_runs.md`, including the ownership question and presenter notes on custom equality.

## Discussion

<details class="discussion-question">
<summary>Does deduplication mean one copy of every distinct value?</summary>

- Not here: `[5, 5, 25, 5]` becomes `[5, 25, 5]`, because the last `5` begins a new run.
- A set would discard separated repeats, while sorting before deduplication would change both order and which repeats become adjacent.
- The contract concerns runs in the original sequence, not membership in a collection of distinct values.

</details>

<details class="discussion-question">
<summary>Can the existing <code>unwrap</code> panic when the output is empty?</summary>

- The left side of `result.is_empty() || ...` short-circuits when the result is empty, so `last().unwrap()` is reached only for a nonempty result.
- The loop condition also keeps `values[i]` within bounds.
- This is a working baseline, not an example where removing `unwrap` fixes an existing empty-input bug.
- The refactoring opportunity is to stop maintaining these local proofs and the index ourselves.

</details>

<details class="discussion-question">
<summary>What does owning the input vector let us do?</summary>

- The caller moves the vector into this function, so the function may change its elements and length without affecting another owner of that vector.
- The starter nevertheless creates a second vector and discards the original buffer on return.
- A mutable local binding does not change the public signature or require callers to declare their argument binding mutable.
- With only an immutable slice, the function could read the elements but could not compact and shorten the caller's vector.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/13_dedup_runs.rs`; the argument binding becomes `mut`, but the function still consumes and returns `Vec<i32>`.
- `values.dedup()` removes consecutive equal elements in place, keeping the first value in each run.
- Return that same vector after compaction; no sorting, set construction, or new result buffer is needed.
- For `n` integers, both versions perform a linear scan, even when the result contains only one value.
- The original additionally grows a result buffer for `r` runs; the main solution uses constant auxiliary space beyond the owned vector.
- In-place compaction does not shrink the vector's capacity, so a much shorter result can still retain a large allocation.
- The output values match the original contract, but callers inspecting capacity should not expect the original and refactored vectors to have identical capacities.

### Alternative approaches

- No alternative implementation is checked in for this exercise; the following are design options, not additional tested solutions.
- `Vec::dedup_by` and `Vec::dedup_by_key`, mentioned in the slide notes, keep the adjacent-run operation while changing how equality is determined.
  - Define the comparison or key explicitly before using these for records; neither method means global uniqueness.
- The solution comments and slides also suggest `Itertools::dedup` for lazy iteration over consecutive duplicates.
  - An iterator result lets the caller stop without collecting the complete output, but returning an iterator changes the API.
  - `itertools` is not a dependency in the current `Cargo.toml`; do not present this as an already available local variant.
- A borrowed-input version taking `&[i32]` could copy into a vector and then deduplicate it, retaining ownership of the original at the cost of a new buffer.
  - Callers would pass a borrow rather than move their vector, and tests would need to verify that the input remains unchanged.
- An in-place `&mut Vec<i32>` API could return `()` instead of transferring ownership back to the caller.
  - Callers would pass a mutable borrow and use the changed vector afterward rather than assign a returned vector.
- A generic owned-vector variant needs `T: PartialEq`, not `T: Copy` or `T: Clone`, because compaction moves retained elements and drops removed ones.

### Edge cases and test coverage

- Existing tests in both files cover mixed runs with a separated repeat, empty input, no duplicates, and an all-equal input.
- `collapses_runs_but_keeps_separated_repeats` protects the distinction from sorting or global deduplication by retaining the final separated `5`.
- Suggested tests: a singleton, alternating values, and duplicate runs at both ends of the vector.
- Suggested tests: negative integers and `i32::MIN`/`i32::MAX`, confirming that the operation depends on equality rather than arithmetic.
- Suggested property checks: the output has no adjacent equal values, preserves run order, and is unchanged by a second application.
- Keep capacity observations separate from content assertions; returning fewer elements does not imply returning memory to the allocator.

### Trade-offs

<!-- Final comparison matrices are deferred until the prose is drafted. -->

- **Type-safety:** Original uses a safe short-circuit guard around `unwrap`; the main solution removes that local proof without adding a stronger public input type.
- **Readability:** Original spells out the index and last-output bookkeeping; `dedup` names the operation directly, provided its adjacent-only meaning is explained.
- **API design:** Original and main both consume and return a vector; slice, mutable-borrow, and iterator variants change ownership or result handling and require caller migration.
- **Performance:** Original allocates a second output buffer while retaining the input during the scan; main compacts in place but retains the original capacity, with no change to linear scan complexity.
- **Flexibility:** Original hard-codes integer equality; standard custom-comparison methods and a generic variant offer extensions, while lazy iteration adds an API and dependency decision.
- **Error reporting:** Original's guarded `unwrap` does not reject empty input, and main also returns an empty vector; neither exposes recoverable errors or an allocation-failure result.

</details>

## Takeaways For Our Own Codebase

> - When a function says “deduplicate,” does it mean consecutive runs or global uniqueness?
> - Are we allocating a second buffer even though the function already owns one it may modify?
> - Would retaining capacity help the next use of this vector, or keep substantially more memory than the result needs?

## References

- [`Vec::dedup`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup): consecutive equality and in-place removal.
- [`Vec::dedup_by_key`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup_by_key): adjacent runs under a derived comparison key.
- [`Itertools::dedup`](https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.dedup): the iterator alternative mentioned in the sources.
- [`spell_check`](15-spell-check.md): a contrasting use of a set for membership while preserving repeated output values.
