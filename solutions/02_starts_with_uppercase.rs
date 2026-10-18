// ANCHOR: listing
/// Does this string start with an uppercase letter?
pub fn starts_with_uppercase(s: String) -> bool {
    s.chars().next().is_some_and(char::is_uppercase)
}

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_signature() {
        let check: fn(String) -> bool = starts_with_uppercase;
        assert!(check(String::from("Hello")));
    }

    #[test]
    fn uppercase_first() {
        assert!(starts_with_uppercase("Hello".to_string()));
    }

    #[test]
    fn lowercase_first() {
        assert!(!starts_with_uppercase("hello".to_string()));
    }

    #[test]
    fn empty_string() {
        assert!(!starts_with_uppercase(String::new()));
    }

    #[test]
    fn unicode_uppercase() {
        assert!(starts_with_uppercase("Über".to_string()));
    }

    #[test]
    fn non_letter_first() {
        assert!(!starts_with_uppercase("1Hello".to_string()));
        assert!(!starts_with_uppercase(" Hello".to_string()));
    }
}
// ANCHOR_END: tests
