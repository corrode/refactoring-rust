---
layout: default
zoom: 0.9
---

# 17 · `parse_srt_timestamp`

<div class="opacity-80 mb-4">
Parse an SRT timestamp like <code>"00:01:23,456"</code> into a <code>Duration</code>.
</div>

```rust
use core::time::Duration;

pub fn parse_srt_timestamp(s: &str) -> Option<Duration> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().unwrap();
    let minutes = parts[1].parse::<u64>().unwrap();

    let sec_ms: Vec<&str> = parts[2].split(',').collect();
    if sec_ms.len() != 2 {
        return None;
    }
    let seconds = sec_ms[0].parse::<u64>().unwrap();
    let millis = sec_ms[1].parse::<u64>().unwrap();

    let total_ms = hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis;
    Some(Duration::from_millis(total_ms))
}
```

<div class="absolute top-20 right-12 text-sm opacity-60">
<code>cargo test --example 17_parse_srt_timestamp</code>
</div>

<!--
Discoveries:
  - `.unwrap()` after `parse()` will panic on bad input - `?` is what we want.
  - Allocating a `Vec` to count two pieces is wasteful - `split_once` is the tool.
  - `Option` loses the reason; a small `ParseError` enum is more honest.
  - This screams "I want to be a `FromStr` impl" for a wrapper type.
-->

---

# 17 · Observations

- What happens to <code>"ab:cd:ef,gh"</code> today?
- Do we really need a <code>Vec</code> just to grab "before colon" and "after colon"?
- `Option` says *whether* it failed; would the caller want to know *why*?
- Could this be a <code>FromStr</code> impl on a newtype?


---
zoom: 0.75
---

# 17 · Possible solution

```rust
use core::time::Duration;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
    BadFormat,
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self { ParseError::BadNumber(e) }
}

pub fn parse_srt_timestamp(s: &str) -> Result<Duration, ParseError> {
    let (h, rest) = s.split_once(':').ok_or(ParseError::BadFormat)?;
    let (m, rest) = rest.split_once(':').ok_or(ParseError::BadFormat)?;
    let (sec, ms) = rest.split_once(',').ok_or(ParseError::BadFormat)?;

    let total_ms = h.parse::<u64>()? * 3_600_000
        + m.parse::<u64>()? * 60_000
        + sec.parse::<u64>()? * 1_000
        + ms.parse::<u64>()?;
    Ok(Duration::from_millis(total_ms))
}
```

<div class="mt-8 text-base opacity-80">

- `split_once` returns `Option<(&str, &str)>` (no allocation)
- `?` propagates both the format error and the numeric error via `From`.
- Every failure mode is now reported

</div>
<!--
Natural follow-up: wrap this in `impl FromStr for SrtTimestamp` so callers can
write `"00:01:23,456".parse()`. That's the canonical Rust shape for "string in,
typed value or error out".
-->
