# 05 · `path`

## The problem

A source-file classifier receives a `Path` and checks its extension. The starter works,
but it handles two nested `Option` values: one for a missing extension and another for
an extension that isn't valid UTF-8. That second conversion isn't needed for an exact
comparison with `"rs"`. Rust lets an `OsStr` compare directly with a string literal.

## Starter code

> **EXERCISE**
>
> Review the nested matches and the tests together. Preserve the current case-sensitive,
> lexical check, including the result for a path with no extension and for the dotfile
> `.rs`.

```rust
{{#include ../../../examples/05_path.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/05_path.rs` without changing the
> behaviour:
>
> ```sh
> cargo test --example 05_path
> ```
>
> Try to express “the extension exists and equals `rs`” without converting the extension
> to UTF-8 text first.

## Review

> **REVIEW**
>
> - Why does `Path::extension` return `Option<&OsStr>` rather than `Option<&str>`?
> - What new absence case does `OsStr::to_str` introduce?
> - Does equality with the ASCII literal `"rs"` require that conversion?
> - Which outcomes do both `None` arms produce?
> - Why does the `.rs` test return `false` while `main.rs` returns `true`?
> - Does this function inspect the filesystem or only the path value?

`Path::extension` returns `None` when the final path component has no extension. When an
extension exists, it returns `&OsStr` because platform paths are not required to contain
valid UTF-8. The starter calls `to_str`, which adds another `Option`: a non-UTF-8
extension becomes `None` even though the extension itself exists.

Both kinds of absence lead to `false`. A missing extension does not match, and an
extension that cannot be represented as `&str` cannot equal the UTF-8 literal `"rs"`.
The nested matches preserve that rule, but they make you trace two branches that reach
the same result.

The conversion is unnecessary. `OsStr` can be compared directly with `str`, so
`ext == "rs"` performs the exact check while retaining the platform-native path
representation. It does not allocate a `String`, and it does not require a separate
UTF-8 validity branch.

The tests also establish a detail of `Path::extension`: a leading dot alone does not
create an extension. The final component `.rs` is treated as a file stem with no
extension. This is a lexical rule about a `Path`; no filesystem access occurs.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/05_path.rs}}
```

The function now asks one compound question:

1. Does `path.extension()` return an extension?
2. If it does, is that extension equal to `"rs"`?

`Option::is_some_and` returns `false` for `None`. For `Some(ext)`, it calls the predicate
and returns its result. That accounts for both outer branches in the starter without an
explicit `match`.

The closure compares `&OsStr` with the string literal directly. A non-UTF-8 extension
is still a non-match, but the code no longer converts every extension to `&str` before
it can make that decision.

The tests remain focused on path syntax. `main.rs` and `src/lib.rs` match, `Cargo.toml`
and `README` do not, and `.rs` has no extension under `Path`'s rules.

</details>

## Alternative approaches

The solution file shows `map_or` as an equally valid spelling:

```rust
# use std::path::Path;
pub fn is_rust_source_file(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "rs")
}
```

`map_or(false, predicate)` says “use `false` when absent; otherwise map the extension to
a boolean.” `is_some_and(predicate)` says “present and satisfies this predicate.” I
prefer `is_some_and` for a predicate because its name states the question directly.
`map_or` remains reasonable when your team already uses it widely or supports a Rust
version from before `is_some_and` was available.

An explicit `match` is also sound:

```rust
# use std::path::Path;
pub fn is_rust_source_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ext == "rs",
        None => false,
    }
}
```

This removes the unnecessary UTF-8 conversion while keeping the two cases visible. The
choice between this form and `is_some_and` is mostly a readability decision.

## Edge cases and production concerns

> **EDGE CASE**
>
> `Path::extension` examines path syntax. It does not prove that the path exists or that
> an existing filesystem object is a regular file.

A directory named `generated.rs` and a path to a file that does not exist can both
return `true`. If the contract requires an existing regular file, filesystem metadata
must be checked separately. That check performs I/O, can fail, and introduces races
between inspection and later use, so it should not be hidden inside this small lexical
predicate without changing the interface and its documentation.

The extension check is exact and case-sensitive. `lib.RS` does not match. A dotfile such
as `.rs` has no extension, while `.config.rs` has the extension `rs`. A name such as
`lib.rs.bak` has the extension `bak`.

A non-UTF-8 extension compares unequal to `"rs"` and produces `false`. That is suitable
for a yes-or-no classifier. If callers need to distinguish “not Rust” from “extension
cannot be represented as UTF-8,” a `bool` cannot report enough information. The
function would need a richer return type and an explicit encoding policy.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `is_some_and` implementation with the starter.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions accept `&Path` and return `bool`; neither adds a new invariant or filesystem guarantee. |
| Readability | wins | The implementation states one predicate and removes two nested matches with identical `false` outcomes. |
| API design | neutral | The signature stays the same, including the fact that a `bool` cannot explain why a path did not match. |
| Performance | neutral | Both versions allocate nothing and do work proportional to the short extension; removing UTF-8 validation is not a meaningful speed-up here. |
| Flexibility | neutral | Both versions implement the same exact, case-sensitive extension policy. |
| Error reporting | neutral | Both collapse missing, different, and non-UTF-8 extensions into `false`. |

The main trade-off is familiarity. A teammate who does not know `is_some_and` may read
the explicit `match` faster. I would still remove `to_str`: that conversion introduces a
case the operation does not need. Whether the remaining two branches become a
combinator is a smaller judgement call.

## Quiz

### 1. Should this predicate keep `to_str` because Rust source is UTF-8 text?

*Answer:* No. The function checks only the extension. It never reads the file contents.
Comparing the `OsStr` extension with `"rs"` answers that question without requiring the
complete path component to be valid UTF-8. Keep a conversion when later work genuinely
needs `&str` or when invalid encoding must be reported separately.

### 2. A caller needs to accept both `.rs` and `.RS`. Is `eq_ignore_ascii_case` on `&str` the right next step?

*Answer:* Not without deciding how non-UTF-8 extensions should behave. Converting with
`to_str` can support ASCII-insensitive comparison, but it adds a failed-conversion case.
The interface or policy should state whether that case is a non-match or an error.

### 3. Can a `true` result be used as proof that compilation can read a regular file?

*Answer:* No. The function only examines path syntax. The path may be absent, may name a
directory, or may become invalid before use. Filesystem checks need I/O and error
handling, and they still cannot remove every race.

### 4. Which implementation should a review require: `is_some_and`, `map_or`, or `match`?

*Answer:* Require removal of the unnecessary UTF-8 conversion if the contract is exact
extension equality. Treat the remaining spelling as a team readability choice. All
three forms can preserve the tested behaviour.

## Follow-up questions

> **FOLLOW-UP**
>
> - Should extension matching in your application be case-sensitive on every platform?
> - Does a non-UTF-8 extension mean “not a match,” or should callers receive an error?
> - Does the function name promise more than a lexical extension check provides?
> - Which call site, if any, is responsible for checking existence and file type?
> - Would your team read `is_some_and`, `map_or(false, ...)`, or an explicit `match` most
>   quickly?

## Concepts recap

> **CONCEPT**
>
> - [`Path::extension`](../appendix/concept-index.md#path-extension): inspect the suffix
>   of the final path component without accessing the filesystem.
> - [`OsStr` and non-UTF-8 paths](../appendix/concept-index.md#osstr-and-non-utf-8-paths):
>   keep platform path data in its native string type until text operations require a
>   conversion.
> - [`Option::is_some_and`](../appendix/concept-index.md#option-is-some-and): combine
>   presence and a predicate in one expression.
> - [`Option::map_or`](../appendix/concept-index.md#option-map-or): provide a result for
>   `None` and transform the value inside `Some`.
> - [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): path
>   operations can classify syntax without proving filesystem identity or existence.
