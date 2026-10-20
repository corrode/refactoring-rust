//! String helpers with the starter's free-function API and Unicode casing.

pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn sparkle(s: &str) -> String {
    format!("✨ {s} ✨")
}

pub fn spongebob_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .fold(String::new(), |mut out, (i, c)| {
            // Extend rather than push: Unicode casing can expand a character.
            if i % 2 == 0 {
                out.extend(c.to_uppercase());
            } else {
                out.extend(c.to_lowercase());
            }
            out
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouts() {
        assert_eq!(shout("hi"), "HI");
        assert_eq!(shout("straße"), "STRASSE");
    }

    #[test]
    fn sparkles() {
        assert_eq!(sparkle("x"), "✨ x ✨");
    }

    #[test]
    fn spongebob_cases() {
        assert_eq!(spongebob_case("spongebob"), "SpOnGeBoB");
    }

    #[test]
    fn composes_inside_out() {
        assert_eq!(sparkle(&shout("  hi  ".trim())), "✨ HI ✨");
    }

    #[test]
    fn empty_strings() {
        assert_eq!(shout(""), "");
        assert_eq!(sparkle(""), "✨  ✨");
        assert_eq!(spongebob_case(""), "");
    }

    #[test]
    fn preserves_unicode_expansions_and_input_character_positions() {
        assert_eq!(spongebob_case("éÉ"), "Éé");
        assert_eq!(spongebob_case("ßİx"), "SSi\u{307}X");
        assert_eq!(spongebob_case("a🦀b"), "A🦀B");
        assert_eq!(spongebob_case("a\u{301}b"), "A\u{301}B");
        assert_eq!(spongebob_case("a b!c"), "A B!C");
    }
}
