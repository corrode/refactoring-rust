# 12 · `excluded_path`

## The problem

A file scanner receives a candidate path and a configured set of directory prefixes.
It must skip the path when any prefix contains it. At first, this looks like a small
control-flow cleanup: the starter checks optional configuration, loops over every
prefix, and returns as soon as one matches. The signature raises the more interesting
question. Why represent missing configuration and an empty set as different states if
this function treats them exactly the same?

## Starter code

> **EXERCISE**
>
> Review the implementation and the interface separately. Start by preserving the
> current behaviour. Then ask whether the parameter types express a distinction the
> function uses.

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
> Compare the tests for `None` and an empty `HashSet`. Do they require different
> results?

## Review

> **REVIEW**
>
> - What distinction does `&Option<HashSet<PathBuf>>` expose to the function?
> - Do the tests require that distinction to survive?
> - What iterator operation means “does any element satisfy this predicate?”
> - Can you return from the absence case before the matching logic begins?
> - Does `HashSet` make this prefix search faster than another collection would?
> - Does `Path::starts_with` compare raw strings, path components, or filesystem
>   locations?

Look through the nested `if let`, `for`, and `if`, and the operation is quite specific:
does any prefix match? The handwritten loop already stops after the first matching prefix.
`Iterator::any` gives that operation a name and keeps the same short-circuit behaviour.

The outer `if let` handles a valid input with no collection to search. We can return
from that case early and leave the matching logic at the main indentation level. That
makes the function easier to read, but it doesn't tell us whether `Option` belongs in
the interface.

The tests help with that API question. Both `None` and `Some(HashSet::new())` return
`false`. If callers don't attach any other meaning to the difference, an empty
collection already says “exclude nothing.” If absence matters while loading or merging
configuration, keep that distinction in the configuration layer until you know where
it belongs.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/12_excluded_path.rs}}
```

`let ... else` deals with `None` first. After that statement, `excluded` is a borrowed
`HashSet<PathBuf>`, and the prefix search no longer sits inside another branch.

The final expression asks whether any prefix satisfies `path.starts_with(prefix)`.
`any` stops polling the iterator after the first `true`, just as the starter returns
from its loop. Nothing new is allocated, and the worst case remains a linear scan.

Notice what this solution doesn't change: the signature. We can recommend the
control-flow refactor without asking callers or tests to change. Changing the parameter
types is a separate API decision, so it deserves a separate discussion.

</details>

## Alternative approaches

The current signature bundles two choices: exclusions may be absent, and the function
borrows the `Option` that owns the set.

If absence has domain meaning, accept an optional borrow instead:

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

A caller that owns an `Option<HashSet<PathBuf>>` can pass `excluded.as_ref()`. The
function gets the optional view it needs without receiving access to the owning
wrapper.

What if absence and emptiness mean the same thing? Remove `Option`, and put the path
being tested first:

```rust
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/12_excluded_path_alt.rs:plain_function}}
```

This is the smallest interface that matches the tested behaviour. It still exposes
`HashSet`, even though hashing cannot speed up a query that visits entries one by one.
The set may still earn its place by deduplicating configured prefixes.

If you don't want callers to know how prefixes are stored, give the policy its own
type:

```rust
# use std::collections::HashSet;
# use std::path::{Path, PathBuf};
{{#include ../../../solutions/12_excluded_path_alt.rs:excluder}}
```

`Excluder` gives path policy one home. It can handle normalization and validation now,
then switch to a different index later without exposing that change to callers. Its
default value excludes nothing, which preserves a valid state from the current tests.

Run both alternatives with:

```sh
cargo test --example 12_excluded_path_alt
```

An extension trait offers another call style: `path.is_excluded(&prefixes)`. The cost is
that a local policy method now appears on the standard-library `Path` type, and callers
must bring the trait into scope. The implementation must also keep prefix matching:

```rust,ignore
impl Excludable for Path {
    fn is_excluded(&self, excluded: &HashSet<PathBuf>) -> bool {
        excluded.iter().any(|prefix| self.starts_with(prefix))
    }
}
```

Why not write `excluded.contains(self)`? That checks exact set membership. It would no
longer let `/tmp` match `/tmp/foo.log`.

We could also require a non-empty `Excluder`, but that invariant conflicts with the
current requirement that an empty rule set is valid. Only a domain rule would justify
that restriction.

## Edge cases and production concerns

> **EDGE CASE**
>
> `Path::starts_with` performs a lexical, component-aware comparison. It works for path
> filtering, but it cannot prove that a filesystem object stays below a directory.

For example, `/tmp/file` starts with `/tmp`, while `/tmpfile` does not. The method does
not resolve `..`, follow symbolic links, canonicalize relative paths, or protect against
filesystem races. If this check guards a security boundary, you need a documented
normalization and filesystem policy as well.

An empty prefix needs an explicit rule because it can match every path. You also need to
decide how absolute and relative prefixes relate before mixing them. Configuration
validation or a domain type such as `Excluder` is a better home for those rules than
each individual call site.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `let ... else` and `any` refactor with the
> starter. The alternative signatures change the public interface and need their own
> review.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept the same optional set and encode no new path invariant. |
| Readability | wins | The absence case returns first, then `any` states the matching question without nested control flow. |
| API design | neutral | The signature still borrows an `Option<HashSet<PathBuf>>` and preserves two states that produce the same result here. |
| Performance | neutral | Both versions allocate nothing, stop at the first match, and scan all prefixes in the worst case. |
| Flexibility | neutral | The concrete optional container remains part of the interface; only the implementation changes. |
| Error reporting | N/A | Missing rules and non-matching paths remain ordinary `false` results under the current contract. |

If your team is new to iterator predicates, a loop may be easier to step through in a
debugger. Here, `any` stays short, names the operation, and preserves the early exit. I
would take that readability win.

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
