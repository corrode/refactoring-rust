//! Parse an SRT timestamp like `"00:01:23,456"` into a `Duration`.
//!
//! Solution highlights:
//! - `split_once` returns `Option<(&str, &str)>` (no allocation).
//! - `?` propagates both the format error and the numeric error via `From`.
//! - Every failure mode is now reported to the caller.

use core::time::Duration;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseError {
    BadFormat,
    BadNumber(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::BadNumber(e)
    }
}

pub fn parse_srt_timestamp(s: &str) -> Result<Duration, ParseError> {
    let (h, rest) = s.split_once(':').ok_or(ParseError::BadFormat)?;
    let (m, rest) = rest.split_once(':').ok_or(ParseError::BadFormat)?;
    let (sec, ms) = rest.split_once(',').ok_or(ParseError::BadFormat)?;

    let total_ms = h.parse::<u64>()? * 3_600_000
        + m.parse::<u64>()? * 60_000
        + sec.parse::<u64>()? * 1_000
        + ms.parse::<u64>()?;
    Ok(Duration::from_millis(total_ms))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(parse_srt_timestamp("00:00:00,000").unwrap(), Duration::ZERO);
    }

    #[test]
    fn one_minute_twentythree_seconds() {
        assert_eq!(
            parse_srt_timestamp("00:01:23,456").unwrap(),
            Duration::from_millis(83_456)
        );
    }

    #[test]
    fn wrong_separator_returns_none() {
        assert!(matches!(
            parse_srt_timestamp("00:01:23.456"),
            Err(ParseError::BadFormat)
        ));
    }
}
