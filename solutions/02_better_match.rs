//! Classify a number as "prime", "square", "nothing", or "something else".
//!
//! Solution highlights:
//! - Or-patterns nest inside `Some(_)` since Rust 1.53.
//! - The wildcard `_` still catches every other `Some(n)`.
//! - Same behaviour, half the noise.

pub fn describe(value: Option<u32>) -> &'static str {
    match value {
        Some(2 | 3 | 5 | 7) => "prime",
        Some(0 | 1 | 4 | 9) => "square",
        None => "nothing",
        _ => "something else",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes() {
        for n in [2, 3, 5, 7] {
            assert_eq!(describe(Some(n)), "prime");
        }
    }

    #[test]
    fn squares() {
        for n in [0, 1, 4, 9] {
            assert_eq!(describe(Some(n)), "square");
        }
    }

    #[test]
    fn nothing_and_other() {
        assert_eq!(describe(None), "nothing");
        assert_eq!(describe(Some(11)), "something else");
    }
}
