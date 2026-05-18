---
layout: default
---

# 14 · `read_file_contents`

<div class="opacity-80 mb-4">
Read the entire contents of a file into a <code>String</code>.
</div>

```rust
use std::fs::File;
use std::io::{self, Read};

pub fn read_file_contents(path: &str) -> io::Result<String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 14_error_handling</code>
</div>

<!--
Discoveries usually arrive in this order:
  - Both `match` blocks are just "if Err return Err, else continue" - that's `?`.
  - Do we even need to open + read manually? `std::fs::read_to_string` does both.
  - The buffer + `Read` import disappear with the std helper.
-->

---

# 14 · Observations

- What does each `match` actually do with the `Err` branch?
- Is there an operator that means "unwrap or return the error"?
- Does `std::fs` already have a one-shot helper for this?
- After switching, which `use` lines are still needed?


---

# 14 · Possible solution

```rust
use std::fs;
use std::io;

pub fn read_file_contents(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
```

<div class="mt-8 text-base opacity-80">

- `match … Err(e) => return Err(e)` is literally what `?` expands to.
- Even better: `std::fs::read_to_string` opens, reads, and closes the file for us.
- The function is now small enough that it almost stops needing to exist.

</div>
<!--
If someone keeps the manual version, a `?`-only rewrite is also valid:
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
Worth showing as the intermediate step before reaching for `fs::read_to_string`.
-->
