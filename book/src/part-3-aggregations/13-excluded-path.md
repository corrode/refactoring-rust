# 13 · `excluded_path`

## Code to refactor

> **EXERCISE**
>
> Review the implementation and the interface as two separate concerns. Preserve the
> current behaviour first. Then check whether the parameter types express a distinction
> this function uses.

```rust,editable
{{#include ../../../examples/13_excluded_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/13_excluded_path.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/13_excluded_path.rs:tests}}
</script>

> **TRY THIS**
>
> Run the tests, then refactor `examples/13_excluded_path.rs`:
>
> ```sh
> cargo test --example 13_excluded_path
> ```
>
> Compare the tests for `None` and an empty `HashSet`. Do they ask for different
> results?

## Discussion

> **DISCUSSION**
>
> - Which distinction does `&Option<HashSet<PathBuf>>` expose to the function?
> - Do the tests need that distinction to survive?
> - Which iterator operation asks “does any element satisfy this predicate?”
> - Can the absence case return before the prefix check begins?
> - Does `HashSet` make this prefix search faster than another collection would?
> - Does `Path::starts_with` compare raw strings, path components, or filesystem
>   locations?

The nested `if let`, `for`, and `if` check whether any prefix matches. The loop stops
at the first match. `Iterator::any` expresses the same check and stops at the same point.

The outer `if let` handles the case where there is no collection to search. Returning
early for `None` removes one level of nesting from the matching code. This simplifies
control flow without changing the parameter type.

The tests return `false` for both `None` and `Some(HashSet::new())`. If callers don't
need to distinguish them, an empty collection can represent “exclude nothing.”
Configuration loading or merging may still need to distinguish “missing” from “present
but empty.” Keep that distinction in the layers that need it; the prefix check does not.

<details class="solution" id="a-possible-solution">
<summary><h2><span aria-hidden="true">💡</span> A possible solution</h2></summary>

```rust,editable
{{#include ../../../solutions/13_excluded_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/13_excluded_path.rs:tests}}
</script>

`let ... else` returns for `None` before the search starts. After that statement,
`excluded` is a borrowed `HashSet<PathBuf>`, and the prefix check needs no enclosing
branch.

The final expression checks whether any prefix satisfies `path.starts_with(prefix)`.
`any` stops at the first `true`, just as the starter returns from its loop.
The refactor allocates nothing, and its worst case is still a linear scan through all
prefixes.

The signature stays the same, so callers and tests need no changes. Changing the
parameter types is a separate API decision. Neither `let ... else` nor `any` requires it.

</details>

## Alternative approaches

The current signature combines two choices: exclusions may be absent, and the function
borrows the `Option` that owns the set.

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

A caller with an `Option<HashSet<PathBuf>>` can pass `excluded.as_ref()`. The function
receives an optional reference to the set rather than a reference to the original
`Option`.

If absence and emptiness mean the same thing, remove `Option` and put the candidate path
first:

```rust,editable
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/13_excluded_path_alt.rs:plain_function}}
```

This signature removes the distinction between absent and empty exclusions. It still
exposes `HashSet`, although this prefix search checks entries one by one rather than
using hash lookups. The set does prevent duplicate prefixes.

A dedicated type can hide the collection and provide the exclusion check as a method:

```rust,editable
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/13_excluded_path_alt.rs:excluder}}
```

`Excluder` groups the exclusion rules and the matching method. It can also normalize
and validate prefixes if needed. Keeping storage private lets the implementation use a
different index later without changing callers. Its default value excludes nothing,
which preserves a valid state from the current tests.

Run both alternatives with:

```sh
cargo test --example 13_excluded_path_alt
```

An extension trait allows the call `path.is_excluded(&prefixes)`. This adds a
project-specific method to the standard-library `Path` type. Callers using method syntax
must import the trait. The implementation must still check prefixes:

```rust,ignore
impl Excludable for Path {
    fn is_excluded(&self, excluded: &HashSet<PathBuf>) -> bool {
        excluded.iter().any(|prefix| self.starts_with(prefix))
    }
}
```

`excluded.contains(self)` checks for an exact path, not a prefix. It would stop `/tmp`
from matching `/tmp/foo.log`.

A non-empty `Excluder` is another possible invariant, but it contradicts the current
requirement that an empty rule set is valid. Add that restriction only when the domain
requires at least one exclusion.

## Edge cases and production concerns

> **EDGE CASE**
>
> `Path::starts_with` performs a lexical, component-aware comparison. That is useful for
> path filtering, but it cannot prove that a filesystem object remains below a
> directory.

For example, `/tmp/file` starts with `/tmp`, but `/tmpfile` does not. The method does not
resolve `..`, follow symbolic links, canonicalize relative paths, or protect against
filesystem races. If this predicate guards a security boundary, it also needs a
documented normalization and filesystem policy.

An empty prefix needs an explicit rule because it can match every path. Absolute and
relative prefixes also need a defined relationship before you mix them. Put those rules
in configuration validation or a domain type such as `Excluder`, rather than making
every call site remember them.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `let ... else` and `any` refactor with the
> starter. The alternative signatures change the public interface and need a separate
> review.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept the same optional set, with no additional path invariant in the type system. |
| Readability | wins | The absence case returns first, and `any` states the matching question without nested control flow. |
| API design | neutral | The signature still borrows an `Option<HashSet<PathBuf>>`, preserving two states that have the same result here. |
| Performance | neutral | Both versions allocate nothing, stop at the first match, and inspect every prefix in the worst case. |
| Flexibility | neutral | The concrete optional container remains part of the interface; only the implementation changes. |
| Error reporting | N/A | Missing rules and non-matching paths are still ordinary `false` results under the current contract. |

A loop may be easier to inspect in a debugger, especially if your team is unfamiliar
with iterator methods. Here, the iterator expression is short: `any` names the check
and preserves the early exit.

## Quiz

### 1. Which plain-function signature best matches the current tests?

```rust,ignore
fn is_excluded(excluded: &Option<HashSet<PathBuf>>, path: &Path) -> bool
fn is_excluded(path: &Path, excluded: &HashSet<PathBuf>) -> bool
fn is_excluded(path: &Path, excluded: Option<&HashSet<PathBuf>>) -> bool
```

*Answer:* The second signature has the fewest states when absence and emptiness mean the
same thing. Use the third when absence must remain observable. The first makes the
function borrow an owning wrapper that it does not otherwise need.

### 2. Can `excluded.contains(path)` replace the call to `any`?

*Answer:* No. `contains` asks whether the full path is an element of the set. This
function needs `/tmp` to match `/tmp/foo.log`, so it must compare prefixes.

### 3. Should `Excluder` require at least one prefix?

*Answer:* Only when the domain says that running without exclusions is invalid. The
current tests treat “exclude nothing” as a valid state, so a mandatory non-empty
collection would remove supported behaviour.

### 4. Is this predicate enough for a filesystem security boundary?

*Answer:* No. It compares lexical path components. It does not resolve symbolic links,
`..`, mount changes, or races between checking and using a path.

## Follow-up questions

> **FOLLOW-UP**
>
> - Does your configuration layer distinguish “missing” from “configured but empty”?
>   Which layer needs that information?
> - Would callers read more naturally with the candidate path first?
> - Does uniqueness justify exposing `HashSet`, or should the storage stay private?
> - Which normalization rules belong in an `Excluder` constructor?
> - If prefix matching becomes a hot path, which data structure could avoid inspecting
>   every rule?

## Concepts recap

> **CONCEPT**
>
> - [`let ... else`](../appendix/concept-index.md#let-else): handle a non-matching case
>   early and keep the main path unindented.
> - [`Iterator::any`](../appendix/concept-index.md#iterator-any): express an
>   check for at least one match and stop when found.
> - [Optional borrows](../appendix/concept-index.md#optional-borrows): use `Option<&T>`
>   when a function needs an optional view rather than a borrowed owning wrapper.
> - [Empty collections as identity values](../appendix/concept-index.md#empty-collections):
>   use an empty collection for “no configured items” when absence carries no additional
>   information.
> - [Newtypes](../appendix/concept-index.md#newtype): hide storage and put domain policy
>   behind a dedicated interface.
> - [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): path
>   component checks do not resolve filesystem identity.
