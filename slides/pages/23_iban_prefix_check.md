---
layout: default
zoom: 0.9
---

# 23 · `is_valid_iban`

<div class="opacity-80 mb-4">
Country code (2 letters), check digits (2 digits), then ASCII alphanumerics
up to a country-specific length.
</div>

```rust
pub fn is_valid_iban(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 4 { return false; }

    let country = &s[0..2];

    if !chars[0].is_ascii_alphabetic() || !chars[1].is_ascii_alphabetic() { return false; }
    if !chars[2].is_ascii_digit() || !chars[3].is_ascii_digit() { return false; }
    for i in 4..chars.len() {
        if !chars[i].is_ascii_alphanumeric() { return false; }
    }

    let expected = if country == "DE" { 22 }
        else if country == "GB" { 22 }
        else if country == "FR" { 27 }
        else if country == "CH" { 21 }
        else if country == "NL" { 18 }
        else if country == "AT" { 20 }
        else { return false; };

    chars.len() == expected
}
```

<div class="absolute top-20 right-12 text-sm opacity-60">
<code>cargo test --example 23_iban_prefix_check</code>
</div>

<!--
Discoveries:
  - Allocating a `Vec<char>` just to index by position is wasteful - IBANs are ASCII.
  - `s.as_bytes()` gives O(1) indexing without allocating.
  - That cascade of `if country == "..."` is a `match` with `|` arms.
  - The per-position checks reduce to a single `bytes().all(...)`.
-->

---

# 23 · Review

- IBANs are ASCII-only. Do we need <code>chars()</code> at all?
- Six <code>if/else if</code> arms on a string - what would express that better?
- Can we collapse the position checks into one pass?
- We check the prefix bytes individually *and* then the rest. Is there overlap?


---
zoom: 0.85
---

# 23 · Possible solution

```rust
pub fn is_valid_iban(s: &str) -> bool {
    let expected = match s.get(0..2) {
        Some("DE") | Some("GB") => 22,
        Some("FR") => 27,
        Some("CH") => 21,
        Some("NL") => 18,
        Some("AT") => 20,
        _ => return false,
    };
    if s.len() != expected { return false; }

    let b = s.as_bytes();
    b[0].is_ascii_alphabetic()
        && b[1].is_ascii_alphabetic()
        && b[2].is_ascii_digit()
        && b[3].is_ascii_digit()
        && s.bytes().all(|c| c.is_ascii_alphanumeric())
}
```

<div class="mt-8 text-base opacity-80">

- `match` with `|` arms maps country → length declaratively.
- `as_bytes()` indexing is O(1) and avoids allocating a `Vec<char>`.
- One `bytes().all(...)` replaces the per-index loop; the four explicit checks pin down the prefix shape.

</div>
<!--
A `HashMap<&str, usize>` for the lookup table is also reasonable but heavier
than a `match` for six countries. Real IBAN validation also runs a mod-97
checksum - out of scope here.
-->
