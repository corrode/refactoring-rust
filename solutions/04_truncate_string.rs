//! Truncate a string to at most `max_bytes` bytes, respecting char boundaries.
//!
//! Solution highlights:
//! - `is_char_boundary` lets us back up to the nearest safe cut.
//! - Slicing a `&str` at a boundary is panic-free and allocation-cheap.
//! - No more mid-codepoint crash on `"café"`.

pub fn truncate(s: &str, max_bytes: usize) -> String {
    s[..s.floor_char_boundary(s.len().min(max_bytes))].to_string()
}

pub fn truncate_chars(s: &str, num_chars: usize) -> String {
    s.chars().take(num_chars).collect()
}


// Alternative interpretations:
// - char-count semantics: `s.chars().take(n).collect()`
// - nightly: `s.floor_char_boundary(max_bytes)`

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii() {
        assert_eq!(truncate("hello world", 5), "hello");
    }

    #[test]
    fn unchanged_when_short_enough() {
        assert_eq!(truncate("hi", 100), "hi");
    }

    #[test]
    fn whole_unicode_string_fits() {
        assert_eq!(truncate("café", 5), "café");
    }
}
