//! A bunch of little string helpers. Notice how call sites read
//! inside-out: `sparkle(&shout(s.trim()))`.

pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

pub fn sparkle(s: &str) -> String {
    format!("✨ {} ✨", s)
}

pub fn spongebob_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            out.extend(c.to_uppercase());
        } else {
            out.extend(c.to_lowercase());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouts() {
        assert_eq!(shout("hi"), "HI");
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
}
