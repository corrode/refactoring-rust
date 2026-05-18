---
layout: default
---

# 24 · `handle_response`

<div class="opacity-80 mb-4">
Decide what to do with an HTTP response based on its status code.
</div>

```rust
pub struct Response { pub status: u16, pub body: Vec<u8> }
pub enum Action { Ok(String), Retry, Fail(String) }

pub fn handle_response(resp: Response) -> Action {
    if resp.status >= 100 && resp.status < 200 {
        // ...
    } else if resp.status >= 200 && resp.status < 300 {
        // ...
    }
    // ... (similar arms for 3xx, 4xx, 5xx) ...
    else if resp.status >= 500 && resp.status < 600 {
        return Action::Retry;
    } else {
        return Action::Fail(format!("unknown status {}", resp.status));
    }
}
```

<div class="absolute bottom-6 right-12 text-sm opacity-60">
<code>cargo test --example 24_http_response_router</code>
</div>

<!--
Body trimmed on this slide - the original has explicit 3xx and 4xx arms
(including the 408/429 retry case). They follow the same shape.

Discoveries:
  - Two things are tangled: "what class is this status?" and "what do we do?".
  - Classify first, then act. The classifier is a `match` on ranges.
  - `match status { 100..200 => ..., 200..300 => ..., ... }` is exhaustive.
  - The handler shrinks to one match arm per class.
-->

---

# 24 · Observations

- The function answers two questions at once: "what *kind* of status is this?" and "what do we do?". Could those split?
- Half the lines are <code>status &gt;= X && status &lt; Y</code> ladders - what does Rust offer instead?
- If we named the classes (<code>Informational</code>, <code>Success</code>, …), what would the dispatcher look like?
- 408 and 429 are special inside 4xx. Where do they belong after splitting?


---

# 24 · Solution &mdash; classify

```rust
enum StatusClass { Informational, Success, Redirect, ClientError, ServerError, Unknown }

impl StatusClass {
    fn classify(status: u16) -> Self {
        match status {
            100..=199 => Self::Informational,
            200..=299 => Self::Success,
            300..=399 => Self::Redirect,
            400..=499 => Self::ClientError,
            500..=599 => Self::ServerError,
            _         => Self::Unknown,
        }
    }
}
```

<div class="mt-6 text-base opacity-80">

`StatusClass::classify` is the only place that knows the numeric ranges. The dispatcher on the next slide just matches on the class.

</div>

---
zoom: 0.9
---

# 24 · Solution &mdash; dispatch

```rust
pub fn handle_response(resp: Response) -> Action {
    use StatusClass::*;
    match StatusClass::classify(resp.status) {
        Success => match String::from_utf8(resp.body) {
            Ok(_) if resp.status == 204 => Action::Ok(String::new()),
            Ok(s)  => Action::Ok(s),
            Err(_) => Action::Fail("invalid utf-8".into()),
        },
        ServerError                                     => Action::Retry,
        ClientError if matches!(resp.status, 408 | 429) => Action::Retry,
        ClientError   => Action::Fail(format!("client error {}", resp.status)),
        Informational => Action::Retry,
        Redirect      => Action::Fail(format!("unhandled redirect {}", resp.status)),
        Unknown       => Action::Fail(format!("unknown status {}", resp.status)),
    }
}
```

<div class="mt-6 text-base opacity-80">

- `handle_response` is a flat `match` &mdash; one arm per policy decision.
- Adding a new class or special-case status now changes exactly one place.
- Notice the scoped `use` of the `StatusClass` enum. First you don't pollute the namespace, second the matches become much shorter.

</div>
<!--
This is as much a "split the function" exercise as a typing exercise.
The classifier is reusable; the policy is testable in isolation. Real-world
crates (`http::StatusCode`) already give you `.is_success()`, `.is_server_error()`
etc. - same idea, just upstream.
-->
