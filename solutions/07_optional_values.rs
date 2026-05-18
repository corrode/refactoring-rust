//! Sum all `Some` values in the vector. Ignore `None` entries.
//!
//! Solution highlights:
//! - `Option<T>` is an iterator of 0 or 1 element; `flatten` drops the empties.
//! - `sum` infers its result type from the return type.
//! - No mutable state, no `match`.

pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
    options.into_iter().flatten().sum()
}

// Equivalent:
// pub fn sum_options(options: Vec<Option<i32>>) -> i32 {
//     options.into_iter().filter_map(|x| x).sum()
// }

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
