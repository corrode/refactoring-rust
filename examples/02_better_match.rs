//! Classify a number as "prime", "square", "nothing", or "something else".

pub fn describe(value: Option<u32>) -> &'static str {
    match value {
        Some(2) | Some(3) | Some(5) | Some(7) => "prime",
        Some(0) | Some(1) | Some(4) | Some(9) => "square",
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
