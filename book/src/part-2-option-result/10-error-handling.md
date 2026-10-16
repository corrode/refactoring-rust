# `error_handling`

This function reads a file's contents into a string.
Refactor it without changing the public signature or turning I/O failures into panics.
Preserve the returned errors, including `io::ErrorKind::NotFound` for a missing file.

Run the tests with `cargo test --example 10_error_handling`.

## Code to refactor

```rust,editable
{{#include ../../../examples/10_error_handling.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../examples/10_error_handling.rs:tests}}
</script>

<script type="text/plain" class="playground-tests-fallback">
{{#include ../../../solutions/10_error_handling.rs:tests}}
</script>


## Discussion

<details class="discussion-question">
<summary>Are the two <code>match</code> expressions wrong?</summary>

The manual propagation is correct.
When `File::open` returns `Err(e)`, the function returns that same error.
The second `match` does the same for `read_to_string`.

</details>

<details class="discussion-question">
<summary>How would this look with <code>?</code>?</summary>

Since the function already returns `io::Error`, `?` can express both early returns without changing which errors reach the caller:

```rust,ignore
let mut file = File::open(path)?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
Ok(contents)
```

</details>

<details class="discussion-question">
<summary>Do we need to keep the <code>File</code> handle?</summary>

The separate steps are useful when you need the `File` handle or want to inspect the buffer between steps.
This function needs neither.

</details>

<details class="discussion-question">
<summary>Can the standard library do the whole job?</summary>

`std::fs::read_to_string` already opens a file and reads its contents into a string.
Using it also removes the local file handle and buffer variables.

</details>

<details class="discussion-question">
<summary>What if the file contains invalid UTF-8?</summary>

`read_to_string` returns an error rather than a string.
If binary data or an unknown encoding is valid input, use `fs::read` and return `Vec<u8>` instead.
That would be a separate change to the public signature.

</details>

<details class="discussion-question">
<summary>Why do the tests use <code>unwrap</code>?</summary>

The tests check both success and failure.
A readable file returns its text.
A missing file remains an `io::Error` whose kind is `NotFound`.
The test's `unwrap` calls serve a different purpose from production error handling: a failed temporary-file write or an unexpected read result should fail the test immediately.
A production caller needs the error value instead.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

```rust,editable
{{#include ../../../solutions/10_error_handling.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/10_error_handling.rs:tests}}
</script>

`fs::read_to_string(path)` opens the file, reads all bytes as UTF-8 text, and returns the resulting `String`.
Its internal file handle closes when it is dropped.
The helper already returns `io::Result<String>`, which matches this function's return type.
The function can return that result directly, without unwrapping or converting it.

If opening or reading fails, the caller still gets an `io::Error`.
The missing-file test requires `io::ErrorKind::NotFound`, not just any error.

`read_to_string` requires valid UTF-8.
It does not accept arbitrary bytes: invalid UTF-8 makes the read fail.
If binary data or an unknown encoding is valid input, use `fs::read` and return `Vec<u8>` instead.

The wrapper can still be useful if its name describes the operation in application terms or if it provides a place to add error context or a size limit later.
If neither is needed, callers can use `fs::read_to_string` directly.

### Alternative approaches

The canonical solution keeps `path: &str` because this exercise changes the implementation, not the public interface.
If callers use native filesystem paths, consider a path parameter as a separate API change:

```rust,ignore
use std::path::Path;

pub fn read_file_contents(path: impl AsRef<Path>) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

`AsRef<Path>` accepts common path-like values, including references to strings and paths.
A concrete `&Path` is also reasonable when you want a smaller public interface.
Either choice admits native paths that cannot be represented as UTF-8 text on every platform.

This API change is optional.
Keep `&str` when paths enter the surrounding interface as text and callers don't need other path types.
Choose `AsRef<Path>` or `&Path` when support for native paths is part of the contract.

### Edge cases and production concerns

> **EDGE CASE**
>
> `read_to_string` reads the entire file into memory and requires valid UTF-8.
> Those are suitable requirements for small text files, but not for every input.

Large or untrusted files may need a size limit or a buffered streaming interface.
Binary files need a byte-oriented return type.
And if callers must distinguish opening, reading, decoding, or policy failures, a bare `io::Error` may lack the domain context they need.
A surrounding error type can add that context while preserving the low-level I/O error as its source.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.
Readability depends on familiarity.

| Criterion | Original | Open/read with `?` | `fs::read_to_string` (main) | Helper with `AsRef<Path>` |
|---|---|---|---|---|
| Type-safety | Fallible UTF-8 text result | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same result guarantees | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same result guarantees | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same result guarantees |
| Readability | Explicit error branches | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Less propagation boilerplate | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names whole operation | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Names whole operation |
| API design | Text path, fallible result | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same public signature | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same public signature | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Accepts native paths too |
| Performance | Whole-file read into `String` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Same read and buffer | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Still reads whole file | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Still reads whole file |
| Flexibility | Separate open/read steps | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Retains intermediate access | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" title="Disadvantage" width="20" height="20"> Hides intermediate steps | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" title="Advantage" width="20" height="20"> Accepts more path types |
| Error reporting | Propagates bare `io::Error` | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Preserves I/O errors | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Preserves I/O errors | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" title="Comparable" width="20" height="20"> Preserves I/O errors |

`AsRef<Path>` broadens accepted paths, not content encodings; it still requires UTF-8 contents and hides intermediate steps.
Use separate steps for limits or streaming.
None adds error context.
The readability scores assume familiarity with `?`.

</details>

## Takeaways For Our Own Codebase

> - Does this function need its own name or policy rather than calling `fs::read_to_string`?
> - Are your application's paths text values, native `Path` values, or both?
> - Which files can grow large enough that reading them in full would be unsafe?
> - Where should you add domain context to an `io::Error` without losing its source?
> - Which manual error-propagation blocks in your codebase could become one standard-library operation?

## References

- [Error propagation](../appendix/concept-index.md#error-propagation): return a failure to the caller instead of handling or discarding it locally.
- [The `?` operator](../appendix/concept-index.md#question-mark-operator): continue with a success value or return early with a compatible error.
- [Standard-library convenience functions](../appendix/concept-index.md#standard-library-convenience-functions): replace separate steps with a function that performs the complete operation.
- [`io::Result`](../appendix/concept-index.md#io-result): report filesystem and stream failures with `io::Error`.
- [Path parameter types](../appendix/concept-index.md#path-parameter-types): choose `&str`, `&Path`, or `AsRef<Path>` according to the inputs your API intends to accept.
