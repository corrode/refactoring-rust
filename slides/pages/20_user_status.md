---
layout: default
---

# 20 · `can_access`

<div class="opacity-80 mb-4">
Decide whether a user can access a given path.
</div>

```rust
pub fn can_access(path: &str, is_authenticated: bool, is_admin: bool) -> bool {
    if path.starts_with("/public") {
        return true;
    }
    if is_admin {
        if is_authenticated {
            return true;
        } else {
            return false;
        }
    }
    if path.starts_with("/admin") {
        return false;
    }
    if is_authenticated {
        return true;
    }
    false
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 20_user_status</code>
</div>

<!--
Discoveries:
  - Two booleans = four states, but `(is_authenticated=false, is_admin=true)` is nonsense.
  - The type system should make that state unrepresentable.
  - Once we have an enum, the whole thing becomes one `match`.
  - Easier to test, easier to extend.
-->

---

# 20 · Observations

- Two booleans give four combinations. How many are *meaningful*?
- What does <code>(is_authenticated=false, is_admin=true)</code> even mean?
- Could a single enum encode "Anonymous / Authenticated / Admin" instead?
- After that, what does the control flow look like?

<div class="mt-12 opacity-70">
Hint: make illegal states unrepresentable.
</div>

---

# 20 · Possible solution

```rust
pub enum UserStatus { Anonymous, Authenticated, Admin }

pub fn can_access(path: &str, status: UserStatus) -> bool {
    match (path, status) {
        (p, _)                  if p.starts_with("/public") => true,
        (p, UserStatus::Admin)  if p.starts_with("/admin")  => true,
        (p, _)                  if p.starts_with("/admin")  => false,
        (_, UserStatus::Anonymous) => false,
        _ => true,
    }
}
```

<div class="mt-4 text-sm opacity-80">

- Two booleans collapse into three real states; the impossible "unauthenticated admin" no longer exists.
- The decision is one flat `match`, read top to bottom.
- Adding a new role is "add a variant, handle it" &mdash; the compiler tells you where.

</div>
<!--
"Parse, don't validate" applies here too: once a caller has a `UserStatus`,
they can't accidentally pass an incoherent pair of booleans. The type carries
the invariant.

Alternative: also classify `path` into a `PathClass` enum (`Public`, `Admin`,
`Other`) and `match` on the pair. Cleaner, but arguably belongs to exercise 25
where routing is the actual subject - keep it `&str` with guards here.
-->
