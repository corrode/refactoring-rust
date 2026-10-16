//! Preserve the starter's line-based parsing rules and map-returning API.
//! See `28_env_file_parser_alt.rs` for a richer grammar and explicit errors.

// ANCHOR: listing
use std::collections::HashMap;

pub fn parse_env(content: &str) -> HashMap<String, String> {
    content.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let without_comment = line.split_once('#').map_or(line, |(prefix, _)| prefix);
    let (key, value) = without_comment.split_once('=')?;
    // The original rejects extra equals signs, even inside quoted values.
    if value.contains('=') {
        return None;
    }
    let value = value.trim();
    let value = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .unwrap_or(value);
    Some((key.trim().to_string(), value.to_string()))
}
// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;

    fn expected(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect()
    }

    #[test]
    fn simple_key_value() {
        assert_eq!(parse_env("KEY=value"), expected(&[("KEY", "value")]));
    }

    #[test]
    fn skips_blank_lines_and_comments() {
        assert_eq!(
            parse_env("\n  \n# comment\n  # ignored=1\nKEY=value\n"),
            expected(&[("KEY", "value")])
        );
        assert!(parse_env("").is_empty());
    }

    #[test]
    fn trims_whitespace_and_handles_crlf_and_unicode() {
        assert_eq!(
            parse_env("  KEY = value  \r\n\u{2003}名前\u{2003}=\u{2003}値\u{2003}\r\n"),
            expected(&[("KEY", "value"), ("名前", "値")])
        );
    }

    #[test]
    fn strips_only_paired_surrounding_double_quotes() {
        assert_eq!(
            parse_env(
                "GREETING=\" hello world \"\nEMPTY=\"\"\nSINGLE=\"\nOPEN=\"oops\nCLOSE=oops\"\nTRAILING=\"v\" junk\nQUOTED='v'"
            ),
            expected(&[
                ("GREETING", " hello world "),
                ("EMPTY", ""),
                ("SINGLE", "\""),
                ("OPEN", "\"oops"),
                ("CLOSE", "oops\""),
                ("TRAILING", "\"v\" junk"),
                ("QUOTED", "'v'"),
            ])
        );
    }

    #[test]
    fn empty_keys_values_and_internal_key_spaces_are_allowed() {
        assert_eq!(
            parse_env("EMPTY=\n =value\nTWO WORDS=yes"),
            expected(&[("EMPTY", ""), ("", "value"), ("TWO WORDS", "yes")])
        );
        assert_eq!(parse_env("="), expected(&[("", "")]));
    }

    #[test]
    fn malformed_lines_are_skipped_without_losing_valid_lines() {
        assert_eq!(
            parse_env("FIRST=1\nJUSTAKEY\nBAD=a=b\nQUOTED=\"a=b\"\n==\nLAST=2"),
            expected(&[("FIRST", "1"), ("LAST", "2")])
        );
    }

    #[test]
    fn comments_are_stripped_before_quotes_and_equals_are_processed() {
        assert_eq!(
            parse_env(
                "KEY=value # comment=ignored\nHASH=\"not # a comment\"\nMISSING#=ignored\nEMPTY=#comment"
            ),
            expected(&[("KEY", "value"), ("HASH", "\"not"), ("EMPTY", "")])
        );
    }

    #[test]
    fn last_accepted_duplicate_wins() {
        assert_eq!(
            parse_env("KEY=first\n KEY = second\nKEY=bad=ignored\nKEY\n=first\n =last"),
            expected(&[("KEY", "second"), ("", "last")])
        );
        assert_eq!(parse_env("KEY=first\nKEY="), expected(&[("KEY", "")]));
    }

    #[test]
    fn escapes_and_multiline_quotes_are_not_interpreted() {
        assert_eq!(
            parse_env("ESCAPES=\"a\\nb\\t\"\nMULTI=\"first\nNEXT=second\""),
            expected(&[
                ("ESCAPES", "a\\nb\\t"),
                ("MULTI", "\"first"),
                ("NEXT", "second\"")
            ])
        );
    }
}
// ANCHOR_END: tests
