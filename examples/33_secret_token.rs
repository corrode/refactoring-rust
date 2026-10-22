// ANCHOR: listing
//! Review how credentials behave when formatted for diagnostics.
//!
//! You may change the API in this domain-modeling exercise. Keep the
//! username visible and preserve deliberate access to the original token.
//! The debug_does_not_leak_token test intentionally fails until you fix the leak.

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
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_preserves_both_strings() {
        for secret in ["demo-token", "", "line one\n秘密🔑", "alice", "[REDACTED]"] {
            let credentials = Credentials::new("alice".into(), secret.into());
            assert_eq!(credentials.username, "alice");
            assert_eq!(credentials.token, secret);
        }
    }

    // Expected to fail until diagnostic output hides the token.
    #[test]
    fn debug_does_not_leak_token() {
        let credentials = Credentials::new("alice".into(), "demo-token".into());
        for output in [format!("{credentials:?}"), format!("{credentials:#?}")] {
            assert!(output.contains("alice"), "keep the username visible");
            assert!(
                !output.contains("demo-token"),
                "Debug output must not contain the token"
            );
        }
    }
}
// ANCHOR_END: tests

// Provenance: instructor-created starter inspired by secrecy's existing
// redaction and explicit-access design [1], not historical upstream code
// or evidence of an upstream derived-Debug vulnerability. PR #1140 [2]
// restructured SecretBox; it is not an insecure-before/secure-after example.
// Upstream MIT notice: ../THIRD_PARTY_NOTICES.md (secrecy).
// [1]: https://github.com/iqlusioninc/crates/tree/main/secrecy
// [2]: https://github.com/iqlusioninc/crates/pull/1140
// [3]: https://github.com/iqlusioninc/crates/blob/f98d4ccf1e81d2fb9daaa7a5458f8064ecb563a2/secrecy/src/lib.rs#L141-L162
