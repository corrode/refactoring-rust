//! Apply a sequence of `Command`s to strings.
//!
//! Solution highlights:
//! - `into_iter()` consumes the input -- no clones.
//! - `match` on the enum is exhaustive: add a `Command` variant and the
//!   compiler complains here.
//! - `"bar".repeat(n)` replaces the inner loop; `String + &str` reuses the
//!   existing buffer.
//! - The pipeline starts with its input, so empty output remains valid data.

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

pub trait PipelineCommand {
    fn run(&self, input: &str) -> String;
}

/// Feed each command's output into the next command.
pub fn apply_pipeline(commands: &[Box<dyn PipelineCommand>], input: &str) -> String {
    let mut current = input.to_string();
    for command in commands {
        current = command.run(&current);
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Append(&'static str);

    impl PipelineCommand for Append {
        fn run(&self, input: &str) -> String {
            input.to_string() + self.0
        }
    }

    struct Uppercase;

    impl PipelineCommand for Uppercase {
        fn run(&self, input: &str) -> String {
            input.to_uppercase()
        }
    }

    struct Erase;

    impl PipelineCommand for Erase {
        fn run(&self, _input: &str) -> String {
            String::new()
        }
    }

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

    #[test]
    fn pipeline_passes_each_output_to_the_next_command() {
        let commands: Vec<Box<dyn PipelineCommand>> =
            vec![Box::new(Append("-next")), Box::new(Uppercase)];

        assert_eq!(apply_pipeline(&commands, "start"), "START-NEXT");
    }

    #[test]
    fn empty_pipeline_returns_the_input() {
        assert_eq!(apply_pipeline(&[], "start"), "start");
    }

    #[test]
    fn empty_command_output_is_still_pipeline_data() {
        let commands: Vec<Box<dyn PipelineCommand>> = vec![Box::new(Erase), Box::new(Append("!"))];

        assert_eq!(apply_pipeline(&commands, "start"), "!");
    }
}
