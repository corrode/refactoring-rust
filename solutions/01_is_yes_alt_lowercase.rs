// ANCHOR: listing
pub fn is_yes(s: &str) -> bool {
    matches!(s.to_ascii_lowercase().as_str(), "y" | "yes")
}

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_signature() {
        let check: fn(&str) -> bool = is_yes;
        let s = String::from("yes");
        assert!(check(&s));
        assert_eq!(s, "yes");
        assert!(check(&s[..1]));
    }

    #[test]
    fn accepted_spellings() {
        for s in ["y", "yes", "Y", "YES"] {
            assert!(is_yes(s), "{s:?}");
        }
    }

    #[test]
    fn mixed_case() {
        for s in ["yeS", "yEs", "yES", "Yes", "YeS", "YEs"] {
            assert!(is_yes(s), "{s:?}");
        }
    }

    #[test]
    fn empty_and_whitespace() {
        for s in [
            "", " ", "\t", "\n", " y", "y ", " yes", "yes ", "YES\n", "\tyes",
        ] {
            assert!(!is_yes(s), "{s:?}");
        }
    }

    #[test]
    fn other_words_and_partial_matches() {
        for s in [
            "n",
            "no",
            "true",
            "1",
            "ye",
            "YE",
            "yess",
            "yesplease",
            "yesterday",
            "sayyes",
            "yy",
            "yes\0",
        ] {
            assert!(!is_yes(s), "{s:?}");
        }
    }

    #[test]
    fn non_ascii() {
        for s in [
            "ý",
            "Ý",
            "yés",
            "yeſ",
            "Yeſ",
            "Ｙ",
            "ｙｅｓ",
            "yes🦀",
            "🦀yes",
            "\u{a0}yes",
        ] {
            assert!(!is_yes(s), "{s:?}");
        }
    }
}
// ANCHOR_END: tests
