//! Truncate a string to at most `max_bytes` bytes.

pub fn truncate(s: &str, max_bytes: usize) -> String {
    let mut owned = s.to_string();
    owned.truncate(max_bytes);
    owned
}

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
        assert_eq!(truncate("café", 4), "caf");
    }
}
