# `path`

This function checks whether a path has the extension `rs`.
Refactor it without changing its signature or results.
Keep the check case-sensitive and based only on the path's syntax, including the existing results for paths without an extension and for dotfiles.

Run the local tests with `cargo test --example 05_path`.

## Code to refactor

```rust,editable
{{#include ../../../examples/05_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/05_path.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/05_path.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>Why does <code>Path::extension</code> return <code>Option&lt;&amp;OsStr&gt;</code> instead of <code>Option&lt;&amp;str&gt;</code>?</summary>

`Path::extension` returns `None` when the final path component has no extension.
If an extension exists, the method returns `&OsStr` because platform paths aren't guaranteed to be valid UTF-8.

</details>

<details class="discussion-question">
<summary>When does <code>OsStr::to_str</code> return <code>None</code>?</summary>

Calling `to_str` adds a second `Option`: the extension is present, but a non-UTF-8 value cannot become `&str`.
In that case, `to_str` returns `None`.

</details>

<details class="discussion-question">
<summary>Do we need to convert the extension to compare it with <code>"rs"</code>?</summary>

`OsStr` supports direct comparison with `str`, so `ext == "rs"` performs the same check without a separate UTF-8 validity branch.
Neither version allocates a `String`.

</details>

<details class="discussion-question">
<summary>What do the two <code>None</code> branches return?</summary>

Both return `false`: a missing extension cannot match, and a non-UTF-8 extension cannot equal `"rs"`.

</details>

<details class="discussion-question">
<summary>Why does <code>.rs</code> return <code>false</code> while <code>main.rs</code> returns <code>true</code>?</summary>

A leading dot alone doesn't create an extension.
`Path` treats `.rs` as a file stem with no extension, while `main.rs` has the extension `rs`.
The `.rs` test preserves that distinction.

</details>

<details class="discussion-question">
<summary>Does this function access the filesystem?</summary>

No, it only checks the path's syntax.
It doesn't check whether the path exists or points to a regular file.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/05_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/05_path.rs:tests}}
</script>


`Option::is_some_and` returns `false` for `None`.
For `Some(ext)`, it runs the predicate and returns that result.
One expression therefore covers both outer branches from the starter.

The closure compares `&OsStr` directly with `"rs"`.
A non-UTF-8 extension still returns `false`.
The tested results are unchanged: `main.rs` and `src/lib.rs` match; `Cargo.toml`, `README`, and `.rs` do not.

### Alternative approaches

The solution file also includes `map_or`, which is just as valid here:

```rust,editable
# use std::path::Path;
pub fn is_rust_source_file(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "rs")
}
```

`map_or(false, predicate)` returns `false` for `None` and applies the predicate to `Some`.
`is_some_and` expresses this presence-and-predicate check directly.
Use `map_or` if your team finds it clearer or supports a Rust version that predates `is_some_and`.

An explicit `match` is sound too:

```rust,editable
# use std::path::Path;
pub fn is_rust_source_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ext == "rs",
        None => false,
    }
}
```

This version removes the UTF-8 conversion but leaves the two cases in view.
Choosing between it and `is_some_and` is mostly about which form your team reads faster.

### Edge cases and production concerns

> **EDGE CASE**
>
> `Path::extension` classifies path syntax.
> It does not establish that the path exists or that an existing filesystem object is a regular file.

A directory called `generated.rs` can return `true`.
So can a path to a file that does not exist.
If the contract requires an existing regular file, check filesystem metadata separately.
That check performs I/O, can fail, and leaves races between inspection and later use.
Hiding it inside this small lexical predicate would require a different interface and documentation.

The comparison is exact and case-sensitive, so `lib.RS` does not match.
The dotfile `.rs` has no extension, while `.config.rs` has the extension `rs`.
For `lib.rs.bak`, the extension is `bak`.

A non-UTF-8 extension compares unequal to `"rs"`, producing `false`.
That policy works for a yes-or-no classifier.
If the caller must tell “not Rust” apart from “the extension cannot be represented as UTF-8,” a `bool` is too small.
The function needs a richer return type and an explicit encoding policy.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | `is_some_and` | `map_or(false, …)` | Explicit `match` on extension |
|---|---|---|---|---|
| Type-safety | No filesystem guarantees | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No filesystem guarantees | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No filesystem guarantees | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No filesystem guarantees |
| Readability | Two nested absence checks | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> One presence-and-predicate expression | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Explicit default, no nesting | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Two cases, no nesting |
| API design | `&Path` → `bool` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same borrowed predicate | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same borrowed predicate | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same borrowed predicate |
| Performance | No allocation; validates UTF-8 | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No allocation; skips UTF-8 validation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No allocation; skips UTF-8 validation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No allocation; skips UTF-8 validation |
| Flexibility | Exact, case-sensitive extension check | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same matching policy | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same matching policy | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same matching policy |
| Error reporting | All non-matches become `false` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No reason for non-match | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No reason for non-match | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No reason for non-match |

Skipping UTF-8 validation is not a measured speed-up.
`is_some_and` requires Rust 1.70; `map_or` and explicit `match` work on older versions.

</details>

## Takeaways For Our Own Codebase

> - Should your application compare extensions case-sensitively on every platform?
> - Is a non-UTF-8 extension just a non-match, or does the caller need an error?
> - Does the function name promise more than a lexical extension check can deliver?
> - Which call site, if any, owns the checks for existence and file type?
> - Would your team understand `is_some_and`, `map_or(false, ...)`, or an explicit `match` fastest?

## References

- [`Path::extension`](../appendix/concept-index.md#path-extension): inspect the suffix of the final path component without touching the filesystem.
- [`OsStr` and non-UTF-8 paths](../appendix/concept-index.md#osstr-and-non-utf-8-paths): keep platform path data in its native string type until a text operation requires conversion.
- [`Option::is_some_and`](../appendix/concept-index.md#option-is-some-and): ask whether a value is present and satisfies a predicate in one expression.
- [`Option::map_or`](../appendix/concept-index.md#option-map-or): choose a result for `None` and transform the value inside `Some`.
- [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): classify path syntax without claiming that a filesystem object exists or has a particular identity.
