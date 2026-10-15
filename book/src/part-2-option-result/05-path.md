# 05 · `path`

## The problem

`is_rust_source_file` checks whether a `Path` has the extension `"rs"`. The starter uses
nested matches to handle missing and non-UTF-8 extensions. Direct comparison with
`OsStr` removes the UTF-8 conversion and one level of branching.

## Starter code

> **EXERCISE**
>
> Read the nested matches alongside the tests. Keep the check lexical and case-sensitive,
> including the existing results for a path without an extension and for the dotfile
> `.rs`.

```rust,editable
{{#include ../../../examples/05_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/05_path.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/05_path.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/05_path.rs` without changing its
> behaviour:
>
> ```sh
> cargo test --example 05_path
> ```
>
> Check whether the extension exists and equals `"rs"` without first converting it
> to UTF-8 text.

## Review

> **REVIEW**
>
> - Why does `Path::extension` return `Option<&OsStr>` instead of `Option<&str>`?
> - Which additional absence case appears when the code calls `OsStr::to_str`?
> - Does comparison with the ASCII literal `"rs"` need that conversion?
> - What result do the two `None` branches share?
> - Why is `.rs` false while `main.rs` is true?
> - Is the function looking at the filesystem, or only at the path value?

`Path::extension` returns `None` when the final
path component has no extension. If an extension exists, the method returns `&OsStr`
because platform paths are not guaranteed to be valid UTF-8. Calling `to_str` adds a
second `Option`: the extension is present, but a non-UTF-8 value cannot become `&str`.

Both `None` branches return `false`: a missing extension cannot match, and a non-UTF-8
extension cannot equal `"rs"`.

`OsStr` supports direct comparison with `str`, so `ext == "rs"` performs the same check
without a separate UTF-8 validity branch. Neither version allocates a `String`.

The `.rs` test checks that a leading dot alone does not create an extension. `Path`
treats `.rs` as a file stem with no extension. The function checks path syntax without
accessing the filesystem.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust,editable
{{#include ../../../solutions/05_path.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/05_path.rs:tests}}
</script>


`Option::is_some_and` returns `false` for `None`. For `Some(ext)`, it runs the predicate
and returns that result. One expression therefore covers both outer branches from the
starter.

The closure compares `&OsStr` directly with `"rs"`. A non-UTF-8 extension still returns
`false`. The tested results are unchanged: `main.rs` and `src/lib.rs` match;
`Cargo.toml`, `README`, and `.rs` do not.

</details>

## Alternative approaches

The solution file also includes `map_or`, which is just as valid here:

```rust,editable
# use std::path::Path;
pub fn is_rust_source_file(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "rs")
}
```

`map_or(false, predicate)` returns `false` for `None` and applies the predicate to
`Some`. `is_some_and` expresses this presence-and-predicate check directly. Use
`map_or` if your team finds it clearer or supports a Rust version that predates
`is_some_and`.

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

This version removes the UTF-8 conversion but leaves the two cases in view. Choosing
between it and `is_some_and` is mostly about which form your team reads faster.

## Edge cases and production concerns

> **EDGE CASE**
>
> `Path::extension` classifies path syntax. It does not establish that the path exists or
> that an existing filesystem object is a regular file.

A directory called `generated.rs` can return `true`. So can a path to a file that does
not exist. If the contract requires an existing regular file, check filesystem metadata
separately. That check performs I/O, can fail, and leaves races between inspection and
later use. Hiding it inside this small lexical predicate would require a different
interface and documentation.

The comparison is exact and case-sensitive, so `lib.RS` does not match. The dotfile `.rs`
has no extension, while `.config.rs` has the extension `rs`. For `lib.rs.bak`, the
extension is `bak`.

A non-UTF-8 extension compares unequal to `"rs"`, producing `false`. That policy works for
a yes-or-no classifier. If the caller must tell “not Rust” apart from “the extension
cannot be represented as UTF-8,” a `bool` is too small. The function needs a richer
return type and an explicit encoding policy.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `is_some_and` implementation with the starter.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions take `&Path` and return `bool`; neither establishes a new invariant or promises anything about the filesystem. |
| Readability | wins | One predicate replaces two nested matches whose failure branches both return `false`. |
| API design | neutral | The signature is unchanged, so a caller still cannot learn why a path failed to match. |
| Performance | neutral | Neither version allocates, and both do work proportional to the short extension; skipping UTF-8 validation is not a meaningful speed-up here. |
| Flexibility | neutral | Both versions keep the same exact, case-sensitive extension policy. |
| Error reporting | neutral | Missing, different, and non-UTF-8 extensions all remain `false`. |

An explicit `match` may be easier to read if `is_some_and` is unfamiliar. Either form
can remove the unnecessary `to_str` conversion.

## Quiz

### 1. Should this predicate keep `to_str` because Rust source is UTF-8 text?

*Answer:* No. This function never reads the source file; it only checks the extension.
Comparing the `OsStr` with `"rs"` answers that question without requiring the entire path
component to be valid UTF-8. Keep the conversion only when later work needs `&str` or
when invalid encoding must be reported as its own outcome.

### 2. A caller needs to accept both `.rs` and `.RS`. Is `eq_ignore_ascii_case` on `&str` the right next step?

*Answer:* Not until you decide what a non-UTF-8 extension means. Converting with `to_str`
makes ASCII-insensitive comparison possible, but it also creates a conversion-failure
case. The policy or interface must say whether that case is an error or another
non-match.

### 3. Can a `true` result be used as proof that compilation can read a regular file?

*Answer:* No. A true result says something only about path syntax. The path can be
missing, can refer to a directory, or can become invalid before it is used. Filesystem
checks require I/O and error handling, and even those checks cannot eliminate every
race.

### 4. Which implementation should a review require: `is_some_and`, `map_or`, or `match`?

*Answer:* If the contract is exact extension equality, require the unnecessary UTF-8
conversion to go. The remaining choice is about team readability. `is_some_and`,
`map_or`, and `match` can all preserve the tested behaviour.

## Follow-up questions

> **FOLLOW-UP**
>
> - Should your application compare extensions case-sensitively on every platform?
> - Is a non-UTF-8 extension just a non-match, or does the caller need an error?
> - Does the function name promise more than a lexical extension check can deliver?
> - Which call site, if any, owns the checks for existence and file type?
> - Would your team understand `is_some_and`, `map_or(false, ...)`, or an explicit `match`
>   fastest?

## Concepts recap

> **CONCEPT**
>
> - [`Path::extension`](../appendix/concept-index.md#path-extension): inspect the suffix
>   of the final path component without touching the filesystem.
> - [`OsStr` and non-UTF-8 paths](../appendix/concept-index.md#osstr-and-non-utf-8-paths):
>   keep platform path data in its native string type until a text operation requires
>   conversion.
> - [`Option::is_some_and`](../appendix/concept-index.md#option-is-some-and): ask whether a
>   value is present and satisfies a predicate in one expression.
> - [`Option::map_or`](../appendix/concept-index.md#option-map-or): choose a result for
>   `None` and transform the value inside `Some`.
> - [Lexical path matching](../appendix/concept-index.md#lexical-path-matching): classify
>   path syntax without claiming that a filesystem object exists or has a particular
>   identity.
