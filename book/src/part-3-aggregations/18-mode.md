# `mode`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Find a value with the highest occurrence count in a slice of integers.
- Keep `most_common(numbers: &[i32]) -> i32` as the first solution's public API.
- Review the result-selection logic as well as the counting stage.

## Code to refactor

- Source and tests: `examples/18_mode.rs`; the exercise filename is `mode`, while the function is named `most_common`.
- Starting approach: count occurrences in a `HashMap<i32, i32>`, seed a `(value, count)` pair from the map, and scan for a larger count.
- Companion source: `slides/pages/18_mode.md`, including notes that separate the counting idiom from the manual maximum search.

## Discussion

<details class="discussion-question">
<summary>Does the counting loop need to change?</summary>

- `entry(x).or_insert(0)` obtains the current count or inserts zero, then the dereference lets the increment update that entry.
- The loop already expresses frequency counting without a separate membership test and insertion branch.
- The duplicated `iter().next()` calls and mutable `best` belong to result selection, a separate operation with a standard iterator method.
- Seeding calls read the first entry twice without mutating the map; they do not consume two entries from one persistent iterator.

</details>

<details class="discussion-question">
<summary>Which value wins when several have the same highest count?</summary>

- The starter updates only for `count > best.1`, retaining the first maximal entry encountered in map iteration order.
- `Iterator::max_by_key` retains the last maximal entry when keys compare equal.
- Neither order represents the original input order, and `HashMap` does not provide a stable ordering contract.
- The existing all-distinct test allows any input value, so compatibility here means returning a mode, not reproducing one particular tied winner.
- A refactor can satisfy that contract while changing a tied result; deterministic selection requires an explicit additional policy.

</details>

<details class="discussion-question">
<summary>What happens when the slice is empty?</summary>

- No map entry exists, so the starter's first `expect("empty input")` panics while seeding `best`.
- The main solution also panics with `"empty input"`, but does so after the iterator produces no maximum.
- A bare `i32` cannot represent absence without reserving a sentinel, and every `i32` is a legitimate input value here.
- Returning `Option<i32>` would change the API rather than preserve the original function's empty-input behavior.

</details>

<details class="discussion-question">
<summary>Does an <code>i32</code> input value require an <code>i32</code> count?</summary>

- Keys and frequencies represent different quantities even though both currently use `i32`.
- More than `i32::MAX` occurrences of one value overflow its count, panicking with overflow checks or wrapping when those checks are disabled.
- The selection logic cannot recover correct frequencies from a wrapped count.
- A proposed internal `usize` counter would fit counts bounded by a slice's length without changing the public integer result; neither checked-in version makes that change.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/18_mode.rs`, keeping the original slice parameter, integer return type, and `HashMap<i32, i32>` counts.
- Leave the entry-based counting loop intact.
- Consume the finished map with `into_iter`, yielding owned `(value, count)` pairs instead of references.
- `max_by_key(|&(_, count)| count)` compares counts while retaining the complete winning pair.
- The closure pattern copies the `i32` count from the borrowed pair supplied to the key function; it does not clone the map or input slice.
- Map the optional pair to its value, then use one `expect("empty input")` to preserve the empty-input panic.
- `max_by_key` must inspect the whole iterator to know the maximum; this is not a short-circuiting search and does not sort the entries.
- TODO(prose): State the tie-compatibility boundary explicitly rather than calling this identical behavior for every input; choose any deterministic tie rule only as a separately specified and tested variant.

### Alternative approaches

- No alternative solution file is checked in; the slides recommend an `Option` result as a possible API redesign.
- An `Option<i32>` variant would keep the optional winning value instead of calling `expect`, returning `None` for an empty slice.
  - Callers must handle absence, and tests must distinguish `Some(mode)` from `None` instead of treating an integer sentinel as special.
- A smallest-value tie rule could compare frequency first and the value in reverse order second, so equal counts prefer the smaller integer.
  - Use a comparator or `Reverse` rather than negating the value, since negating `i32::MIN` overflows.
- A first-in-input tie rule requires retaining first positions or scanning the original slice after determining the maximum frequency.
  - A hash map's iteration order cannot stand in for occurrence order, even if a particular run appears stable.
- Sorting a copied slice and counting runs is another possible design, with a copied buffer and `O(n log n)` sorting work rather than expected hash-table insertion costs.
  - Mutating the original slice would require a different parameter contract, and any tie rule still needs to be stated.
- A `usize` frequency type is a separate internal hardening option, not a requirement to change `most_common` into an API returning `usize`.

### Edge cases and test coverage

- Existing tests in both files cover a unique mode in mixed positive and negative values, a singleton, and all-distinct input.
- `all_distinct_returns_some_element` checks membership rather than a fixed winner, correctly allowing every value when all counts are one.
- Neither file currently tests the empty-input panic, despite both implementations having it.
- Suggested test: an empty slice with an expected `"empty input"` panic, preserving the current API contract.
- Suggested test: `[1, 1, 2, 2, 3]` must return `1` or `2`, not just any input element.
- Suggested tests: an all-equal slice and repeated `i32::MIN`/`i32::MAX` values, which are keys rather than sentinels.
- For an optional-result or deterministic-tie variant, add API-specific assertions instead of silently tightening the canonical solution's tie contract.
- Counter overflow is a source-level limitation, not exercised by the small existing tests; billions of repeated elements are not a practical routine regression fixture.
- With `n` inputs and `k` distinct values, both versions perform `n` expected constant-time hash updates and retain `O(k)` entries.
- Maximum selection traverses the table, whose bucket capacity can exceed `k`; the main solution removes bookkeeping, not the full counting-and-selection passes.

### Trade-offs

<!-- Final comparison matrices are deferred until the prose is drafted. -->

- **Type-safety:** Original and main accept empty slices despite returning a bare integer and use signed frequencies; an `Option` result exposes absence, while an internal `usize` count addresses a different limitation.
- **Readability:** Original repeats iterator seeding and maintains a mutable pair; main separates frequency counting, maximum selection, and value extraction with named iterator operations.
- **API design:** Original and main preserve the signature and empty-input panic, not a specific tied winner; optional results or deterministic ties need explicit contracts and matching caller expectations.
- **Performance:** Original and main build the same frequency table and traverse it to select a maximum; consuming entries avoids borrowed result handling but does not establish a measured speed-up or remove the allocation.
- **Flexibility:** Original's explicit comparison and main's key selector can both express tie policies, but first-in-input selection needs information the unordered table alone does not preserve.
- **Error reporting:** Original and main panic with the same empty-input message; the main solution has one visible failure point, while `Option` would let callers handle absence without adding a descriptive error payload.

</details>

## Takeaways For Our Own Codebase

> - Do tied maxima have an explicit policy, or are callers relying on incidental collection order?
> - Are counters typed for the number of observations rather than copied from the measured value's type?
> - Can an empty collection legitimately reach this API, and should the caller handle that case instead of encountering a panic?

## References

- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry): frequency updates without a separate lookup-and-insert branch.
- [`Iterator::max_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by_key): optional maxima and last-equal-item selection.
- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html): unordered iteration rather than input-order guarantees.
- [`Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html): reversing an ordering without arithmetic negation.
- [`text_stats`](17-text-stats.md): distinguishing frequency values from the number of map entries before choosing an aggregation.
