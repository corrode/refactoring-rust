// ANCHOR: listing
//! Parse every string in the input as `i32`. Return all parsed
//! integers, or the first error encountered.

use std::num::ParseIntError;

pub fn parse_values(values: Vec<String>) -> Result<Vec<i32>, ParseIntError> {
    let mut result = Vec::new();
    for value in values {
        match value.parse::<i32>() {
            Ok(num) => result.push(num),
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}

pub trait Validator {
    fn check(&self, input: &str) -> Result<(), String>;
}

/// Run every validator and return all errors in validator order.
pub fn collect_errors(validators: &[&dyn Validator], input: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for validator in validators {
        let result = validator.check(input);
        if result.is_err() {
            errors.push(result.err().expect("is_err should always have an error"));
        }
    }
    errors
}

// ANCHOR_END: listing

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
