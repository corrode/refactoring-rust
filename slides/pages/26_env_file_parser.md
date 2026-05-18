---
layout: default
zoom: 0.95
---

# 26 · `env_file_parser`

<div class="opacity-80 mb-2 text-sm">
Parse a <code>.env</code> file into key/value pairs.
</div>

```rust
pub fn parse_env(content: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
        let without_comment = match trimmed.find('#') {
            Some(i) => &trimmed[..i],
            None    => trimmed,
        };
        let parts: Vec<&str> = without_comment.split('=').collect();
        if parts.len() != 2 { continue; }
        let key = parts[0].trim().to_string();
        let mut value = parts[1].trim().to_string();
        if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
            value = value[1..value.len() - 1].to_string();
        }
        result.insert(key, value);
    }
    result
}
```

<div class="absolute top-25 right-12 text-sm opacity-60">
<code>cargo test --example 26_env_file_parser</code>
</div>

<!--
The capstone. Try parsing these lines against the starter:

  WITH_EQUALS=a=b=c            # silently dropped (parts.len() != 2)
  WITH_HASH="not # a comment"  # the `#` stripping eats the rest
  ESCAPED="line one\nline two" # no escape handling
  MULTILINE="first             # quoted value can't span lines
  second"

Discoveries:
  - Five flag variables waiting to be born. The fix is a `State` enum and `match (state, ch)`.
  - `split('=').collect()` and asserting two parts is the wrong split. `split_once('=')` is the right one.
  - Stripping `#` globally and stripping quotes are the same bug class: parsing without a state.
  - Returning `HashMap` with no error type is a different bug. Bad lines need a line number.
-->

---

# 26 · Observations

- What happens to `WITH_EQUALS=a=b=c`? To `WITH_HASH="not # a comment"`?
- The starter strips `#` before it knows whether it's inside quotes. What does that suggest?
- A multi-line quoted value spans line breaks. `lines()` already split them apart. Is `lines()` the right primitive?
- The function returns a `HashMap` no matter what. If line 47 is malformed, how does the caller find out?
- "Inside quotes" / "expecting `=`" / "skipping a comment" are not booleans. What are they?


---
zoom: 1.0
---

# 26 · Possible solution

```rust
enum State {
    LineStart, InComment, ReadingKey(String), AwaitingValue(String),
    ReadingBareValue { key: String, buf: String },
    ReadingQuotedValue { key: String, buf: String },
    Escape { key: String, buf: String },
}
```
