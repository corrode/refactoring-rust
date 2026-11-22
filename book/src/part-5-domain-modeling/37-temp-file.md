# `temp_file`

> **Draft outline.** The exercise and solution are runnable; the bullet notes are a reference for the chapter's prose.

- `read_back` creates a scratch file, writes the supplied bytes, and reads them back as UTF-8 text.
  Review the file's lifecycle and make the tests pass without weakening their assertions.
- Keep `read_back(path: &Path, bytes: &[u8]) -> io::Result<String>`.
  You may redesign the creation API and update its callers in this domain-modeling exercise.
- Preserve exclusive creation: an existing file must not be truncated or removed.
  Preserve best-effort cleanup: a removal error must not replace the result of writing or reading.
- Assume a private, stable parent directory controlled by the caller.
  Generating temporary names, setting a special permission policy, and defending against external path replacement are outside this exercise.

One starter test intentionally fails.

## Code to refactor

```rust,editable
{{#include ../../../examples/37_temp_file.rs:listing}}
```

<script type="text/plain" class="playground-tests" data-expected-failures="tests::scratch_file_is_removed_after_read_error">
{{#include ../../../examples/37_temp_file.rs:tests}}
</script>

<div class="playground-local-setup">

**Run locally**

From the repository root:

```sh
cargo test --example 37_temp_file
```

</div>

## Discussion

<details class="discussion-question">
<summary>Which exits reach the removal call?</summary>

- The successful path reaches `fs::remove_file` after reading the text.
  Either earlier `?` can return before that line.
- The failing test writes `[0xff]`, which is not valid UTF-8.
  Writing succeeds, reading returns `InvalidData`, and the scratch file remains.
- Adding a removal call to this one error branch would repair that branch, not the underlying requirement.
  Consider another fallible operation, an explicit early return, or a panic that unwinds through the function.

</details>

<details class="discussion-question">
<summary>What does owning a <code>PathBuf</code> own?</summary>

- A `PathBuf` owns a filesystem path, not the file at that path or responsibility for removing it.
  Dropping it releases the path's storage, not the directory entry.
- The constructor returns a path and leaves every caller responsible for pairing creation with removal.
  That obligation is not part of the returned type's behaviour.
- Cloning a path copies the name.
  It does not extend the file's lifetime or establish that the path still identifies the same file.

</details>

<details class="discussion-question">
<summary>Does dropping <code>File</code> remove the file?</summary>

- Dropping an ordinary `std::fs::File` closes its handle, not its directory entry.
  The handle returned by `File::create_new` is already dropped at the end of that statement.
- `fs::write` and `fs::read_to_string` manage their own handles.
  Closing those handles still leaves removal as a separate operation.
- Handle lifetime and temporary-file lifetime are related design choices, not interchangeable concepts.

</details>

<details class="discussion-question">
<summary>What should callers observe if removal fails?</summary>

- The starter deliberately ignores the result of `remove_file`.
  A successful read stays successful, and write or read errors must remain the reported operation errors.
- Cleanup is best effort in both versions.
  This refactor changes when removal is attempted, not the policy for reporting removal errors.
- If callers need confirmation of deletion, the API needs an explicit fallible operation as well as a decision about competing operation and cleanup errors.

</details>

<details class="solution" id="solutions">
<summary><h2><span aria-hidden="true">💡</span> Show Solutions</h2></summary>

### Give cleanup an owner

```rust,editable
{{#include ../../../solutions/37_temp_file.rs:listing}}
```

<script type="text/plain" class="playground-tests">
{{#include ../../../solutions/37_temp_file.rs:tests}}
</script>

- `TempFile` owns the cleanup responsibility, with `Drop` attempting removal when the value is destroyed.
  This is a resource guard, commonly described as RAII.
- Construct the guard before the subsequent fallible work.
  Creating a file, doing the work, and only then wrapping its path would leave the same gap on early exits.
- Keep the path private so callers outside the module cannot construct a guard around an arbitrary existing file.
  `File::create_new` rejects collisions rather than truncating or adopting somebody else's file.
  Code inside the privacy boundary remains responsible for preserving that rule.
- Do not derive `Clone`.
  Two independent guards for the same path would both attempt removal, and dropping either could invalidate the other's use.
- `path()` returns a borrow tied to the guard.
  `read_back` keeps the named `scratch` binding alive while writing and reading, with no manual removal at the call site.

### Migrate callers and tests

- Replace `create_temp_file(path)?` with `TempFile::create(path)?` and keep the returned guard in a named binding.
  Replace uses of the owned path with `scratch.path()` and remove caller-side cleanup.
- `read_back` keeps its signature and successful return value.
  Its error paths now attempt cleanup before returning the original I/O error, which fixes behaviour rather than merely shortening the code.
- Direct callers of the creation API must migrate from `PathBuf` to `TempFile`.
  When a file must outlive a helper, return the guard, not a copied path.
- The solution has its own attached tests for the redesigned creation API.
  They retain the successful round trip, collision protection, missing-parent error, and invalid-UTF-8 case, then cover scope exit, moves, `?`, early return, and panic unwinding.
- Repository integration tests compile callers against the actual solution to check private construction, missing `Clone`, a borrowed path escaping its owner, and dropping an owner while its path borrow is still needed.
  Runtime tests alone cannot prove those calls are rejected.
- The starter's failing test remains an ordinary assertion failure in the browser and in `cargo test --example 37_temp_file`.
  `make test` checks that expected failure separately; the solution's read-error test runs normally and must pass.

### Lifetime follows the owner

- Leaving the owner's scope, returning early, propagating an error with `?`, and calling `drop(scratch)` all destroy the guard unless ownership has moved elsewhere.
  The last use of a borrow is not the same event as dropping its owner.
- Moving the guard transfers cleanup responsibility without removing the file.
  A returned guard can outlive the scope that created it.
- A named binding such as `scratch` or `_scratch` retains the value.
  In contrast, `let _ = TempFile::create(path)?;` discards the newly created guard, so it can remove the file immediately.
  `#[must_use]` is a warning aid, not enforcement of the required lifetime.
- A borrowed path cannot be used after its guard is dropped.
  An owned copy from `scratch.path().to_path_buf()` can outlive the guard, but it does not keep the file alive.
- With unwinding enabled, a panic also drops local guards as the stack unwinds.
  The solution test uses `catch_unwind` only to observe that behaviour; application callers do not need it to activate cleanup.

### Limits and production choices

- `Drop::drop` cannot return a cleanup error to the caller.
  Ignore removal errors under this exercise's best-effort policy rather than calling `unwrap`, which could cause a second panic during unwinding.
  Removing a file that is already absent must not panic either.
- Destruction is not guaranteed on process abort, `std::process::exit`, or forced termination.
  Deliberately leaking a guard with `mem::forget` also bypasses its destructor.
  Ownership arranges cleanup on the supported exits; it cannot promise deletion.
- Both versions resolve the supplied path with `std::path::absolute` before creation.
  This avoids a later working-directory change redirecting cleanup, but an absolute path does not establish filesystem identity.
- Both versions close the creation handle before returning.
  There is no stored open handle to order against removal here; a design owning both a handle and a path must account for platforms where open handles can prevent deletion.
- Keep the private, stable parent-directory assumption visible.
  External replacement, renaming, or symlink changes can make subsequent writes or removal act on a different file, or leave the original file behind.
  `create_new` protects the creation step, not every later path-based operation.
- This is a small standard-library exercise, not a production temporary-file implementation.
  It neither generates secure temporary names nor sets a special permission policy.
  Evaluate [`tempfile::NamedTempFile`](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html) for production use, including its documented cleanup and security limitations.

### Alternative approaches

- For one function, capturing the work's `Result` before an explicit cleanup call can cover ordinary errors without a new type.
  The fallible work must stay inside that boundary; it does not provide cleanup during unwinding.
  Repeating that convention across callers is the problem the guard addresses.
- If cleanup errors matter, design an explicit fallible cleanup method with `Drop` as a best-effort fallback.
  Specify whether a failed explicit attempt is retried and how its error relates to a write or read error.
  That API is outside the implemented solution, not a guarantee supplied by `Drop`.
- If callers need to retain the file permanently, add a deliberate ownership-transfer or persistence operation rather than teaching `mem::forget` as the normal interface.
  The minimal guard intentionally has no such operation.

### Trade-offs

Compared with the original: <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> advantage, <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> comparable, <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> disadvantage, **N/A** not applicable.

| Criterion | Original | Owning cleanup guard |
|---|---|---|
| Type-safety | Path plus caller convention | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> One cleanup owner; path borrows tied to it |
| Readability | Cleanup mixed with operation control flow | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Straight-line caller; one cleanup policy |
| API design | Creation returns an ordinary path | <img class="tradeoff-icon" src="../icons/advantage.svg" alt="Advantage" width="20" height="20"> Temporary lifetime expressed by returned type |
| Performance | Owns a path; removes after success | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same path storage; removal also attempted on errors |
| Flexibility | Caller chooses when and whether to remove | <img class="tradeoff-icon" src="../icons/disadvantage.svg" alt="Disadvantage" width="20" height="20"> File lifetime follows owner; no persistence operation |
| Error reporting | I/O errors returned; removal errors discarded | <img class="tradeoff-icon" src="../icons/comparable.svg" alt="Comparable" width="20" height="20"> Same reporting policy; deletion still unconfirmed |

The guard adds a type and requires familiarity with destruction timing; it does not make every caller shorter by itself.
The type-safety advantage concerns ownership of a cleanup attempt, not proof that the filesystem accepted it or that destructors always run.
There is no claimed speed-up: the successful path does the same filesystem work, and error paths now do the cleanup they previously skipped.

</details>

## Takeaways For Our Own Codebase

> - Which resources require callers to remember a matching cleanup operation?
> - Check `?`, early returns, and unwinding, not just the successful path.
> - Decide whether callers need a cleanup attempt or a reported cleanup result.
> - Keep a cleanup owner alive for the whole use, and transfer that owner when the resource must outlive a helper.
> - Distinguish owning a resource from owning a name that refers to it.
> - Prefer a maintained resource-management library when the real problem includes temporary naming, permissions, or adversarial filesystem changes.

## References

- [Drop guards](../appendix/concept-index.md#drop-guards): cleanup responsibility tied to an owning value.
- [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) and [RAII](https://doc.rust-lang.org/rust-by-example/scope/raii.html): destruction and resource lifetime.
- [Destructors](https://doc.rust-lang.org/reference/destructors.html), including [not running destructors](https://doc.rust-lang.org/reference/destructors.html#not-running-destructors): drop scopes and their limits.
- [`File::create_new`](https://doc.rust-lang.org/std/fs/struct.File.html#method.create_new) and [`remove_file`](https://doc.rust-lang.org/std/fs/fn.remove_file.html): exclusive creation and fallible removal.
- [`std::path::absolute`](https://doc.rust-lang.org/std/path/fn.absolute.html): an absolute path is not a guarantee of file identity.
- [`tempfile::NamedTempFile`](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html): production-library context, not an added dependency or tested alternative here.
- [config_typestate](36-config-typestate.md) and [friends](38-friends.md): ownership can control available operations and object lifetimes, not only memory allocation.
