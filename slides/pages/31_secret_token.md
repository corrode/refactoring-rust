---
layout: default
zoom: 0.9
---

# 31 · `secret_token`

```rust
#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub token: String,
}

impl Credentials {
    pub fn new(username: String, token: String) -> Self {
        Self { username, token }
    }
}
```

- Review diagnostic output and caller access. API changes are allowed.
- One test intentionally fails. Fix the code, not the expectation.

<div class="absolute bottom-5 left-12 right-12 text-xs opacity-70">
Instructor-created example inspired by <a href="https://github.com/iqlusioninc/crates/tree/main/secrecy">secrecy</a>, not historical upstream code or an upstream vulnerability.
</div>

<!--
Full starter: examples/31_secret_token.rs; run cargo test --example 31_secret_token.
Keep the username visible and preserve deliberate access to the original token.
The existing secrecy design already redacts Debug. Do not present this starter
as the before-version of PR #1140.
-->

---

# 31 · Review

- What does `format!("{credentials:?}")` expose? What about `{credentials:#?}`?
- Would private fields change derived `Debug` output?
- Where should redaction live so it works inside other structs?
- Which traits would make secret access implicit again?
- How do you test a token equal to the username or redaction marker?

<!--
The debug_does_not_leak_token test fails because diagnostic output exposes the token.
Preserve that test's requirement when adapting callers to the solution API.
Privacy does not change formatting. A custom Debug on the token composes through
Credentials and enclosing structs. Exact field output avoids false failures
from searching for an empty secret, the visible username, or the marker itself.
-->

---
zoom: 0.8
---

# 31 · Possible solution: `Token`

```rust
pub struct Token(String);

impl Token {
    pub fn new(token: String) -> Self { Self(token) }
    pub fn expose(&self) -> &str { &self.0 }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub token: Token,
}
```

- Construct with `Credentials::new(username, Token::new(token_string))`.
- No `Deref`, `Display`, `Clone`, or serialization by default.
- Toy scope: accidental `Debug` leakage. No zeroization or general security guarantee.
- For production secret handling, evaluate `secrecy`.

<div class="absolute bottom-4 left-12 right-12 text-xs opacity-70">
Design reference: <a href="https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/src/lib.rs#L141-L162">existing redacted Debug and explicit access</a>.
<a href="https://github.com/iqlusioninc/crates/pull/1140">PR #1140</a> restructures SecretBox; it is not an insecure-derived-Debug fix.
</div>

<!--
Full solution and constructor: solutions/31_secret_token.rs.
Run cargo test --example 31_secret_token_solution.
The constructor now accepts Token instead of String. Callers read through
credentials.token.expose(); this is deliberate access, not prevention of access.
Both normal and pretty formatting redact, including nested Credentials.
Tests cover empty and multiline Unicode values and username/marker collisions.
Private storage restricts code outside the defining module, not its descendants.
No runtime formatting test proves a missing trait or external field privacy.
The toy does not erase memory, validate tokens, or stop callers from logging,
copying, or serializing the result of expose(). Username visibility is deliberate.
-->
