//! Apply a sequence of `Command`s to strings.
//!
//! Solution highlights:
//! - `into_iter()` consumes the input -- no clones.
//! - `match` on the enum is exhaustive: add a `Command` variant and the
//!   compiler complains here.
//! - `"bar".repeat(n)` replaces the inner loop; `String + &str` reuses the
//!   existing buffer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub fn apply_commands(input: Vec<(String, Command)>) -> Vec<String> {
    input
        .into_iter()
        .map(|(s, cmd)| match cmd {
            Command::Uppercase => s.to_uppercase(),
            Command::Trim => s.trim().to_string(),
            Command::Append(n) => s + &"bar".repeat(n),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<(String, Command)> {
        vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ]
    }

    #[test]
    fn applies_each_command() {
        let expected = vec![
            "HELLO",
            "all roads lead to rome!",
            "foobar",
            "barbarbarbarbarbar",
        ];
        assert_eq!(apply_commands(sample()), expected);
    }

    #[test]
    fn empty_input_yields_empty_output() {
        assert!(apply_commands(vec![]).is_empty());
    }

    #[test]
    fn append_zero_is_a_noop() {
        let out = apply_commands(vec![("x".into(), Command::Append(0))]);
        assert_eq!(out, vec!["x".to_string()]);
    }
}
