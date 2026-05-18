//! Does this string start with an uppercase letter?

pub fn starts_with_uppercase(s: String) -> bool {
    if s.len() == 0 {
        return false;
    }
    let first = s.chars().nth(0).unwrap();
    if first.is_uppercase() == true {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
