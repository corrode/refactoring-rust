//! If the input is `Some` and non-empty after trimming, return it uppercased. Otherwise `None`.
//!
//! Solution highlights:
//! - `map(str::trim)` transforms the inner string.
//! - `filter` turns the `Some("")` case into `None` declaratively.
//! - Final `map(str::to_uppercase)` allocates only when we actually have content.
//! - `Option` supports the same combinators as iterators, because it *is* one.

pub fn shout(s: Option<&str>) -> Option<String> {
    s.map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_uppercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercases_after_trimming() {
        assert_eq!(shout(Some("  hi  ")), Some("HI".to_string()));
    }

    #[test]
    fn none_on_none() {
        assert_eq!(shout(None), None);
    }

    #[test]
    fn none_on_empty() {
        assert_eq!(shout(Some("")), None);
    }

    #[test]
    fn none_on_whitespace_only() {
        assert_eq!(shout(Some("   ")), None);
    }
}
