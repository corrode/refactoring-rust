// ANCHOR: listing
/// Truncate a string to at most `max_bytes` bytes, respecting char boundaries.
///
/// Solution highlights:
/// - `floor_char_boundary` finds the nearest safe cut at or below the limit.
/// - Slicing a `&str` at that boundary preserves valid UTF-8.
/// - The function allocates only the returned prefix.
pub fn truncate(s: &str, max_bytes: usize) -> String {
    s[..s.floor_char_boundary(s.len().min(max_bytes))].to_string()
}

pub fn truncate_chars(s: &str, num_chars: usize) -> String {
    s.chars().take(num_chars).collect()
}

// Alternative interpretation:
// - char-count semantics: `s.chars().take(n).collect()`
// `floor_char_boundary` is stable since Rust 1.91.

// ANCHOR_END: listing

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

    #[test]
    fn backs_up_to_a_character_boundary() {
        assert_eq!(truncate("café", 4), "caf");
    }
}
