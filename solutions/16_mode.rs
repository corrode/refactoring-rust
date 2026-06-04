//! Find the value that occurs most often in the input.
//!
//! Solution highlights:
//! - `HashMap::entry(...).or_insert(0)` is the canonical counting idiom.
//! - `max_by_key` replaces the hand-rolled tracking - no temporaries, no double seeding.
//! - The empty-input case is now a single, honest `expect`.

use std::collections::HashMap;

pub fn most_common(numbers: &[i32]) -> i32 {
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for &x in numbers {
        *occurrences.entry(x).or_insert(0) += 1;
    }
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .expect("empty input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_mode() {
        assert_eq!(most_common(&[4, 23, 10, -1, 10, 10, 4]), 10);
    }

    #[test]
    fn single_element() {
        assert_eq!(most_common(&[7]), 7);
    }

    #[test]
    fn all_distinct_returns_some_element() {
        let input = [1, 2, 3, 4];
        assert!(input.contains(&most_common(&input)));
    }
}
