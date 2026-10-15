//! Parse a `.env` file into key/value pairs, with real error reporting.
//!
//! Solution highlights:
//! - A character-level state machine drives the parse; every byte is handled
//!   in exactly one place, so quoting rules and comment-stripping can't drift.
//! - Quoted values preserve `#` and `=` verbatim; bare values stop at `#`
//!   and trim their trailing whitespace.
//! - Errors carry the line number, so callers can locate bad input.
//!
//! State diagram (transitions on the next character):
//!
//!     LineStart ──┬─ ws ──→ LineStart
//!                 ├─ '#' ──→ InComment
//!                 ├─ '\n' ──→ LineStart (line++)
//!                 ├─ '='  ──→ ERROR EmptyKey
//!                 └─ any  ──→ InKey
//!
//!     InKey     ──┬─ '='  ──→ BeforeValue
//!                 ├─ '\n' ──→ ERROR MissingEquals
//!                 ├─ ws   ──→ AfterKey
//!                 └─ any  ──→ InKey
//!
//!     AfterKey  ──┬─ '='  ──→ BeforeValue
//!                 ├─ ws   ──→ AfterKey
//!                 └─ else ──→ ERROR MissingEquals
//!
//!     BeforeValue ─┬─ '"'  ──→ InQuotedValue
//!                  ├─ '#' / '\n' / EOF ──→ commit empty, …
//!                  ├─ ws   ──→ BeforeValue
//!                  └─ any  ──→ InBareValue
//!
//!     InBareValue ─┬─ '\n' / '#' / EOF ──→ commit (trim_end)
//!                  └─ any  ──→ InBareValue
//!
//!     InQuotedValue ┬─ '"'  ──→ AfterQuotedValue
//!                   ├─ '\\' ──→ InQuotedEscape
//!                   ├─ EOF  ──→ ERROR UnterminatedQuote
//!                   └─ any  ──→ InQuotedValue
//!
//!     InQuotedEscape ─ any ──→ InQuotedValue (decode \n \t \r else literal)
//!
//!     AfterQuotedValue ┬─ '\n' / '#' / EOF ──→ commit
//!                      ├─ ws   ──→ AfterQuotedValue
//!                      └─ else ──→ ERROR TrailingGarbage
//!
//!     InComment ──┬─ '\n' ──→ LineStart
//!                 └─ any  ──→ InComment

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub kind: ErrorKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorKind {
    MissingEquals,
    EmptyKey,
    UnterminatedQuote,
    TrailingGarbage,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum State {
    LineStart,
    InKey,
    AfterKey,
    BeforeValue,
    InBareValue,
    InQuotedValue,
    InQuotedEscape,
    AfterQuotedValue,
    InComment,
}

pub fn parse_env(content: &str) -> Result<HashMap<String, String>, ParseError> {
    let mut out = HashMap::new();
    let mut state = State::LineStart;
    let mut key = String::new();
    let mut value = String::new();
    let mut quoted = false;
    let mut line: usize = 1;

    for c in content.chars() {
        state = step(
            state,
            c,
            &mut key,
            &mut value,
            &mut quoted,
            &mut line,
            &mut out,
        )?;
    }

    // Handle whatever state we end up in at end-of-input.
    match state {
        State::LineStart | State::InComment => {}
        State::InKey | State::AfterKey => {
            return Err(ParseError {
                line,
                kind: ErrorKind::MissingEquals,
            });
        }
        State::BeforeValue | State::InBareValue | State::AfterQuotedValue => {
            commit(&mut out, &mut key, &mut value, quoted);
        }
        State::InQuotedValue | State::InQuotedEscape => {
            return Err(ParseError {
                line,
                kind: ErrorKind::UnterminatedQuote,
            });
        }
    }

    Ok(out)
}

fn step(
    state: State,
    c: char,
    key: &mut String,
    value: &mut String,
    quoted: &mut bool,
    line: &mut usize,
    out: &mut HashMap<String, String>,
) -> Result<State, ParseError> {
    let lineno = *line;
    let next = match (state, c) {
        // --- LineStart: skipping leading whitespace / blank lines.
        (State::LineStart, '\n') => {
            *line += 1;
            State::LineStart
        }
        (State::LineStart, '#') => State::InComment,
        (State::LineStart, c) if c.is_whitespace() => State::LineStart,
        (State::LineStart, '=') => {
            return Err(ParseError {
                line: lineno,
                kind: ErrorKind::EmptyKey,
            });
        }
        (State::LineStart, c) => {
            key.push(c);
            State::InKey
        }

        // --- InKey: accumulating key characters.
        (State::InKey, '=') => {
            *quoted = false;
            State::BeforeValue
        }
        (State::InKey, '\n') => {
            return Err(ParseError {
                line: lineno,
                kind: ErrorKind::MissingEquals,
            });
        }
        (State::InKey, c) if c.is_whitespace() => State::AfterKey,
        (State::InKey, c) => {
            key.push(c);
            State::InKey
        }

        // --- AfterKey: saw whitespace after the key, still waiting for '='.
        (State::AfterKey, '=') => {
            *quoted = false;
            State::BeforeValue
        }
        (State::AfterKey, '\n') => {
            return Err(ParseError {
                line: lineno,
                kind: ErrorKind::MissingEquals,
            });
        }
        (State::AfterKey, c) if c.is_whitespace() => State::AfterKey,
        (State::AfterKey, _) => {
            return Err(ParseError {
                line: lineno,
                kind: ErrorKind::MissingEquals,
            });
        }

        // --- BeforeValue: just saw '=', skipping leading whitespace.
        (State::BeforeValue, '"') => {
            *quoted = true;
            State::InQuotedValue
        }
        (State::BeforeValue, '\n') => {
            commit(out, key, value, *quoted);
            *line += 1;
            State::LineStart
        }
        (State::BeforeValue, '#') => {
            commit(out, key, value, *quoted);
            State::InComment
        }
        (State::BeforeValue, c) if c.is_whitespace() => State::BeforeValue,
        (State::BeforeValue, c) => {
            value.push(c);
            State::InBareValue
        }

        // --- InBareValue: unquoted value chars; '#' starts an inline comment.
        (State::InBareValue, '\n') => {
            commit(out, key, value, *quoted);
            *line += 1;
            State::LineStart
        }
        (State::InBareValue, '#') => {
            commit(out, key, value, *quoted);
            State::InComment
        }
        (State::InBareValue, c) => {
            value.push(c);
            State::InBareValue
        }

        // --- InQuotedValue: read until the matching '"'.
        (State::InQuotedValue, '"') => State::AfterQuotedValue,
        (State::InQuotedValue, '\\') => State::InQuotedEscape,
        (State::InQuotedValue, c) => {
            value.push(c);
            State::InQuotedValue
        }

        // --- InQuotedEscape: decode the next character (\n \t \r else literal).
        (State::InQuotedEscape, c) => {
            value.push(match c {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                other => other,
            });
            State::InQuotedValue
        }

        // --- AfterQuotedValue: only whitespace, '#', or newline are allowed.
        (State::AfterQuotedValue, '\n') => {
            commit(out, key, value, *quoted);
            *line += 1;
            State::LineStart
        }
        (State::AfterQuotedValue, '#') => {
            commit(out, key, value, *quoted);
            State::InComment
        }
        (State::AfterQuotedValue, c) if c.is_whitespace() => State::AfterQuotedValue,
        (State::AfterQuotedValue, _) => {
            return Err(ParseError {
                line: lineno,
                kind: ErrorKind::TrailingGarbage,
            });
        }

        // --- InComment: skip everything until end-of-line.
        (State::InComment, '\n') => {
            *line += 1;
            State::LineStart
        }
        (State::InComment, _) => State::InComment,
    };
    Ok(next)
}

fn commit(out: &mut HashMap<String, String>, key: &mut String, value: &mut String, quoted: bool) {
    let k = std::mem::take(key);
    let v = if quoted {
        std::mem::take(value)
    } else {
        let v = value.trim_end().to_string();
        value.clear();
        v
    };
    out.insert(k, v);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> HashMap<String, String> {
        parse_env(input).unwrap()
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

    // New tests enabled by the refactor.

    #[test]
    fn value_may_contain_equals() {
        let env = parse("WITH_EQUALS=a=b=c");
        assert_eq!(env.get("WITH_EQUALS").map(String::as_str), Some("a=b=c"));
    }

    #[test]
    fn hash_inside_quotes_is_preserved() {
        let env = parse(r#"WITH_HASH="not # a comment""#);
        assert_eq!(
            env.get("WITH_HASH").map(String::as_str),
            Some("not # a comment")
        );
    }

    #[test]
    fn unterminated_quote_is_an_error() {
        let err = parse_env(r#"BAD="oops"#).unwrap_err();
        assert_eq!(err.kind, ErrorKind::UnterminatedQuote);
        assert_eq!(err.line, 1);
    }

    #[test]
    fn missing_equals_is_an_error() {
        let err = parse_env("JUSTAKEY").unwrap_err();
        assert_eq!(err.kind, ErrorKind::MissingEquals);
    }

    // Extra tests exercising the state machine more thoroughly.

    #[test]
    fn escape_sequences_in_quoted_value() {
        let env = parse(r#"MSG="line1\nline2\t!""#);
        assert_eq!(env.get("MSG").map(String::as_str), Some("line1\nline2\t!"));
    }

    #[test]
    fn inline_comment_after_bare_value() {
        let env = parse("KEY=value   # trailing comment\n");
        assert_eq!(env.get("KEY").map(String::as_str), Some("value"));
    }

    #[test]
    fn garbage_after_closing_quote_is_an_error() {
        let err = parse_env(r#"K="v" junk"#).unwrap_err();
        assert_eq!(err.kind, ErrorKind::TrailingGarbage);
    }

    #[test]
    fn empty_key_is_an_error() {
        let err = parse_env("=novalue").unwrap_err();
        assert_eq!(err.kind, ErrorKind::EmptyKey);
    }

    #[test]
    fn line_number_tracks_across_lines() {
        let err = parse_env("OK=1\n\nJUSTAKEY").unwrap_err();
        assert_eq!(err.kind, ErrorKind::MissingEquals);
        assert_eq!(err.line, 3);
    }
}
