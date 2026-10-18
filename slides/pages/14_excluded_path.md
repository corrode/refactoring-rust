---
layout: default
---

# 14 · `is_excluded`

<div class="opacity-80 mb-4">
Does <code>path</code> start with any of the excluded prefixes?
<code>None</code> means no exclusions configured.
</div>

```rust
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn is_excluded(excluded: &Option<HashSet<PathBuf>>, path: &Path) -> bool {
    if let Some(excluded) = excluded {
        for prefix in excluded {
            if path.starts_with(prefix) {
                return true;
            }
        }
    }
    false
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 14_excluded_path</code>
</div>

<!--
Discoveries:
  - `&Option<T>` in the signature is a code smell. Usually `Option<&T>` or just `&T` is enough.
  - The nested `if let` + `for` + `return true` is `iter().any(...)`.
  - `let else` flattens the early-out path nicely.
  - An empty `HashSet` already means "exclude nothing", so the `Option` may be redundant.
-->

---

# 14 · Review

- Does `&Option<HashSet<…>>` preserve a useful distinction between "no config" and "empty config"?
- The inner loop is "is there any element that satisfies …". What iterator method is that?
- Can <code>let … else</code> remove the indentation?
- Could the signature just take <code>&HashSet&lt;PathBuf&gt;</code>?


---

# 14 · Possible solution

```rust
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn is_excluded(excluded: &Option<HashSet<PathBuf>>, path: &Path) -> bool {
    let Some(excluded) = excluded else { return false };
    excluded.iter().any(|prefix| path.starts_with(prefix))
}
```

<div class="mt-8 text-base opacity-80">

- `let … else` handles the "no exclusions" case up front, no nesting.
- `iter().any(...)` is the idiomatic spelling of "does any element match?"
- If the API allows it, take `&HashSet<PathBuf>` and let the empty set mean "nothing excluded".

</div>
<!--
Worth pointing out: `&Option<T>` is almost always less useful than `Option<&T>`.
And an empty collection is often a better "no config" sentinel than `Option`.
-->
