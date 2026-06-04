//! Collapse consecutive duplicate values, keeping one per run.
//!
//! `[1, 2, 4, 5, 5, 5, 5, 25, 5]` becomes `[1, 2, 4, 5, 25, 5]`.
//! Only *adjacent* duplicates collapse, so the trailing `5` survives
//! because a `25` sits between it and the earlier run of fives.
//!
//! This works, but it hand-rolls something the standard library
//! already does in one call. Can you make it shorter?

pub fn dedup_runs(values: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < values.len() {
        if result.is_empty() || *result.last().unwrap() != values[i] {
            result.push(values[i]);
        }
        i += 1;
    }
    result
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
