---
layout: default
zoom: 0.95
---

# 32 · `env_file_parser`

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
<code>cargo test --example 32_env_file_parser</code>
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

# 32 · Review

- What happens to `WITH_EQUALS=a=b=c`? To `WITH_HASH="not # a comment"`?
- The starter strips `#` before it knows whether it's inside quotes. What does that suggest?
- A multi-line quoted value spans line breaks. `lines()` already split them apart. Is `lines()` the right primitive?
- The function returns a `HashMap` no matter what. If line 47 is malformed, how does the caller find out?
- "Inside quotes" / "expecting `=`" / "skipping a comment" are not booleans. What are they?


---
zoom: 1.0
---

# 32 · Possible solution

```rust
use std::collections::HashMap;

pub fn parse_env(content: &str) -> HashMap<String, String> {
    content.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let without_comment = line.split_once('#').map_or(line, |(prefix, _)| prefix);
    let (key, value) = without_comment.split_once('=')?;
    if value.contains('=') { return None; }
    let value = value.trim();
    let value = value.strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value);
    Some((key.trim().to_string(), value.to_string()))
}
```

<div class="mt-4 text-base opacity-80">
Keep the map-returning API and existing rules, including their limitations.
</div>

<!--
This is a refactor, not a grammar fix. Extra equals signs still reject the line;
# still starts a comment inside quotes. Escapes and multiline quoting remain unsupported.
Duplicate keys still use the last value. Empty keys remain accepted.
-->

---
zoom: 1.0
---

# 32 · Alternative: a state machine

```rust
// Excerpt from solutions/32_env_file_parser_alt.rs.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State {
    LineStart,
    InKey,
    AfterKey,
    BeforeValue,
    InBareValue,
    InQuotedValue,
    InQuotedEscape,
    AfterQuotedValue,
    InComment,
}
```

- The alternative returns `Result<HashMap<String, String>, ParseError>`.
- Quoted `#`, extra `=`, escapes, and multiline values get a richer grammar.
- Invalid input becomes an error rather than a silently skipped line.

<!--
The complete implementation is in solutions/32_env_file_parser_alt.rs.
State holds the parser position; separate key and value buffers hold the data.
This changes both the API and accepted input. It is not a drop-in refactor.
-->
