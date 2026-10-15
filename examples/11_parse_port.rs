//! Parse a TCP port string into a `u16`.

pub fn parse_port(s: &str) -> Option<u16> {
    let n: i32 = s.parse().ok()?;
    if n < 1 || n > 65535 {
        return None;
    }
    Some(n as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_port() {
        assert_eq!(parse_port("8080"), Some(8080));
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
    }

    #[test]
    fn non_numeric_rejected() {
        assert_eq!(parse_port("nope"), None);
    }
}
