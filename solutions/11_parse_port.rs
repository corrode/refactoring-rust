//! Parse a TCP port string into a `NonZeroU16`.
//!
//! Solution highlights:
//! - Parsing directly as `u16` removes the manual upper-bound check.
//! - `NonZeroU16` makes "port 0" unrepresentable in the success type.
//! - A small `enum` tells the caller *why* parsing failed instead of swallowing it.

use std::num::NonZeroU16;

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidPort {
    Invalid,
    Zero,
}

pub fn parse_port(s: &str) -> Result<NonZeroU16, InvalidPort> {
    let n: u16 = s.parse().map_err(|_| InvalidPort::Invalid)?;
    NonZeroU16::new(n).ok_or(InvalidPort::Zero)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_port() {
        assert_eq!(parse_port("8080"), Ok(NonZeroU16::new(8080).unwrap()));
    }

    #[test]
    fn max_port() {
        assert_eq!(parse_port("65535"), Ok(NonZeroU16::new(65535).unwrap()));
    }

    #[test]
    fn zero_rejected() {
        assert_eq!(parse_port("0"), Err(InvalidPort::Zero));
    }

    #[test]
    fn out_of_range_rejected() {
        assert_eq!(parse_port("65536"), Err(InvalidPort::Invalid));
    }

    #[test]
    fn non_numeric_rejected() {
        assert_eq!(parse_port("nope"), Err(InvalidPort::Invalid));
    }
}
