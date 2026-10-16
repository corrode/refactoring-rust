# `excluded_path`

This function checks whether a path starts with any configured exclusion prefix.
Refactor it while preserving its signature and behaviour: absent or empty exclusions return `false`.
Afterward, consider interface changes separately from the implementation.

Run the tests with `cargo test --example 13_excluded_path`.

## Code to refactor

```rust,editable
{{#include ../../../examples/13_excluded_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/13_excluded_path.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/13_excluded_path.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What does the optional set distinguish?</summary>

`&Option<HashSet<PathBuf>>` distinguishes a missing collection from a present one, which may be empty.
The function borrows the `Option` that owns the set.

</details>

<details class="discussion-question">
<summary>Does the prefix check need that distinction?</summary>

The tests return `false` for both `None` and `Some(HashSet::new())`.
If callers don't need to distinguish them, an empty collection can represent “exclude nothing.”
Configuration loading or merging may still need to distinguish “missing” from “present but empty.”
Keep that distinction in the layers that need it; the prefix check does not.

</details>

<details class="discussion-question">
<summary>Which iterator method finds any matching prefix?</summary>

The nested `if let`, `for`, and `if` check whether any prefix matches.
The loop stops at the first match.
`Iterator::any` expresses the same check and stops at the same point.

</details>

<details class="discussion-question">
<summary>Can we handle absence before searching?</summary>

The outer `if let` handles the case where there is no collection to search.
Returning early for `None` removes one level of nesting from the matching code.
This simplifies control flow without changing the parameter type.

</details>

<details class="discussion-question">
<summary>Does <code>HashSet</code> speed up this prefix search?</summary>

This search checks entries one by one rather than using hash lookups.
Its worst case is still a linear scan through all prefixes.
The set does prevent duplicate prefixes, but `contains` would check for an exact path, not a prefix.

</details>

<details class="discussion-question">
<summary>What does <code>Path::starts_with</code> compare?</summary>

It compares path components, not raw string prefixes or filesystem locations.
For example, `/tmp/file` starts with `/tmp`, but `/tmpfile` does not.
It does not resolve `..`, follow symbolic links, canonicalize relative paths, or protect against filesystem races.
A prefix match alone cannot prove filesystem containment.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/13_excluded_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/13_excluded_path.rs:tests}}
</script>

`let ... else` returns for `None` before the search starts.
After that statement, `excluded` is a borrowed `HashSet<PathBuf>`, and the prefix check needs no enclosing branch.

The final expression checks whether any prefix satisfies `path.starts_with(prefix)`.
`any` stops at the first `true`, just as the starter returns from its loop.
The refactor allocates nothing, and its worst case is still a linear scan through all prefixes.

The signature stays the same, so callers and tests need no changes.
Changing the parameter types is a separate API decision.
Neither `let ... else` nor `any` requires it.

### Alternative approaches

The current signature combines two choices: exclusions may be absent, and the function borrows the `Option` that owns the set.

If absence means something in the domain, accept an optional borrow instead:

```rust,ignore
pub fn is_excluded(
    path: &Path,
    excluded: Option<&HashSet<PathBuf>>,
) -> bool {
    excluded.is_some_and(|prefixes| {
        prefixes.iter().any(|prefix| path.starts_with(prefix))
    })
}
```

A caller with an `Option<HashSet<PathBuf>>` can pass `excluded.as_ref()`.
The function receives an optional reference to the set rather than a reference to the original `Option`.

If absence and emptiness mean the same thing, remove `Option` and put the candidate path first:

```rust,editable
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/13_excluded_path_alt.rs:plain_function}}
```

This signature removes the distinction between absent and empty exclusions.
It still exposes `HashSet`, although this prefix search checks entries one by one rather than using hash lookups.
The set does prevent duplicate prefixes.

A dedicated type can hide the collection and provide the exclusion check as a method:

```rust,editable
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/13_excluded_path_alt.rs:excluder}}
```

`Excluder` groups the exclusion rules and the matching method.
It can also normalize and validate prefixes if needed.
Keeping storage private lets the implementation use a different index later without changing callers.
Its default value excludes nothing, which preserves a valid state from the current tests.

Run both alternatives with:

```sh
cargo test --example 13_excluded_path_alt
```

An extension trait allows the call `path.is_excluded(&prefixes)`.
This adds a project-specific method to the standard-library `Path` type.
Callers using method syntax must import the trait.
The implementation must still check prefixes:

```rust,ignore
impl Excludable for Path {
    fn is_excluded(&self, excluded: &HashSet<PathBuf>) -> bool {
        excluded.iter().any(|prefix| self.starts_with(prefix))
    }
}
```

`excluded.contains(self)` checks for an exact path, not a prefix.
It would stop `/tmp` from matching `/tmp/foo.log`.

A non-empty `Excluder` is another possible invariant, but it contradicts the current requirement that an empty rule set is valid.
Add that restriction only when the domain requires at least one exclusion.

### Edge cases and production concerns

> **EDGE CASE**
>
> `Path::starts_with` performs a lexical, component-aware comparison.
> That is useful for path filtering, but it cannot prove that a filesystem object remains below a directory.

For example, `/tmp/file` starts with `/tmp`, but `/tmpfile` does not.
The method does not resolve `..`, follow symbolic links, canonicalize relative paths, or protect against filesystem races.
If this predicate guards a security boundary, it also needs a documented normalization and filesystem policy.

An empty prefix needs an explicit rule because it can match every path.
Absolute and relative prefixes also need a defined relationship before you mix them.
Put those rules in configuration validation or a domain type such as `Excluder`, rather than making every call site remember them.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

#### Keeping an optional parameter

| Criterion | Original | `let ... else` + `any` (main) | `Option<&HashSet<PathBuf>>` |
| --- | --- | --- | --- |
| Type-safety | No validated path invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No added path invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No added path invariant |
| Readability | Nested absence and match checks | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Early return, named search | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Compact, familiarity-dependent combinators |
| API design | Borrows optional owning set | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same public signature | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Optional borrow without owning wrapper |
| Performance | Short-circuiting linear prefix scan | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same prefix scan | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same prefix scan |
| Flexibility | Requires optional `HashSet` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same container requirement | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Borrows standalone sets too |
| Error reporting | Boolean match, no failure channel | N/A | N/A |

#### Changing the interface

| Criterion | Original | Plain `&HashSet<PathBuf>` | `Excluder` | `Excludable` for `Path` |
| --- | --- | --- | --- | --- |
| Type-safety | No validated path invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No added path invariant | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Constructor does not validate | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> No added path invariant |
| Readability | Nested absence and match checks | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> No absence branch | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Named rules and operation | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Method syntax, trait import |
| API design | Borrows optional owning set | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Removes unused absence distinction | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Hides storage behind methods | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Method still exposes set |
| Performance | Short-circuiting linear prefix scan | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same prefix scan | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same scan after construction | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same prefix scan |
| Flexibility | Requires optional `HashSet` | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> No owning `Option` required | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Storage can change privately | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Still requires `HashSet` |
| Error reporting | Boolean match, no failure channel | N/A | N/A | N/A |

Readability depends on your team's iterator familiarity; loops remain easier to step through.
Removing `Option` changes the interface, not path validity.
Preserve absence in configuration layers that need it.
`Excluder::new` collects owned prefixes into a set; the scan score excludes construction.
None provides filesystem containment guarantees.

</details>

## Takeaways For Our Own Codebase

> - Does your configuration layer distinguish “missing” from “configured but empty”?
>   Which layer needs that information?
> - Would callers read more naturally with the candidate path first?
> - Does uniqueness justify exposing `HashSet`, or should the storage stay private?
> - Which normalization rules belong in an `Excluder` constructor?
> - If prefix matching becomes a hot path, which data structure could avoid inspecting every rule?

## References

- [`let ... else`](../appendix/concept-index.md#let-else): handle a non-matching case early and keep the main path unindented.
- [`Iterator::any`](../appendix/concept-index.md#iterator-any): express a check for at least one match and stop when found.
- [Optional borrows](../appendix/concept-index.md#optional-borrows): use `Option<&T>` when a function needs an optional view rather than a borrowed owning wrapper.
- [Empty collections as identity values](../appendix/concept-index.md#empty-collections): use an empty collection for “no configured items” when absence carries no additional information.
- [Newtypes](../appendix/concept-index.md#newtype): hide storage and put domain policy behind a dedicated interface.
- [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): path component checks do not resolve filesystem identity.
