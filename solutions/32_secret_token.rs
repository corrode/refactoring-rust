// ANCHOR: listing
//! Limit accidental Debug leakage with a small, explicit token API.
//!
//! Teaching example only: no zeroization or general security guarantee.
//! For production secret handling, evaluate the secrecy crate.

use std::fmt;

pub struct Token(String);

impl Token {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub token: Token,
}

impl Credentials {
    pub fn new(username: String, token: Token) -> Self {
        Self { username, token }
    }
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    const SECRETS: &[&str] = &["demo-token", "", "line one\n秘密🔑", "alice", "[REDACTED]"];

    #[test]
    fn token_redacts_normal_and_pretty_debug() {
        for &secret in SECRETS {
            let token = Token::new(secret.into());
            assert_eq!(format!("{token:?}"), "[REDACTED]");
            assert_eq!(format!("{token:#?}"), "[REDACTED]");
            assert_eq!(token.expose(), secret);
        }
    }

    #[test]
    fn credentials_keep_username_visible_and_token_explicit() {
        for &secret in SECRETS {
            let credentials = Credentials::new("alice".into(), Token::new(secret.into()));
            assert_eq!(credentials.username, "alice");
            assert_eq!(credentials.token.expose(), secret);
            assert_eq!(
                format!("{credentials:?}"),
                "Credentials { username: \"alice\", token: [REDACTED] }"
            );
            assert_eq!(
                format!("{credentials:#?}"),
                "Credentials {\n    username: \"alice\",\n    token: [REDACTED],\n}"
            );
        }
    }

    #[test]
    fn redaction_composes_with_another_derived_debug() {
        #[derive(Debug)]
        struct Request {
            credentials: Credentials,
        }

        for &secret in SECRETS {
            let request = Request {
                credentials: Credentials::new("alice".into(), Token::new(secret.into())),
            };
            assert_eq!(request.credentials.token.expose(), secret);
            // Exact field output avoids false failures for empty tokens or
            // tokens equal to the visible username or redaction marker.
            assert_eq!(
                format!("{request:?}"),
                "Request { credentials: Credentials { username: \"alice\", token: [REDACTED] } }"
            );
            assert_eq!(
                format!("{request:#?}"),
                "Request {\n    credentials: Credentials {\n        username: \"alice\",\n        token: [REDACTED],\n    },\n}"
            );
        }
    }
}
// ANCHOR_END: tests

// Provenance: instructor-created teaching implementation inspired by secrecy
// [1], whose existing Debug implementation redacts and whose ExposeSecret
// trait makes access explicit [3]. This toy omits its memory-handling features.
// PR #1140 [2] restructured SecretBox, not an insecure derived-Debug starter.
// Upstream MIT notice: ../THIRD_PARTY_NOTICES.md (secrecy).
// [1]: https://github.com/iqlusioninc/crates/tree/main/secrecy
// [2]: https://github.com/iqlusioninc/crates/pull/1140
// [3]: https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/src/lib.rs#L141-L162
