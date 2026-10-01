# 12 · `excluded_path`

## The problem

A file scanner gets a candidate path and a set of directory prefixes. It should skip the
path as soon as one prefix contains it. The loop is not the difficult part. The awkward
bit is the interface: it distinguishes missing configuration from an empty set, even
though this function gives both states the same answer.

That leaves two separate jobs. First, make the prefix check say what it does. Then decide
whether the extra state in the signature carries information anywhere else in the
system.

## Starter code

> **EXERCISE**
>
> Review the implementation and the interface as two separate concerns. Preserve the
> current behaviour first. Then check whether the parameter types express a distinction
> this function uses.

```rust
{{#rustdoc_include ../../../examples/12_excluded_path.rs:listing}}
```

> **TRY THIS**
>
> Run the tests, then refactor `examples/12_excluded_path.rs`:
>
> ```sh
> cargo test --example 12_excluded_path
> ```
>
> Compare the tests for `None` and an empty `HashSet`. Do they ask for different
> results?

## Review

> **REVIEW**
>
> - Which distinction does `&Option<HashSet<PathBuf>>` expose to the function?
> - Do the tests need that distinction to survive?
> - Which iterator operation asks “does any element satisfy this predicate?”
> - Can the absence case return before the prefix check begins?
> - Does `HashSet` make this prefix search faster than another collection would?
> - Does `Path::starts_with` compare raw strings, path components, or filesystem
>   locations?

Ignore the nesting for a moment. The `if let`, `for`, and inner `if` ask one question:
does any prefix match? The loop already stops at the first match. `Iterator::any` names
that operation and short-circuits in the same place.

The outer `if let` handles another ordinary case: there is no collection to search. An
early return gets that case out of the way and leaves the matching code at the main
indentation level. That is a worthwhile control-flow cleanup, but it says nothing yet
about whether `Option` belongs in the interface.

For that, look at the tests. Both `None` and `Some(HashSet::new())` produce `false`. If
callers attach no further meaning to the distinction, the empty collection already says
“exclude nothing.” Configuration loading or merging may still need to tell “missing”
from “present but empty.” Keep that information there until another layer has a reason
to see it.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#rustdoc_include ../../../solutions/12_excluded_path.rs:listing}}
```

`let ... else` handles `None` before the search starts. Past that statement,
`excluded` is a borrowed `HashSet<PathBuf>`, so the main operation no longer lives
inside another branch.

The final expression asks whether any prefix satisfies `path.starts_with(prefix)`.
`any` stops polling after the first `true`, just as the starter returns from its loop.
The refactor allocates nothing, and its worst case is still a linear scan through all
prefixes.

The signature has not changed. That restraint matters. We can improve the local control
flow without making callers and tests move with it. Whether the parameter types should
change is an API decision, not a prerequisite for using `let ... else` and `any`.

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
receives the optional view it needs, without access to the owning wrapper.

If absence and emptiness mean the same thing, remove `Option` and put the candidate path
first:

```rust
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/12_excluded_path_alt.rs:plain_function}}
```

This is the smallest interface that fits the tested behaviour. It still exposes
`HashSet`, although hashing cannot speed up a query that inspects entries one by one.
The set can still justify itself by deduplicating configured prefixes.

You can also stop exposing the storage and give the policy its own type:

```rust
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/12_excluded_path_alt.rs:excluder}}
```

`Excluder` gives the path policy one home. Normalisation and validation can live there
when the domain requires them, and the type can use a different index later without
changing its callers. Its default value excludes nothing, so it retains a valid state
from the current tests.

Run both alternatives with:

```sh
cargo test --example 12_excluded_path_alt
```

An extension trait supports a different call style: `path.is_excluded(&prefixes)`.
That convenience has a cost. A local policy method now appears on the standard-library
`Path` type, and callers using method syntax must have the trait in scope. The
implementation must still do a prefix check:

```rust,ignore
impl Excludable for Path {
    fn is_excluded(&self, excluded: &HashSet<PathBuf>) -> bool {
        excluded.iter().any(|prefix| self.starts_with(prefix))
    }
}
```

Why not use `excluded.contains(self)`? Set membership checks for an exact path. It would
stop `/tmp` from matching `/tmp/foo.log`.

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

A team that is new to iterator predicates may find a loop easier to inspect in a
debugger. That is a fair objection to long adapter chains. This one stays short, names
the operation, and keeps the early exit. I would take the readability win here.

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
>   existential test with short-circuit evaluation.
> - [Optional borrows](../appendix/concept-index.md#optional-borrows): use `Option<&T>`
>   when a function needs an optional view rather than a borrowed owning wrapper.
> - [Empty collections as identity values](../appendix/concept-index.md#empty-collections):
>   use an empty collection for “no configured items” when absence carries no additional
>   information.
> - [Newtypes](../appendix/concept-index.md#newtype): hide storage and put domain policy
>   behind a dedicated interface.
> - [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): path
>   component checks do not resolve filesystem identity.
