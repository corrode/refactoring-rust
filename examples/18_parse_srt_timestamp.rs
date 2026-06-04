//! Parse an SRT timestamp like `"00:01:23,456"` into a `Duration`.

use core::time::Duration;

pub fn parse_srt_timestamp(s: &str) -> Option<Duration> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().unwrap();
    let minutes = parts[1].parse::<u64>().unwrap();

    let sec_ms: Vec<&str> = parts[2].split(',').collect();
    if sec_ms.len() != 2 {
        return None;
    }
    let seconds = sec_ms[0].parse::<u64>().unwrap();
    let millis = sec_ms[1].parse::<u64>().unwrap();

    let total_ms = hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis;
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
}
