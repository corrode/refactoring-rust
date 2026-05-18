//! Parse every string in the input as `i32`. Return all parsed
//! integers, or the first error encountered.
//!
//! Solution highlights:
//! - `collect` on an `Iterator<Item = Result<T, E>>` produces
//!   `Result<Vec<T>, E>` and short-circuits on the first `Err`.
//! - The return type drives inference for both `parse` and `collect`.
//! - Borrowed slice means callers don't have to give up ownership.

use std::num::ParseIntError;

pub fn parse_values(values: &[String]) -> Result<Vec<i32>, ParseIntError> {
    values.iter().map(|v| v.parse()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_valid() {
        let values = vec!["10".to_string(), "20".to_string(), "30".to_string()];
        assert_eq!(parse_values(&values), Ok(vec![10, 20, 30]));
    }

    #[test]
    fn one_invalid() {
        let values = vec!["10".to_string(), "twenty".to_string(), "30".to_string()];
        assert!(parse_values(&values).is_err());
    }

    #[test]
    fn empty_input() {
        assert_eq!(parse_values(&[]), Ok(vec![]));
    }
}
