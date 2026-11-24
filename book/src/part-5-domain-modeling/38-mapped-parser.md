# `mapped_parser`

> **Draft outline.** The exercise and solution are runnable; the bullet notes are a reference for the chapter's prose.

## Code to refactor

```rust,editable
{{#include ../../../examples/38_mapped_parser.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/38_mapped_parser.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 38_mapped_parser
```

</div>

- `Decimal` parses an unsigned integer, `map` transforms a successful result, and `parse_all` applies a parser to a sequence of inputs.
  Review the trait, adapter, and helper together, and simplify their API.
- Preserve parsing results, original errors, mapping composition, and support for stateful transformations.
  Keep the input type generic rather than specializing the whole interface to strings.
- You may change the trait and wrapper types in this domain-modeling exercise.
  Update their implementations, callers, and tests when the interface changes, and distinguish preserved runtime behaviour from changes to which implementations are permitted.
- The parsing is deliberately ordinary `str::parse`, not a new grammar to implement.
  This standard-library-only example adapts a real refactor in [nom](https://github.com/rust-bakery/nom); it is not a verbatim upstream excerpt.

The starter tests pass.

## Discussion

<details class="discussion-question">
<summary>What does each type parameter represent?</summary>

- `I` is the input, `O` is the parser's successful output, and `E` is its error.
  The adapter adds `P` for the inner parser, `F` for the transformation, and `U` for the transformed output.
- `Map<P, F, O>` mentions the inner parser's output, not the adapter's final output.
  For example, mapping a parsed integer to a string still leaves `O` as `u32`.
- The generic trait permits one parser type to implement `Parser` several times for the same input type, provided the output or error parameters differ.
  The starter's `Flexible` test demonstrates that capability; it is legal code, not a bug.
- Ask which choices the intended API needs and which are already settled by the particular parser implementation.

</details>

<details class="discussion-question">
<summary>Why does <code>Map</code> mention a type it never stores?</summary>

- `Map` stores the parser and mapping function, not a parsed output.
  Its `PhantomData<O>` field records a type relationship without storing an `O`.
- Removing only the marker leaves an unused struct parameter.
  Removing `O` from the wrapper as well leaves the implementation's `O` unconstrained: `P: Parser<I, O, E>` can describe more than one output for the same `P` and `I`.
  A bound mentioning a parameter is not always enough to constrain it for an implementation.
- The marker is zero-sized and does not allocate.
  It is meaningful type-system bookkeeping, not an expensive container waiting to be optimized away.

</details>

<details class="discussion-question">
<summary>What must survive a change to the signatures?</summary>

- `map` calls its inner parser once, moves a successful output into the transformation, and passes errors through unchanged.
  It must not call the transformation on an error or require `Clone` for the output or error.
- `FnMut` permits the transformation to retain state across calls.
  Replacing it with `Fn` would reject valid callers; a transformation consumed on its first call would not support repeated parsing.
- `parse_all` preserves input order, collects successful outputs into a vector, and stops at the first error.
  It returns an empty vector for empty input without calling the parser.
- Stopping does not roll back earlier parser or mapper side effects.
  On error, the partially collected vector is discarded and later inputs remain unprocessed.

</details>

<details class="discussion-question">
<summary>Do callers already have to write all these types?</summary>

- Usually not at an ordinary call such as `Decimal.parse("42")`.
  The compiler can infer types from the implementation and surrounding expressions.
- The repetition is more visible in trait implementations, generic helpers, and annotated adapter types such as `Map<Decimal, _, u32>`.
  Count the relationships that those declarations must repeat, not just explicit type arguments at a call site.
- A shorter signature is useful only if it still expresses the choices callers need.
  Fewer generic parameters alone do not establish better inference, clearer diagnostics, or faster code.

</details>

<div id="solutions"></div>

## Solution

Compare the parser interfaces by the relationships their declarations repeat and the implementation choices they permit.

### Let the implementation determine its result types

**Stop asking callers to supply a type that the implementation already determines.**

```rust,editable
{{#include ../../../solutions/38_mapped_parser.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/38_mapped_parser.rs:tests}}
</script>

- `Parser<I>` keeps the input generic and moves `Output` and `Error` into the implementation as associated types.
  For a fixed `Self` and `I`, there is one output type and one error type.
  This does not require the same output or error for every possible input type.
- `Decimal` chooses `Output = u32` and `Error = ParseIntError` for `&str`.
  Its parsing operation stays `input.parse()`; the refactor changes the trait contract, not the integer syntax or error values.
- `Map<P, F>` can now refer to `P::Output` under the bound `P: Parser<I>`.
  The extra `O` parameter and its marker are no longer needed to identify the inner parser's output.
- The adapter's output is `U`, selected by the mapping function, while its error remains `P::Error`.
  `Result::map(&mut self.f)` keeps the success-only transformation and the same `FnMut` instance across calls.
- `parse_all` names `P::Output` and `P::Error` directly.
  Its collection algorithm stays unchanged, including ordering, empty input, allocation of the result vector, and first-error short-circuiting.
- This is not the removal of generics.
  Inputs, parser implementations, transformations, and transformed outputs still vary.
  As with `Iterator::Item` in [iterators](../part-4-strings-parsing/23-iterators.md), an associated type describes a result of choosing an implementation rather than an additional independent trait argument.

### Migrate implementations, callers, and tests

- Change `impl Parser<&str, u32, ParseIntError> for Decimal` to `impl Parser<&str> for Decimal`, with `type Output = u32` and `type Error = ParseIntError` inside the implementation.
  Do the same for helper parsers such as the tests' `Counting` and `Forward`.
- Replace general bounds such as `P: Parser<I, O, E>` with `P: Parser<I>`, then use `P::Output` and `P::Error` in the helper's result type.
  When a helper requires a particular output, state that constraint explicitly, for example `P: Parser<I, Output = u32>`.
- Change annotated wrapper types from `Map<Decimal, _, u32>` to `Map<Decimal, _>`.
  Calls such as `Decimal.map(|number| number.to_string())` usually need no source change.
- Explicit `parse_all::<I, O, E, P>` arguments become `parse_all::<I, P>`.
  The helper still takes its parser by value; this exercise does not introduce a forwarding implementation for `&mut P`.
- Trait-object annotations change from `dyn Parser<&str, u32, ParseIntError>` to `dyn Parser<&str, Output = u32, Error = ParseIntError>`.
  Both designs support parsing through a trait object.
  Keep `Self: Sized` on the consuming `map` method, not on the whole trait.
- The ten mirrored behavioural tests cover parsing, output-changing and composed mappings, unchanged errors, mapper state, collection order, empty input, early stopping, non-`Clone` values, and trait-object calls.
  Each editable listing has tests for its own API; changing the starter requires updating its attached tests too.
- The starter's extra `Flexible` test intentionally has no equivalent passing solution test: it implements two output/error pairs for the same parser and input.
  The solution forbids that contract rather than preserving it silently.
- Integration tests compile callers against the actual solution and require `E0119` for conflicting output types and, separately, conflicting error types.
  Positive fixtures demonstrate different input types on the same parser, generic helpers, and mapped parsers.
  Runtime tests alone cannot establish that conflicting implementations are rejected.

### The contract is narrower

- `Self` and `I` now determine the output/error pair.
  Two implementations of `Parser<&str>` for the same type conflict even if their associated types differ.
  This is the useful restriction in this exercise, not proof that the original generic trait was unsafe.
- If callers genuinely need several result types for the same parser and input, keep an appropriate generic parameter or distinguish the parser types explicitly.
  Associated types should record a real dependency, not suppress an intentional choice.
- Output and error are separate decisions.
  An API could use an associated output while keeping an error parameter if callers need to select an error representation.
- A helper may also keep a named `E` with a bound such as `P: Parser<I, Error = E>` for readability.
  That equality still ties the helper's error to the parser's chosen error; it does not restore several implementations of `Parser<I>` for the same type.
- Removing the original owning `PhantomData<O>` can relax `Send` and `Sync` restrictions inherited from `O`.
  A marker can also affect variance and drop checking; review its purpose before deleting it in another library.
  Preserved parsing results are not a claim of identical public trait bounds.
- Neither the marker nor associated types imply a runtime allocation here.
  Mapping can still allocate if the closure does, and `parse_all` still constructs a vector.
  There is no claimed runtime speed-up or universal improvement in type inference.

### What the real refactor changed

- [nom PR #1626][nom-pr], merged January 22, 2023, changed `Parser<I, O, E>` to `Parser<Input>` with associated `Output` and `Error: ParseError<Input>`.
  It also changed the mapping adapter from `Map<Self, G, O>` to `Map<Self, G>`, removing its output marker.
- The [before][nom-before] and [after][nom-after] sources are pinned to the merge commit's parent and merge commit, rather than today's nom API.
  Our `Result` omits nom's remaining-input tracking, streaming, and error abstractions.
  `Decimal`, `parse_all`, and the tests are exercise-specific, with no nom dependency.
- The discussion is useful evidence against a blanket recommendation: the author retained explicit error parameters in many combinators because removing them made signatures harder to read.
  A later winnow experiment reported `E0207` when its error type was no longer constrained by the implementation.
  Assigning an otherwise unconstrained `E` to `type Error = E` does not determine which `E` to use.
- The pinned [MIT notice][nom-license], copyright 2014–2019 Geoffroy Couprie, is recorded in the repository's `THIRD_PARTY_NOTICES.md`.
  Keep that notice with redistributed adapted code; the exercise is an adaptation of a real change, not an upstream code quotation.

## Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.

| Criterion | Original | Associated output and error |
|---|---|---|
| Type-safety | Several output/error pairs permitted per parser and input | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> One pair enforced for fixed `Self` and `I` |
| Readability | Output/error parameters repeated in bounds and helpers | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Result types named through the parser |
| API design | Adapter carries a separate intermediate-output parameter | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Implementation determines results; breaking API change |
| Performance | Zero-sized marker; parser and closure stored | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same parsing and collection work |
| Flexibility | Same parser/input can select different outputs or errors | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> Those choices require different parser types or another API |
| Error reporting | Original parse error returned unchanged | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same runtime errors and propagation |

The type-safety advantage is enforcement of the chosen one-pair contract, not a repair of invalid or memory-unsafe code.
Readability assumes some familiarity with associated types; a named equality constraint can be clearer than a long projection.
The two APIs allow different sets of implementations, and removing the marker can change auto-trait behaviour, so they are not interchangeable public interfaces.

## Takeaways For Our Own Codebase

> - Which generic parameters are real caller choices, and which follow from an implementation already selected?
> - Trace a proposed change through adapters and generic helpers, not just a concrete call where inference already works.
> - Before narrowing a trait, look for existing implementations that differ only in the parameters you plan to remove.
> - Check output and error policies independently; a useful associated output does not require an associated error.
> - Treat `PhantomData` as type-system information, and review auto traits before removing it.

## References

- [Associated types](../appendix/concept-index.md#associated-types): one result type per selected trait implementation.
- [Phantom data](../appendix/concept-index.md#phantom-data): zero-sized markers with real type-system effects.
- [Rust Reference: associated types](https://doc.rust-lang.org/reference/items/associated-items.html#associated-types) and [implementation generics](https://doc.rust-lang.org/reference/items/implementations.html#generic-implementations): declarations, projections, and constrained parameters.
- [E0207](https://doc.rust-lang.org/error_codes/E0207.html) and [E0119](https://doc.rust-lang.org/error_codes/E0119.html): unconstrained parameters and conflicting implementations.
- [`Iterator::Item`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#associatedtype.Item): the same distinction in a familiar standard-library trait.
- [nom PR #1626][nom-pr]: the actual refactor and its discussion of readability, inference, and a winnow counterexample.
- [Pinned generic parser and adapter][nom-before] and [pinned associated-type version][nom-after]: historical evidence behind the reduced exercise.

[nom-pr]: https://github.com/rust-bakery/nom/pull/1626
[nom-before]: https://github.com/rust-bakery/nom/blob/4c6e45eded6e76a227f8069ec3f77f8a14b988c8/src/internal.rs
[nom-after]: https://github.com/rust-bakery/nom/blob/bf8cddbdbdf768a2128b7817c4670da059351f9f/src/internal.rs
[nom-license]: https://raw.githubusercontent.com/rust-bakery/nom/bf8cddbdbdf768a2128b7817c4670da059351f9f/LICENSE
