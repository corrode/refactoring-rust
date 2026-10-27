# `fun_strings_ext`

> **Draft outline.** Bullet notes for the chapter; explanations still to be written.

- Review three string helpers and how callers combine them.
- Preserve `shout(s: &str) -> String`, `sparkle(s: &str) -> String`, and `spongebob_case(s: &str) -> String` in the first solution.
- Keep the helpers' current text transformations while considering a separate caller-facing design.

## Code to refactor

- Starter and its tests: `examples/24_fun_strings_ext.rs`.
- Workshop prompts and speaker notes: `slides/pages/24_fun_strings_ext.md`.
- `shout` delegates to string uppercase conversion.
- `sparkle` formats the input between two sparkle symbols and spaces.
- `spongebob_case` enumerates input characters, alternating uppercase and lowercase conversion.
- Each character conversion extends one output buffer.
- The composition example nests free functions: `sparkle(&shout(s.trim()))`.

## Discussion

<details class="discussion-question">
<summary>What is being counted when the casing alternates?</summary>

- `chars().enumerate()` counts Unicode scalar values, not bytes, grapheme clusters, or letters alone.
- Spaces, punctuation, emoji, and combining marks all occupy an input position.
- `a🦀b` becomes `A🦀B` because the emoji occupies the lowercase position.
- A combining mark also shifts the next letter's parity, even when it belongs visually to the preceding letter.
- Counting only letters or grapheme clusters would require a different contract.

</details>

<details class="discussion-question">
<summary>Why extend the string instead of pushing one character?</summary>

- Unicode casing can return more than one scalar value for a single input character.
- Uppercasing `ß` yields `SS`; lowercasing `İ` yields `i` followed by a combining dot.
- `extend` retains all characters produced by those casing iterators.
- Alternation follows the original input position, not the length of the expanded output.
- ASCII case conversion returns one character but leaves non-ASCII letters unchanged.
- Both choices keep valid UTF-8; they do not provide the same transformation.

</details>

<details class="discussion-question">
<summary>Can these helpers become methods on <code>str</code>?</summary>

- A crate cannot add inherent methods to the foreign type `str`.
- A local extension trait can be implemented for `str`, making the methods available when the trait is in scope.
- The relevant coherence rule permits this implementation because the trait is local.
- This changes caller syntax and imports; it is not part of the signature-preserving first solution.
- Ordinary method lookup through `String`'s dereference to `str` allows chaining without another implementation for `String`.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Main solution

- Source: `solutions/24_fun_strings_ext.rs`, keeping all three free functions.
- `shout` remains Unicode uppercase conversion, and `sparkle` uses captured formatting arguments without changing output.
- `spongebob_case` folds over enumerated characters, passing its growing `String` through each step.
- Keep `extend` in both branches to preserve Unicode expansions.
- The fold owns the accumulator but does not clone it for every iteration.
- The original loop already implements the same behavior and may be easier to read.
- Do not frame changing a loop to a fold as a correctness fix or a necessary improvement.
- The free-function call site still nests inside-out; the first solution deliberately does not solve that API question.
- Traversal and output construction remain linear in input plus output bytes, with storage for the returned string.
- No benchmark in the exercise establishes a loop-versus-fold performance advantage.

### Alternative: the `FunStr` extension trait

- Source: `solutions/24_fun_strings_ext_alt.rs`, defining `FunStr` and implementing it for `str`.
- Move caller syntax from nested functions to `s.trim().shout().sparkle()` and bring `FunStr` into scope.
- Update tests to call methods; the alternative does not retain the original free-function entry points.
- Borrowed `&self` receivers do not consume existing strings, while each helper still returns an owned `String`.
- Chaining therefore does not fuse transformations or eliminate intermediate owned results.
- `shout` still uses Unicode uppercase conversion, but this alternative's `spongebob_case` uses ASCII-only case mapping.
- For `éÉ`, starter and main produce `Éé`, while the alternative leaves `éÉ` unchanged.
- The alternative still accepts arbitrary UTF-8 text; its type does not restrict callers to ASCII.
- This combines an API redesign with a non-ASCII behavior change, not just a different call-site style.
- A Unicode-preserving extension trait could delegate to the existing helpers, but that is a proposed variation rather than the current alternative.
- Keep a free-function API when trait imports, method-name collisions, or a small number of call sites make chaining less useful.

<!-- TODO(prose): Decide whether the ASCII alternative should remain an explicit contrasting contract or be replaced later by a Unicode-preserving extension trait.
The slides flag the behavior change, but the alternative source's shorter-pipeline explanation is not sufficient to establish equivalence. -->

### Edge cases and test coverage

- Current starter tests cover shouting ASCII text, sparkle formatting, ASCII alternating case, and nested composition.
- Current main tests add `straße` uppercasing, all three empty-string cases, and Unicode/position regression cases.
- Those regressions cover `éÉ`, `ßİx`, emoji, combining marks, spaces, and punctuation.
- `ßİx` becomes `SSi\u{307}X`; the inserted characters do not alter the parity of the later input `x`.
- Empty input remains empty for shout and alternating case, but sparkle produces `✨  ✨` with two spaces.
- Current alternative tests use the same ASCII scenarios with method calls and do not cover its changed non-ASCII behavior.
- Proposed alternative tests: assert its current non-ASCII behavior explicitly, or reuse Unicode expectations only after an intentional implementation change.
- Proposed semantic test: compare composed `áb` with decomposed `a\u{301}b` to show that scalar-position alternation is not grapheme-aware or normalization-invariant.
- These methods do not perform locale-specific casing or Unicode normalization.
- No numeric overflow is central to these algorithms; output growth and allocation remain resource limits for very large strings.

### Trade-offs

<!-- Final comparison matrices are deferred while this chapter is a bullet outline. -->

- **Type-safety:** **Original:** any UTF-8 string and owned output; main and trait retain that domain, but no type advertises or enforces the alternative's ASCII-only case mapping.
- **Readability:** **Original:** a direct loop and nested helper calls; main's fold is a style choice, while trait chaining improves call order only for callers familiar with its imports.
- **API design:** **Original:** three free functions; main preserves them, while `FunStr` changes names in scope and call syntax without changing ownership of the results.
- **Performance:** **Original:** one growing result per helper and intermediate strings during composition; main retains that work, and the trait chain does not remove those allocations.
- **Flexibility:** **Original:** helpers callable without a trait import; an extension trait composes with `str` methods and autoderef, but adds method-discovery and name-collision considerations.
- **Error reporting:** **Original:** transformations return strings without errors; all versions keep that policy, and the ASCII alternative silently leaves non-ASCII casing unchanged.

</details>

## Takeaways For Our Own Codebase

> - Decide whether “character” means a byte, scalar value, grapheme cluster, or letter before rewriting text transformations.
> - Review API ergonomics and text semantics as separate changes.
> - Keep the original loop when an iterator rewrite does not make the work easier to follow.

## References

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars): scalar values rather than bytes or grapheme clusters.
- [`char::to_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.to_uppercase) and [`to_lowercase`](https://doc.rust-lang.org/std/primitive.char.html#method.to_lowercase): potentially expanding Unicode mappings.
- [`char::to_ascii_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.to_ascii_uppercase): the narrower behavior used in the alternative.
- [Trait implementation coherence](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence): why a local trait can extend `str`.
- [Whitespace contracts in `trim_log_line`](20-trim-log-line.md#discussion): replacing text helpers can change behavior outside ASCII examples.
