//! Collapse consecutive duplicate values, keeping one per run.
//!
//! Solution highlights:
//! - `Vec::dedup` removes consecutive equal elements in place.
//! - It touches only *adjacent* duplicates, which is exactly the spec:
//!   values separated by a different element are left alone.
//! - For a lazy, iterator-based equivalent, reach for `Itertools::dedup`.

pub fn dedup_runs(mut values: Vec<i32>) -> Vec<i32> {
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_runs_but_keeps_separated_repeats() {
        assert_eq!(
            dedup_runs(vec![1, 2, 4, 5, 5, 5, 5, 25, 5]),
            vec![1, 2, 4, 5, 25, 5]
        );
    }

    #[test]
    fn empty() {
        assert_eq!(dedup_runs(vec![]), Vec::<i32>::new());
    }

    #[test]
    fn no_duplicates_unchanged() {
        assert_eq!(dedup_runs(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn all_the_same_collapses_to_one() {
        assert_eq!(dedup_runs(vec![7, 7, 7, 7]), vec![7]);
    }
}
