---
layout: default
zoom: 0.9
---

# 36 · `temp_file`

```rust
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

pub fn create_temp_file(path: &Path) -> io::Result<PathBuf> {
    let path = std::path::absolute(path)?;
    File::create_new(&path)?;
    Ok(path)
}

pub fn read_back(path: &Path, bytes: &[u8]) -> io::Result<String> {
    let path = create_temp_file(path)?;
    fs::write(&path, bytes)?;
    let text = fs::read_to_string(&path)?;
    let _ = fs::remove_file(&path);
    Ok(text)
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Review the file lifecycle. Keep <code>read_back</code>'s signature; the creation API may change.<br>
Full source: <code>examples/36_temp_file.rs</code>. One test intentionally fails.
</div>

<!--
Run cargo test --example 36_temp_file. Do not reveal the guard before discussion.
The task is a small filesystem round trip, not a claimed production incident.
Preserve exclusive creation and best-effort removal; keep the original I/O error.
Assume a private, stable parent directory controlled by the caller. Secure name
allocation, special permissions, and external path changes are out of scope.
-->

---

# 36 · Review

- Which exits reach `remove_file`?
- What if writing succeeds but reading as UTF-8 fails?
- Does dropping a `PathBuf` remove the file?
- Does dropping a `File` remove it, or close its handle?
- Who should be responsible for cleanup?

<!--
The failing test uses [0xff]: write succeeds, read_to_string returns InvalidData,
and ? skips removal. Each caller must remember manual cleanup. Adding removal
only to that branch leaves the next early exit exposed. A PathBuf owns a name,
not the file. File::create_new's handle already closes at its statement boundary.
Discuss error reporting too: the starter deliberately discards removal errors.
The exercise must not silently change that policy while repairing cleanup.
-->

---
zoom: 0.85
---

# 36 · Possible solution: a cleanup guard

```rust
#[must_use = "keep the temporary file alive while its path is in use"]
#[derive(Debug)]
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn create(path: &Path) -> io::Result<Self> {
        let path = std::path::absolute(path)?;
        File::create_new(&path)?;
        Ok(Self { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
```

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Uses the starter's imports. Full source: <code>solutions/36_temp_file.rs</code>.<br>
Private path, exclusive creation, no <code>Clone</code>. The guard owns a cleanup attempt.
</div>

<!--
The creation handle closes before returning the guard. No open handle is stored
that would need closing before removal. Private fields prevent outside callers
from wrapping arbitrary existing files. No Clone avoids two independent guards
competing to remove the same path. The module must uphold these invariants.
This is RAII: associate a resource's cleanup with the lifetime of an owner.
-->

---

# 36 · Keep the owner, borrow the path

```rust
pub fn read_back(path: &Path, bytes: &[u8]) -> io::Result<String> {
    let scratch = TempFile::create(path)?;
    fs::write(scratch.path(), bytes)?;
    fs::read_to_string(scratch.path())
}
```

- Create the guard before later fallible work.
- `?`, early return, scope exit, and explicit `drop` attempt cleanup.
- Move the guard to transfer responsibility or extend the file's lifetime.
- Copying its path does not keep the file alive.

<!--
Migrate direct creation callers to TempFile::create and borrow scratch.path().
read_back keeps its signature; its error paths now attempt cleanup. A helper
must return the guard, not just a copied PathBuf, if its file must stay alive.
A named _scratch binding also retains ownership. In contrast,
let _ = TempFile::create(path)?; discards the new guard immediately. must_use is
only a warning aid. Borrow lifetimes and owner destruction are different events.
Tests cover moves, ?, return, scope exit, and invalid UTF-8. Integration tests
check private construction, no Clone, and paths that outlive their guard.
-->

---

# 36 · Cleanup is an attempt

- Removal can fail; `Drop` cannot return that error.
- Never `unwrap` cleanup during unwinding.
- Unwinding runs destructors; abort and process exit do not guarantee cleanup.
- An absolute path is not a stable file identity.
- For production, evaluate `tempfile::NamedTempFile` and its documented limits.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
<a href="https://doc.rust-lang.org/std/ops/trait.Drop.html">Drop</a> · <a href="https://doc.rust-lang.org/reference/destructors.html#not-running-destructors">Destructor limits</a> · <a href="https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html">NamedTempFile</a>
</div>

<!--
Both versions intentionally discard removal errors. If confirmation matters,
provide explicit fallible cleanup with a best-effort Drop fallback and a policy
for competing errors and retries. Do not hide that API change in the refactor.
Unwinding is tested with catch_unwind only in the test harness, not as a required
application mechanism. Forced termination and deliberate leaks (mem::forget)
also bypass destruction. If persistence is required, design an explicit operation.
Absolute paths avoid dependence on later working-directory changes, not external
renames, replacements, or symlink races. Keep the private stable directory
assumption. create_new protects creation, not later path-based operations.
No secure temporary names or special permissions are implemented here.
This solution adds a type and changes lifetime semantics, not I/O performance.
-->
