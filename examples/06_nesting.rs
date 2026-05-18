//! If the input is `Some` and not empty after trimming, return the
//! uppercased trimmed text. Otherwise return `None`.

pub fn shout(s: Option<&str>) -> Option<String> {
    if let Some(s) = s {
        let trimmed = s.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_uppercase());
        } else {
            return None;
        }
    } else {
        return None;
    }
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
