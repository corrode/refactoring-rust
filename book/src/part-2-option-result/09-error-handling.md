# 09 · `error_handling`

## The problem

Reading a text file requires several fallible operations. The starter opens the file,
creates a buffer, reads into it, and manually returns each `io::Error`. That code is
correct, but it exposes plumbing that the standard library already provides as one
operation. The refactoring question is not how to hide an error. It is how to remove the
plumbing while preserving the same error contract.

## Starter code

> **EXERCISE**
>
> Review the error paths before shortening the function. Identify what each `match`
> returns, then look for a standard-library operation that already reads a whole file
> into a `String`.

```rust
{{#include ../../../examples/09_error_handling.rs}}
```

> **TRY THIS**
>
> Run the starter tests, then refactor `examples/09_error_handling.rs` without changing
> the public signature or turning I/O failures into panics:
>
> ```sh
> cargo test --example 09_error_handling
> ```
>
> Keep the missing-file test passing, including its `io::ErrorKind::NotFound` check.

## Review

> **REVIEW**
>
> - Are the two `match` expressions wrong, or are they verbose versions of valid error
>   propagation?
> - What would each `match` look like with the `?` operator?
> - Does the function need direct access to the `File` after opening it?
> - Which standard-library function opens a path and reads its contents into a `String`?
> - What happens when the file contains bytes that are not valid UTF-8?
> - Do the `unwrap` calls in the tests imply that the production function should panic?

The manual propagation is correct. If `File::open` returns `Err(e)`, the function returns
that error unchanged. The second `match` does the same for `read_to_string`. With this
function's `io::Error` return type, the corresponding `?` version would bind the success
value and return early on the same errors:

```rust,ignore
let mut file = File::open(path)?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
Ok(contents)
```

That is a useful refactor when you need the `File` handle or the buffer between steps.
Here, nothing happens between opening and reading. `std::fs::read_to_string` already
performs the complete operation and returns `io::Result<String>`, so even the shorter
`?` version retains unnecessary plumbing.

The tests cover both branches of the public contract. A readable file produces its text,
and a missing file remains an `io::Error` with kind `NotFound`. The `unwrap` calls are in
test setup and assertions. A failed temporary-file write or an unexpected read result
should fail the test immediately. That is different from a production path, where a
caller needs the returned error.

## A possible solution

<details>
<summary>Show a possible solution</summary>

```rust
{{#include ../../../solutions/09_error_handling.rs}}
```

`fs::read_to_string(path)` opens the file, reads all of its bytes as UTF-8 text, and
returns the resulting `String`. The internal file handle is closed when it is dropped.
The helper returns `io::Result<String>`, which is already the function's return type, so
we can return its result directly.

No error has been swallowed or replaced. If opening or reading fails, the caller still
receives an `io::Error`. The missing-file test demonstrates this by checking for
`io::ErrorKind::NotFound` rather than accepting any failure.

The name `read_to_string` also makes an input requirement visible. This function does
not accept arbitrary file bytes. If the bytes are not valid UTF-8, the read fails. Use
`fs::read` and return `Vec<u8>` when binary or unknown-encoding input is valid.

The wrapper function is now small enough to question, but it can still earn its place.
It gives the operation a domain-specific name and leaves one location for later policy,
such as adding context or enforcing a size limit. If it adds no policy and has no value
to callers, calling `fs::read_to_string` directly would be reasonable.

</details>

## Alternative approaches

The canonical solution deliberately keeps `path: &str`. That avoids an API change while
we refactor the implementation. If callers work with native filesystem paths, choosing a
path-oriented parameter can be worthwhile:

```rust,ignore
use std::path::Path;

pub fn read_file_contents(path: impl AsRef<Path>) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

`AsRef<Path>` accepts common path-like inputs, including string and path references. A
concrete `&Path` parameter is another sound choice when you prefer a smaller public
interface. Both choices admit paths that cannot be represented as UTF-8 text on every
platform.

This API change is optional. Keep `&str` when the surrounding interface receives textual
paths and changing callers would add no value.
Choose `AsRef<Path>` or `&Path` when accepting native paths is part of the function's
contract.

## Edge cases and production concerns

> **EDGE CASE**
>
> `read_to_string` reads the whole file into memory and requires valid UTF-8. Those are
> useful guarantees for small text files, but poor defaults for every kind of input.

For large or untrusted files, consider a size limit or a buffered streaming interface.
For binary data, return bytes. If callers need to distinguish opening, reading, decoding,
or policy failures, `io::Error` alone may not carry enough domain context. A surrounding
error type can add that context without changing how the low-level I/O error is
propagated.

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
| Performance | neutral | Both versions open the file once and build one `String`; the standard helper does not reduce the I/O or allocation work. |
| Flexibility | loses | The single helper call offers no point to inspect the file, limit reads, or report progress after opening it. |
| Error reporting | neutral | Both versions propagate `io::Error`, and the missing-file error remains observable as `NotFound`. |

I would take the readability win while the requirement remains “read this whole text
file.” If file-opening options, streaming, or limits become part of the contract, expand
the implementation again rather than forcing them around the convenience function.

## Quiz

### 1. Should a review stop after replacing both `match` expressions with `?`?

*Answer:* Stop there if the function needs access to the `File` or buffer between the two
fallible operations. If its complete job is to read a whole text file, use
`fs::read_to_string`. It expresses the same operation with less state to inspect.

### 2. Should the production function use `unwrap` because the tests do?

*Answer:* No. The test helper uses `unwrap` to make broken setup fail the test, and the
assertions use it only after choosing the expected branch. The production function must
return I/O failures because missing, unreadable, and invalid text files are recoverable
conditions for its caller.

### 3. Should every file-reading helper accept `impl AsRef<Path>`?

*Answer:* No. Use it when accepting several path-like input types helps real callers or
when native non-UTF-8 paths matter. Keep `&str` when paths enter the API as text and a
generic parameter would broaden the interface without solving a requirement.

### 4. Is `read_to_string` suitable for a file with unknown encoding?

*Answer:* Not by itself. It succeeds only when the bytes are valid UTF-8. Read bytes with
`fs::read` when arbitrary data is valid, then decode according to an explicit encoding
policy.

## Follow-up questions

> **FOLLOW-UP**
>
> - Does this function add enough policy or vocabulary to justify wrapping
>   `fs::read_to_string`?
> - Are your application's paths text values, native `Path` values, or both?
> - Which files can be large enough that reading them fully would be unsafe?
> - Where should you add domain context to an `io::Error` without losing its source?
> - Which manual error-propagation blocks in your codebase conceal one standard-library
>   operation?

## Concepts recap

> **CONCEPT**
>
> - [Error propagation](../appendix/concept-index.md#error-propagation): return a failure
>   to the caller instead of handling or discarding it locally.
> - [The `?` operator](../appendix/concept-index.md#question-mark-operator): continue with
>   a success value or return early with a compatible error.
> - [Standard-library convenience functions](../appendix/concept-index.md#standard-library-convenience-functions):
>   replace low-level plumbing with an operation that states the complete intent.
> - [`io::Result`](../appendix/concept-index.md#io-result): report filesystem and stream
>   failures with `io::Error`.
> - [Path parameter types](../appendix/concept-index.md#path-parameter-types): choose
>   `&str`, `&Path`, or `AsRef<Path>` according to the inputs your API intends to accept.
