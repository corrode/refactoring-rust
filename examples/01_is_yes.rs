// ANCHOR: listing
pub fn is_yes(s: String) -> bool {
    if s == "y" {
        return true;
    }
    if s == "yes" {
        return true;
    }
    if s == "Y" {
        return true;
    }
    if s == "YES" {
        return true;
    }
    false
}

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_signature() {
        let check: fn(String) -> bool = is_yes;
        assert!(check(String::from("yes")));
    }

    #[test]
    fn accepted_spellings() {
        for s in ["y", "yes", "Y", "YES"] {
            assert!(is_yes(s.to_string()), "{s:?}");
        }
    }

    #[test]
    fn mixed_case() {
        for s in ["yeS", "yEs", "yES", "Yes", "YeS", "YEs"] {
            assert!(!is_yes(s.to_string()), "{s:?}");
        }
    }

    #[test]
    fn empty_and_whitespace() {
        for s in [
            "", " ", "\t", "\n", " y", "y ", " yes", "yes ", "YES\n", "\tyes",
        ] {
            assert!(!is_yes(s.to_string()), "{s:?}");
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
            assert!(!is_yes(s.to_string()), "{s:?}");
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
            assert!(!is_yes(s.to_string()), "{s:?}");
        }
    }
}
// ANCHOR_END: tests
