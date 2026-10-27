# `iterators`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Produce individual Fibonacci numbers and prefixes of the sequence beginning with `0, 1, 1`.
- Keep `fib(n: u32) -> u64` and `first_n(n: usize) -> Vec<u64>` for the first solution.
- Treat a different way of requesting sequence values as a separate API alternative.

## Code to refactor

- Starter and its tests: `examples/22_iterators.rs`.
- Workshop prompts and speaker notes: `slides/pages/22_iterators.md`.
- `fib` handles zero and one explicitly, then adds two recursive calls.
- `first_n` grows a vector with a `while` counter and calls `fib` for each index.
- Each index is cast from `usize` to `u32` before that call.
- Separate the recurrence, prefix construction, and public return types when reviewing the implementation.

## Discussion

<details class="discussion-question">
<summary>Which change removes the repeated work?</summary>

- The recursive definition recomputes the same subproblems in different branches.
- Work is O(φⁿ), where φ is the golden ratio, with O(n) recursive stack depth.
- Replacing the prefix's `while` loop with `map(fib)` would leave that repeated work intact.
- Computing successive values from the previous pair reduces prefix generation to O(n).
- The improvement comes from retained state, not from iterator syntax by itself.

</details>

<details class="discussion-question">
<summary>Is a lazy implementation still lazy after returning a vector?</summary>

- The original contract returns all requested values in an owned `Vec<u64>`.
- An internal iterator can defer individual steps, but `collect` consumes them before the function returns.
- The caller cannot stop halfway through that completed call.
- A returned iterator allows partial consumption, at the cost of a changed interface and different failure timing.

</details>

<details class="discussion-question">
<summary>How many Fibonacci values fit in <code>u64</code>?</summary>

- F(93), `12_200_160_415_121_876_738`, fits; F(94) does not.
- Because the sequence starts at F(0), `first_n(94)` requests the last representable prefix.
- An implementation can overflow too early if it computes a future state before returning the requested value.
- Beyond the representable range, ordinary addition panics with overflow checks enabled and wraps when those checks are disabled.
- A finite checked sequence would be another explicit behavior policy, not an incidental syntax cleanup.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/22_iterators.rs`, preserving both functions and their caller-facing types.
- `fib` starts with `(previous, current) = (1, 0)` and advances the pair n times.
- Tuple assignment evaluates the right-hand values before replacing the old pair.
- After each step, `current` is the Fibonacci number for the number of completed steps.
- `first_n` scans `0..n`, retaining the pair between requested values rather than restarting `fib`.
- At index zero, emit the initial zero without advancing; at later indices, advance once before emitting.
- This avoids computing F(94) merely to return F(93).
- `fib` takes O(n) time and O(1) auxiliary space; `first_n` takes O(n) time and O(n) output space.
- Prefix order and duplicates are unchanged, and zero requested values produce an empty vector.
- Ordinary overflow remains for `fib(94)` and `first_n(95)`; there is no new error channel.
- The starter's `usize as u32` truncates enormous indices on wider targets, whereas the scan has no such index conversion.
- Limit equivalence claims to the intended representable sequence rather than promising identical huge-prefix behavior after overflow and index truncation.

### Alternative: a lazy Fibonacci iterator

- Source: `solutions/22_iterators_alt.rs`, exposing `Fibonacci::new`, `Default`, `Iterator<Item = u64>`, and `FusedIterator`.
- Callers replace `first_n(n)` with `Fibonacci::new().take(n).collect()` when they still need a vector.
- Direct `next`, `take`, `map`, and `zip` can consume only the requested prefix without a result-vector allocation.
- Each successful `next` uses O(1) state and work; obtaining the kth value by iteration still takes O(k) steps.
- The implementation advances both state fields before returning the saved current value.
- With overflow checks enabled, the call that would yield F(92) computes F(94) and panics first.
- `take(92)` can yield F(0) through F(91), but `take(93)` already reaches that premature overflow.
- There is no normal `None` path; with overflow checks disabled, values wrap and the iterator keeps yielding.
- `FusedIterator` promises permanent exhaustion after `None`, not finite length or checked arithmetic.
- Collecting without a bound is therefore not a way to obtain all representable Fibonacci numbers.

### Alternative direction: checked exhaustion

- Design only, not an implemented solution: retain pending valid values and an explicit terminal state.
- A correct finite iterator must still yield F(92) and F(93) before returning `None` permanently.
- Replacing addition with `checked_add(...)?` alone loses a valid current value when the lookahead overflows.
- Decide whether exhaustion means normal completion or whether callers need an explicit overflow error.

<!-- TODO(prose): Set the documented domain for compatibility claims, including first_n's removed usize-to-u32 cast.
Keep the lazy source's checked_add suggestion qualified by the pending-value problem; the updated slide notes already require a terminal state. -->

### Edge cases and test coverage

- Current starter tests cover the first ten values and `fib(0)`, `fib(1)`, and `fib(10)`.
- Current main tests add `fib(2)`, prefixes of lengths zero through two, and the full representable boundary through `first_n(94)`.
- That boundary test checks every collected value against the iterative `fib` implementation.
- Current alternative tests cover the first ten values and four successive `next` calls, not overflow or exhaustion.
- Proposed alternative tests: `Default`, zero-length consumption without advancement, and continued iteration after a short borrowed prefix.
- Proposed overflow tests should state whether checks are enabled; do not write a build-profile-independent assertion expecting a panic.
- If a checked variant is added, test both final valid values and repeated `None` after exhaustion.
- Avoid running the recursive starter at F(93) as a routine regression test; its exponential work makes that impractical.

### Trade-offs

<!-- Final comparison matrices are deferred while this chapter is a bullet outline. -->

- **Type-safety:** **Original:** fixed-width integers with unchecked addition; main retains that policy, while the iterator's `FusedIterator` marker provides no overflow guarantee.
- **Readability:** **Original:** a recognizable recursive recurrence; main requires explaining the state pair, and the iterator adds a lifecycle whose lookahead must be reviewed carefully.
- **API design:** **Original:** separate scalar and eager-prefix functions; main preserves both, while the iterator changes how callers request values and when failures occur.
- **Performance:** **Original:** exponential recomputation and recursive stack use; main is linear with constant working state, while lazy consumption avoids an output vector only when callers do not collect.
- **Flexibility:** **Original:** fixed prefix allocation; the iterator composes with consumers and adapters, but it is neither random access nor a finite checked sequence as written.
- **Error reporting:** **Original:** no overflow result; main still panics or wraps beyond the numeric range, and the current iterator can panic before yielding an otherwise valid requested value.

</details>

## Takeaways For Our Own Codebase

> - Look for repeated computation before changing loop syntax.
> - Check whether advancing an iterator computes beyond the value the caller requested.
> - Specify exhaustion, overflow, and allocation behavior independently when introducing a lazy API.

## References

- [`Iterator::scan`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan): retain state while transforming a finite input sequence.
- [`Iterator::next`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next) and [`take`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take): demand-driven consumption.
- [`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html): what repeated calls after exhaustion guarantee.
- [Checked timestamp arithmetic](21-parse-srt-timestamp.md#solutions): arithmetic policy belongs in the contract, not just the implementation.
