//! Sum all `Some` values in the vector. Ignore `None` entries.

pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    for option in options {
        match option {
            Some(value) => sum += value,
            None => {}
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typical() {
        let values = vec![Some(1), None, Some(3), None, Some(5)];
        assert_eq!(sum_options(values), 9);
    }

    #[test]
    fn empty() {
        let values: Vec<Option<i32>> = vec![];
        assert_eq!(sum_options(values), 0);
    }

    #[test]
    fn all_none() {
        let values = vec![None, None, None];
        assert_eq!(sum_options(values), 0);
    }
}
