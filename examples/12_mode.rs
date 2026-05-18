//! Find the value that occurs most often in the input.

use std::collections::HashMap;

pub fn most_common(numbers: &[i32]) -> i32 {
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    for &x in numbers {
        *occurrences.entry(x).or_insert(0) += 1;
    }

    let mut best: (i32, i32) = (
        *occurrences.iter().next().expect("empty input").0,
        *occurrences.iter().next().expect("empty input").1,
    );
    for (&value, &count) in &occurrences {
        if count > best.1 {
            best = (value, count);
        }
    }
    best.0
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
        // With no repetition every value is a valid mode; just check the
        // result is one of the inputs.
        let input = [1, 2, 3, 4];
        assert!(input.contains(&most_common(&input)));
    }
}
