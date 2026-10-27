use core::time::Duration;

/// Parse an SRT timestamp like `"00:01:23,456"` into a `Duration`.
///
/// Return `None` for malformed numbers or a total that overflows `u64` milliseconds.
/// Like the starter, accept variable field widths and out-of-range components.
pub fn parse_srt_timestamp(s: &str) -> Option<Duration> {
    let (hours, rest) = s.split_once(':')?;
    let (minutes, rest) = rest.split_once(':')?;
    let (seconds, millis) = rest.split_once(',')?;

    let hours = hours.parse::<u64>().ok()?;
    let minutes = minutes.parse::<u64>().ok()?;
    let seconds = seconds.parse::<u64>().ok()?;
    let millis = millis.parse::<u64>().ok()?;

    let total_ms = hours
        .checked_mul(3_600_000)?
        .checked_add(minutes.checked_mul(60_000)?)?
        .checked_add(seconds.checked_mul(1_000)?)?
        .checked_add(millis)?;
    Some(Duration::from_millis(total_ms))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(parse_srt_timestamp("00:00:00,000"), Some(Duration::ZERO));
    }

    #[test]
    fn one_minute_twentythree_seconds() {
        assert_eq!(
            parse_srt_timestamp("00:01:23,456"),
            Some(Duration::from_millis(83_456))
        );
    }

    #[test]
    fn wrong_separator_returns_none() {
        assert_eq!(parse_srt_timestamp("00:01:23.456"), None);
    }

    #[test]
    fn malformed_input_returns_none() {
        for input in [
            "",
            "00:00",
            "00:00:00",
            "00:00:00,000:00",
            "00:00:00,000,000",
            "x:00:00,000",
            "00:x:00,000",
            "00:00:x,000",
            "00:00:00,x",
            ":00:00,000",
            "00::00,000",
            "00:00:,000",
            "00:00:00,",
            "-1:00:00,000",
            " 00:00:00,000",
            "00:00:00,18446744073709551616",
        ] {
            assert_eq!(parse_srt_timestamp(input), None, "{input}");
        }
    }

    #[test]
    fn preserves_permissive_component_parsing() {
        assert_eq!(
            parse_srt_timestamp("1:2:3,4"),
            Some(Duration::from_millis(3_723_004))
        );
        assert_eq!(
            parse_srt_timestamp("0:60:60,1000"),
            Some(Duration::from_secs(3_661))
        );
        assert_eq!(
            parse_srt_timestamp("+0:+0:+0,+1"),
            Some(Duration::from_millis(1))
        );
    }

    #[test]
    fn checks_multiplication_and_addition_overflow() {
        for input in [
            "18446744073709551615:0:0,0",
            "0:18446744073709551615:0,0",
            "0:0:18446744073709551615,0",
            "0:0:1,18446744073709551615",
        ] {
            assert_eq!(parse_srt_timestamp(input), None, "{input}");
        }
        assert_eq!(
            parse_srt_timestamp("0:0:0,18446744073709551615"),
            Some(Duration::from_millis(u64::MAX))
        );
    }
}
