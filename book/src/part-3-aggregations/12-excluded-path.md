# 12 · `excluded_path`

## The problem

A file scanner receives a candidate path and a configured set of directory prefixes.
It must skip the path when any prefix contains it. The starter handles optional
configuration, loops over every prefix, and stops at the first match. Its control flow
can be shorter, but the larger question sits in the signature: this function treats
missing configuration and an empty set exactly the same.

## Starter code

> **EXERCISE**
>
> Review the implementation and its interface separately. First preserve the current
> behaviour. Then decide whether the parameter types describe a distinction the
> function actually uses.

```rust
{{#include ../../../examples/12_excluded_path.rs}}
```

> **TRY THIS**
>
> Run the tests, then refactor `examples/12_excluded_path.rs`:
>
> ```sh
> cargo test --example 12_excluded_path
> ```
>
> Pay particular attention to the tests for `None` and an empty `HashSet`.

## Review

> **REVIEW**
>
> - What distinction does `&Option<HashSet<PathBuf>>` expose to the function?
> - Do the tests require that distinction to survive?
> - What iterator operation means “does any element satisfy this predicate?”
> - Can the absence case return before the matching logic begins?
> - Does `HashSet` make this prefix search faster than another collection would?
> - Does `Path::starts_with` compare raw strings, path components, or filesystem
>   locations?

The nested `if let`, `for`, and `if` implement an existential query: does any prefix
match? The manual loop already short-circuits by returning on the first match.
`Iterator::any` names the same operation and has the same short-circuit behaviour.

The outer `if let` handles a valid case where there is no collection to search. An
early return can keep that branch separate from the main path. This is a local
readability change. It does not answer whether `Option` belongs in the interface.

The tests expose the API question. `None` returns `false`, and `Some(HashSet::new())`
also returns `false`. If callers and this function attach no further meaning to that
difference, an empty collection already represents “exclude nothing.” If absence has
meaning elsewhere in configuration handling, the distinction may still belong at
that outer layer.

## A possible solution

```rust
{{#include ../../../solutions/12_excluded_path.rs}}
```

`let ... else` handles `None` first. After the statement, `excluded` is a borrowed
`HashSet<PathBuf>`, so the prefix search stays at the main indentation level.

The final expression asks whether any prefix satisfies
`path.starts_with(prefix)`. `any` stops polling the iterator after the first `true`,
just as the starter returns from its loop. The refactor adds no allocation and keeps
the worst-case linear scan.

This solution deliberately leaves the signature alone. A reviewer can suggest the
control-flow refactor with confidence because the tests and call sites need no
changes. Altering the parameter types needs a separate API discussion.

## Alternative approaches

The current signature combines two independent choices: whether exclusions are
optional, and whether this function borrows the owning `Option`.

If absence carries domain meaning, an optional borrow asks for less:

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

A caller with `Option<HashSet<PathBuf>>` can pass `excluded.as_ref()`. The function no
longer needs access to the owning wrapper.

If absence and emptiness are equivalent, we can remove `Option` and put the subject
first:

```rust
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/12_excluded_path_alt.rs:plain_function}}
```

This is the smallest interface that matches the tested behaviour. It still exposes
`HashSet` even though hashing does not accelerate a query that visits entries one by
one. The set may still be useful for deduplicating configuration values.

A domain type can hide that storage choice and own the matching policy:

```rust
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/12_excluded_path_alt.rs:excluder}}
```

`Excluder` gives path policy a place to live. That policy can cover normalization and
validation now, then adopt a different index later. Its default value excludes nothing,
preserving a valid state from the current tests.
The alternatives are runnable with:

```sh
cargo test --example 12_excluded_path_alt
```

An extension trait could make the call read `path.is_excluded(&prefixes)`. That puts a
local policy method on the standard-library `Path` type and requires the trait to be in
scope. The implementation must also retain prefix matching:

```rust,ignore
impl Excludable for Path {
    fn is_excluded(&self, excluded: &HashSet<PathBuf>) -> bool {
        excluded.iter().any(|prefix| self.starts_with(prefix))
    }
}
```

Using `excluded.contains(self)` would check exact set membership and break the nested
path case.

A non-empty `Excluder` would encode a stronger invariant, but that invariant conflicts
with the current requirement that an empty rule set is valid. We should not add a type
restriction until the domain requires it.

## Edge cases and production concerns

> **EDGE CASE**
>
> `Path::starts_with` performs a lexical, component-aware comparison. It is useful for
> filtering paths, but it does not prove that a filesystem object stays below a
> directory.

For example, `/tmp/file` starts with `/tmp`, while `/tmpfile` does not. The method does
not resolve `..`, follow symbolic links, canonicalize relative paths, or protect
against filesystem races. Security-sensitive containment needs a documented
normalization and filesystem policy.

An empty prefix also deserves an explicit decision because it can match every path.
Absolute and relative prefixes should not be mixed without defining how they relate.
These rules belong in configuration validation or in a domain type such as `Excluder`,
rather than being rediscovered at each call site.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `let ... else` and `any` refactor with the
> starter. The alternative signatures change the public interface and need their own
> review.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept the same optional set and encode no new path invariant. |
| Readability | wins | The absence case exits first, then `any` states the matching question without nested control flow. |
| API design | neutral | The signature still borrows an `Option<HashSet<PathBuf>>` and preserves two states with the same result here. |
| Performance | neutral | Both versions allocate nothing, stop at the first match, and scan all prefixes in the worst case. |
| Flexibility | neutral | The concrete optional container remains part of the interface; only the implementation changes. |
| Error reporting | N/A | Missing rules and non-matching paths are ordinary `false` results under the current contract. |

A loop may be easier to debug for a team new to iterator predicates. In this function,
`any` is short, names the operation, and preserves the early exit, so the readability
trade favours the refactor.

## Quiz

### 1. Which plain-function signature best matches the current tests?

```rust,ignore
fn is_excluded(excluded: &Option<HashSet<PathBuf>>, path: &Path) -> bool
fn is_excluded(path: &Path, excluded: &HashSet<PathBuf>) -> bool
fn is_excluded(path: &Path, excluded: Option<&HashSet<PathBuf>>) -> bool
```

*Answer:* The second signature has the fewest states when absence and emptiness mean
the same thing. Choose the third when absence must remain observable. The first asks
the function to borrow an owning wrapper it does not otherwise need.

### 2. Can `excluded.contains(path)` replace the call to `any`?

*Answer:* No. `contains` tests whether the full path is an element of the set. The
function needs `/tmp` to match `/tmp/foo.log`, which requires prefix comparison.

### 3. Should `Excluder` require at least one prefix?

*Answer:* Only if running without exclusions is invalid in the domain. The current
tests require “exclude nothing” to be valid, so a mandatory non-empty collection would
remove a supported state.

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
> - If prefix matching becomes a hot path, which data structure could avoid scanning
>   every rule?

## Concepts recap

> **CONCEPT**
>
> - [`let ... else`](../appendix/concept-index.md#let-else): handle a non-matching case
>   early and keep the main path unindented.
> - [`Iterator::any`](../appendix/concept-index.md#iterator-any): express an
>   existential test with short-circuit evaluation.
> - [Optional borrows](../appendix/concept-index.md#optional-borrows): use `Option<&T>`
>   when a function needs an optional view rather than a borrowed owning wrapper.
> - [Empty collections as identity values](../appendix/concept-index.md#empty-collections):
>   let an empty collection represent “no configured items” when absence adds no
>   information.
> - [Newtypes](../appendix/concept-index.md#newtype): hide storage and put domain policy
>   behind a dedicated interface.
> - [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): path
>   component checks do not resolve filesystem identity.
