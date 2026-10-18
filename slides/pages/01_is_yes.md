---
layout: default
---

# 01 · `is_yes`

<div class="opacity-80 mb-4">
Refactor this check. Keep its signature and accepted inputs.
</div>

```rust {all} {maxHeight:'330px'}
pub fn is_yes(s: String) -> bool {
    if s == "y" {
        return true;
    }
    if s == "yes" {
        return true;
    }
    if s == "Y" {
        return true;
    }
    if s == "YES" {
        return true;
    }
    false
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 01_is_yes</code>
</div>

<!--
First exercise: let participants review the function before showing alternatives.
In the book, Run uses the editable implementation and its attached Tests and callers.
Keep the first pass compatible with the existing callers and assertions.
-->

---

# 01 · Review

- What do the four branches have in common?
- Does checking the string require ownership?
- Does the original accept `"Yes"`? What about `" yes "`?
- Where could an allocation happen: here or at the call site?

<div class="mt-10 text-base opacity-80">

Separate three decisions: how to express the check, whether to borrow, and which inputs to accept.

</div>

<!--
The original accepts only y, yes, Y, YES. It rejects all six mixed-case spellings.
Comparisons do not allocate. A caller may allocate or clone to supply a String.
Borrowing is useful, but changes the API. Save it for after the first solution.
-->

---

# 01 · First solution: same contract

```rust
pub fn is_yes(s: String) -> bool {
    matches!(s.as_str(), "y" | "yes" | "Y" | "YES")
}
```

<div class="mt-8 text-base opacity-80">

- Keep `String`: existing callers need no changes.
- `as_str()` borrows the contents without copying or allocating.
- `matches!` returns a boolean; `|` separates alternative patterns.
- Accept the same four spellings. Still reject `"Yes"`.

</div>

<!--
This still consumes the caller's String. Borrowing inside the implementation
is not the same as accepting a borrowed argument at the API boundary.
No benchmark claim: grouping comparisons doesn't establish a speed-up.
-->

---

# 01 · Alternative: borrow, keep exact matches

```rust
pub fn is_yes(s: &str) -> bool {
    matches!(s, "y" | "yes" | "Y" | "YES")
}
```

```rust
let answer = String::from("yes");
assert!(is_yes(&answer));
assert_eq!(answer, "yes"); // The caller keeps its string.
assert!(is_yes("Y"));
```

<div class="mt-6 text-base opacity-80">

Same accepted inputs, different API.
Change owned calls to borrows and update function-pointer types and tests.
A caller that keeps its string no longer needs to clone it for this check.

</div>

---

# 01 · Alternatives: accept mixed case

Compare without a lowercase copy:

```rust
pub fn is_yes(s: &str) -> bool {
    s.eq_ignore_ascii_case("y") || s.eq_ignore_ascii_case("yes")
}
```

Or create a lowercase copy, then match:

```rust
pub fn is_yes(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "y" | "yes")
}
```

<div class="mt-6 text-base opacity-80">

Both now accept `"Yes"` and all other ASCII case variants: **a behavior change**.
Only the second copies and converts the whole input, allocating for nonempty strings.
Neither trims whitespace or performs Unicode case folding.

</div>

<!--
Both also change String to &str. Their own tests use borrowed callers and expect
mixed-case input to succeed. Do not weaken original tests to hide a behavior change.
All versions reject " yes ", "yesterday", and "yés".
-->

---

# 01 · Compare the consequences

<style scoped>
table {
  margin: 1rem 0;
}
th,
td {
  padding: 0.5rem 0.75rem;
}
</style>

<div class="text-sm">

| Variant | Parameter | Accepts `"Yes"`? | Allocates inside? |
| --- | --- | --- | --- |
| Original | `String` | No | No |
| Exact `matches!` | `String` | No | No |
| Borrowed exact | `&str` | No | No |
| ASCII comparison | `&str` | Yes | No |
| ASCII lowercase + match | `&str` | Yes | Yes, for nonempty input |

</div>

<div class="mt-6 text-base opacity-80">

- **Readability:** group exact spellings or name the case rule.
- **API design:** borrowing helps callers, but requires migration.
- **Flexibility:** ignoring case gives up exact case distinctions.
- **Type-safety / errors:** no new invariant; all return a boolean.

</div>

<div class="mt-6 text-base">
Choose the accepted inputs first. Allocation counts aren't benchmark results.
</div>

<!--
The performance column covers internal work. Borrowing may also avoid caller-side
allocations and clones. Readability depends on familiarity with the methods.
For the existing contract, keep exact matches. If ASCII case-insensitivity is the
requirement, prefer eq_ignore_ascii_case over a temporary lowercase copy here.
The book compares all six criteria with the original as its baseline.
-->
