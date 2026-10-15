# 10 · `error_handling`

## Code to refactor

> **EXERCISE**
>
> Follow the error paths before shortening the function. Work out what each `match`
> returns, then find the standard-library operation that opens a file and reads all of it
> into a `String`.

```rust,editable
{{#include ../../../examples/10_error_handling.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/10_error_handling.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/10_error_handling.rs:tests}}
</script>

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/10_error_handling.rs` without changing
> the public signature or turning I/O failures into panics:
>
> ```sh
> cargo test --example 10_error_handling
> ```
>
> Keep the missing-file test passing, including its `io::ErrorKind::NotFound` check.

## Discussion

> **DISCUSSION**
>
> - Are the two `match` expressions incorrect, or just verbose error propagation?
> - How would the same two operations look with `?`?
> - Does anything need the `File` after it has been opened?
> - Which standard-library function opens a path and returns its contents as a `String`?
> - What happens if the file contains invalid UTF-8?
> - Do `unwrap` calls in tests mean the production path should panic too?

The manual propagation is correct. When `File::open` returns `Err(e)`, the function
returns that same error. The second `match` does the same for `read_to_string`. Since the
function already returns `io::Error`, `?` can express both early returns without changing
which errors reach the caller:

```rust,ignore
let mut file = File::open(path)?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
Ok(contents)
```

This version is useful when you need the `File` handle or want to inspect the buffer
between steps. This function needs neither. `std::fs::read_to_string` already opens a
file and reads its contents into a string. Using it also removes the local file handle
and buffer variables.

The tests check both success and failure. A readable file returns its text. A
missing file remains an `io::Error` whose kind is `NotFound`. The test's `unwrap` calls
serve a different purpose from production error handling: a failed temporary-file write
or an unexpected read result should fail the test immediately. A production caller needs
the error value instead.

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/10_error_handling.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/10_error_handling.rs:tests}}
</script>

`fs::read_to_string(path)` opens the file, reads all bytes as UTF-8 text, and returns the
resulting `String`. Its internal file handle closes when it is dropped. The helper already
returns `io::Result<String>`, which matches this function's return type. The function can
return that result directly, without unwrapping or converting it.

If opening or reading fails, the caller still gets an `io::Error`. The missing-file test
requires `io::ErrorKind::NotFound`, not just any error.

`read_to_string` requires valid UTF-8. It does not accept arbitrary bytes: invalid UTF-8
makes the read fail. If binary data or an unknown
encoding is valid input, use `fs::read` and return `Vec<u8>` instead.

The wrapper can still be useful if its name describes the operation in application
terms or if it provides a place to add error context or a size limit later. If neither
is needed, callers can use `fs::read_to_string` directly.

### Alternative approaches

The canonical solution keeps `path: &str` because this exercise changes the
implementation, not the public interface. If callers use native filesystem paths,
consider a path parameter as a separate API change:

```rust,ignore
use std::path::Path;

pub fn read_file_contents(path: impl AsRef<Path>) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

`AsRef<Path>` accepts common path-like values, including references to strings and paths.
A concrete `&Path` is also reasonable when you want a smaller public interface. Either
choice admits native paths that cannot be represented as UTF-8 text on every platform.

This API change is optional. Keep `&str` when paths enter the surrounding interface as
text and callers don't need other path types. Choose `AsRef<Path>` or `&Path` when support for
native paths is part of the contract.

</details>

## Edge cases and production concerns

> **EDGE CASE**
>
> `read_to_string` reads the entire file into memory and requires valid UTF-8. Those are
> suitable requirements for small text files, but not for every input.

Large or untrusted files may need a size limit or a buffered streaming interface. Binary
files need a byte-oriented return type. And if callers must distinguish opening,
reading, decoding, or policy failures, a bare `io::Error` may lack the domain context
they need. A surrounding error type can add that context while preserving the low-level
I/O error as its source.

## Trade-offs

> **TRADE-OFF**
>
> These scores compare the canonical `fs::read_to_string` implementation with the
> starter's explicit open and read steps.

| Axis | Score | Why |
|---|---|---|
| Type-safety | neutral | Both versions return `io::Result<String>` and require the file contents to be valid UTF-8. |
| Readability | wins | The implementation names the complete operation and removes matches, mutable state, and intermediate bookkeeping. |
| API design | neutral | The public `&str` parameter and `io::Result<String>` return type stay unchanged. |
| Performance | neutral | Both versions open the file once and build one `String`; the standard helper does not reduce I/O or allocation work. |
| Flexibility | loses | The helper leaves no point to inspect the file, limit reads, or report progress after opening it. |
| Error reporting | neutral | Both versions propagate `io::Error`, and callers can still observe a missing file as `NotFound`. |

Use the shorter implementation when the requirement is to read a whole text file.
If you need custom opening options, streaming, or limits, use separate open and read
steps to control those operations.

## Follow-up questions

> **FOLLOW-UP**
>
> - Does this function need its own name or policy rather than calling
>   `fs::read_to_string`?
> - Are your application's paths text values, native `Path` values, or both?
> - Which files can grow large enough that reading them in full would be unsafe?
> - Where should you add domain context to an `io::Error` without losing its source?
> - Which manual error-propagation blocks in your codebase could become one standard-library
>   operation?

## Important Concepts

> **CONCEPT**
>
> - [Error propagation](../appendix/concept-index.md#error-propagation): return a failure
>   to the caller instead of handling or discarding it locally.
> - [The `?` operator](../appendix/concept-index.md#question-mark-operator): continue with
>   a success value or return early with a compatible error.
> - [Standard-library convenience functions](../appendix/concept-index.md#standard-library-convenience-functions):
>   replace separate steps with a function that performs the complete operation.
> - [`io::Result`](../appendix/concept-index.md#io-result): report filesystem and stream
>   failures with `io::Error`.
> - [Path parameter types](../appendix/concept-index.md#path-parameter-types): choose
>   `&str`, `&Path`, or `AsRef<Path>` according to the inputs your API intends to accept.
