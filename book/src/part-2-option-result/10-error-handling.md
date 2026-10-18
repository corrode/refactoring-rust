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


## Discussion

<details class="discussion-question">
<summary>Are the two <code>match</code> expressions wrong?</summary>

No, both matches return errors correctly.
When `File::open` returns `Err(e)`, the function returns that same error.
The second `match` does the same for `read_to_string`.

</details>

<details class="discussion-question">
<summary>How would this look with <code>?</code>?</summary>

Use `?` after each fallible call to return early on an error.
The function already returns `io::Error`, so the same errors reach the caller:

```rust,ignore,noplayground
let mut file = File::open(path)?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
Ok(contents)
```

</details>

<details class="discussion-question">
<summary>Do we need to keep the <code>File</code> handle?</summary>

No, this function only needs the contents.
Keep the separate steps when you need the `File` handle or want to inspect the buffer between steps.

</details>

<details class="discussion-question">
<summary>Can the standard library do the whole job?</summary>

Yes, `std::fs::read_to_string` opens the file and reads its contents into a string.
You no longer need local variables for the file handle and buffer.

</details>

<details class="discussion-question">
<summary>What if the file contains invalid UTF-8?</summary>

`read_to_string` returns an error rather than a string.
If binary data or an unknown encoding is valid input, use `fs::read` and return `Vec<u8>` instead.
That would be a separate change to the public signature.

</details>

<details class="discussion-question">
<summary>Why do the tests use <code>unwrap</code>?</summary>

A failed temporary-file write or an unexpected read result should fail the test immediately, so the tests use `unwrap`.
Production callers need the error value instead of a panic.
The tests check both outcomes: a readable file returns its text, and a missing file returns an `io::Error` whose kind is `NotFound`.

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
Dropping its internal file handle closes the file.
The helper already returns `io::Result<String>`, so return that result directly.
There is nothing to unwrap or convert.

If opening or reading fails, the caller still gets an `io::Error`.
The missing-file test requires `io::ErrorKind::NotFound`, not just any error.

`read_to_string` requires valid UTF-8.
It does not accept arbitrary bytes: invalid UTF-8 makes the read fail.
If binary data or an unknown encoding is valid input, use `fs::read` and return `Vec<u8>` instead.

Keep the wrapper if its name describes an application operation or you need a place to add error context or a size limit later.
Otherwise, call `fs::read_to_string` directly.

### Alternative approaches

The canonical solution keeps `path: &str` because this exercise changes the implementation, not the public interface.
If callers use native filesystem paths, consider a path parameter as a separate API change:

```rust,ignore,noplayground
use std::path::Path;

pub fn read_file_contents(path: impl AsRef<Path>) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

`AsRef<Path>` accepts common path-like values, including references to strings and paths.
Use a concrete `&Path` if you want a smaller public interface.
Both choices accept native paths, including paths that some platforms cannot represent as UTF-8 text.

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
Do callers need to distinguish opening, reading, decoding, or policy failures?
A bare `io::Error` may not give them enough domain context.
Add that context in a surrounding error type and keep the low-level I/O error as its source.

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

The `AsRef<Path>` helper accepts more path types, not more content encodings.
It still requires UTF-8 contents and hides the intermediate steps.
Keep separate steps when you need limits or streaming.
None of these versions adds error context.
The readability scores assume familiarity with `?`.

</details>

## Takeaways For Our Own Codebase

> - Does your file-reading wrapper name an application operation or enforce a policy?
>   If not, could callers use `fs::read_to_string` directly?
> - Do callers need text paths, native `Path` values, or both?
>   Choose the parameter type to match.
> - Which files need a size limit or streaming rather than a whole-file read?
> - Where do callers need domain context around an `io::Error`?
>   Preserve the original error as its source.
> - Look for manual error propagation that `?` or a standard-library function could replace.

## References

- [Error propagation](../appendix/concept-index.md#error-propagation): return a failure to the caller instead of handling or discarding it locally.
- [The `?` operator](../appendix/concept-index.md#question-mark-operator): continue with a success value or return early with a compatible error.
- [Standard-library convenience functions](../appendix/concept-index.md#standard-library-convenience-functions): replace separate steps with a function that performs the complete operation.
- [`io::Result`](../appendix/concept-index.md#io-result): report filesystem and stream failures with `io::Error`.
- [Path parameter types](../appendix/concept-index.md#path-parameter-types): choose `&str`, `&Path`, or `AsRef<Path>` according to the inputs your API intends to accept.
