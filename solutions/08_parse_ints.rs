// ANCHOR: listing
//! Parse every string in the input as `i32`. Return all parsed
//! integers, or the first error encountered.
//!
//! Solution highlights:
//! - `collect` on an `Iterator<Item = Result<T, E>>` produces
//!   `Result<Vec<T>, E>` and short-circuits on the first `Err`.
//! - The return type drives inference for both `parse` and `collect`.
//! - Keep the owned input so existing callers need no changes.
//! - Validation uses `filter_map(Result::err)` because it must collect every error.

use std::num::ParseIntError;

pub fn parse_values(values: Vec<String>) -> Result<Vec<i32>, ParseIntError> {
    values.into_iter().map(|v| v.parse()).collect()
}

pub trait Validator {
    fn check(&self, input: &str) -> Result<(), String>;
}

/// Run every validator and return all errors in validator order.
pub fn collect_errors(validators: &[&dyn Validator], input: &str) -> Vec<String> {
    validators
        .iter()
        .filter_map(|validator| validator.check(input).err())
        .collect()
}

// ANCHOR_END: listing

// ANCHOR: tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    struct CountedAccept<'a>(&'a Cell<usize>);

    impl Validator for CountedAccept<'_> {
        fn check(&self, _input: &str) -> Result<(), String> {
            self.0.set(self.0.get() + 1);
            Ok(())
        }
    }

    struct Reject(&'static str);

    impl Validator for Reject {
        fn check(&self, _input: &str) -> Result<(), String> {
            Err(self.0.to_string())
        }
    }

    #[test]
    fn original_signatures() {
        let parse: fn(Vec<String>) -> Result<Vec<i32>, ParseIntError> = parse_values;
        let validate: fn(&[&dyn Validator], &str) -> Vec<String> = collect_errors;
        assert_eq!(parse(vec!["42".to_string()]), Ok(vec![42]));
        assert!(validate(&[], "input").is_empty());
    }

    #[test]
    fn first_error_wins() {
        let values = vec![
            "1".to_string(),
            "nope".to_string(),
            "2147483648".to_string(),
        ];
        let error = parse_values(values).unwrap_err();
        assert_eq!(error.kind(), &std::num::IntErrorKind::InvalidDigit);
    }

    #[test]
    fn all_valid() {
        let values = vec!["10".to_string(), "20".to_string(), "30".to_string()];
        assert_eq!(parse_values(values), Ok(vec![10, 20, 30]));
    }

    #[test]
    fn one_invalid() {
        let values = vec!["10".to_string(), "twenty".to_string(), "30".to_string()];
        assert!(parse_values(values).is_err());
    }

    #[test]
    fn empty_input() {
        assert_eq!(parse_values(vec![]), Ok(vec![]));
    }

    #[test]
    fn validation_collects_every_error_in_order() {
        let first = Reject("first error");
        let calls = Cell::new(0);
        let valid = CountedAccept(&calls);
        let second = Reject("second error");
        let validators: [&dyn Validator; 3] = [&first, &valid, &second];

        assert_eq!(
            collect_errors(&validators, "input"),
            vec!["first error", "second error"]
        );
        assert_eq!(calls.get(), 1);
    }
}
// ANCHOR_END: tests
