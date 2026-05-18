---
layout: default
---

# 19 · `is_excluded`

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
<code>cargo test --example 19_excluded_path</code>
</div>

<!--
Discoveries:
  - `&Option<T>` in the signature is a code smell - usually `Option<&T>` or just `&T`.
  - The nested `if let` + `for` + `return true` is `iter().any(...)`.
  - `let else` flattens the early-out path nicely.
  - An empty `HashSet` already means "exclude nothing", so the `Option` may be redundant.
-->

---

# 19 · Observations

- `&Option<HashSet<…>>` - does the caller really need to distinguish "no config" from "empty config"?
- The inner loop is "is there any element that satisfies …". What iterator method is that?
- Can <code>let … else</code> remove the indentation?
- Could the signature just take <code>&HashSet&lt;PathBuf&gt;</code>?


---

# 19 · Possible solution

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
- Even better - if the API allows it - make the parameter `&HashSet<PathBuf>` and let the empty set carry the "nothing excluded" meaning.

</div>
<!--
Worth pointing out: `&Option<T>` is almost always less useful than `Option<&T>`.
And an empty collection is often a better "no config" sentinel than `Option`.
-->
