//! A bunch of little string helpers, exposed as an extension trait on `str`.
//!
//! Solution highlights:
//! - An extension trait lets us add methods to a foreign type (`str`) because we own the trait.
//! - Call sites flow in reading order: `s.trim().shout().sparkle()`.
//! - `spongebob_case` becomes a single iterator pipeline — no manual buffer.

pub trait FunStr {
    fn shout(&self) -> String;
    fn sparkle(&self) -> String;
    fn spongebob_case(&self) -> String;
}

impl FunStr for str {
    fn shout(&self) -> String {
        self.to_uppercase()
    }

    fn sparkle(&self) -> String {
        format!("✨ {} ✨", self)
    }

    fn spongebob_case(&self) -> String {
        self.chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouts() {
        assert_eq!("hi".shout(), "HI");
    }

    #[test]
    fn sparkles() {
        assert_eq!("x".sparkle(), "✨ x ✨");
    }

    #[test]
    fn spongebob_cases() {
        assert_eq!("spongebob".spongebob_case(), "SpOnGeBoB");
    }

    #[test]
    fn composes_inside_out() {
        assert_eq!("  hi  ".trim().shout().sparkle(), "✨ HI ✨");
    }
}
