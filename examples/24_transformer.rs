//! Apply a sequence of `Command`s to strings.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

pub fn apply_commands(input: Vec<(String, Command)>) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..input.len() {
        let item = input[i].clone();
        let s = item.0;
        let c = item.1;

        if let Command::Uppercase = c {
            result.push(s.to_uppercase());
        } else if let Command::Trim = c {
            result.push(s.trim().to_string());
        } else if let Command::Append(n) = c {
            let mut temp_string = s;
            for _ in 0..n {
                temp_string.push_str("bar");
            }
            result.push(temp_string);
        }
    }
    result
}

pub trait PipelineCommand {
    fn run(&self, input: &str) -> String;
}

/// Feed each command's output into the next command.
pub fn apply_pipeline(commands: &[Box<dyn PipelineCommand>], input: &str) -> String {
    if commands.is_empty() {
        return input.to_string();
    }

    let mut result = String::new();
    for command in commands {
        result = command.run(if result.is_empty() {
            input
        } else {
            result.as_str()
        });
    }
    result
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
}
