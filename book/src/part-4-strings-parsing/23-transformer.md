# `transformer`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Apply one command to each string in a batch, then examine a separate pipeline of commands on one input.
- Preserve `apply_commands(input: Vec<(String, Command)>) -> Vec<String>` for the first solution.
- Also preserve `apply_pipeline(commands: &[Box<dyn PipelineCommand>], input: &str) -> String`, the enum, and the trait's `run` signature.
- Keep batch output order and pipeline execution order explicit.

## Code to refactor

- Starter and its tests: `examples/23_transformer.rs`.
- Workshop prompts, pipeline follow-up, and speaker notes: `slides/pages/23_transformer.md`.
- The batch function indexes an owned vector and clones each `(String, Command)` pair.
- An `if let` chain selects uppercase conversion, trimming, or repeated appending of `"bar"`.
- Each branch pushes an owned string into the result vector.
- The pipeline accepts heterogeneous boxed implementations of `PipelineCommand`.
- It special-cases an empty command slice, then chooses each command's input by checking whether the accumulated string is empty.

## Discussion

<details class="discussion-question">
<summary>Why clone a string from a vector the function already owns?</summary>

- Indexing gives access through a borrow, so the starter clones before taking the tuple apart.
- Consuming the vector can move each string and command out instead.
- Moving preserves input order and avoids copying every source string before transformation.
- Removing that clone does not make uppercase conversion, trimming into owned text, or appending allocation-free.

</details>

<details class="discussion-question">
<summary>What does an exhaustive match add here?</summary>

- The current three `if let` branches cover all existing variants, but the language does not require that chain to remain exhaustive.
- A future unhandled variant could fall through without adding a batch result.
- A `match` without a catch-all arm forces a decision for every new variant.
- Keep the existing `Command` type and its derives; eliminating an internal clone does not require removing a caller-visible capability.

</details>

<details class="discussion-question">
<summary>Can an empty string mean both valid output and “not started”?</summary>

- The starter pipeline uses an empty result as an initialization sentinel.
- After a command erases its input, the next command receives the original input instead of that empty output.
- Trace `"start"` through `Erase` and append `"!"`: the starter produces `"start!"`, while the intended pipeline produces `"!"`.
- An empty command sequence should return an owned copy of the input.
- This is a state bug, independent of choosing a loop or an iterator adapter.

</details>

<details class="discussion-question">
<summary>Are the enum commands and trait-object commands interchangeable?</summary>

- `apply_commands` transforms independent input/command pairs; it does not feed one pair's result into the next.
- `apply_pipeline` feeds each command's output into the next command in slice order.
- Enum dispatch has a fixed set of variants and compile-time exhaustiveness checking.
- Trait-object dispatch accepts different implementations behind one interface and uses dynamic dispatch.
- Neither the trait nor `&self` promises that command implementations are pure or cannot panic.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/23_transformer.rs`, preserving both interfaces and their caller-facing types.
- Batch: consume with `into_iter`, destructure the tuple, match the command, and collect one output per input.
- The iterator adapter is lazy internally, but `collect` completes the entire batch before returning.
- `Uppercase` uses Unicode casing, which can expand text such as `ß` into `SS`.
- `Trim` recognizes Unicode whitespace and copies the retained borrowed substring into an owned string.
- `Append(n)` creates `"bar".repeat(n)` and appends it with `String + &str`.
- Addition consumes the existing string and can reuse its buffer, but may reallocate if capacity is insufficient.
- The repeated suffix is a separate temporary allocation; eliminating input clones does not eliminate that temporary.
- Batch traversal is O(m) for m entries, plus string processing and output construction, including 3n suffix bytes for each append command.
- Pipeline: initialize `current` from the actual input, then replace it with every command's output.
- The same loop handles no commands and empty intermediate results without a sentinel.
- This pipeline change fixes behavior; keep it distinct from the ownership and dispatch refactor in the batch function.
- Copying the input up front can add a copy for nonempty pipelines compared with borrowing it for the first command.

### Alternative approaches

- Same-signature variation: keep a consuming `for` loop with tuple destructuring and an exhaustive `match` if that reads better than `map`.
- Keeping the append loop avoids allocating a complete temporary suffix, while still performing work proportional to the appended bytes.
- A pipeline `fold` seeded with `input.to_string()` has the same state invariant as the canonical loop.
- Folding from an empty string and retaining the sentinel test would preserve the bug, not fix it.
- A consuming `run(String) -> String` could let implementations reuse buffers, but that is a separate trait redesign requiring implementation and caller migration.
- No separate alternative source file exists for these variations; do not present them as already tested repository implementations.

### Edge cases and test coverage

- Current starter tests cover all batch variants, an empty batch, append-zero, a two-command pipeline, and an empty pipeline.
- The sample checks both output count and order through full vector equality.
- The canonical suite adds `empty_command_output_is_still_pipeline_data` using `Erase` followed by append.
- The starter suite does not contain that regression, so its passing tests do not establish correct pipeline state handling.
- Proposed batch tests: whitespace-only trim, empty uppercase input, Unicode expansion, and appending to an empty string.
- Proposed pipeline tests: empty initial input, erasure as the last command, consecutive erasures, and multiple commands after erasure.
- `Append(0)` preserves the text, while an empty batch produces no output entries.
- Extremely large append counts can exceed string capacity or available memory; `repeat` can panic on size overflow and neither API returns an allocation error.
- Pipeline command failures are not represented by `run`; a future fallible pipeline must decide how to report the failed stage.
- For k stages, account for each stage's processing and output size rather than claiming O(k) regardless of string lengths.

### Trade-offs

<!-- Final comparison matrices are deferred while this chapter is a bullet outline. -->

- **Type-safety:** **Original:** enum values plus an unchecked sentinel convention; main adds exhaustive dispatch and removes the sentinel, without making command execution fallible in the type system.
- **Readability:** **Original:** indexing, cloned tuples, and empty-state branches; main exposes the batch mapping and pipeline state directly, with the loop alternative remaining reasonable.
- **API design:** **Original:** owned batch inputs and borrowed trait-object pipelines; main preserves both, while a consuming command trait would require a separate migration.
- **Performance:** **Original:** clones each batch string and grows append buffers incrementally; main removes those clones but allocates repeated suffixes and copies the pipeline seed up front.
- **Flexibility:** **Original:** closed enum commands alongside extensible trait implementations; main keeps that distinction, and adding an enum variant now requires updating the match.
- **Error reporting:** **Original:** no error return and empty output used as state; main preserves empty output as data, but command panics and resource failures still have no structured error path.

</details>

## Takeaways For Our Own Codebase

> - Move from owned collections before paying for clones that only work around indexing.
> - Test a valid empty result in the middle of every stateful pipeline.
> - Choose enum or trait dispatch based on who can add operations, not on which syntax looks shorter.

## References

- [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html): consuming iteration and moved items.
- [`match` expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html): exhaustive handling of enum variants.
- [`str::repeat`](https://doc.rust-lang.org/std/primitive.str.html#method.repeat): owned repetition and capacity-overflow behavior.
- [`Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold): an accumulator with an explicit initial value.
- [String helper composition](24-fun-strings-ext.md#solutions): method chains change presentation, not the need to account for intermediate strings.
