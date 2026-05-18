//! Does this string start with an uppercase letter?
//!
//! Solution highlights:
//! - Take `&str` (read-only check, no need to own the string).
//! - `chars().next()` returns `Option<char>` -> handles empty for free.
//! - `is_some_and` collapses "exists and matches predicate" into one call.

pub fn starts_with_uppercase(s: &str) -> bool {
    s.chars().next().is_some_and(char::is_uppercase)
}

// Alternative one-liner (also fine):
// pub fn starts_with_uppercase(s: &str) -> bool {
//     s.starts_with(char::is_uppercase)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_first() {
        assert!(starts_with_uppercase("Hello"));
    }

    #[test]
    fn lowercase_first() {
        assert!(!starts_with_uppercase("hello"));
    }

    #[test]
    fn empty_string() {
        assert!(!starts_with_uppercase(""));
    }

    #[test]
    fn unicode_uppercase() {
        assert!(starts_with_uppercase("Über"));
    }
}
