//! Given a string of space-separated integers, return the highest
//! and lowest value.
//!
//! Solution highlights:
//! - `split_whitespace` handles tabs and runs of spaces.
//! - Pull the first number to seed both `max` and `min`, then `fold` the rest.
//! - `i64::max` / `i64::min` replace the two `if` statements.

pub fn highest_and_lowest(input: &str) -> (i64, i64) {
    let mut nums = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let first = nums.next().expect("at least one number");
    nums.fold((first, first), |(max, min), n| (max.max(n), min.min(n)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typical() {
        assert_eq!(
            highest_and_lowest("123 3534 534 0 100000 44 222"),
            (100_000, 0)
        );
    }

    #[test]
    fn single_number() {
        assert_eq!(highest_and_lowest("42"), (42, 42));
    }

    #[test]
    fn negatives() {
        assert_eq!(highest_and_lowest("-1 -2 -3"), (-1, -3));
    }
}
