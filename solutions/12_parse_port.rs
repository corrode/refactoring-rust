// ANCHOR: listing
/// Parse a TCP port string into a `u16`.
pub fn parse_port(s: &str) -> Option<u16> {
    s.parse::<u16>().ok().filter(|&port| port != 0)
}

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_signature() {
        let parse: fn(&str) -> Option<u16> = parse_port;
        assert_eq!(parse("8080"), Some(8080));
    }

    #[test]
    fn valid_port() {
        assert_eq!(parse_port("8080"), Some(8080));
    }

    #[test]
    fn min_port() {
        assert_eq!(parse_port("1"), Some(1));
    }

    #[test]
    fn max_port() {
        assert_eq!(parse_port("65535"), Some(65535));
    }

    #[test]
    fn zero_rejected() {
        assert_eq!(parse_port("0"), None);
    }

    #[test]
    fn out_of_range_rejected() {
        assert_eq!(parse_port("65536"), None);
        assert_eq!(parse_port("2147483648"), None);
    }

    #[test]
    fn non_numeric_rejected() {
        assert_eq!(parse_port("nope"), None);
    }

    #[test]
    fn negative_empty_and_whitespace_rejected() {
        for input in ["-1", "-0", "", " 80", "80 "] {
            assert_eq!(parse_port(input), None, "input: {input:?}");
        }
    }

    #[test]
    fn plus_sign_and_leading_zeros_accepted() {
        assert_eq!(parse_port("+80"), Some(80));
        assert_eq!(parse_port("00080"), Some(80));
    }
}
// ANCHOR_END: tests
