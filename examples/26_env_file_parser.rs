//! Parse a `.env` file into key/value pairs.
//!
//! Supports `KEY=value` lines, blank lines, and `#` comments. Quoted
//! values get their surrounding double quotes stripped.
//!
//! The starter handles the easy lines and silently mangles the rest:
//! values containing `=`, quoted values containing `#`, escape
//! sequences, and multi-line quoted values. There is no error type at
//! all, so callers can't tell a parse failure from a missing key.

use std::collections::HashMap;

pub fn parse_env(content: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('#') {
            continue;
        }
        // Strip inline comments too. Naive, but "good enough".
        let without_comment = match trimmed.find('#') {
            Some(i) => &trimmed[..i],
            None => trimmed,
        };
        let parts: Vec<&str> = without_comment.split('=').collect();
        if parts.len() != 2 {
            continue;
        }
        let key = parts[0].trim().to_string();
        let mut value = parts[1].trim().to_string();
        if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
            value = value[1..value.len() - 1].to_string();
        }
        result.insert(key, value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> HashMap<String, String> {
        parse_env(input)
    }

    #[test]
    fn simple_key_value() {
        let env = parse("KEY=value");
        assert_eq!(env.get("KEY").map(String::as_str), Some("value"));
    }

    #[test]
    fn skips_blank_lines_and_comments() {
        let input = "\
# a comment
\n\
KEY=value
";
        let env = parse(input);
        assert_eq!(env.len(), 1);
        assert_eq!(env.get("KEY").map(String::as_str), Some("value"));
    }

    #[test]
    fn strips_surrounding_double_quotes() {
        let env = parse(r#"GREETING="hello world""#);
        assert_eq!(env.get("GREETING").map(String::as_str), Some("hello world"));
    }

    #[test]
    fn trims_surrounding_whitespace() {
        let env = parse("  KEY = value  ");
        assert_eq!(env.get("KEY").map(String::as_str), Some("value"));
    }

    #[test]
    fn empty_value_is_allowed() {
        let env = parse("EMPTY=");
        assert_eq!(env.get("EMPTY").map(String::as_str), Some(""));
    }
}
