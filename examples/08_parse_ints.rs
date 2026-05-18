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

#[cfg(test)]
mod tests {
    use super::*;

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
}
