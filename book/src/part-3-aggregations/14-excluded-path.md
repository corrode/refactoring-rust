# `excluded_path`

This function checks whether a path starts with any configured exclusion prefix.
Refactor it while preserving its signature and behaviour: absent or empty exclusions return `false`.
Then consider whether you would change the interface as a separate step.

Run the tests with `cargo test --example 14_excluded_path`.

## Code to refactor

```rust,editable
{{#include ../../../examples/14_excluded_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/14_excluded_path.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>What does the optional set distinguish?</summary>

`&Option<HashSet<PathBuf>>` distinguishes a missing collection from a present one, which may be empty.
The function borrows the `Option` that owns the set.

</details>

<details class="discussion-question">
<summary>Does the prefix check need that distinction?</summary>

No, the function returns `false` for both `None` and `Some(HashSet::new())`, as the tests confirm.
If callers don't need the distinction either, use an empty collection to mean “exclude nothing.”
Configuration loading or merging may still need to distinguish “missing” from “present but empty.”
Keep that distinction in the layers that need it; the prefix check does not.

</details>

<details class="discussion-question">
<summary>Which iterator method finds any matching prefix?</summary>

`Iterator::any` checks for at least one match and stops as soon as it finds one.
The nested `if let`, `for`, and `if` perform the same check.
Replacing the loop with `any` preserves that early exit.

</details>

<details class="discussion-question">
<summary>Can we handle absence before searching?</summary>

Yes, return early for `None` before searching the collection.
The outer `if let` currently handles absence by nesting the search inside a branch.
An early return removes that nesting without changing the parameter type.

</details>

<details class="discussion-question">
<summary>Does <code>HashSet</code> speed up this prefix search?</summary>

No, this search checks entries one by one rather than using hash lookups.
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
{{#include ../../../solutions/14_excluded_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/14_excluded_path.rs:tests}}
</script>

`let ... else` returns for `None` before the search starts.
After that statement, `excluded` is a borrowed `HashSet<PathBuf>`, so you can check prefixes without another enclosing branch.

`any` calls `path.starts_with(prefix)` for each prefix until one returns `true`.
It then stops, just as the starter returns from its loop.
The refactor allocates nothing, and its worst case is still a linear scan through all prefixes.

The signature stays the same, so callers and tests need no changes.
Changing the parameter types is a separate API decision.
Neither `let ... else` nor `any` requires it.

### Alternative approaches

Do you need optional exclusions, a borrowed owning `Option`, or both?
The current signature requires both.

If absence means something in the domain, accept an optional borrow instead:

```rust,ignore,noplayground
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
{{#include ../../../solutions/14_excluded_path_alt.rs:plain_function}}
```

<script type="text/plain" class="playground-tests">
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excludes_paths_below_a_prefix() {
        let excluded = HashSet::from([PathBuf::from("/tmp")]);
        assert!(is_excluded(Path::new("/tmp/foo.log"), &excluded));
        assert!(!is_excluded(Path::new("/home/me/x"), &excluded));
    }

    #[test]
    fn path_prefixes_respect_component_boundaries() {
        let excluded = HashSet::from([PathBuf::from("/tmp")]);
        assert!(!is_excluded(Path::new("/tmpfile"), &excluded));
    }

    #[test]
    fn an_empty_set_excludes_nothing() {
        assert!(!is_excluded(Path::new("/tmp/foo.log"), &HashSet::new()));
    }
}
</script>

This signature removes the distinction between absent and empty exclusions.
It still exposes `HashSet`, although this prefix search checks entries one by one rather than using hash lookups.
The set does prevent duplicate prefixes.

To hide the collection from callers, give it a dedicated type with an exclusion-checking method:

```rust,editable
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/14_excluded_path_alt.rs:excluder}}
```

<script type="text/plain" class="playground-tests">
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excluder_owns_the_matching_policy() {
        let excluder = Excluder::new([PathBuf::from("/tmp")]);
        assert!(excluder.is_excluded(Path::new("/tmp/foo.log")));
        assert!(!excluder.is_excluded(Path::new("/home/me/x")));
        assert!(!excluder.is_excluded(Path::new("/tmpfile")));
    }

    #[test]
    fn default_excluder_excludes_nothing() {
        assert!(!Excluder::default().is_excluded(Path::new("/tmp/foo.log")));
    }

    #[test]
    fn empty_constructor_excludes_nothing() {
        assert!(!Excluder::new([]).is_excluded(Path::new("/tmp/foo.log")));
    }
}
</script>

`Excluder` keeps the exclusion rules and the matching method together.
You can add prefix normalization and validation here if needed.
Private storage lets you switch to a different index later without changing callers.
The default value excludes nothing, just as the current tests require for an empty set.

Run both alternatives with:

```sh
cargo test --example 14_excluded_path_alt
```

If you prefer `path.is_excluded(&prefixes)`, an extension trait can add that project-specific method to the standard-library `Path` type.
Callers must import the trait to use method syntax.
The implementation must still check prefixes:

```rust,ignore,noplayground
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

Decide whether to allow an empty prefix: it can match every path.
Define how absolute and relative prefixes should interact before mixing them.
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

Choose based on your team's familiarity with iterators.
Loops remain easier to step through.
Removing `Option` changes the interface, not path validity.
Preserve absence in configuration layers that need it.
`Excluder::new` collects owned prefixes into a set.
The performance comparison covers the scan, not construction.
None of these versions guarantees filesystem containment.

</details>

## Takeaways For Our Own Codebase

> - Does your configuration layer distinguish “missing” from “configured but empty”?
>   Which layer needs that information?
> - Read a few call sites with the candidate path first.
>   Does that order make the call clearer?
> - Do callers need `HashSet`'s uniqueness guarantee, or could you keep the storage private?
> - Decide how to handle empty, absolute, and relative prefixes in configuration validation or an `Excluder` constructor.
> - If prefix matching becomes a hot path, which data structure could avoid inspecting every rule?

## References

- [`let ... else`](../appendix/concept-index.md#let-else): handle a non-matching case early and keep the main path unindented.
- [`Iterator::any`](../appendix/concept-index.md#iterator-any): express a check for at least one match and stop when found.
- [Optional borrows](../appendix/concept-index.md#optional-borrows): use `Option<&T>` when a function needs an optional view rather than a borrowed owning wrapper.
- [Empty collections as identity values](../appendix/concept-index.md#empty-collections): use an empty collection for “no configured items” when absence carries no additional information.
- [Newtypes](../appendix/concept-index.md#newtype): hide storage and put domain policy behind a dedicated interface.
- [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): path component checks do not resolve filesystem identity.
