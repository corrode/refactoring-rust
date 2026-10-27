# `floor`

This function follows a sequence of instructions to find the final floor, starting at zero.
`(` moves up one floor and `)` moves down one.
Can you express the calculation more directly?
Keep the signature, result, and error for the first invalid instruction unchanged.

This exercise comes from [Advent of Code 2015, Day 1: Not Quite Lisp](https://adventofcode.com/2015/day/1).


> **Draft outline.** The exercise and solutions are runnable; the discussion and comparison notes still need to be expanded into prose.

## Code to refactor

```rust,editable
{{#include ../../../examples/10_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/10_floor.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 10_floor
```

</div>

## Discussion

<details class="discussion-question">
<summary>Can we reduce the instructions without storing the steps?</summary>

- The original loop keeps only a running floor and returns early on an invalid character.
- An iterator can produce one step at a time and pass it directly to a reduction; a temporary `Vec` is not required.
- Separate translating an instruction from combining the resulting steps.
- Revisit [`parse_ints`](09-parse-ints.md): `collect` builds a collection there, but this task only needs one final value.

</details>

<details class="discussion-question">
<summary>What happens to an error inside <code>sum</code>?</summary>

- The iterator can yield `Result<i64, String>` rather than unwrapped integers.
- The `Sum` implementation for `Result` combines the successful values and returns the first error without consuming the rest of the instructions.
- The function's return type supplies the result type for `sum`; no extra type annotation or error-handling loop is needed here.
- Counting opening and closing parentheses separately, or discarding errors with `filter_map`, would lose the invalid-instruction contract.

</details>

<details class="discussion-question">
<summary>What do an empty input and a negative floor mean?</summary>

- Empty input returns `Ok(0)`, the identity for addition, rather than an error or missing result.
- Negative floors are valid; entering the basement does not end this task.
- Whitespace is not ignored, including a trailing newline from a file.
- `chars` makes an invalid Unicode character appear intact in the error message; iterating over bytes would need separate diagnostic handling.

</details>

<details class="discussion-question">
<summary>When is the loop or <code>try_fold</code> the better choice?</summary>

- Keep the loop when its explicit state and early return are clearer to the team; this exercise does not establish a rule against loops.
- `sum` fits independent `+1` and `-1` steps; `try_fold` gives each step access to the accumulated floor.
- The puzzle's second part asks when the basement is first reached, which needs intermediate state and a position rather than only the final sum.
- Treat that second task as an optional follow-up with its own contract, not a reason to complicate this function.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Using `sum`

```rust,editable
{{#include ../../../solutions/10_floor.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/10_floor.rs:tests}}
</script>

- Map each instruction to `Ok(1)`, `Ok(-1)`, or the unchanged `Err(format!("Invalid instruction: {c}"))`.
- Sum the fallible steps directly; successful input has no intermediate collection or string allocation.
- Keep `fn floor(instructions: &str) -> Result<i64, String>` and the caller's ownership of the input unchanged.
- Preserve traversal order and the first invalid character, rather than accumulating all errors or normalizing the input.
- The expression is shorter, but explaining the standard library's `Sum` implementation is the main teaching point.

### Using `try_fold`

```rust,editable
{{#include ../../../solutions/10_floor_alt.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/10_floor_alt.rs:tests}}
</script>

- Start at zero; the closure receives the current floor and one instruction, then returns the next floor or an error.
- `Err` stops the fold immediately, matching the original loop's early return.
- Unlike a plain `fold` that carries a `Result` accumulator, `try_fold` does not keep calling the closure for later items after an error.
- Use this version if the next calculation needs the current floor; for the present independent steps, `sum` needs less explanation of state.
- Neither solution changes callers or requires a dependency.

### Edge cases and test coverage

- **Already tested in all three versions:** empty input, balanced instructions, positive and negative totals, and single instructions.
- **Already tested:** an invalid character at the start, middle, and end, plus `(x)y` to establish first-error precedence.
- **Already tested:** spaces, newlines, accented characters, and an emoji are rejected with the same message.
- **Preserved limitation:** the error contains the offending character but no byte offset or instruction position.
- **Possible extension, not implemented:** use `char_indices` for byte offsets or `enumerate` over `chars` for instruction positions; name the unit and change the error contract deliberately.
- **Follow-up tests:** if adding the basement task, distinguish first arrival from the final floor and define what happens when the basement is never reached.

### Trade-offs

- **Type-safety:** Original, `sum`, and `try_fold` all use `Result<i64, String>`; iterator syntax adds no new domain invariant.
- **Readability:** Original exposes the state and early return; `sum` highlights the arithmetic once `Sum<Result<_, _>>` is familiar; `try_fold` keeps the state transition visible inside a closure.
- **API design:** all three preserve the borrowed input, return type, and error message; no caller migration is needed.
- **Performance:** all process instructions once up to the first error with constant auxiliary state; successful paths allocate nothing, while each error formats a `String`.
- **Flexibility:** `sum` is specific to additive steps; Original and `try_fold` accommodate state-dependent steps without pretending every reduction is addition.
- **Error reporting:** all stop at the first invalid character and discard the partial floor; none reports a position or all invalid characters.

</details>

## Takeaways For Our Own Codebase

> - Check whether a standard reduction already handles the container type, including `Result`, before writing another accumulation loop.
> - Decide whether callers need the final result, intermediate states, or every failure before choosing an iterator operation.
> - Keep an explicit loop when it communicates the contract better; removing mutable local state is not a goal on its own.

## References

- [`Sum`](https://doc.rust-lang.org/std/iter/trait.Sum.html): summing integers and fallible items without an intermediate collection.
- [`Iterator::try_fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold): a reduction that stops on failure.
- [`parse_ints`](09-parse-ints.md): compare fail-fast collection of `Result` values with running every independent validation rule.
- [Advent of Code 2015, Day 1](https://adventofcode.com/2015/day/1): the original puzzle and its separate first-basement task.
